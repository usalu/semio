#!/usr/bin/env bun
/** 🌬️ Publishes and tests the DIN16798 TypeScript owner's exact snapshot facets. */
import {runArtifactTypeScriptPackageMain} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";
await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/norm-din16798",{suites:["🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});
