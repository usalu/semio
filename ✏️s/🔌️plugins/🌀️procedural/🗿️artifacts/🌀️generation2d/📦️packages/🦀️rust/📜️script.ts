#!/usr/bin/env repo
/** 📦️ procedural-generation2d Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { generation2dSnapshotFixtureAssetSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️snapshot-fixture-asset/🟦️.ts";
import { generation2dGestureLeafTwinSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧪️gesture-leaves/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedVerifyScript, familyPackages } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-procedural-generation2d", { packages: familyPackages, ...{
  twins: [{ name: "generation2d-snapshot-fixture-asset", run: generation2dSnapshotFixtureAssetSelfTests }, { name: "generation2d-gesture-leaves", run: generation2dGestureLeafTwinSelfTests }],
}, snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"], snapshotSqliteTestBudgetMs: 120000, commands: router });

