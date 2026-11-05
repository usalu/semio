import{readFileSync,writeFileSync}from"node:fs";
import{resolve}from"node:path";
import{pathToFileURL}from"node:url";
const root=process.cwd(),entry=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/🦀️.rs");
const{readRustSourceClosureV1}=await import(pathToFileURL(resolve(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts")).href);
const files=await readRustSourceClosureV1([entry],root,["test","debug_assertions","windows",'target_os="windows"','target_arch="x86_64"','target_family="windows"','target_pointer_width="64"'],async(path:string)=>{try{return readFileSync(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error;}});
let removed=0,bytes=0;for(const file of files){if(!file.path.endsWith(".rs"))continue;const path=resolve(root,file.path),text=readFileSync(path,"utf8"),next=text.replace(/^[ \t]*\/\/#(?:end)?region[^\n]*(?:\n|$)/gmu,line=>{removed++;bytes+=Buffer.byteLength(line);return"";});if(process.argv.includes("--apply")&&next!==text)writeFileSync(path,next);}console.log(JSON.stringify({removed,bytes}));
