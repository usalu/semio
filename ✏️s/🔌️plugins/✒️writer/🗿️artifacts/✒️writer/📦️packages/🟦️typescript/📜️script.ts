#!/usr/bin/env bun
/** 📦️ Writer artifact TypeScript public package and semantic snapshot suite. */
import {runArtifactTypeScriptPackageMain} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";
import {readdirSync} from "node:fs";
import {join,resolve} from "node:path";
import {runRepositoryCommand} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
if(process.argv[2]==="verify"&&process.argv[3]==="snapshot-sqlite-source"){
 const workspace=resolve(import.meta.dir,"../../../../../../.."),roots=[
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards",
  "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards"
 ],files:string[]=[],pending=roots.map(root=>join(workspace,root));
 while(pending.length){const directory=pending.pop()!;for(const entry of readdirSync(directory,{withFileTypes:true})){const path=join(directory,entry.name);if(entry.isDirectory()&&entry.name!=="🧪️tests")pending.push(path);else if(entry.isFile()&&entry.name.endsWith(".ts"))files.push(path);}}
 await runRepositoryCommand(process.execPath,["x","tsc",...files,"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--skipLibCheck"],workspace,"writer-equation-gis:all-published-consumers",120000);
 console.log(`[writer-equation-gis] checked published consumers=${files.length}`);
}else
await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/writer-writer",{suites:["🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts","📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts"]});
