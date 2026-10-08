#!/usr/bin/env bun
/** 🏢️ BIM model owning SQLite and Rust package commands. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import {runCmd} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import {BundleScript} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {resolve} from "node:path";
/** 📏️ Checks the defining Source schema and owning provider without publishing artifacts. */
class SqliteVerify extends BundleScript { run(args:string[]):void{if(args.length!==1||args[0]!=="source")throw Error("verify-snapshot-sqlite source");const owner=resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any");runCmd("bun",[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--skipLibCheck",resolve(owner,"🧬️schema/🟦️.ts"),resolve(owner,"🧬️schema/📸️snapshot/🟦️.ts"),resolve(owner,"🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts"),resolve(owner,"🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts")],{cwd:this.repoRoot});}}
await runArtifactRustPackageMain(import.meta.dir,"semio-s-artifact-bim-model",{commands:{"verify-snapshot-sqlite":SqliteVerify},snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});
