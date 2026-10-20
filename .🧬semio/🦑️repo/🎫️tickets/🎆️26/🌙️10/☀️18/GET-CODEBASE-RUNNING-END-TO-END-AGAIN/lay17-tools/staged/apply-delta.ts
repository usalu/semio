/**
 * 🧷 Applies `batch-delta.json` to the live tree after the exports-stage batch was applied (the batch baselines no longer match, so the delta has its own guards).
 *
 *   bun apply-delta.ts                         dry run against the repository (default)
 *   bun apply-delta.ts --write                 writes manifests, glue and importer rewrites
 *   SEMIO_ROOT=<dir> bun apply-delta.ts ...    run against a scratch mirror
 *
 * Refuses when a key exists with another value, a glue file exists with other content, a manifest does not round-trip through JSON.stringify(…, 2),
 * a dependency sync/lock is running (live repository only) or a rewrite finds no occurrence.
 */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

const write = process.argv.includes("--write");
const scratch = process.env.SEMIO_ROOT;
const ROOT = scratch ? resolve(scratch) : resolve(import.meta.dir, "../../../../../../../../..");
const delta = JSON.parse(readFileSync(resolve(import.meta.dir, "batch-delta.json"), "utf8"));
const problems: string[] = [];
const fail = (m: string): void => { problems.push(m); };
if (!scratch) {
  const ps = spawnSync("ps", ["-axo", "pid=,command="], { encoding: "utf8" }).stdout ?? "";
  const hits = ps.split("\n").filter(l => /dependencies\s+(sync|lock)|deps4-(run|loop2?)\.sh|chain[0-9]*\.sh/u.test(l) && !/apply-delta|grep|spawnSync/u.test(l));
  if (hits.length) fail("dependency sync/lock is running:\n  " + hits.slice(0, 3).join("\n  "));
}
const patched = new Map<string, string>((delta.patchGlue ?? []).map((p: any) => [p.path, p.content]));
const manifests = new Map<string, { text: string; obj: any }>();
const manifest = (pj: string): { text: string; obj: any } => {
  const hit = manifests.get(pj);
  if (hit) return hit;
  const text = readFileSync(resolve(ROOT, pj), "utf8");
  const obj = JSON.parse(text);
  if (JSON.stringify(obj, null, 2) + (text.endsWith("\n") ? "\n" : "") !== text) fail(`manifest does not round-trip: ${pj}`);
  const row = { text, obj };
  manifests.set(pj, row);
  return row;
};
const files = new Map<string, string>();
for (const g of delta.glue) {
  const p = resolve(ROOT, g.path);
  const want = patched.get(g.path) ?? g.content;
  if (existsSync(p) && readFileSync(p, "utf8") !== want) fail(`glue exists with other content: ${g.path}`);
  files.set(g.path, want);
}
for (const [path, content] of patched) {
  const p = resolve(ROOT, path);
  if (!existsSync(p)) continue;
  const now = readFileSync(p, "utf8");
  const comment = content.split("\n").filter((l: string) => !l.startsWith("import ")).join("\n");
  if (now !== content && now.replace(/\n*$/u, "\n") !== comment.replace(/\n*$/u, "\n")) fail(`glue to patch changed: ${path}`);
  files.set(path, content);
}
for (const e of delta.entries) {
  const { obj } = manifest(e.packageJson);
  const bag = (obj[e.field] ??= {}) as Record<string, unknown>;
  if (e.key in bag) { if (JSON.stringify(bag[e.key]) !== JSON.stringify(e.value)) fail(`${e.packageJson}: ${e.field}[${e.key}] exists with ${JSON.stringify(bag[e.key])}`); continue; }
  const keys = Object.keys(bag);
  const sorted = keys.every((k, i) => i === 0 || keys[i - 1]! < k);
  const next = Object.entries(bag);
  if (sorted) { const at = next.findIndex(([k]) => k > e.key); if (at < 0) next.push([e.key, e.value]); else next.splice(at, 0, [e.key, e.value]); } else next.push([e.key, e.value]);
  for (const k of keys) delete bag[k];
  for (const [k, v] of next) bag[k] = v;
}
const edits = new Map<string, string>();
for (const r of delta.rewrites) {
  if (!existsSync(resolve(ROOT, r.file))) { fail(`rewrite file missing: ${r.file}`); continue; }
  const t = edits.get(r.file) ?? readFileSync(resolve(ROOT, r.file), "utf8");
  const n = t.replaceAll(`"${r.from}"`, `"${r.to}"`).replaceAll(`'${r.from}'`, `'${r.to}'`);
  if (n === t) { if (!t.includes(r.to)) fail(`no occurrence of ${r.from} in ${r.file}`); continue; }
  edits.set(r.file, n);
}
console.log(`delta: ${delta.entries.length} entries, ${files.size} glue files, ${edits.size} files rewritten, ${problems.length} problems`);
if (problems.length) { console.error("REFUSED:\n- " + problems.join("\n- ")); process.exit(3); }
if (!write) { console.log("dry run: nothing written"); process.exit(0); }
for (const [path, content] of files) { mkdirSync(dirname(resolve(ROOT, path)), { recursive: true }); writeFileSync(resolve(ROOT, path), content); }
for (const [pj, { text, obj }] of manifests) writeFileSync(resolve(ROOT, pj), JSON.stringify(obj, null, 2) + (text.endsWith("\n") ? "\n" : ""));
for (const [file, content] of edits) writeFileSync(resolve(ROOT, file), content);
console.log("written. NEXT: bun repo dependencies lock (export fields only: no lock change expected), then re-run lay16-tools/scan.ts with a fresh policy dump.");
