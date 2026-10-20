import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { resolveTestLevel } from "@semio-tech/framework/process/test-budget";
import { resolve } from "node:path";
import { runCmd } from "@semio-tech/framework/process/execution";

/** 🏃️ Runs the `TestScript` command under the caller's roots and invocation. */
export async function runTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  resolveTestLevel(segments, "quick");
  runCmd(process.execPath, ["test", ...["✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts","✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts"].map(path => resolve(context.repoRoot, path))], { cwd: context.repoRoot });
 console.log("layout ts ok"); }
