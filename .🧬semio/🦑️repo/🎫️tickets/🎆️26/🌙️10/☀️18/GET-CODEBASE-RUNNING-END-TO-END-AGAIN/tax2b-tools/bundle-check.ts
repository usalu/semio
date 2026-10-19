// 📦 Bundles every listed file with the Bun target; prints failures. usage: bun bundle-check.ts <list.txt> <outdir>
import {readFileSync, mkdirSync} from "node:fs";
import {spawnSync} from "node:child_process";
const [list, outdir] = process.argv.slice(2) as [string, string];
mkdirSync(outdir, {recursive: true});
let ok = 0; const bad: {file: string; message: string}[] = [];
for (const file of readFileSync(list, "utf8").split("\n").filter(Boolean)) {
  const r = spawnSync("bun", ["build", "--target=bun", "--outdir", outdir, file], {cwd: "/Users/ueli/Documents/semio", encoding: "utf8", timeout: 90000});
  if (r.status === 0) ok++; else bad.push({file, message: ((r.stderr ?? "") + (r.stdout ?? "")).split("\n").filter((l) => /error|Could not resolve|Cannot find/iu.test(l)).slice(0, 2).join(" | ").slice(0, 300) || String(r.signal ?? r.status)});
}
console.log(JSON.stringify({ok, bad: bad.length}));
for (const b of bad) console.log(b.file.slice(-90), "::", b.message);
