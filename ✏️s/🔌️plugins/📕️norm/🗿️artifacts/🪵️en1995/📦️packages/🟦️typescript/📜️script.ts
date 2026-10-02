#!/usr/bin/env bun
/** 🪵️ Publishes and tests the EN1995 TypeScript owner's exact snapshot facets. */
import {runArtifactTypeScriptPackageMain} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";
await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/norm-en1995",{suites:["🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"]});
