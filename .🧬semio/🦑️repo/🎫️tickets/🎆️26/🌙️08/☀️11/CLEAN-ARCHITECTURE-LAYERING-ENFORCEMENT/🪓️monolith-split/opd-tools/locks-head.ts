// 🔎️ Same unresolved-token check against `git show HEAD:<lock>` for the locks the opd slice touched, to separate pre-existing problems from new ones.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {join} from "node:path";
import {blocksOf, nameOf, depsOf} from "./dd.ts";
const ROOT = "/Users/ueli/Documents/semio";
const touched = readFileSync(join(import.meta.dir, "touched.txt"), "utf8").split("\n").filter(p => p.endsWith("Cargo.lock"));
const problems = (text: string, lock: string): Set<string> => {
  const {blocks} = blocksOf(text);
  const names = new Set(blocks.map(nameOf));
  const out = new Set<string>();
  for (const b of blocks) for (const d of depsOf(b)) if (!names.has(d.split(" ")[0]!)) out.add(`${lock}: ${nameOf(b)} -> ${d}`);
  return out;
};
let now = 0, head = 0, fresh = 0;
for (const lock of touched) {
  const a = problems(readFileSync(join(ROOT, lock), "utf8"), lock);
  let b = new Set<string>();
  try { b = problems(execFileSync("git", ["-c", "core.fsmonitor=false", "show", `HEAD:${lock}`], {cwd: ROOT, maxBuffer: 1 << 28, stdio: ["ignore", "pipe", "ignore"]}).toString("utf8"), lock); } catch {}
  now += a.size; head += b.size;
  for (const p of a) if (!b.has(p)) { fresh++; console.log("NEW", p); }
}
console.log(`unresolved tokens: now ${now}, at HEAD ${head}, new ${fresh}`);
