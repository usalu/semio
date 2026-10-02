#!/usr/bin/env bun
/** 🧩️ Publishes and tests the EN1994 TypeScript owner's exact snapshot facets. */
import {runArtifactTypeScriptPackageMain} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";
await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/norm-en1994",{suites:["🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"]});
