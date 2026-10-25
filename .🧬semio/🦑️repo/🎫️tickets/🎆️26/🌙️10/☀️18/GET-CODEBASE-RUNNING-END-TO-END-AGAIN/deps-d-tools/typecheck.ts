import ts from "typescript";
import {resolve} from "node:path";
const root=process.cwd(),files=process.argv.slice(2).map(file=>resolve(root,file));
const config=ts.getParsedCommandLineOfConfigFile(resolve(root,"tsconfig.json"),{noEmit:true},{...ts.sys,onUnRecoverableConfigFileDiagnostic:()=>{}})!;
const program=ts.createProgram({rootNames:files,options:{...config.options,noEmit:true,incremental:false}});
let count=0;
for(const file of files){const source=program.getSourceFile(file);if(!source){console.log("MISSING",file);continue;}
 for(const diagnostic of [...program.getSyntacticDiagnostics(source),...program.getSemanticDiagnostics(source)]){count++;const {line}=source.getLineAndCharacterOfPosition(diagnostic.start??0);console.log(file.replace(root+"/","")+":"+(line+1)+" TS"+diagnostic.code+" "+ts.flattenDiagnosticMessageText(diagnostic.messageText,"\n").slice(0,300));}}
console.log("diagnostics in changed files:",count);
