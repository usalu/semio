#!/usr/bin/env bun
/** 📦️ procedural-generation3d Rust artifact package router. */
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts";
import { generation3dTerminologySelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️unit/🟦️.ts";
import { generation3dSnapshotFixtureAssetSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️snapshot-fixture-asset/🟦️.ts";
import { generation3dGraphKeyboardSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️navigate-graph/🧪️tests/🔬️unit/🟦️.ts";
import { generation3dWidgetCreationSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️add-widget/🧪️tests/🔬️unit/🟦️.ts";
import { resolve } from "node:path";
import { testGeneration3dIoInputContracts } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🗿️artifact-surface/🟦️.ts";
import { testGeneration3dIoAuthorityFixture } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔁️round-trip/🛡️authority/🟦️.ts";
if (process.argv[2] === "canonical-io") {
  console.log(`generation3d-io-authority checks=${testGeneration3dIoAuthorityFixture()}`);
  console.log(`generation3d-io-input-contracts checks=${testGeneration3dIoInputContracts()}`);
  await runArtifactRustTests("semio-s-artifact-procedural-generation3d", resolve(import.meta.dir, "../../../../../../../"), ["quick", "--locked", "--test", "io-round-trip"]);
} else await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-procedural-generation3d", {
  testFeatures: ["component-app-assembly"],
  twins: [
    { name: "generation3d-widget-creation", run: generation3dWidgetCreationSelfTests },
    { name: "generation3d-terminology", run: generation3dTerminologySelfTests },
    { name: "generation3d-snapshot-fixture-asset", run: generation3dSnapshotFixtureAssetSelfTests },
    { name: "generation3d-graph-keyboard", run: generation3dGraphKeyboardSelfTests },
  ],
});
