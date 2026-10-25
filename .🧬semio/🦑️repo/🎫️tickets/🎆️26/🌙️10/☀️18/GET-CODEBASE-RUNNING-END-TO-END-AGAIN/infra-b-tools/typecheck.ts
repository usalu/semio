import ts from "typescript";
import {resolve} from "node:path";
const roots=process.argv.slice(2).map(path=>resolve(path));
const options:ts.CompilerOptions={noEmit:true,skipLibCheck:true,allowImportingTsExtensions:true,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,target:ts.ScriptTarget.ES2022,resolveJsonModule:true,strict:true,esModuleInterop:true,allowJs:false,types:["bun"],noUncheckedIndexedAccess:false,lib:["lib.esnext.d.ts"]};
const program=ts.createProgram(roots,options),mine=new Set(roots);
const diagnostics=ts.getPreEmitDiagnostics(program).filter(row=>row.file&&mine.has(resolve(row.file.fileName)));
for(const row of diagnostics){const {line}=row.file!.getLineAndCharacterOfPosition(row.start!);console.log(`${row.file!.fileName.split("/").slice(-4).join("/")}:${line+1} TS${row.code} ${ts.flattenDiagnosticMessageText(row.messageText,"\n").slice(0,300)}`);}
console.log("files checked",roots.length,"diagnostics",diagnostics.length);
