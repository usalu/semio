/** ✂️ Rewrites flagged importers to the owning package specifier and adds the workspace dependency to the importer's package. Dry-run by default.
 * Usage: bun rewrite.ts <plan.json> <scan.json> [--area <prefix>]... [--owners value,validator,schema] [--write] [--ignore-skip] [--snapshot <out.json>] [--report <out.json>]
 * `--ignore-skip` is for the owner of a skipped area (renderer, library, stdio artifacts, hub script, ts3 files) who rewrites their own files.
 * Re-reads and re-scans every file right before editing; touches only module-specifier literals and the binding of default-JSON imports.
 */
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { ROOT, scanSource } from "../sdkb-tools/lib.ts";
import { nfc, OWNERS, PKG_NAME, ownedRegion, isTestPath, loadAllPackages, type OwnerKey } from "./model.ts";
import { skipReason } from "./skip.ts";

const args = process.argv.slice(2);
const [planPath, scanPath] = args;
const opt = (n: string) => { const i = args.indexOf(n); return i >= 0 ? args[i + 1] : undefined; };
const multi = (n: string) => args.flatMap((a, i) => (a === n ? [args[i + 1]] : []));
const write = args.includes("--write");
const areas = multi("--area").map(nfc);
const owners = new Set((opt("--owners") ?? "value,validator,schema").split(","));
const snapshotPath = opt("--snapshot"), reportPath = opt("--report");
const BOOTSTRAP = /(^|\/)(?:📜️script\.ts|🏗️builder\/🌐️vite\/🟦️\.ts|🧪️tests\/🎚️config\/🟦️\.ts|(?:⚙️|🧪️)?(?:vite|vitest)\.config\.[cm]?[jt]s)$/u;

const plan = JSON.parse(readFileSync(planPath, "utf8"));
const edges: any[] = JSON.parse(readFileSync(scanPath, "utf8"));
const targetMap: Record<string, { owner: OwnerKey; facade: string; subpath: string; jsonExport?: string }> = plan.targetMap;
const pkgs = loadAllPackages();
const regions = Object.fromEntries((Object.keys(OWNERS) as OwnerKey[]).map(k => [k, ownedRegion(pkgs, OWNERS[k].sourceOwner)])) as Record<OwnerKey, (p: string) => boolean>;

const files = [...new Set<string>(edges.map(e => e.file))].filter(f => !areas.length || areas.some(a => f.startsWith(a))).sort();
const report: any = { files: {}, skippedEdges: [] as any[], unhandled: [] as any[], deps: {} as Record<string, any>, total: 0 };
const snapshot: Record<string, string> = {};
const depNeeds = new Map<string, Map<string, { prod: boolean }>>();

function owningPackage(file: string): { dir: string; name: string } | null {
  let best: { dir: string; name: string; len: number; ts: boolean } | null = null;
  for (const p of pkgs) {
    if (!(file === p.sourceOwner || file.startsWith(p.sourceOwner + "/"))) continue;
    const ts = p.owner.includes("/🟦️typescript");
    if (!best || p.sourceOwner.length > best.len || (p.sourceOwner.length === best.len && ts && !best.ts)) best = { dir: p.owner, name: p.name, len: p.sourceOwner.length, ts };
  }
  if (best) return { dir: best.dir, name: best.name };
  let d = dirname(file);
  while (d && d !== ".") {
    if (existsSync(join(ROOT, d, "package.json"))) { try { return { dir: d, name: JSON.parse(readFileSync(join(ROOT, d, "package.json"), "utf8")).name }; } catch { return null; } }
    d = dirname(d);
  }
  return null;
}

let edits = 0, touched = 0;
for (const f of files) {
  const why = args.includes("--ignore-skip") ? null : skipReason(f);
  let text: string;
  try { text = readFileSync(join(ROOT, f), "utf8"); } catch { continue; }
  const recs = scanSource(f, text).filter(r => r.resolved && targetMap[nfc(r.resolved)] && owners.has(targetMap[nfc(r.resolved)].owner) && !regions[targetMap[nfc(r.resolved)].owner](f));
  if (!recs.length) continue;
  if (why) { for (const r of recs) report.skippedEdges.push({ file: f, line: r.line, target: nfc(r.resolved!), slice: why }); continue; }
  const bootstrap = BOOTSTRAP.test(f);
  const patches: { start: number; end: number; text: string }[] = [];
  const used = new Set<string>();
  for (const r of recs) {
    const m = targetMap[nfc(r.resolved!)];
    if (r.spec.includes("?")) { report.unhandled.push({ file: f, line: r.line, why: "query suffix", spec: r.spec }); continue; }
    const rel = relative(dirname(f), m.facade).normalize("NFC");
    const spec = bootstrap ? (rel.startsWith(".") ? rel : "./" + rel) : PKG_NAME[m.owner] + (m.subpath === "." ? "" : m.subpath.slice(1));
    const isJson = /\.(json)$/u.test(r.resolved!);
    const quote = text[r.litStart];
    if (isJson && r.kind === "import" && r.defaultName && !r.namespaceName && !r.named.length) {
      const original = text.slice(r.stmtStart, r.stmtEnd);
      const semi = original.trimEnd().endsWith(";") ? ";" : "";
      const typeKw = r.typeOnly ? "type " : "";
      const binding = m.jsonExport === r.defaultName ? r.defaultName : `${m.jsonExport} as ${r.defaultName}`;
      patches.push({ start: r.stmtStart, end: r.stmtEnd, text: `import ${typeKw}{ ${binding} } from ${quote}${spec}${quote}${semi}` });
      used.add(m.owner); continue;
    }
    if (isJson && r.kind === "export" && r.named.length === 1 && r.named[0].imported === "default") {
      const original = text.slice(r.stmtStart, r.stmtEnd);
      const exported = r.named[0].local;
      const binding = m.jsonExport === exported ? exported : `${m.jsonExport} as ${exported}`;
      const semi = original.trimEnd().endsWith(";") ? ";" : "";
      patches.push({ start: r.stmtStart, end: r.stmtEnd, text: `export { ${binding} } from ${quote}${spec}${quote}${semi}` });
      used.add(m.owner); continue;
    }
    if (isJson && r.kind === "dynamic") {
      const before = text.slice(Math.max(0, r.stmtStart - 80), r.stmtStart);
      const mm = /\bdefault(\s*:\s*([A-Za-z_$][\w$]*))(\s*\}\s*=\s*await\s*)$/u.exec(before);
      if (!mm || !(r.members?.length === 1 && r.members[0] === "default")) { report.unhandled.push({ file: f, line: r.line, why: "dynamic json shape", spec: r.spec }); continue; }
      const defStart = r.stmtStart - mm[0].length;
      patches.push({ start: defStart, end: defStart + "default".length + mm[1].length, text: mm[2] === m.jsonExport ? mm[2] : `${m.jsonExport}${mm[1]}` });
      patches.push({ start: r.litStart, end: r.litEnd, text: `${quote}${spec}${quote}` });
      used.add(m.owner); continue;
    }
    if (isJson) { report.unhandled.push({ file: f, line: r.line, why: `json ${r.kind} shape`, spec: r.spec }); continue; }
    if (r.defaultName) { report.unhandled.push({ file: f, line: r.line, why: "default import of ts", spec: r.spec }); continue; }
    if (r.membersUnknown && r.kind !== "dynamic" && r.kind !== "importType") { report.unhandled.push({ file: f, line: r.line, why: "membersUnknown", spec: r.spec }); continue; }
    patches.push({ start: r.litStart, end: r.litEnd, text: `${quote}${spec}${quote}` });
    used.add(m.owner);
  }
  const effective = patches.filter(p => text.slice(p.start, p.end) !== p.text);
  if (!effective.length) continue;
  effective.sort((a, b) => b.start - a.start);
  for (let i = 1; i < effective.length; i++) if (effective[i].end > effective[i - 1].start) throw new Error(`overlapping patches in ${f}`);
  let next = text;
  for (const p of effective) next = next.slice(0, p.start) + p.text + next.slice(p.end);
  report.files[f] = effective.length;
  edits += effective.length; touched++;
  if (!bootstrap) {
    const owning = owningPackage(f);
    for (const o of used) {
      const dep = PKG_NAME[o as OwnerKey];
      if (!owning) { report.unhandled.push({ file: f, why: "no owning package", dep }); continue; }
      if (owning.name === dep) continue;
      const needs = depNeeds.get(owning.dir) ?? new Map();
      depNeeds.set(owning.dir, needs);
      const prev = needs.get(dep) ?? { prod: false };
      if (!isTestPath(f)) prev.prod = true;
      needs.set(dep, prev);
    }
  }
  if (write) { snapshot[f] = text; writeFileSync(join(ROOT, f), next); }
}

const depLog: string[] = [];
for (const [dir, needs] of depNeeds) {
  const pj = join(ROOT, dir, "package.json");
  const original = readFileSync(pj, "utf8");
  const manifest = JSON.parse(original);
  const roundtrip = JSON.stringify(manifest, null, 2) + (original.endsWith("\n") ? "\n" : "");
  if (roundtrip !== original) { report.unhandled.push({ file: dir + "/package.json", why: "package.json not round-trip stable", deps: [...needs.keys()] }); continue; }
  const changed: string[] = [];
  for (const [dep, info] of needs) {
    if (manifest.dependencies?.[dep] || manifest.devDependencies?.[dep] || manifest.peerDependencies?.[dep] || manifest.optionalDependencies?.[dep]) continue;
    const key = info.prod ? "dependencies" : "devDependencies";
    manifest[key] ??= {};
    manifest[key][dep] = "workspace:*";
    const sorted = Object.keys(manifest[key]).sort((a, b) => (a < b ? -1 : 1));
    const wasSorted = Object.keys(manifest[key]).filter(k => k !== dep).every((k, i, arr) => i === 0 || arr[i - 1] < k);
    if (wasSorted) manifest[key] = Object.fromEntries(sorted.map(k => [k, manifest[key][k]]));
    changed.push(`${key}:${dep}`);
  }
  if (!changed.length) continue;
  depLog.push(`${dir}: ${changed.join(", ")}`);
  report.deps[dir] = changed;
  if (write) { snapshot[dir + "/package.json"] = original; writeFileSync(pj, JSON.stringify(manifest, null, 2) + (original.endsWith("\n") ? "\n" : "")); }
}
report.total = edits;
if (write && snapshotPath) writeFileSync(snapshotPath, JSON.stringify(snapshot));
if (reportPath) writeFileSync(reportPath, JSON.stringify(report, null, 1));
console.log(`${write ? "WROTE" : "DRY"}: files=${touched} edits=${edits} skippedEdges=${report.skippedEdges.length} unhandled=${report.unhandled.length} packageJsonEdits=${depLog.length}`);
for (const u of report.unhandled.slice(0, 40)) console.log("UNHANDLED", JSON.stringify(u).slice(0, 240));
for (const d of depLog.slice(0, 80)) console.log("DEP", d.replace(/🧰️framework\/🔨️modules\//g, "").slice(0, 220));
