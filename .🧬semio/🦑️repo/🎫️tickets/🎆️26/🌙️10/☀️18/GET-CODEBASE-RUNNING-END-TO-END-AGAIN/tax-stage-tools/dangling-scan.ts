// 🔎 Finds every quoted reference to a 📜️script.ts that does not exist (relative references and repo-relative paths) in the mirror state. usage: bun dangling-scan.ts <mirror>
import {existsSync, readFileSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import {C} from "./vfs.ts";
const REPO = "/Users/ueli/Documents/semio", mirror = resolve(process.argv[2]!);
const ops = JSON.parse(readFileSync(`${C}/tax-stage-tools/staged/ops.json`, "utf8")).ops as {path: string; kind: string}[];
const deleted = new Set(ops.filter((o) => o.kind === "delete").map((o) => o.path)), written = new Set(ops.filter((o) => o.kind === "write").map((o) => o.path));
const exists = (rel: string): boolean => written.has(rel) || (!deleted.has(rel) && existsSync(join(REPO, rel)));
const files = [...new Set([...readFileSync(`${C}/🗑️generated/tax-stage/grep-files.txt`, "utf8").split("\n").filter(Boolean), ...written])];
let n = 0;
for (const rel of files) {
  if (deleted.has(rel)) continue;
  let text: string; try { text = readFileSync(existsSync(join(mirror, rel)) ? join(mirror, rel) : join(REPO, rel), "utf8"); } catch { continue; }
  if (!text.includes("📜️script.ts")) continue;
  const lines = text.split("\n");
  for (let i = 0; i < lines.length; i++) {
    for (const m of lines[i]!.matchAll(/(["'`])((?:(?!\1)[^\n\\$])*?📜️script\.ts)\1/gu)) {
      const spec = m[2]!;
      let target: string | null = null;
      if (spec.startsWith("./") || spec.startsWith("../")) target = resolve(dirname(join(REPO, rel)), spec).slice(REPO.length + 1);
      else if (/^(?:🧰️|✏️|🌎️|♻️|🏢️|🎓️)/u.test(spec) || spec.startsWith(".🧬semio")) target = spec;
      if (!target || target.includes("${") || target.includes("*")) continue;
      if (!exists(target)) { n++; console.log(`${rel.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", "L").slice(-110)}:${i + 1} -> ${spec.slice(-90)}`); }
    }
  }
}
console.log(JSON.stringify({dangling: n}));
