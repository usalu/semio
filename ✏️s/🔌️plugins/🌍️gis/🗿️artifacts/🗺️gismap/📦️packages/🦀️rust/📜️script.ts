#!/usr/bin/env bun
/** 📦️ gis-gismap Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts";
if (process.argv[2] === "test") process.env.RUST_MIN_STACK ??= "268435456";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-gis-gismap");
