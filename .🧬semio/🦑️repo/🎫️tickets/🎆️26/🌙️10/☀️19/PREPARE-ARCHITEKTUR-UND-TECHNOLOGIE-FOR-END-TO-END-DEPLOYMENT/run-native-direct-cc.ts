import {join} from "node:path";
import {pathToFileURL} from "node:url";
const {runCmd}=await import(pathToFileURL(join(process.cwd(),"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts")).href);
const executable=join(import.meta.dir,"🗑️generated","native-direct-cc.exe");
runCmd("powershell.exe",["-NoProfile","-NonInteractive","-Command","$output = & $env:SEMIO_DIRECT_CC_EXE; if ($LASTEXITCODE -ne 0 -or $output -cne 'native-direct-cc') { throw 'Native compiler result failed' }; Write-Output $output"],{env:{...process.env,SEMIO_DIRECT_CC_EXE:executable}});
