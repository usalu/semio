import { spawnSync } from "node:child_process";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
const R = "/Users/ueli/Documents/semio/";
const OUT = process.argv[2]!;
const list = spawnSync("git", ["grep", "-l", "-E", "launch\\.seed\\.jsonc|\\.vscode/launch\\.json|launch\\.json", "--", "*.ts", "*.tsx", "*.mjs", ":!.🧬semio", ":!node_modules"], { cwd: R, encoding: "utf8" }).stdout.split("\n").filter(Boolean);
const extra = ["🧰️framework/🔨️modules/🏃️process/🏗️build/🎛️environment/🧪️tests/🟦️.ts"];
const files = [...new Set([...list, ...extra])].filter((f) => /\/(🧪️tests|🧪️test)\//u.test(f) || f.includes("🧪️tests")).filter((f) => existsSync(R + f));
const done: Record<string, any> = existsSync(OUT) ? JSON.parse(readFileSync(OUT, "utf8")) : {};
for (const f of files) {
  if (done[f]) continue;
  const t0 = Date.now();
  const r = spawnSync("bun", ["test", "./" + f], { cwd: R, encoding: "utf8", timeout: 170_000, maxBuffer: 1 << 26 });
  const text = (r.stdout ?? "") + (r.stderr ?? "");
  const pass = Number(/(\d+) pass/.exec(text)?.[1] ?? NaN), fail = Number(/(\d+) fail/.exec(text)?.[1] ?? NaN), errors = Number(/(\d+) error/.exec(text)?.[1] ?? 0);
  const failed = [...text.matchAll(/\(fail\) (.+?)(?: \[[\d.]+ms\])?$/gm)].map((m) => m[1]!.slice(0, 160));
  done[f] = { pass, fail, errors, ms: Date.now() - t0, timedOut: r.error?.name === "Error" && (r.signal === "SIGTERM"), failed };
  writeFileSync(OUT, JSON.stringify(done, null, 1));
  console.log(f.slice(-60), JSON.stringify({ pass, fail, errors, ms: done[f].ms }));
}
