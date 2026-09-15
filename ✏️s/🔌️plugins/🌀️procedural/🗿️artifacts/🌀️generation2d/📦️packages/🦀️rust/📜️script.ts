#!/usr/bin/env bun
/** 📦️ procedural-generation2d Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts";
import { generation2dSnapshotFixtureAssetSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️snapshot-fixture-asset/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-procedural-generation2d", {
  twins: [{ name: "generation2d-snapshot-fixture-asset", run: generation2dSnapshotFixtureAssetSelfTests }],
});
