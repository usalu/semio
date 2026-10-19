import { readFileSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio/";
const list = readFileSync(process.argv[2], "utf8").split("\n").filter(Boolean);
let next = 0, bad = 0;
const fails: string[] = [];
const worker = async () => {
  while (next < list.length) {
    const f = list[next++];
    const abs = ROOT + f;
    const cmd = f.endsWith(".rs") ? ["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--check", "--config", "skip_children=true,disable_all_formatting=true", abs] : ["bun", "build", "--no-bundle", abs];
    const pr = Bun.spawn(cmd, { stdout: "ignore", stderr: "pipe", cwd: ROOT });
    const err = await new Response(pr.stderr).text();
    const code = await pr.exited;
    const isErr = f.endsWith(".rs") ? /^error/m.test(err) : code !== 0;
    if (isErr) { bad++; fails.push(`${f}: ${err.split("\n")[0].slice(0, 160)}`); }
  }
};
await Promise.all(Array.from({ length: 6 }, worker));
console.log(`verified ${list.length} files, ${bad} failures`);
for (const f of fails.slice(0, 20)) console.log(f);
