/** 🧪 Execute every currently registered IFC4x3 suite, including nonstandard test filenames. */
import {readFile,writeFile,stat} from "node:fs/promises";
import {resolve} from "node:path";
import {spawnSync} from "node:child_process";

const root=resolve(import.meta.dir,"../../../../../../.."),artifact=resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"),registration=await readFile(resolve(artifact,"📦️packages/🟦️typescript/📜️script.ts"),"utf8"),paths=[...registration.matchAll(/"(🏅️standards\/🔖️4x3\/[^"\r\n]+)"/gu)].map(match=>resolve(artifact,match[1]!));
if(paths.length<20||!paths.some(path=>path.endsWith("🧬️editing/🟦️.ts")||path.endsWith("🧬️editing\\🟦️.ts")))throw Error("Complete IFC4x3 registration was not found");
for(const path of paths)if(!(await stat(path)).isFile())throw Error("Registered suite missing: "+path);
const result=spawnSync(process.execPath,["test",...paths],{cwd:root,encoding:"utf8",windowsHide:true,maxBuffer:32*1024*1024});
const output=result.stdout+result.stderr;await writeFile(resolve(import.meta.dir,"🗑️generated/ifc4x3-registered-tests.txt"),output);await writeFile(resolve(import.meta.dir,"🗑️generated/ifc4x3-registered-selection.json"),JSON.stringify({suites:paths.length,paths,exit:result.status,signal:result.signal,error:result.error?String(result.error):undefined},null,2));console.log("[DEBUG] "+JSON.stringify({registeredSuites:paths.length,exit:result.status,signal:result.signal}));console.log(output.trim().split(/\r?\n/).slice(-7).join("\n"));process.exitCode=result.status??1;
