// 📋️ Lists the source closure of a lib root grouped by directory prefix.
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {readFileSync} from "node:fs";
const entry = process.argv[2]!;
const depth = Number(process.argv[3] ?? 8);
const rustc = (await import("node:child_process")).execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfg = (await import("node:child_process")).execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const files = await readRustSourceClosureV1([entry], entry.replace(/\/[^/]*$/, ""), cfg, async (p: string) => { try { return readFileSync(p); } catch { return null; } }, {manifestDirectory: entry.replace(/\/[^/]*$/, ""), generated: () => undefined}).catch((e: Error) => { console.error(e.message); return []; });
const groups = new Map<string, {n: number; b: number}>();
for (const f of files as any[]) {
  const rel = f.path.replace(entry.replace(/\/[^/]*$/, "") + "/", "");
  const key = rel.split("/").slice(0, depth).join("/");
  const g = groups.get(key) ?? {n: 0, b: 0}; g.n++; g.b += f.activeBytes ?? f.bytes; groups.set(key, g);
}
for (const [k, v] of [...groups].sort((a, b) => b[1].b - a[1].b)) console.log(v.n, v.b, k);
console.log("total", (files as any[]).length);
