/**
 * 🧷 Applies the staged manifest batch (`batch.json`) once the HARD FREEZE is lifted.
 *
 *   bun apply.ts check                      read-only: guards + plan (default)
 *   bun apply.ts apply --dry-run            same guards and plan as check, writes nothing even when --freeze-lifted is passed (--ignore-sync allowed)
 *   bun apply.ts apply --freeze-lifted      write glue, manifests, importer rewrites, glue deletions
 *
 * Exit codes: 2 = a dependency sync/lock is running, 3 = baseline/guard problem or failed verification, 64 = usage. Any unknown flag is refused.
 *   --include=hygiene,physical              also apply those tiers (default: export, move, removal, third-party, types, requested)
 *   --root=<dir>                            run against a scratch mirror of the touched files (apply logic test: no process guard, plain grep)
 *   --ignore-sync                           report instead of refusing while a sync/lock runs (explicit operator choice; the planner passes it only with its own --ignore-sync)
 *   --ignore-removed                        check mode only: report instead of refusing (removed key still imported)
 *
 * Refuses when: a target package.json (or an extended/deleted glue file) changed since staging, a `bun repo dependencies sync|lock`
 * (or the deps4 loop) is running, a created glue file exists with other content, a removed key is still imported somewhere,
 * or a manifest does not round-trip through JSON.stringify(…, null, 2). It never runs `bun repo dependencies lock`: do that afterwards.
 */
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const args = process.argv.slice(2);
const KNOWN = ["--root=", "--include=", "--freeze-lifted", "--ignore-sync", "--ignore-removed", "--dry-run"];
const unknownFlags = args.filter(a => a.startsWith("--") && !KNOWN.some(k => a === k || (k.endsWith("=") && a.startsWith(k))));
if (unknownFlags.length) { console.error(`REFUSED: unknown flag(s) ${unknownFlags.join(" ")} (this set has no rebase mode)`); process.exit(64); }
const scratchRoot = args.find(a => a.startsWith("--root="))?.slice(7);
const findRoot = (from: string): string => { for (let at = from; ; at = dirname(at)) { if (existsSync(resolve(at, "bunfig.toml")) && existsSync(resolve(at, "bun.lock"))) return at; if (dirname(at) === at) throw new Error("repository root not found"); } };
const ROOT = scratchRoot ? resolve(scratchRoot) : findRoot(import.meta.dir);
const batch = JSON.parse(readFileSync(resolve(import.meta.dir, "batch.json"), "utf8"));
const requestedMode = args.find(a => !a.startsWith("--")) ?? "check";
if (!["check", "apply"].includes(requestedMode)) { console.error(`REFUSED: unknown mode ${requestedMode}`); process.exit(64); }
const dryRun = args.includes("--dry-run");
const mode = dryRun ? "check" : requestedMode;
const flag = (n: string): string | undefined => args.find(a => a.startsWith(`--${n}=`))?.split("=")[1];
const DEFAULT_TIERS = ["export", "move", "removal", "third-party", "types", "requested", "tst6"];
const tiers = new Set([...DEFAULT_TIERS, ...(flag("include")?.split(",") ?? [])]);
const abs = (p: string): string => resolve(ROOT, p);
const sha = (b: Buffer | string): string => createHash("sha256").update(b).digest("hex");
const problems: string[] = [];
let syncBlocked = false;
const fail = (m: string): void => { problems.push(m); };
const refuse = (): never => { console.error("REFUSED:\n- " + problems.join("\n- ")); process.exit(syncBlocked ? 2 : 3); };

type Entry = { id: string; optional?: boolean; op: "add" | "remove"; packageJson: string; package: string; field: string; key: string; value: string; tier: string; apply: boolean };
const entries: Entry[] = batch.entries.filter((e: Entry) => (e.apply && tiers.has(e.tier)) || (e.optional && tiers.has(e.tier) && flag("include")?.split(",").includes(e.tier)));
const selected = new Set(entries.map(e => e.id));
const glue = batch.glue.filter((g: any) => g.entry === null || selected.has(g.entry));
const rewrites = batch.rewrites.filter((r: any) => selected.has(r.entry));

function guardProcesses(): void {
  if (scratchRoot) return;
  const ps = spawnSync("ps", ["-axo", "pid=,command="], { encoding: "utf8" }).stdout ?? "";
  const hits = ps.split("\n").filter(l => /dependencies\s+(sync|lock)|deps4-(run|loop)\.sh|chain[0-9]*\.sh/u.test(l) && !/apply\.ts|grep|spawnSync/u.test(l));
  if (hits.length) { if (args.includes("--ignore-sync")) console.log("[guard] ignored running sync/lock processes:\n" + hits.slice(0, 5).join("\n")); else { syncBlocked = true; fail("dependency sync/lock is running:\n" + hits.slice(0, 5).join("\n")); } }
}

function guardBaselines(): void {
  for (const pj of new Set(entries.map(e => e.packageJson))) {
    const b = batch.baseline[pj];
    if (!existsSync(abs(pj))) { fail(`target manifest vanished: ${pj}`); continue; }
    if (!b || sha(readFileSync(abs(pj))) !== b.sha256) fail(`manifest changed since staging: ${pj}`);
  }
  for (const g of glue) {
    if (g.kind === "create") { if (existsSync(abs(g.path)) && readFileSync(abs(g.path), "utf8") !== g.content) fail(`glue to create exists with other content: ${g.path}`); }
    else if (!existsSync(abs(g.path)) || sha(readFileSync(abs(g.path))) !== g.baselineSha256) fail(`glue changed since staging (${g.kind}): ${g.path}`);
  }
}

function guardTargets(): void {
  for (const g of glue) {
    if (g.kind === "delete") continue;
    for (const t of g.targets ?? []) {
      if (!existsSync(abs(t.path))) { fail(`glue ${g.path}: re-export target vanished: ${t.path}`); continue; }
      const text = readFileSync(abs(t.path), "utf8");
      for (const n of t.names) if (n !== "default" && !new RegExp(`(?<![\\w$])${n.replace(/[$]/gu, "\\$")}(?![\\w$])`, "u").test(text)) fail(`glue ${g.path}: ${t.path} no longer mentions the exported name ${n} (the body moved? re-stage with stage.ts)`);
      if (sha(readFileSync(abs(t.path))) !== t.sha256) console.log(`[note] re-export target changed since staging (names still present): ${t.path}`);
    }
  }
}
const normalizeEmpty = (t: string): string => t.replace(/\{\s*\}/gu, "{}");
function planManifest(pj: string, ops: Entry[]): string {
  const text = readFileSync(abs(pj), "utf8");
  const indent = /^(\s+)"/mu.exec(text)?.[1] ?? "  ";
  const obj = JSON.parse(text);
  const re = JSON.stringify(obj, null, indent) + (text.endsWith("\n") ? "\n" : "");
  if (re !== text && normalizeEmpty(re) !== normalizeEmpty(text)) { fail(`manifest does not round-trip: ${pj}`); return text; }
  for (const op of [...ops].sort((a, b) => (a.op === b.op ? (a.field + a.key < b.field + b.key ? -1 : 1) : a.op === "remove" ? -1 : 1))) {
    if (op.field === "exports" || (op.field === "devDependencies" || op.field === "dependencies")) {
      let bag = obj[op.field] as Record<string, unknown> | undefined;
      if (op.op === "remove") {
        if (!bag || !(op.key in bag)) { fail(`${pj}: ${op.field}[${op.key}] to remove is absent`); continue; }
        if (JSON.stringify(bag[op.key]) !== JSON.stringify(op.value)) { fail(`${pj}: ${op.field}[${op.key}] changed (${JSON.stringify(bag[op.key])} vs ${JSON.stringify(op.value)})`); continue; }
        delete bag[op.key]; continue;
      }
      if (!bag) {
        bag = {};
        const entriesTop = Object.entries(obj); for (const [k] of entriesTop) delete obj[k];
        const afterKey = op.field === "devDependencies" && "dependencies" in Object.fromEntries(entriesTop) ? "dependencies" : "exports" in Object.fromEntries(entriesTop) ? "exports" : entriesTop[entriesTop.length - 1]![0];
        for (const [k, v] of entriesTop) { obj[k] = v; if (k === afterKey) obj[op.field] = bag; }
        if (!(op.field in obj)) obj[op.field] = bag;
      }
      if (op.key in bag) { if (JSON.stringify(bag[op.key]) !== JSON.stringify(op.value)) fail(`${pj}: ${op.field}[${op.key}] exists with ${JSON.stringify(bag[op.key])}`); continue; }
      const keys = Object.keys(bag);
      const sorted = keys.every((k, i) => i === 0 || keys[i - 1]! < k);
      const next: [string, unknown][] = Object.entries(bag);
      if (sorted) { const at = next.findIndex(([k]) => k > op.key); if (at < 0) next.push([op.key, op.value]); else next.splice(at, 0, [op.key, op.value]); } else next.push([op.key, op.value]);
      for (const k of keys) delete bag[k];
      for (const [k, v] of next) bag[k] = v;
    }
  }
  return JSON.stringify(obj, null, indent) + (text.endsWith("\n") ? "\n" : "");
}

function guardRemovedKeys(): void {
  const removed = entries.filter(e => e.op === "remove" && e.field === "exports");
  if (!removed.length) return;
  const covered = new Set(rewrites.map((r: any) => r.file + "\0" + r.from));
  const specs = removed.map(e => { const pkg = e.package; return pkg + (e.key === "." ? "" : e.key.slice(1)); });
  const patterns = resolve(scratchRoot ? ROOT : resolve(import.meta.dir, "../🗑️generated/exports-stage"), "removed-key-patterns.txt");
  mkdirSync(dirname(patterns), { recursive: true });
  writeFileSync(patterns, specs.flatMap(s => [`"${s}"`, `'${s}'`]).join("\n") + "\n");
  const out = scratchRoot || !existsSync(abs(".git")) ? spawnSync("grep", ["-rl", "-F", "-f", patterns, "."], { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 26 }) : spawnSync("git", ["grep", "-l", "-F", "-f", patterns, "--", ".", ":(exclude).🧬semio", ":(exclude)bun.lock"], { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 26 });
  for (const f0 of out.stdout.split("\n").filter(Boolean)) {
    const f = f0.replace(/^\.\//u, "");
    if (f.endsWith("removed-key-patterns.txt") || f.startsWith(".applied-") || f.startsWith("t/")) continue;
    const text = readFileSync(abs(f), "utf8");
    for (const s of specs) if ((text.includes(`"${s}"`) || text.includes(`'${s}'`)) && !covered.has(f + "\0" + s)) { if (args.includes("--ignore-removed") && mode === "check") console.log(`[guard] ignored: removed key ${s} is still imported by ${f}`); else fail(`removed key ${s} is still imported by ${f}`); }
  }
}

function main(): void {
  console.log(`batch staged ${batch.stagedAt}: ${entries.length} entries selected (tiers ${[...tiers].join(",")}), ${glue.length} glue ops, ${rewrites.length} importer rewrites`);
  guardProcesses();
  guardBaselines();
  guardTargets();
  if (problems.length) refuse();
  const byManifest = new Map<string, Entry[]>();
  for (const e of entries) (byManifest.get(e.packageJson) ?? byManifest.set(e.packageJson, []).get(e.packageJson)!).push(e);
  const manifests = new Map<string, string>();
  for (const [pj, ops] of byManifest) manifests.set(pj, planManifest(pj, ops));
  guardRemovedKeys();
  if (problems.length) refuse();
  const created = glue.filter((g: any) => g.kind === "create" && !existsSync(abs(g.path)));
  console.log(`plan: ${manifests.size} manifests, ${created.length} glue files to create, ${glue.filter((g: any) => g.kind === "extend").length} to extend, ${glue.filter((g: any) => g.kind === "delete").length} to delete, ${new Set(rewrites.map((r: any) => r.file)).size} importer files to rewrite`);
  if (mode !== "apply") { console.log(dryRun ? "dry-run: nothing written" : "check mode: nothing written"); return; }
  if (!args.includes("--freeze-lifted")) { console.error("REFUSED: pass --freeze-lifted to confirm the coordinator lifted the HARD FREEZE"); process.exit(3); }
  const stamp = new Date().toISOString().replace(/[:.]/gu, "-");
  const backup = scratchRoot ? resolve(ROOT, `.applied-${stamp}`) : resolve(import.meta.dir, `../🗑️generated/exports-stage/applied-${stamp}`);
  mkdirSync(backup, { recursive: true });
  const index: Record<string, string> = {};
  const save = (p: string): void => { if (!existsSync(abs(p))) return; const name = sha(p).slice(0, 24); copyFileSync(abs(p), resolve(backup, name + ".bak")); index[name] = p; };
  const write = (p: string, content: string): void => { mkdirSync(dirname(abs(p)), { recursive: true }); const tmp = abs(p) + ".tmp-exports-stage"; writeFileSync(tmp, content); renameSync(tmp, abs(p)); };
  for (const g of glue) {
    if (g.kind === "create") { if (!existsSync(abs(g.path))) write(g.path, g.content); }
    else if (g.kind === "extend") { save(g.path); const t = readFileSync(abs(g.path), "utf8"); write(g.path, t.replace(/\n*$/u, "\n") + g.lines.join("\n") + "\n"); }
  }
  for (const [pj, text] of manifests) { save(pj); write(pj, text); }
  const touched = new Set<string>();
  for (const r of rewrites) {
    const t = readFileSync(abs(r.file), "utf8");
    const next = t.replaceAll(`"${r.from}"`, `"${r.to}"`).replaceAll(`'${r.from}'`, `'${r.to}'`);
    if (next === t) { console.warn(`[warn] no occurrence of ${r.from} in ${r.file}`); continue; }
    if (!touched.has(r.file)) { save(r.file); touched.add(r.file); }
    write(r.file, next);
  }
  for (const g of glue.filter((x: any) => x.kind === "delete")) { save(g.path); rmSync(abs(g.path)); }
  writeFileSync(resolve(backup, "index.json"), JSON.stringify(index, null, 1));
  const bad: string[] = [];
  for (const e of entries.filter(e => e.op === "add" && e.field === "exports")) { const j = JSON.parse(readFileSync(abs(e.packageJson), "utf8")); const target = resolve(dirname(abs(e.packageJson)), String(j.exports[e.key] ?? "")); if (!existsSync(target)) bad.push(`${e.packageJson} ${e.key} -> missing ${e.value}`); }
  for (const g of glue.filter((x: any) => x.kind !== "delete")) { try { new Bun.Transpiler({ loader: g.path.endsWith("x") ? "tsx" : "ts" }).transformSync(readFileSync(abs(g.path), "utf8")); } catch (error) { bad.push(`glue does not parse: ${g.path}: ${String(error).slice(0, 120)}`); } }
  console.log(`applied. backup: ${backup} (${Object.keys(index).length} files). Verification problems: ${bad.length}`);
  for (const b of bad) console.error(" - " + b);
  console.log("NEXT: `bun repo dependencies lock` then `bun repo dependencies sync` (dependency fields changed), then re-run exports-stage-tools/simulate.ts");
  if (bad.length) process.exit(3);
}
void copyFileSync;
main();
