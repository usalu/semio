/** 🏃️ Runs the browser bundle's component isolation laws through their explicit test owner. */

import { lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import ts from "typescript";
import { exactExecutableFingerprint } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import type { ScriptInvocation } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createBrowserBundleTests } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🟦️.ts";
import { browserActorAsyncImports, browserActorImportAdmissionV1, browserActorInterfaces, browserBundleValidator, buildBrowserCodegenModule, buildClosedBrowserActorArtifactOwned, buildClosedBrowserActorArtifactV1, captureBrowserActorRuntime, captureBrowserCodegenSources, closeBrowserCodegenModule, closedBrowserActorBundle, closedBrowserActorBundleFromRuntime, closedBrowserComponentFactory, parseBrowserActorCodegenManifest, sealBrowserCodegenPolicy, validateAsyncTaskReturnLift } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts";

/** 🧰️ Builds the component isolation laws over the live bindings of the bundle script. */
export function browserBundleTestOwners() {
  return createBrowserBundleTests({ browserActorAsyncImports, browserActorImportAdmissionV1, browserActorInterfaces, browserBundleValidator, buildBrowserCodegenModule, buildClosedBrowserActorArtifactOwned, buildClosedBrowserActorArtifactV1, captureBrowserActorRuntime, captureBrowserCodegenSources, closeBrowserCodegenModule, closedBrowserActorBundle, closedBrowserActorBundleFromRuntime, closedBrowserComponentFactory, validateAsyncTaskReturnLift, dirname, exactExecutableFingerprint, join, lstatSync, mkdirSync, mkdtempSync, parseBrowserActorCodegenManifest, readdirSync, readFileSync, realpathSync, renameSync, sealBrowserCodegenPolicy, ts, writeFileSync }, { directory: resolve("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏃️execution", ".."), url: pathToFileURL(resolve("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏃️execution", "../📜️script.ts")).href });
}

/** 🧪️ Executes existing component isolation laws through their explicit test owner. */
export async function testClosedBrowserComponentFactory(repoRoot: string, invocation: ScriptInvocation): Promise<void> {
  await browserBundleTestOwners().testClosedBrowserComponentFactory(repoRoot, invocation);
}

/** 🧾️ Executes actual closed actor producer observations and independent JCO byte oracles. */
export async function testClosedBrowserActorProducerV1(repoRoot: string, invocation: ScriptInvocation): Promise<void> {
  await browserBundleTestOwners().testClosedBrowserActorBundle(repoRoot, invocation);
}
