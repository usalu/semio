import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { resolve } from "node:path";
import { runCmd } from "@semio-tech/framework/process/execution";

const suites = ["🗿️artifacts/**/📚️examples/**/🧪️tests/🧩️example/🟦️.ts", "🗿️artifacts/**/🧬️schema/🧪️tests/🧩️suite/🟦️.ts"];

/** 🏃️ Runs the `TestScript` command under the caller's roots and invocation. */
export async function runTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>): Promise<void> {
  const plugin = resolve(context.root, "../..");
  const files = suites.flatMap(pattern => [...new Bun.Glob(pattern).scanSync({ cwd: plugin })].map(path => resolve(plugin, path))).sort();
  runCmd(process.execPath, ["test", ...files], { cwd: context.repoRoot });
}
