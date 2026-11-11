#!/usr/bin/env repo
/** 🪨️ Publishes and tests the EN1996 TypeScript owner's exact snapshot facets. */
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🟦️typescript/🟦️.ts";
await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/norm-en1996",{bundleRuntime:"runtime",suites:["🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});
