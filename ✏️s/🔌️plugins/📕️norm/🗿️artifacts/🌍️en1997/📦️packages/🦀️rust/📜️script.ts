#!/usr/bin/env bun
/** 📦️ norm artifact Rust package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import {resolve} from "node:path";
/** 🛂️ Checks the actual EN1997 facade and complete independent SQLite oracle types. */
class VerifyScript extends BundleScript{
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||segments[0]!=="snapshot-sqlite-source")throw Error("Unknown EN1997 owned verification");
  const snapshot=resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
  await runRepositoryCommand(this.invocation,process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--skipLibCheck",resolve(snapshot,"🟦️.ts"),resolve(snapshot,"🧪️tests/🪶️sqlite/🟦️.ts")],this.repoRoot,"en1997-snapshot-sqlite-public-types",{version:1,maximumElapsedMilliseconds:0,output:null});
 }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-norm-en1997", {packages:["semio-s-artifact-norm-en1997-checks","semio-s-artifact-norm-en1997-diff","semio-s-artifact-norm-en1997-editor","semio-s-artifact-norm-en1997-examples","semio-s-artifact-norm-en1997-inferences","semio-s-artifact-norm-en1997-io","semio-s-artifact-norm-en1997-model","semio-s-artifact-norm-en1997-mut-design","semio-s-artifact-norm-en1997-mut-foundations","semio-s-artifact-norm-en1997-mut-ground","semio-s-artifact-norm-en1997-mutations","semio-s-artifact-norm-en1997-schema","semio-s-artifact-norm-en1997-viewer","semio-s-artifact-norm-en1997"],commands:{verify:VerifyScript},snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪪️capability/🟦️.ts","../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});
