import { receiveScriptProcessInvocation } from "@semio-tech/framework/process/routing";
import { runArtifactTypeScriptPackageMain } from "@semio-tech/framework/process/typescript-check";
import { readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { runRepositoryCommand } from "@semio-tech/framework/process/execution";
import { resolve as scriptDirectoryResolve } from "node:path";

const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dir, "../../📦️packages/🟦️typescript");


/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  if(process.argv[2]==="verify"&&process.argv[3]==="snapshot-sqlite-source"){
   await receiveScriptProcessInvocation(process.env,async original=>{
   const workspace=resolve(SCRIPT_DIR,"../../../../../../.."),roots=[
    "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards",
    "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards",
    "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards",
    "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards"
   ],files:string[]=[],pending=roots.map(root=>join(workspace,root));
   while(pending.length){const directory=pending.pop()!;for(const entry of readdirSync(directory,{withFileTypes:true})){const path=join(directory,entry.name);if(entry.isDirectory()&&entry.name!=="🧪️tests")pending.push(path);else if(entry.isFile()&&entry.name.endsWith(".ts"))files.push(path);}}
   await runRepositoryCommand(original,process.execPath,["x","tsc",...files,"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--skipLibCheck"],workspace,"writer-equation-gis:all-published-consumers",{version:1,maximumElapsedMilliseconds:120000,output:{maximumBytes:134217728,maximumLines:65536}});
   console.log(`[writer-equation-gis] checked published consumers=${files.length}`);
  
   });
  }else
  await runArtifactTypeScriptPackageMain(SCRIPT_DIR,"@semio-tech/writer-writer",{suites:["🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts","📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts"]});
}
