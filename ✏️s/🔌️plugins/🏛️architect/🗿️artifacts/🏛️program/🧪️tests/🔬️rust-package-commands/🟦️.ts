import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runCmd } from "@semio-tech/framework/process/execution";
import { runCargo } from "@semio-tech/framework/process/execution";
import { runRepositoryCommand } from "@semio-tech/framework/process/execution";
import { join, resolve } from "node:path";
import { runOwnedCommand } from "@semio-tech/framework/process/execution";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length === 1 && segments[0] === "physical-codecs") {
    await runOwnedCommand(process.execPath, ["test", resolve(context.root, "./../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔣️transport/🟦️.ts")], context.repoRoot, "owned-physical-codecs", 120_000);
    return;
  }

  if (segments.length === 1 && segments[0] === "snapshot-sqlite-source") {
    const snapshot = resolve(context.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
    await runRepositoryCommand(context.invocation,process.execPath,[resolve(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--skipLibCheck", resolve(snapshot, "🟦️.ts"), resolve(snapshot, "../../🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts"), resolve(snapshot, "../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts")],context.repoRoot,"architect-snapshot-sqlite-public-types",{version:1,maximumElapsedMilliseconds:0,output:null});
    return;
  }

if (segments[0] === "architect-window-ownership") {
    const windowsRoot = join(context.repoRoot, "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows");
    const configRoots = ["📋️register", "↔️adjacency", "🕸️graph", "📓️report"].map((window) => join(windowsRoot, window, "🎚️config"));
    const oracle = join(configRoots[0]!, "🧪️tests/🔬️window-ownership/🟦️.ts");
    const { testArchitectWindowOwnershipOracle } = await import(oracle);
    testArchitectWindowOwnershipOracle();
    runCmd(
      "bun",
      [
        join(context.repoRoot, "node_modules/typescript/bin/tsc"),
        "--noEmit",
        "--strict",
        "--target",
        "ESNext",
        "--module",
        "ESNext",
        "--moduleResolution",
        "bundler",
        "--resolveJsonModule",
        "--allowImportingTsExtensions",
        "--esModuleInterop",
        "--skipLibCheck",
        ...configRoots.map((root) => join(root, "🧬️schema/🟦️.ts")),
        oracle,
      ],
      { cwd: context.repoRoot },
    );
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/framework/process/execution");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-architect-program", "--lib", "architect_window_ownership_", "--", "--nocapture"], context.repoRoot);
    }
    return;
  }
if (segments[0] === "program-document-contract") {
    const { testProgramDocumentContract } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️document-contract/🟦️.ts");
    testProgramDocumentContract();
    const schemaRoot = join(context.repoRoot, "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/framework/process/execution");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-architect-program", "--lib", "program_document_contract", "--", "--nocapture"], context.repoRoot);
    }
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}

/** 🏃️ Runs the `RegisterCatalogScript` command under the caller's roots and invocation. */
export async function runRegisterCatalogScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args: string[]): Promise<void> {
  if (args.length > 1 || (args.length === 1 && args[0] !== "--check")) throw new Error("generate-register-catalog [--check]");
  const { generateCollectionCatalogV1 } = await import("@semio-tech/framework/process/execution");
  const stale = generateCollectionCatalogV1(join(context.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json"), args[0] === "--check" ? "check" : "write");
  if (args[0] === "--check" && stale.length) throw new Error(`register catalog is stale: ${stale.join(", ")}`);
  console.log(`[register-catalog] ${stale.length ? `wrote ${stale.length}` : "current"}`);
}

/** 🏃️ Runs the `SnapshotSourceProfileScript` command under the caller's roots and invocation. */
export async function runSnapshotSourceProfileScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args: string[]): Promise<void> {
  if(args.length)throw new Error("test-snapshot-sqlite-source-profile accepts no arguments");
  const {runRepositoryTestCommand}=await import("@semio-tech/framework/process/test-execution");
  await runRepositoryTestCommand(process.execPath,["test",join(context.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"),"--test-name-pattern","Architect complete sixty-five typed bodies"],{cwd:context.repoRoot});
}
