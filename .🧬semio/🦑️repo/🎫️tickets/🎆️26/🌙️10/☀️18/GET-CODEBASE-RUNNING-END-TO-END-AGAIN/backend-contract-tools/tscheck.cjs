// Type-checks the staged and edited files inside the overlay (strict). Prints only diagnostics located in those files.
const ts=require("/Users/ueli/Documents/semio/node_modules/typescript"),fs=require("fs"),path=require("path");
const here=__dirname,ov=process.argv[2]||"/private/tmp/bkc/ov",R="/Users/ueli/Documents/semio";
const manifest=JSON.parse(fs.readFileSync(path.join(here,"staged/manifest.json"),"utf8"));
const own=new Set(Object.values(manifest).filter(r=>r.destination.endsWith(".ts")).map(r=>path.join(ov,r.destination)));
const edited=JSON.parse(fs.readFileSync(path.join(here,"edited.json"),"utf8")).filter(f=>f.endsWith(".ts")).map(f=>path.join(ov,f));
for(const f of edited)own.add(f);
const options={target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,strict:true,noUncheckedIndexedAccess:true,esModuleInterop:true,isolatedModules:true,resolveJsonModule:true,allowImportingTsExtensions:true,skipLibCheck:true,noEmit:true,types:["bun"],typeRoots:[R+"/node_modules/@types",R+"/node_modules"],baseUrl:R,paths:{"ajv":[R+"/node_modules/ajv"],"*":[R+"/node_modules/*"]}};
const program=ts.createProgram([...own],options);
const diags=ts.getPreEmitDiagnostics(program).filter(d=>d.file&&own.has(d.file.fileName));
for(const d of diags){const{line}=d.file.getLineAndCharacterOfPosition(d.start);console.log(path.relative(ov,d.file.fileName).split("/").slice(-3).join("/")+":"+(line+1),"TS"+d.code,"["+d.file.text.slice(d.start,d.start+50).replace(/\n/g," ")+"]",ts.flattenDiagnosticMessageText(d.messageText,"\n").slice(0,260));}
console.log("errors in own files:",diags.length,"of",own.size,"files");
