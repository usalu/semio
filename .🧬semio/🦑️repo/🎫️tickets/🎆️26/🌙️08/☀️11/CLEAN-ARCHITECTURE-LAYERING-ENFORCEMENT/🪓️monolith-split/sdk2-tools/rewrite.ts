/** ✂️ Rewrites plugin imports of framework internals to SDK specifiers. Dry-run by default.
 * Usage: bun rewrite.ts <plan.json> <files.txt> [--plugin <name>] [--write] [--snapshot <out.json>] [--report <out.json>]
 * Re-reads and re-scans every file right before editing. Only touches literal specifiers and default JSON import statements.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, relative } from "node:path";
import { ROOT, SDK_DIR, scanSource, readLines } from "../sdkb-tools/lib.ts";

const args = process.argv.slice(2);
const [planPath, listPath] = args;
const opt = (name: string) => { const i = args.indexOf(name); return i >= 0 ? args[i + 1] : undefined; };
const write = args.includes("--write");
const plugin = opt("--plugin");
const snapshotPath = opt("--snapshot");
const reportPath = opt("--report");
const preview = opt("--preview");
const BOOTSTRAP_ENTRY = /(^|\/)(?:📜️script\.ts|🏗️builder\/🌐️vite\/🟦️\.ts|🧪️tests\/🎚️config\/🟦️\.ts|(?:⚙️|🧪️)?(?:vite|vitest)\.config\.[cm]?[jt]s)$/u;
const SKIP_PREFIXES: string[] = [];

const plan = JSON.parse(readFileSync(planPath, "utf8"));
const targetMap: Record<string, { facade: string; subpath: string; jsonExport?: string }> = { ...plan.targetMap };
const sdkManifest = JSON.parse(readFileSync(ROOT + "/" + SDK_DIR + "/package.json", "utf8"));
for (const [sub, t] of Object.entries(sdkManifest.exports as Record<string, string>)) {
  const file = (SDK_DIR + "/" + t.slice(2)).normalize("NFC");
  if (!targetMap[file]) targetMap[file] = { facade: file, subpath: sub };
}

const sdkTargets = new Set(Object.values(sdkManifest.exports as Record<string, string>).map(t => (SDK_DIR + "/" + t.slice(2)).normalize("NFC")));
let files = readLines(listPath).map(f => f.startsWith("/") ? f.slice(ROOT.length + 1) : f);
if (plugin) files = files.filter(f => f.startsWith(`✏️s/🔌️plugins/${plugin}/`));
files = files.filter(f => !SKIP_PREFIXES.some(p => f.startsWith(p)));

const snapshot: Record<string, string> = {};
const report: any = { files: {}, skipped: [] as any[], total: 0 };
let edits = 0, touched = 0;

for (const f of files) {
  let text: string;
  try { text = readFileSync(ROOT + "/" + f, "utf8"); } catch { continue; }
  const recs = scanSource(f, text).filter(r => r.resolved && targetMap[r.resolved]);
  if (!recs.length) continue;
  const patches: { start: number; end: number; text: string }[] = [];
  for (const r of recs) {
    const m = targetMap[r.resolved!];
    const bootstrap = BOOTSTRAP_ENTRY.test(f);
    if (bootstrap && sdkTargets.has(r.resolved!)) continue;
    const rel = relative(dirname(f), m.facade).normalize("NFC");
    const spec = bootstrap ? (rel.startsWith(".") ? rel : "./" + rel) : "@semio-tech/framework" + m.subpath.slice(1);
    if (r.spec.includes("?")) { report.skipped.push({ file: f, line: r.line, spec: r.spec, why: "query suffix" }); continue; }
    if (r.kind === "import" && r.defaultName && !r.namespaceName && !r.named.length && m.jsonExport && /\.(json|sql)$/.test(r.resolved!)) {
      const original = text.slice(r.stmtStart, r.stmtEnd);
      const semicolon = original.trimEnd().endsWith(";") ? ";" : "";
      const jsonExport = plan.facades.flatMap((x: any) => x.adds).flatMap((a: any) => a.json).find((j: any) => j.file === r.resolved)?.exportName ?? m.jsonExport;
      patches.push({ start: r.stmtStart, end: r.stmtEnd, text: `import { ${jsonExport} as ${r.defaultName} } from "${spec}"${semicolon}` });
      continue;
    }
    if (r.defaultName || (r.kind === "export" && r.exportStar)) { report.skipped.push({ file: f, line: r.line, spec: r.spec, why: "default or star" }); continue; }
    patches.push({ start: r.litStart, end: r.litEnd, text: `"${spec}"` });
  }
  const effective = patches.filter(x => text.slice(x.start, x.end) !== x.text);
  patches.length = 0; patches.push(...effective);
  if (!patches.length) continue;
  patches.sort((a, b) => b.start - a.start);
  let next = text;
  for (const p of patches) next = next.slice(0, p.start) + p.text + next.slice(p.end);
  if (preview && f.includes(preview)) {
    const a = text.split("\n"), b = next.split("\n");
    for (let i = 0; i < Math.max(a.length, b.length); i++) if (a[i] !== b[i]) console.log(`- ${i + 1}: ${(a[i] ?? "").slice(0, 400)}\n+ ${i + 1}: ${(b[i] ?? "").slice(0, 400)}`);
  }
  report.files[f] = patches.length;
  edits += patches.length; touched++;
  if (write) {
    snapshot[f] = text;
    writeFileSync(ROOT + "/" + f, next);
  }
}
report.total = edits;
if (write && snapshotPath) writeFileSync(snapshotPath, JSON.stringify(snapshot));
if (reportPath) writeFileSync(reportPath, JSON.stringify(report, null, 1));
console.log(`${write ? "WROTE" : "DRY"}: files=${touched} edits=${edits} skipped=${report.skipped.length}`);
for (const s of report.skipped.slice(0, 20)) console.log("SKIP", s.file.slice(-70), s.line, s.why, s.spec.slice(-60));
