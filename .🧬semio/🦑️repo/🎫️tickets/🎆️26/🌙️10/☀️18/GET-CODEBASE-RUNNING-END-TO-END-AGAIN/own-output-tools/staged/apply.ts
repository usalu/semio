/** 🧩️ Applies the staged own-output set: anchored hunks per file, dry run by default, refusal before the first write when any hunk cannot be placed. */
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, readFileSync, realpathSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join, resolve, sep } from "node:path";
import { spawnSync } from "node:child_process";

type Hunk = { old: string; new: string };
type Operation = { path: string; preimageSha256: string; resultSha256: string; hunks: Hunk[] };

const here = import.meta.dir;
const arguments_ = process.argv.slice(2);
const flag = (name: string) => arguments_.includes(name);
const option = (name: string) => { const index = arguments_.indexOf(name); return index < 0 ? undefined : arguments_[index + 1]; };
const root = resolve(option("--root") ?? join(here, "..", "..", "..", "..", "..", "..", "..", "..", ".."));
const write = flag("--yes");
const undo = flag("--undo");
const backup = resolve(option("--backup") ?? join(here, "..", "..", "🗑️generated", "own-output-apply"));
const sha = (text: string) => createHash("sha256").update(text).digest("hex");
const only = option("--only");
const operations: Operation[] = (JSON.parse(readFileSync(join(here, "ops.json"), "utf8")).ops as Operation[]).filter(operation => !only || operation.path.includes(only));

/** 🚦️ Names a running dependency sync or graph construction, which a live write would abort. */
function running(): string[] {
  if (process.platform === "win32") return [];
  const listing = spawnSync("ps", ["-A", "-o", "command="], { encoding: "utf8" }).stdout ?? "";
  return listing.split("\n").filter(line => /\brepo (show projects|dependencies (sync|lock))\b/u.test(line) && !line.includes("apply.ts"));
}

/** 🧮️ Places every hunk of one file; returns the new text, or the reason a hunk cannot be placed. */
function place(operation: Operation, text: string): { text: string; state: "applied" | "pending" } | { error: string } {
  if (sha(text) === operation.resultSha256) return { text, state: "applied" };
  let next = text, changed = false;
  for (const [index, hunk] of operation.hunks.entries()) {
    if (next.split(hunk.new).length === 2) continue;
    const count = next.split(hunk.old).length - 1;
    if (count !== 1) return { error: `hunk ${index + 1} anchor found ${count} times (${hunk.old.slice(0, 80).replaceAll("\n", "\\n")})` };
    next = next.replace(hunk.old, () => hunk.new);
    changed = true;
  }
  return { text: next, state: changed ? "pending" : "applied" };
}

const targets = operations.map(operation => ({ operation, path: join(root, operation.path) }));
if (undo) {
  for (const { operation, path } of targets) {
    const saved = join(backup, createHash("sha256").update(operation.path).digest("hex").slice(0, 16) + ".orig");
    if (!existsSync(saved)) { console.log("no backup", operation.path); continue; }
    if (sha(readFileSync(path, "utf8")) !== operation.resultSha256) { console.log("REFUSED undo (file changed since apply)", operation.path); continue; }
    if (write) { renameSync(saved, path); console.log("restored", operation.path); } else console.log("would restore", operation.path);
  }
  process.exit(0);
}

const busy = running();
if (write && busy.length && !flag("--ignore-running")) { console.error("REFUSED: a dependency sync or graph construction is running; its walk aborts on any write:\n" + busy.join("\n")); process.exit(3); }

const results: { operation: Operation; path: string; text: string; state: string; drift: boolean }[] = [];
const failures: string[] = [];
for (const { operation, path } of targets) {
  if (!existsSync(path)) { failures.push(`${operation.path}: file absent`); continue; }
  const inside = realpathSync(dirname(path)), base = realpathSync(root);
  if (inside !== base && !inside.startsWith(base + sep)) { failures.push(`${operation.path}: resolves outside the root (symlinked ancestor)`); continue; }
  const original = readFileSync(path, "utf8");
  const outcome = place(operation, original);
  if ("error" in outcome) { failures.push(`${operation.path}: ${outcome.error}`); continue; }
  results.push({ operation, path, text: outcome.text, state: outcome.state, drift: outcome.state === "pending" && sha(original) !== operation.preimageSha256 });
}
for (const row of results) console.log(row.state.padEnd(8), row.drift ? "(preimage drifted, anchors hold)" : "", row.operation.path.slice(-90));
if (failures.length) { console.error("REFUSED (nothing written):\n" + failures.join("\n")); process.exit(2); }
const exact = results.every(row => row.state === "applied" || sha(row.text) === row.operation.resultSha256 || row.drift);
console.log(`${results.filter(row => row.state === "pending").length} file(s) pending, ${results.filter(row => row.state === "applied").length} already applied, result hash ${exact ? "exact" : "differs from the staged hash"}`);
if (!write) { console.log("dry run (add --yes to write)"); process.exit(0); }
mkdirSync(backup, { recursive: true });
for (const row of results) {
  if (row.state !== "pending") continue;
  copyFileSync(row.path, join(backup, createHash("sha256").update(row.operation.path).digest("hex").slice(0, 16) + ".orig"));
  const temporary = row.path + ".own-output.tmp";
  writeFileSync(temporary, row.text);
  renameSync(temporary, row.path);
  console.log("wrote", row.operation.path.slice(-90));
}
