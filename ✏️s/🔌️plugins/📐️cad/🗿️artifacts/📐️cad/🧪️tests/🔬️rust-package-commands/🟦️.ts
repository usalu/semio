import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runCmd } from "@semio-tech/framework/process/execution";
import { runCargo } from "@semio-tech/framework/process/execution";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { runRepositoryCommand } from "@semio-tech/framework/process/execution";
import { resolve as scriptDirectoryResolve } from "node:path";

const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dir, "../../📦️packages/🦀️rust");

const familyRoot = resolve(SCRIPT_DIR, "../..");
export const familyPackages = (Bun.TOML.parse(readFileSync(join(familyRoot, "Cargo.toml"), "utf8")) as { workspace: { members: string[] } }).workspace.members.map((member) => (Bun.TOML.parse(readFileSync(join(familyRoot, member, "Cargo.toml"), "utf8")) as { package: { name: string } }).package.name);

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length === 1 && segments[0] === "activation-source") {
    const tests = resolve(context.root, "../../🧪️tests/🚀️activation/🟦️.ts");
    await runRepositoryCommand(context.invocation,process.execPath,["test", tests],context.repoRoot,"cad-owned-activation-source",{version:1,maximumElapsedMilliseconds:0,output:null});
    return;
  }
  if (segments.length === 1 && segments[0] === "reference-owner-syntax") {
    const tests = resolve(context.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📎️references/🧪️tests/🟦️.ts");
    await runRepositoryCommand(context.invocation,process.execPath,["test", tests],context.repoRoot,"cad-unmounted-reference-owner-syntax",{version:1,maximumElapsedMilliseconds:0,output:null});
    return;
  }
  if (segments.length === 1 && segments[0] === "owned-target") {
    const tests = resolve(context.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🪪️owned-target/🟦️.ts");
    await runRepositoryCommand(context.invocation,process.execPath,["test", tests],context.repoRoot,"cad-owned-target-admission",{version:1,maximumElapsedMilliseconds:0,output:null});
    return;
  }
  if (segments.length === 1 && segments[0] === "snapshot-sqlite-source") {
    const subset = resolve(context.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any");
    const sources = ["🧬️schema/📸️snapshot/🟦️.ts", "🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts", "🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"].map(file => resolve(subset, file));
    await runRepositoryCommand(context.invocation,process.execPath,[resolve(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--skipLibCheck", ...sources],context.repoRoot,"cad-snapshot-sqlite-public-types",{version:1,maximumElapsedMilliseconds:0,output:null});
    return;
  }
if (segments[0] === "cad-document-contract") {
    const { testCadDocumentContractOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document/🟦️.ts");
    testCadDocumentContractOracle();
    const { testCadWorldWindowTransientContract } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🫧️transient/🧬️schema/🧪️tests/🪪️document/🟦️.ts");
    console.log(`cad-window-transient-contract rows=${testCadWorldWindowTransientContract()}`);
    const schemaRoot = join(context.repoRoot, "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧪️tests/🪪️document/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/framework/process/execution");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-cad-cad-model", "--test", "cad_model_document_contract", "--", "--nocapture"], context.repoRoot);
    }
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
