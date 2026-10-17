#!/usr/bin/env bun
/** 📦️ ply Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-ply", { packages: ["semio-s-artifact-stdio-ply", "semio-s-artifact-stdio-ply-inferences", "semio-s-artifact-stdio-ply-io-registry", "semio-s-artifact-stdio-ply-mutations", "semio-s-artifact-stdio-ply-schema"], snapshotSqliteTests: ["../../🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
