import { readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
const [a, b, outPath] = process.argv.slice(2);
const paths = [...new Set([...(JSON.parse(readFileSync(a!, "utf8")).applied as string[]), ...(JSON.parse(readFileSync(b!, "utf8")).applied as string[])])];
const ROOT = "/Users/ueli/Documents/semio/";
const bad: string[] = [];
const tsT = new Bun.Transpiler({ loader: "ts" }), tsxT = new Bun.Transpiler({ loader: "tsx" });
let ts = 0, rs = 0;
for (const p of paths) {
  const abs = ROOT + p;
  let text: string;
  try { text = readFileSync(abs, "utf8"); } catch { bad.push(p + " (unreadable)"); continue; }
  if (p.endsWith(".ts") || p.endsWith(".tsx")) {
    ts++;
    try { (p.endsWith(".tsx") ? tsxT : tsT).transformSync(text); } catch (e) { bad.push(`${p}: ${String(e).slice(0, 160)}`); }
  } else if (p.endsWith(".rs")) {
    rs++;
    const r = spawnSync("rustfmt", ["--edition", "2021", "--emit", "stdout"], { input: text, encoding: "utf8" });
    if (r.status !== 0 && /error/i.test(r.stderr)) bad.push(`${p}: ${r.stderr.split("\n")[0]?.slice(0, 160)}`);
  }
}
writeFileSync(outPath!, bad.join("\n") + "\n");
console.log({ ts, rs, bad: bad.length });
