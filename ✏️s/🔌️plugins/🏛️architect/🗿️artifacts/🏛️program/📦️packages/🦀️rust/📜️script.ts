#!/usr/bin/env bun
/** 📦️ architect program Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedVerifyScript, runRegisterCatalogScript, runSnapshotSourceProfileScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🏭️ Generates (or with \`--check\` verifies) the Rust and TypeScript register catalog from the \`x-semio-collection-catalog\` of its snapshot schema \`🧬️schema/📸️snapshot/🔣️.json\`. */
class RegisterCatalogScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runRegisterCatalogScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}

/** ⏱️ Selects the unchanged complete-domain Source law for operation timing. */
class SnapshotSourceProfileScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runSnapshotSourceProfileScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}

const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript).register("generate-register-catalog", RegisterCatalogScript).register("test-snapshot-sqlite-source-profile", SnapshotSourceProfileScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-architect-program", { packages: ["semio-s-artifact-architect-program", "semio-s-artifact-architect-program-architect", "semio-s-artifact-architect-program-catalog", "semio-s-artifact-architect-program-config", "semio-s-artifact-architect-program-context-requirements", "semio-s-artifact-architect-program-inferences", "semio-s-artifact-architect-program-mut-any", "semio-s-artifact-architect-program-mutations", "semio-s-artifact-architect-program-panels", "semio-s-artifact-architect-program-presence", "semio-s-artifact-architect-program-program-structure", "semio-s-artifact-architect-program-registers", "semio-s-artifact-architect-program-schema", "semio-s-artifact-architect-program-snapshot", "semio-s-artifact-architect-program-tests", "semio-s-artifact-architect-program-tests-2", "semio-s-artifact-architect-program-tests-3", "semio-s-artifact-architect-program-tests-exchange"], commands: router, snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });


