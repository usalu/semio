import { readFileSync, rmSync, mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
const files = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
const out = process.argv[3] ?? "/tmp/lay16-bundle";
let ok = 0; const bad: string[] = [];
const pending = [...files];
const worker = async () => {
  for (let f = pending.pop(); f; f = pending.pop()) {
    const r = Bun.spawnSync(["bun", "build", "--target=bun", "--outdir", out + "/" + (ok + bad.length), f], { cwd: "/Users/ueli/Documents/semio", stderr: "pipe", stdout: "pipe" });
    if (r.exitCode === 0) ok++; else bad.push(f + "\t" + r.stderr.toString().split("\n").filter(l => /error|Could not|No matching|not found/i.test(l)).slice(0, 3).join(" | ").slice(0, 400));
  }
};
await Promise.all([worker(), worker()]);
console.log(`ok ${ok} bad ${bad.length}`);
for (const b of bad) console.log(b);
