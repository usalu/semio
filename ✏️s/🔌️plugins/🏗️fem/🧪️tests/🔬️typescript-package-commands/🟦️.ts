import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

/** 🏃️ Runs the `CombinationStressScript` command under the caller's roots and invocation. */
export async function runCombinationStressScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length) throw new Error("verify-combination-stress accepts no arguments");
  const source = resolve(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/⚖️combination-stress/🟦️.ts");
  const { verifyCombinationStressOracle } = await import(pathToFileURL(source).href);
  verifyCombinationStressOracle();
}
