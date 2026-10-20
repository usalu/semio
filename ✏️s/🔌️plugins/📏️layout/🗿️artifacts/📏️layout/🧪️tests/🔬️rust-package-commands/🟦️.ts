import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runCmd } from "@semio-tech/framework/process/execution";
import { runCargo } from "@semio-tech/framework/process/execution";
import { scriptInvocationBudget, scriptInvocationEnvironment } from "@semio-tech/framework/process/routing";
import { join, resolve } from "node:path";
import { runOwnedCommand } from "@semio-tech/framework/process/execution";
import { resolveTypescriptCompilerV1 } from "@semio-tech/framework/process/execution";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length === 1 && segments[0] === "physical-codecs") {
    await runOwnedCommand(process.execPath, ["test", resolve(context.root, "./../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔣️transport/🟦️.ts")], context.repoRoot, "owned-physical-codecs", 120_000);
    return;
  }

if (segments[0] === "layout-window-ownership") {
    const windowsRoot = join(context.repoRoot, "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows");
    const configRoot = join(windowsRoot, "📐️blueprint/🎚️config");
    const { testLayoutWindowOwnershipOracle } = await import(`${configRoot}/🧪️tests/🔬️window/🟦️.ts`);
    testLayoutWindowOwnershipOracle();
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", join(configRoot, "🧬️schema/🟦️.ts"), join(windowsRoot, "📐️blueprint/🫧️transient/🧬️schema/🟦️.ts"), join(configRoot, "🧪️tests/🔬️window/🟦️.ts")], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/framework/process/execution");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-layout-layout", "--lib", "layout_window_ownership_", "--", "--nocapture"], context.repoRoot);
    }
    return;
  }
if (segments[0] === "layout-document-contract") {
    const schemaRoot = join(context.repoRoot, "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "../🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/framework/process/execution");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-layout-layout", "--lib", "layout_document_contract", "--", "--nocapture"], context.repoRoot);
    }
    return;
  }
if (segments[0] === "layout-frame-selection") {
    const mutationsRoot = join(context.repoRoot, "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
    runCmd("bun", ["test", join(mutationsRoot, "🧪️tests/🧪️frame-selection/🟦️.ts")], { cwd: context.repoRoot });
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", "--types", "bun", join(mutationsRoot, "🟦️.ts"), join(mutationsRoot, "🧪️tests/🧪️frame-selection/🟦️.ts")], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-layout-layout", "--lib", "frames", "--", "--nocapture"], context.repoRoot);
    }
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}

/** 🏃️ Runs the `MediaAdoptionSourceScript` command under the caller's roots and invocation. */
export async function runMediaAdoptionSourceScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length) throw new Error("test-media-adoption-source accepts no arguments");
  const source = resolve(context.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🟦️.ts");
  const phases = [
    { name: "strict", budget: 120_000, args: [resolveTypescriptCompilerV1(context.repoRoot), "--noEmit", "--strict", "--noUncheckedIndexedAccess", "--skipLibCheck", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--types", "bun", source] },
    { name: "source", budget: 15_000, args: ["test", source] },
  ];
  for (const phase of phases) await runOwnedCommand(process.execPath, phase.args, context.repoRoot, `layout:media-adoption:${phase.name}`, scriptInvocationBudget(context.invocation, phase.budget), {
    signal: context.invocation.control.signal,
    env: scriptInvocationEnvironment(context.invocation, process.env),
    onProgress: () => context.invocation.control.publish({ owner: context.invocation.policy.owner, command: "test-media-adoption-source", stage: "running" }),
  });
}
