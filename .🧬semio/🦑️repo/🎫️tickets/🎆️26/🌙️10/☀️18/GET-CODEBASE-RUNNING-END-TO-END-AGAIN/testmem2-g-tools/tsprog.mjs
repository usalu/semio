import{createRequire}from"node:module";import{join,dirname,relative}from"node:path";import{realpathSync}from"node:fs";
const ws="/Users/ueli/Documents/semio",require=createRequire(join(ws,"package.json")),ts=require("typescript");
const config=process.argv[2],mode=process.argv[3]??"full";
const mb=()=>Math.round(process.memoryUsage().rss/1048576),h=()=>Math.round(process.memoryUsage().heapUsed/1048576);
const path=join(ws,config),read=ts.readConfigFile(path,ts.sys.readFile),parsed=ts.parseJsonConfigFileContent(read.config,ts.sys,dirname(path),undefined,path);
console.log("root files",parsed.fileNames.length,"rss",mb());
const t=Date.now();
let host=ts.createCompilerHost({...parsed.options,noEmit:true,incremental:false});
if(mode==="stub"){const base=host.getSourceFile.bind(host);host.getSourceFile=(file,lang,...rest)=>{if(/\/node_modules\//.test(file)){const text=host.readFile(file)??"";const s=ts.createSourceFile(file,"",lang,false);return s;}return base(file,lang,...rest);};}
const program=ts.createProgram({rootNames:parsed.fileNames,options:{...parsed.options,noEmit:true,incremental:false,tsBuildInfoFile:undefined},host});
const files=program.getSourceFiles().map(f=>realpathSync(f.fileName));
const out=files.map(f=>relative(ws,f)).filter(f=>!f.startsWith("..")&&!f.split("/").includes("node_modules")&&!f.startsWith(".cache/"));
console.log(mode,"program files",files.length,"workspace",out.length,"nm",files.filter(f=>f.includes("/node_modules/")).length,"ms",Date.now()-t,"rss",mb(),"heap",h());
import{writeFileSync}from"node:fs";writeFileSync(process.argv[4]??"/dev/null",out.sort().join("\n"));
