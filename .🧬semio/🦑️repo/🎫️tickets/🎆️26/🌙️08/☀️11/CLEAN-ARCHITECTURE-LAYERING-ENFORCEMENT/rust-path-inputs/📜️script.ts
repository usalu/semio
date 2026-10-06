/** 🧭️ Runs the owned Rust path argument corpus from its canonical General domain. */
import { resolve } from "node:path";
const root = resolve(import.meta.dir, "../../../../../../../..");
const command = process.argv.slice(2);
const suites: Record<string, string> = {
  "test-current": "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/📁️paths/🧪️tests/🟦️.ts",
  "test-policy": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📁️runtime/🧪️tests/🟦️.ts",
  "test-execution": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📁️runtime/🧪️tests/🏘️execution/🟦️.ts",
};
if (command.length !== 1 || !suites[command[0]!]) throw Error("Expected test-current, test-policy or test-execution");
const child = Bun.spawn([process.execPath, "test", resolve(root, suites[command[0]!]!)], { cwd: root, env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: resolve(import.meta.dir, "../🗑️generated/source-projection/runtime-census") }, stdin: "ignore", stdout: "inherit", stderr: "inherit" });
process.exitCode = await child.exited;
