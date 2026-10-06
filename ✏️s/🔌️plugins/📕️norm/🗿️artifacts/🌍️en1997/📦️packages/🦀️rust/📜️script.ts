#!/usr/bin/env bun
/** 📦️ norm artifact Rust package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import {BundleScript} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {runRepositoryCommand} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import {resolve} from "node:path";
/** 🛂️ Checks the actual EN1997 facade and complete independent SQLite oracle types. */
class VerifyScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||segments[0]!=="snapshot-sqlite-source")throw Error("Unknown EN1997 owned verification");
  const snapshot=resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
  await runRepositoryCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--skipLibCheck",resolve(snapshot,"🟦️.ts"),resolve(snapshot,"🧪️tests/🪶️sqlite/🟦️.ts")],this.repoRoot,"en1997-snapshot-sqlite-public-types");
 }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-norm-en1997",{commands:{verify:VerifyScript},snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪪️capability/🟦️.ts","../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});
