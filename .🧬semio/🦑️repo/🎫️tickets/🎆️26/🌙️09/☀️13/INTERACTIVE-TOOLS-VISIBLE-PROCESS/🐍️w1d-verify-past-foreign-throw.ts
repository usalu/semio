/** 🚧️ W1-D: runs `bun nx run workspace:verify -- interactivity` once with pre-existing foreign self-test throws skipped (listed below), so the full finding report is visible; the root script is restored byte-identical afterwards. */
import { readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";

const root = "/Users/ueli/Documents/semio";
const script = `${root}/📜️script.ts`;
const skipped = process.argv.slice(2);
const before = readFileSync(script, "utf8");
let patched = before;
for (const call of skipped) {
  if (patched.split(call).length !== 2) throw new Error(`skip anchor missing or ambiguous: ${call}`);
  patched = patched.replace(call, `void 0; /* [DEBUG] W1-D skipped ${call.replace(/\W+/g, " ").trim()} */`);
}
writeFileSync(script, patched);
try {
  const result = spawnSync("bun", ["nx", "run", "workspace:verify", "--skip-nx-cache", "--", "interactivity"], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 });
  process.stdout.write(result.stdout);
  process.stderr.write(result.stderr);
  console.log(`[w1d] exit=${result.status}`);
} finally {
  if (readFileSync(script, "utf8") !== patched) throw new Error("root script changed concurrently during the run; restore the [DEBUG] skips by hand");
  writeFileSync(script, before);
  console.log(`[w1d] restored=${readFileSync(script, "utf8") === before}`);
}
