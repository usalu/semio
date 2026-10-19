#!/usr/bin/env bun
/** 📦️ cad cad Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runCmd } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { runCargo } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { runVitest } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length === 1 && segments[0] === "activation-source") {
      const tests = resolve(this.root, "../../🧪️tests/🚀️activation/🟦️.ts");
      await runRepositoryCommand(this.invocation,process.execPath,["test", tests],this.repoRoot,"cad-owned-activation-source",{version:1,maximumElapsedMilliseconds:0,output:null});
      return;
    }
    if (segments.length === 1 && segments[0] === "reference-owner-syntax") {
      const tests = resolve(this.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📎️references/🧪️tests/🟦️.ts");
      await runRepositoryCommand(this.invocation,process.execPath,["test", tests],this.repoRoot,"cad-unmounted-reference-owner-syntax",{version:1,maximumElapsedMilliseconds:0,output:null});
      return;
    }
    if (segments.length === 1 && segments[0] === "owned-target") {
      const tests = resolve(this.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🪪️owned-target/🟦️.test.ts");
      await runRepositoryCommand(this.invocation,process.execPath,["test", tests],this.repoRoot,"cad-owned-target-admission",{version:1,maximumElapsedMilliseconds:0,output:null});
      return;
    }
    if (segments.length === 1 && segments[0] === "snapshot-sqlite-source") {
      const subset = resolve(this.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any");
      const sources = ["🧬️schema/📸️snapshot/🟦️.ts", "🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts", "🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"].map(file => resolve(subset, file));
      await runRepositoryCommand(this.invocation,process.execPath,[resolve(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--skipLibCheck", ...sources],this.repoRoot,"cad-snapshot-sqlite-public-types",{version:1,maximumElapsedMilliseconds:0,output:null});
      return;
    }
if (segments[0] === "cad-document-contract") {
      const { testCadDocumentContractOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document/🟦️.ts");
      testCadDocumentContractOracle();
      const { testCadWorldWindowTransientContract } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🫧️transient/🧬️schema/🧪️tests/🪪️document/🟦️.ts");
      console.log(`cad-window-transient-contract rows=${testCadWorldWindowTransientContract()}`);
      const schemaRoot = join(this.repoRoot, "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧪️tests/🪪️document/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-cad-cad-model", "--test", "cad_model_document_contract", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}

const familyRoot = resolve(import.meta.dir, "../..");
const familyPackages = (Bun.TOML.parse(readFileSync(join(familyRoot, "Cargo.toml"), "utf8")) as { workspace: { members: string[] } }).workspace.members.map((member) => (Bun.TOML.parse(readFileSync(join(familyRoot, member, "Cargo.toml"), "utf8")) as { package: { name: string } }).package.name);

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-cad-cad", { packages: familyPackages, commands: { verify: OwnedVerifyScript }, snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
