import ts from "typescript";
import{resolve}from"node:path";
const roots=JSON.parse(process.env.SEMIO_NORM_TYPECHECK_ROOTS!);
const program=ts.createProgram(roots.map((p:string)=>resolve(p)),{noEmit:true,strict:true,skipLibCheck:true,resolveJsonModule:true,esModuleInterop:true,allowImportingTsExtensions:true,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,target:ts.ScriptTarget.ESNext,types:["bun"]});
const diagnostics=ts.getPreEmitDiagnostics(program).map(row=>{const position=row.file&&row.start!==undefined?row.file.getLineAndCharacterOfPosition(row.start):undefined;return{file:row.file?.fileName,line:position?position.line+1:undefined,code:row.code,message:ts.flattenDiagnosticMessageText(row.messageText,"\n")}});
console.log(JSON.stringify({roots,diagnostics},null,2));process.exitCode=diagnostics.length?1:0;
