// Type-checks the staged and edited TypeScript inside the overlay (strict, noUncheckedIndexedAccess). Prints only diagnostics located in those files.
const ts=require("/Users/ueli/Documents/semio/node_modules/typescript"),fs=require("fs"),path=require("path"),cp=require("child_process");
const here=__dirname,ov=process.argv[2]||"/private/tmp/rexec-inhouse/ov",R="/Users/ueli/Documents/semio";
const targets=JSON.parse(cp.execFileSync("python3",["-c","import sys,json;sys.path.insert(0,'"+here+"');from lib import *;t=load('targets');e=load('edits');print(json.dumps({'new':t.NEW,'replace':t.REPLACE,'edited':sorted({x['file'] for x in e.EDITS})}))"],{encoding:"utf8"}));
const own=new Set([...Object.values(targets.new),...Object.values(targets.replace),...targets.edited].filter(f=>f.endsWith(".ts")).map(f=>path.join(ov,f)));
const options={target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,strict:true,noUncheckedIndexedAccess:true,esModuleInterop:true,isolatedModules:true,resolveJsonModule:true,allowImportingTsExtensions:true,skipLibCheck:true,noEmit:true,types:["bun"],typeRoots:[R+"/node_modules/@types",R+"/node_modules"],baseUrl:R,paths:{"*":[R+"/node_modules/*"]}};
const program=ts.createProgram([...own],options);
const diags=ts.getPreEmitDiagnostics(program).filter(d=>d.file&&own.has(d.file.fileName));
for(const d of diags){const{line}=d.file.getLineAndCharacterOfPosition(d.start);console.log(path.relative(ov,d.file.fileName).split("/").slice(-3).join("/")+":"+(line+1),"TS"+d.code,"["+d.file.text.slice(d.start,d.start+50).replace(/\n/g," ")+"]",ts.flattenDiagnosticMessageText(d.messageText,"\n").slice(0,260));}
console.log("errors in own files:",diags.length,"of",own.size,"files");
