#!/usr/bin/env bun
/** 📦️ wfc-grid2d Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-wfc-grid2d", { packages: ["semio-s-artifact-wfc-grid2d", "semio-s-artifact-wfc-grid2d-create-tile", "semio-s-artifact-wfc-grid2d-diff", "semio-s-artifact-wfc-grid2d-inferences", "semio-s-artifact-wfc-grid2d-schema", "semio-s-artifact-wfc-grid2d-snapshot", "semio-s-artifact-wfc-grid2d-tests-cuts-the-pinned-corner-out-of-the-problem", "semio-s-artifact-wfc-grid2d-v1"], snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"], snapshotSqliteTestBudgetMs: 120000 });
