// runs each listed test file with `bun test ./<file>` (cap per file) and prints one summary row per file. usage: bun runtests.ts <list-file> <out-jsonl> [capSeconds]
import { appendFileSync, readFileSync, writeFileSync } from "node:fs";
const [listFile, out, cap = "240"] = process.argv.slice(2);
const files = readFileSync(listFile!, "utf8").split("\n").filter(Boolean);
writeFileSync(out!, "");
for (const file of files) {
  const started = performance.now();
  const proc = Bun.spawn(["bun", "test", "--tsconfig-override", "/Users/ueli/.cache/vtax/tsconfig.paths.json", `./${file}`], { cwd: "/Users/ueli/Documents/semio", stdout: "pipe", stderr: "pipe", env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: process.env.SEMIO_TEST_ARTIFACT_DIR || "/Users/ueli/.cache/vtax/artifacts" } });
  const timer = setTimeout(() => proc.kill(), Number(cap) * 1000);
  const [stdout, stderr] = await Promise.all([new Response(proc.stdout).text(), new Response(proc.stderr).text()]);
  await proc.exited;
  clearTimeout(timer);
  const text = stdout + stderr;
  const pass = Number(text.match(/\n\s*(\d+) pass/)?.[1] ?? "NaN"), fail = Number(text.match(/\n\s*(\d+) fail/)?.[1] ?? "NaN"), errors = Number(text.match(/\n\s*(\d+) error/)?.[1] ?? "0");
  const reason = (text.match(/error: ([^\n]{0,200})/)?.[1] ?? text.match(/\(fail\) ([^\n]{0,160})/)?.[1] ?? "").trim();
  const row = { file: file.split("/").slice(-2).join("/"), pass, fail, errors, killed: proc.signalCode !== null, seconds: Math.round((performance.now() - started) / 100) / 10, reason };
  appendFileSync(out!, JSON.stringify(row) + "\n");
  console.log(JSON.stringify(row));
}
