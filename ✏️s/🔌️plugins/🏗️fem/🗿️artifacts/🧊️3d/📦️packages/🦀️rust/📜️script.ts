#!/usr/bin/env bun
/** 📦️ fem-3d Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runCmd, runCargo, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
if (segments[0] === "fem3d-numerical-child-native") {
      const { testFem3dNumericalCloseOwners } = await import("../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🧪️tests/📦️numerical-close/🟦️.ts");
      testFem3dNumericalCloseOwners();
      const { testFem3dMountedStiffnessOracle } = await import("../../../../../../🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🧪️tests/🧱️mounted-stiffness/🟦️.ts");
      testFem3dMountedStiffnessOracle();
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", join(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🧪️tests/🧱️mounted-stiffness/🟦️.ts"), join(this.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🧪️tests/📦️numerical-close/🟦️.ts")], { cwd: this.repoRoot });
      const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-2d", "--lib", "mounted_3d_element_interfaces_", "--", "--nocapture", "--test-threads=1"], this.repoRoot);
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-3d", "--lib", "--", "--nocapture", "--test-threads=1", "live_visual::tests::", "mounted_3d_element_interfaces_"], this.repoRoot);
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-fem-3d", { packages: ["semio-s-artifact-fem-3d", "semio-s-artifact-fem-3d-analysis", "semio-s-artifact-fem-3d-boundary", "semio-s-artifact-fem-3d-editor", "semio-s-artifact-fem-3d-editor-commands", "semio-s-artifact-fem-3d-editor-interaction", "semio-s-artifact-fem-3d-editor-model-window", "semio-s-artifact-fem-3d-editor-panels", "semio-s-artifact-fem-3d-editor-results-window", "semio-s-artifact-fem-3d-editor-session", "semio-s-artifact-fem-3d-editor-terminology", "semio-s-artifact-fem-3d-editor-visual-job", "semio-s-artifact-fem-3d-engine", "semio-s-artifact-fem-3d-io", "semio-s-artifact-fem-3d-load", "semio-s-artifact-fem-3d-material", "semio-s-artifact-fem-3d-mesh", "semio-s-artifact-fem-3d-mesh-topology", "semio-s-artifact-fem-3d-model", "semio-s-artifact-fem-3d-mutations", "semio-s-artifact-fem-3d-scene", "semio-s-artifact-fem-3d-viewer"], commands: { verify: OwnedVerifyScript }, snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"], snapshotSqliteTestBudgetMs: 120000 });
