#!/usr/bin/env bun
/** 📦️ procedural-generation3d Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts";
import { generation3dTerminologySelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️unit/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-procedural-generation3d", {
  testFeatures: ["component-app-assembly"],
  twins: [{ name: "generation3d-terminology", run: generation3dTerminologySelfTests }],
});
