import ts from "typescript";
import {resolve} from "node:path";
import {readFileSync} from "node:fs";
const list=process.argv[2]==="-"?readFileSync(0,"utf8").split("\n").filter(Boolean):process.argv.slice(2);
const files=list.map(f=>resolve(f));
const program=ts.createProgram(files,{noEmit:true,strict:true,noUncheckedIndexedAccess:false,target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,allowImportingTsExtensions:true,resolveJsonModule:true,esModuleInterop:true,skipLibCheck:true,jsx:ts.JsxEmit.ReactJSX,types:["bun","node"],lib:["lib.esnext.d.ts","lib.dom.d.ts"]});
let bad=0;
for(const f of files){
 const sf=program.getSourceFile(f);if(!sf){console.log("MISSING",f);bad++;continue;}
 const diags=[...program.getSyntacticDiagnostics(sf),...program.getSemanticDiagnostics(sf)];
 for(const d of diags){bad++;const {line}=ts.getLineAndCharacterOfPosition(sf,d.start??0);console.log(f.replace(process.cwd()+"/","")+":"+(line+1)+": TS"+d.code+" "+ts.flattenDiagnosticMessageText(d.messageText,"\n").slice(0,300));}
}
console.log("checked",files.length,"diagnostics",bad);
