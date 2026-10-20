const ts=require("/Users/ueli/Documents/semio/node_modules/typescript"),fs=require("fs"),path=require("path");
const here=__dirname,R="/Users/ueli/Documents/semio",manifest=JSON.parse(fs.readFileSync(path.join(here,"staged/manifest.json"),"utf8"));
const virtual=new Map(Object.entries(manifest).filter(([k])=>k.endsWith(".ts")||k.endsWith(".json")).map(([flat,row])=>[path.join(R,row.destination),path.join(here,"staged",flat)]));
try{for(const[k,v]of Object.entries(JSON.parse(fs.readFileSync('/private/tmp/bzlincr/tsc/map.json','utf8'))))virtual.set(k,v);}catch{}
const options={target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,strict:true,esModuleInterop:true,isolatedModules:true,resolveJsonModule:true,allowImportingTsExtensions:true,skipLibCheck:true,noEmit:true,lib:["lib.esnext.d.ts","lib.dom.d.ts"],types:["bun","node"],typeRoots:[R+"/node_modules/@types"]};
const host=ts.createCompilerHost(options),rf=host.readFile.bind(host),fe=host.fileExists.bind(host),gs=host.getSourceFile.bind(host);
host.fileExists=f=>virtual.has(f)||fe(f);host.readFile=f=>virtual.has(f)?fs.readFileSync(virtual.get(f),"utf8"):rf(f);
host.getSourceFile=(f,l,e,s)=>virtual.has(f)?ts.createSourceFile(f,fs.readFileSync(virtual.get(f),"utf8"),l):gs(f,l,e,s);
const dirs=new Set();for(const f of virtual.keys()){for(let d=path.dirname(f);d.length>R.length;d=path.dirname(d))dirs.add(d);}
const de=host.directoryExists?.bind(host);host.directoryExists=d=>dirs.has(d)||(de?de(d):fs.existsSync(d));host.realpath=f=>virtual.has(f)||dirs.has(f)?f:fs.realpathSync(f);
const roots=[...virtual.keys()].filter(f=>f.endsWith(".ts")&&f.startsWith(R+"/"+Object.values(manifest)[0].destination.split("/")[0]));
const own0=new Set(Object.values(manifest).map(r=>path.join(R,r.destination)));
const program=ts.createProgram(roots,options,host),own=own0;
const diags=ts.getPreEmitDiagnostics(program).filter(d=>d.file&&own.has(d.file.fileName));
for(const d of diags){const{line}=d.file.getLineAndCharacterOfPosition(d.start);console.log(path.basename(path.dirname(d.file.fileName))+"/"+path.basename(d.file.fileName)+":"+(line+1),ts.flattenDiagnosticMessageText(d.messageText,"\n").slice(0,300));}
console.log("errors in staged files:",diags.length,"files",roots.length);
