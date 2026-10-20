
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedVerifyScript, runMaterialNativeScript, runFixedDofScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** 🏛️ Checks portable event/window oracles and exact native transform, removal, and history laws. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🧱️ Runs actual sealed Material receiving and persistent table laws with independent SQLite ranks. */
class MaterialNativeScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runMaterialNativeScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
/** 🔒️ Verifies the shared finite restraint boundary at both actual model owners. */
class FixedDofScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runFixedDofScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript).register("test-material", MaterialNativeScript).register("test-fixed-dofs", FixedDofScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-fem-2d", { commands: router, snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"], snapshotSqliteTestBudgetMs: 120000 });
