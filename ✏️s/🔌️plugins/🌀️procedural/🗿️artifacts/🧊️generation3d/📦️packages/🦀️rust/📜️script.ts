#!/usr/bin/env bun
/** 📦️ procedural-generation3d Rust artifact package router. */
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { generation3dTerminologySelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️unit/🟦️.ts";
import { generation3dSnapshotFixtureAssetSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️snapshot-fixture-asset/🟦️.ts";
import { generation3dGraphKeyboardSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️navigate-graph/🧪️tests/🔬️unit/🟦️.ts";
import { generation3dMeshSelectionSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🥽️edit-mesh-selection/🧪️tests/🔬️unit/🟦️.ts";
import { generation3dWidgetInputSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🧪️tests/🔬️unit/🟦️.ts";
import { generation3dWidgetCreationSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️add-widget/🧪️tests/🔬️unit/🟦️.ts";
import { resolve } from "node:path";
import { testGeneration3dIoInputContracts } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🗿️artifact-surface/🟦️.ts";
import { testGeneration3dIoAuthorityFixture } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔁️round-trip/🛡️authority/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCmd, runCargo, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "generation3d-widget-inputs") {
      console.log(`generation3d-widget-input checks=${generation3dWidgetInputSelfTests()} independentAjv=true`);
      console.log(`generation3d-widget-creation checks=${generation3dWidgetCreationSelfTests()} independentAjv=true`);
      console.log(`generation3d-terminology checks=${generation3dTerminologySelfTests()}`);
      const testRoot = join(this.repoRoot, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input");
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", "--resolveJsonModule", `${testRoot}/🧪️tests/🔬️unit/🟦️.ts`], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "semantic-wire") {
      const { assertGeneration3dSemanticWire } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🧬️semantic-wire/🟦️.ts");
      console.log(`generation3d-semantic-wire checks=${assertGeneration3dSemanticWire()} independentAjv=true`);
      if (segments[1] === "native") await runCargo(["test", "-p", "semio-s-artifact-procedural-generation3d", "--lib", "semantic_wire_vectors", "--", "--nocapture"], this.repoRoot);
      return;
    }
if (segments[0] === "generation3d-document-io") {
      const testRoot = join(this.repoRoot, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io");
      const { testGeneration3dDocumentIoSurface, verifyPreparedGltfExportOracle } = await import(`${testRoot}/🧪️tests/🗿️artifact-surface/🟦️.ts`);
      testGeneration3dDocumentIoSurface();
      const preparedOracle = process.env.SEMIO_TEST_ARTIFACT_DIR && join(process.env.SEMIO_TEST_ARTIFACT_DIR, "prepared-rich.gltf");
      if (preparedOracle && existsSync(preparedOracle)) await verifyPreparedGltfExportOracle(preparedOracle);
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🧪️tests/🗿️artifact-surface/🟦️.ts`], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        await runArtifactRustTests("semio-s-artifact-procedural-generation3d", this.repoRoot, ["quick", "--offline", "--test", "io-round-trip", "artifact_surface"]);
        await runArtifactRustTests("semio-s-artifact-procedural-generation3d", this.repoRoot, ["quick", "--offline", "--lib", "document_io"], ["component-app-assembly"]);
      }
      return;
    }
if (segments[0] === "generation3d-preview-window-transient") {
      const testRoot = join(this.repoRoot, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient");
      const { testGeneration3dPreviewWindowTransientContract } = await import(`${testRoot}/🧪️tests/🔬️contract/🟦️.ts`);
      testGeneration3dPreviewWindowTransientContract();
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🧬️schema/🟦️.ts`, `${testRoot}/🧪️tests/🔬️contract/🟦️.ts`], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-generation3d", "--features", "component-app-assembly", "--lib", "preview_eval_", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}

if (process.argv[2] === "canonical-io") {
  console.log(`generation3d-io-authority checks=${testGeneration3dIoAuthorityFixture()}`);
  console.log(`generation3d-io-input-contracts checks=${testGeneration3dIoInputContracts()}`);
  await runArtifactRustTests("semio-s-artifact-procedural-generation3d", resolve(import.meta.dir, "../../../../../../.."), ["quick", "--locked", "--test", "io-round-trip"]);
} else await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-procedural-generation3d", { ...{
  testFeatures: ["component-app-assembly"],
  snapshotSqliteTestFeatures: ["component-app-assembly"], snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"],
  snapshotSqliteTestBudgetMs: 120000,
  twins: [
    { name: "generation3d-mesh-selection", run: generation3dMeshSelectionSelfTests },
    { name: "generation3d-widget-input", run: generation3dWidgetInputSelfTests },
    { name: "generation3d-widget-creation", run: generation3dWidgetCreationSelfTests },
    { name: "generation3d-terminology", run: generation3dTerminologySelfTests },
    { name: "generation3d-snapshot-fixture-asset", run: generation3dSnapshotFixtureAssetSelfTests },
    { name: "generation3d-graph-keyboard", run: generation3dGraphKeyboardSelfTests },
  ],
}, commands: { verify: OwnedVerifyScript } });
