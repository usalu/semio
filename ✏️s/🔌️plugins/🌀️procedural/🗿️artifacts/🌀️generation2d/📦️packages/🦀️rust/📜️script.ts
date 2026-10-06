#!/usr/bin/env bun
/** 📦️ procedural-generation2d Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { generation2dSnapshotFixtureAssetSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️snapshot-fixture-asset/🟦️.ts";
import { generation2dGestureLeafTwinSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧪️gesture-leaves/🟦️.ts";
import { runCmd, runCargo, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
if (segments[0] === "generation2d-window-camera-ownership") {
      const editorRoot = join(this.repoRoot, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor");
      const oracle = join(editorRoot, "🧪️tests/🪟️generation2d-window-camera-ownership/🟦️.ts");
      const { testGeneration2dWindowCameraOwnershipOracle } = await import(oracle);
      testGeneration2dWindowCameraOwnershipOracle();
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", join(editorRoot, "🎭️modes/✏️edit/🪟️windows/🕸️flow/🎚️config/🧬️schema/🟦️.ts"), join(editorRoot, "🎭️modes/✏️edit/🪟️windows/👁️preview/🎚️config/🧬️schema/🟦️.ts"), join(editorRoot, "🎭️modes/🧬️generate/🪟️windows/👁️preview/🎚️config/🧬️schema/🟦️.ts"), oracle], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-generation2d", "--features", "component-app-assembly", "--lib", "generation2d_window_camera_ownership", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-procedural-generation2d", { ...{
  testFeatures: ["component-app-assembly"],
  twins: [{ name: "generation2d-snapshot-fixture-asset", run: generation2dSnapshotFixtureAssetSelfTests }, { name: "generation2d-gesture-leaves", run: generation2dGestureLeafTwinSelfTests }],
}, snapshotSqliteTestFeatures: ["component-app-assembly"], snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"], snapshotSqliteTestBudgetMs: 120000, commands: { verify: OwnedVerifyScript } });

