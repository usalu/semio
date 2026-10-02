#!/usr/bin/env bun
/** ⚡️ Publishes and tests the DIN18599 TypeScript owner's exact snapshot facets. */
import {runArtifactTypeScriptPackageMain} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";
await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/norm-din18599",{suites:["🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"]});
