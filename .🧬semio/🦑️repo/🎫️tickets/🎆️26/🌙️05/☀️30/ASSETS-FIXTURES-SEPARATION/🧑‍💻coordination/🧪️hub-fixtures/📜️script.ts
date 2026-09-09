import { spawnSync } from "node:child_process";
import { join } from "node:path";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!;
let failed = 0;
for (const args of [["admin-directory-authority-check", "source"], ["directory-message-authority-check", "source"], ["socket-grant-check", "oracle"], ["directory-ordered-publication-check"]]) {
  console.log(`[DEBUG] START ${args.join(" ")}`);
  const result = spawnSync("bun", [join(root, "🌎️hub/📦️packages/🦀️rust/📜️script.ts"), ...args], { cwd: root, env: process.env, encoding: "utf8", timeout: 45000, maxBuffer: 1024 * 1024 });
  console.log(result.stdout);
  if (result.stderr) console.error(result.stderr);
  if (result.status !== 0) { failed++; console.error(`[DEBUG] FAIL ${args[0]}: ${result.error ?? result.status}`); }
  else console.log(`[DEBUG] PASS ${args[0]}`);
}
process.exitCode = failed ? 1 : 0;
