// 🔎️ Lists unresolved dependency tokens per touched lock, now versus HEAD (orc).
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {execFileSync} from "node:child_process";
import {blocksOf, nameOf, depsOf} from "./dd.ts";
const ROOT = "/Users/ueli/Documents/semio";
const touched = readFileSync(join(import.meta.dir, "touched.txt"), "utf8").split("\n").filter(p => p.endsWith("Cargo.lock"));
const list = (text: string): string[] => {
  const {blocks} = blocksOf(text);
  const names = new Set(blocks.map(nameOf));
  return blocks.flatMap(b => depsOf(b).filter(d => !names.has(d.split(" ")[0]!)).map(d => `${nameOf(b)} -> ${d}`));
};
for (const lock of touched) {
  const now = list(readFileSync(join(ROOT, lock), "utf8"));
  let head: string[] = [];
  try { head = list(execFileSync("git", ["-c", "core.fsmonitor=false", "show", `HEAD:${lock}`], {cwd: ROOT, maxBuffer: 1 << 28, stdio: ["ignore", "pipe", "ignore"]}).toString("utf8")); } catch {}
  const newOnes = now.filter(x => !head.includes(x));
  if (now.length || head.length) console.log(lock, "now", now.length, "head", head.length, newOnes.length ? "NEW: " + newOnes.join("; ") : "");
}
