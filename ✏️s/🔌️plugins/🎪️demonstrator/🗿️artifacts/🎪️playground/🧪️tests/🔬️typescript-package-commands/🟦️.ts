import { receiveScriptProcessInvocation } from "@semio-tech/framework/process/routing";
import { runArtifactTypeScriptPackageMain } from "@semio-tech/framework/process/typescript-check";
import { runRepositoryCommand } from "@semio-tech/framework/process/execution";
import { readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { resolve as scriptDirectoryResolve } from "node:path";

const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dir, "../../📦️packages/🟦️typescript");


/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  if (process.argv[2] === "verify" && process.argv[3] === "snapshot-sqlite-source") {
   await receiveScriptProcessInvocation(process.env,async original=>{
    const repo = resolve(SCRIPT_DIR, "../../../../../../.."), files: string[] = [], pending = [resolve(SCRIPT_DIR, "../../🏅️standards")];
    while (pending.length) {
      const directory = pending.pop()!;
      for (const entry of readdirSync(directory, { withFileTypes: true })) {
        const path = join(directory, entry.name);
        if (entry.isDirectory() && entry.name !== "🧪️tests") pending.push(path);
        else if (entry.isFile() && entry.name.endsWith(".ts")) files.push(path);
      }
    }
    await runRepositoryCommand(original,process.execPath,["x", "tsc", ...files, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck"],repo,"playground:published-source-consumers",{version:1,maximumElapsedMilliseconds:120000,output:{maximumBytes:134217728,maximumLines:65536}});
    console.log(`[playground] checked published consumers=${files.length}`);
  
   });
  } else await runArtifactTypeScriptPackageMain(SCRIPT_DIR, "@semio-tech/demonstrator-playground", { suites: ["🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
}
