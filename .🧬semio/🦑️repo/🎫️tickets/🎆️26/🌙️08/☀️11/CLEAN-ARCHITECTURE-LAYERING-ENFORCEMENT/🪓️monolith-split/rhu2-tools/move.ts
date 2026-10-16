/** 🚚️ Moves TypeScript files and recomputes every relative specifier that touches them (AST based, lossless apart from specifier text).
 * Usage: bun move.ts <spec.json> [--write]
 * spec.json: { "moves": [{"from": "repo/rel/old.ts", "to": "repo/rel/new.ts"}], "importers": ["extra repo/rel files to scan"], "snapshot": "path.json" }
 * Plan phase resolves against the current file system; apply phase edits texts, then renames. Dry run prints the edit table only.
 */
import { readFileSync, writeFileSync, mkdirSync, unlinkSync, existsSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { spawnSync } from "node:child_process";
import { ROOT, scanSource, resolveSpec } from "../sdkb-tools/lib.ts";

const [specPath, flag] = process.argv.slice(2);
const write = flag === "--write";
const spec = JSON.parse(readFileSync(specPath, "utf8")) as { moves: { from: string; to: string }[]; importers?: string[]; snapshot?: string };
const moves = new Map(spec.moves.map(m => [m.from.normalize("NFC"), m.to.normalize("NFC")]));
for (const [from, to] of moves) {
  if (!existsSync(join(ROOT, from))) throw new Error(`missing source ${from}`);
  if (existsSync(join(ROOT, to))) throw new Error(`target exists ${to}`);
}

function git(args: string[]): string[] {
  const r = spawnSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", ...args], { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28 });
  return r.stdout.split("\n").filter(Boolean).map(s => s.normalize("NFC"));
}

const candidates = new Set<string>((spec.importers ?? []).map(s => s.normalize("NFC")));
const needles = [...new Set([...moves.keys()].map(from => from.split("/").slice(-2).join("/")))];
const globs = [":(glob)🧰️framework/**/*.ts", ":(glob)🧰️framework/**/*.tsx", ":(glob)✏️s/**/*.ts", ":(glob)✏️s/**/*.tsx", ":(glob)🌎️hub/**/*.ts", ":(glob)🌎️hub/**/*.tsx", ":(glob)♻️mit-bestand/**/*.ts", ":(glob)♻️mit-bestand/**/*.tsx", ":(glob)🏢️semio-tech/**/*.ts", ":(glob)🏢️semio-tech/**/*.tsx"];
for (const f of git(["grep", "-l", "-F", ...needles.flatMap(n => ["-e", n]), "--", ...globs])) candidates.add(f);
for (const f of git(["ls-files", "--others", "--exclude-standard", "--", ":(glob)**/*.ts", ":(glob)**/*.tsx"])) {
  if (f.startsWith(".🧬semio/")) continue;
  try { const t = readFileSync(join(ROOT, f), "utf8"); if (needles.some(n => t.includes(n))) candidates.add(f); } catch {}
}
for (const from of moves.keys()) {
  const near = [dirname(from), dirname(dirname(from))];
  for (const dir of near) for (const f of [...git(["ls-files", "--", dir]), ...git(["ls-files", "--others", "--exclude-standard", "--", dir])]) if (/\.(ts|tsx)$/.test(f)) candidates.add(f);
}
for (const from of moves.keys()) candidates.add(from);

type Edit = { start: number; end: number; text: string };
const plan = new Map<string, Edit[]>();
const table: string[] = [];
for (const file of candidates) {
  let text: string;
  try { text = readFileSync(join(ROOT, file), "utf8"); } catch { continue; }
  const selfNew = moves.get(file) ?? file;
  const edits: Edit[] = [];
  for (const rec of scanSource(file, text)) {
    if (!rec.spec.startsWith(".")) continue;
    const resolved = resolveSpec(file, rec.spec);
    if (resolved === null) continue;
    const targetNew = moves.get(resolved) ?? resolved;
    if (selfNew === file && targetNew === resolved) continue;
    const suffix = rec.spec.match(/\?.*$/)?.[0] ?? "";
    const explicit = /\.(ts|tsx|json|mjs|cjs)$/.test(rec.spec.replace(/\?.*$/, ""));
    let to = explicit ? targetNew : dirname(targetNew);
    let rel = relative(dirname(join(ROOT, selfNew)), join(ROOT, to)).normalize("NFC");
    if (!rel.startsWith(".")) rel = "./" + rel;
    rel = rel + suffix;
    if (rel === rec.spec) continue;
    edits.push({ start: rec.litStart + 1, end: rec.litEnd - 1, text: rel });
    table.push(`${file}\t${rec.spec}\t${rel}`);
  }
  if (edits.length || moves.has(file)) plan.set(file, edits);
}
console.log(`candidates=${candidates.size} filesWithEdits=${[...plan.values()].filter(e => e.length).length} edits=${table.length} moves=${moves.size}`);
if (!write) { console.log(table.join("\n")); process.exit(0); }

const snapshot: Record<string, string> = {};
for (const [file, edits] of plan) {
  const abs = join(ROOT, file);
  const original = readFileSync(abs, "utf8");
  snapshot[file] = original;
  let out = original;
  for (const e of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  const moved = moves.get(file);
  if (moved === undefined) { writeFileSync(abs, out); continue; }
  const dest = join(ROOT, moved);
  mkdirSync(dirname(dest), { recursive: true });
  writeFileSync(dest, out);
  unlinkSync(abs);
}
if (spec.snapshot) writeFileSync(spec.snapshot, JSON.stringify(snapshot));
console.log("applied");
