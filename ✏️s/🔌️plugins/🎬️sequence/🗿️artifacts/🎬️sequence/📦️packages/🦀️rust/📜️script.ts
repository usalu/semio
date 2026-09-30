#!/usr/bin/env bun
/** 📦️ sequence sequence Rust artifact package router. */
import { resolve } from "node:path";
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts";
import { sequenceSnapshotFixtureAssetSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️snapshot-fixture-asset/🟦️.ts";
import { testArtifactIoDescriptorParity } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️descriptor-parity/🟦️.ts";
const segments = process.argv.slice(2);
if (segments[0] === "test" && segments[1] === "io-descriptor-parity") {
  testArtifactIoDescriptorParity();
  if (segments[2] !== "oracle") {
    const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
    await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-sequence-sequence", "--lib", "artifact_io_", "--", "--nocapture"], resolve(import.meta.dir, "../../../../../../.."));
  }
} else await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-sequence-sequence", {
  twins: [{ name: "sequence-snapshot-fixture-asset", run: sequenceSnapshotFixtureAssetSelfTests }, { name: "artifact-io-descriptor-parity", run: testArtifactIoDescriptorParity }],
});
