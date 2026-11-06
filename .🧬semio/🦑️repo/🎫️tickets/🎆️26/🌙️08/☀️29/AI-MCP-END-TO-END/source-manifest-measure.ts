import{readFileSync}from'node:fs';import{dirname,resolve,relative}from'node:path';import{pathToFileURL}from'node:url';
const root=process.cwd(),{readRustSourceClosureV1}=await import(pathToFileURL(resolve('🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts')).href),cfg=['test','debug_assertions','windows','target_os=windows','target_arch=x86_64','target_family=windows','target_pointer_width=64'];
for(const manifest of process.argv.slice(2)){
 const source=readFileSync(manifest,'utf8'),section=source.match(/\[lib\]([\s\S]*?)(?=\n\s*\[|$)/)?.[1],path=section?.match(/\bpath\s*=\s*"([^"]+)"/)?.[1];if(!path)throw Error('explicit actual library required '+manifest);const entry=resolve(dirname(manifest),path),files=await readRustSourceClosureV1([entry],root,cfg,async(path:string)=>{try{return readFileSync(path)}catch(error){if((error as NodeJS.ErrnoException).code==='ENOENT')return null;throw error}});
 console.log(JSON.stringify({manifest,entry:relative(root,entry),files:files.length,bytes:files.reduce((n:number,f:any)=>n+f.bytes,0),activeLines:files.reduce((n:number,f:any)=>n+f.activeLines,0)}));
}
