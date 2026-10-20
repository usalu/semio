// 🔷️ Builds the hook's real program analysis (`precisionProgramHostV1`, taken from the patched assembly text) over the live tree with receipts under a scratch directory, so tools and harnesses run the exact production code.
import {mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import type {ProgramHostV1} from "/private/tmp/ip/run/p/🟦️.ts";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),RUN="/private/tmp/ip/run";
export async function programHost(options:{data?:string;document:any;checks?:{count:number;limit?:number};report?:(event:any)=>void}):Promise<ProgramHostV1>{
 const text=readFileSync("/private/tmp/ip/assembly2.mjs","utf8"),start=text.indexOf("/** 🔷️ The compiler side of the TypeScript program analysis"),end=text.indexOf("/** 🎯️ The input precision rules and the engine host");
 if(start<0||end<start)throw new Error("patched assembly block not found: run patched-assembly.py");
 const data=options.data??join(RUN,"data");mkdirSync(data,{recursive:true});
 const prelude=`
import {createHash} from "node:crypto";import {dirname,isAbsolute,join} from "node:path";
import {lstatSync as rawLstatSync,mkdirSync as rawMkdirSync,readFileSync as rawReadFileSync,renameSync as rawRenameSync,writeFileSync as rawWriteFileSync} from "node:fs";
import {brotliCompressSync,brotliDecompressSync,constants as zlibConstants} from "node:zlib";
import {inputPrecisionLanguageV1,inputPrecisionTextV1} from "${RUN}/p/🟦️.ts";
const observation=await import("${L}/🏗️build/👁️observation/🟨️.mjs"),inference=await import("${L}/🏗️build/🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const storage=await import("${L}/🏗️build/📥️inputs/💾️storage/🟨️.mjs");
const {observedTypeScriptHost,observedSourceAnalysisV1,observedFs}=observation,{readFileSync}=observedFs,prepareSourceAnalysesV1=inference.prepareSourceAnalysesV1,withBuildGraphOptionalPublicationV1=storage.withBuildGraphOptionalPublicationV1;
const buildPath=p=>p.split("\\\\").join("/"),workspaceDataDirectory=()=>globalThis.__ipData,yieldInferenceV1=async(context,phase,index)=>{const checks=globalThis.__ipChecks;if(checks){checks.count++;if(checks.limit!==undefined&&checks.count>checks.limit)throw new Error("cancelled");}};
`;
 writeFileSync(join(RUN,"program-host-module.mjs"),prelude+text.slice(start,end)+"\nexport {precisionProgramHostV1};\n");
 (globalThis as any).__ipData=data;(globalThis as any).__ipChecks=options.checks;
 const module=await import(join(RUN,"program-host-module.mjs")+"?"+Date.now()),compiler=(await import("node:module")).createRequire(join(W,"package.json"))("typescript");
 const program=module.precisionProgramHostV1(W,{context:{}},()=>options.document,()=>compiler);
 return options.report?{...program,report:options.report,get compiler(){return compiler.version;}}:program;
}
