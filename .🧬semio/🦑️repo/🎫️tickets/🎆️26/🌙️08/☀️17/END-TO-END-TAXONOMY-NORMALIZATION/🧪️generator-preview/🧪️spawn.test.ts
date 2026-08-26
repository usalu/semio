import { test } from "bun:test";

test("spawn", () => {
  const cwd = process.env.GENERATOR_PREVIEW_PROBE_CWD!;
  const result = Bun.spawnSync(["bun", "./📜️script.ts", "preview-generated"], { cwd, stderr: "pipe", stdout: "pipe" });
  process.stdout.write(`${JSON.stringify({ exitCode: result.exitCode, stderr: result.stderr.toString(), stdout: result.stdout.toString(), success: result.success })}\n`);
});
