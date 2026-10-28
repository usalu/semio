// probe1: bun probe1.ts <crate-dir> : runs the LIVE rustSourceFiles on a crate and prints files or the thrown error (the real host swallows it and falls back to <dir>/**/*).
import {join,resolve} from "node:path";
import {existsSync} from "node:fs";
const W="/Users/ueli/Documents/semio",LIB=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const inference:any=await import(join(LIB,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const dir=join(W,process.argv[2]!),toml=Bun.TOML.parse(await Bun.file(join(dir,"Cargo.toml")).text()) as any;
const entry=resolve(dir,toml.lib?.path??"src/lib.rs");
for(const compiled of [true,false]){
 try{const files=inference.cacheInternals.rustSourceFiles([entry],dir,inference.cacheInternals.createRustSourceCache?.(),new Map(),compiled);console.log(JSON.stringify({compiled,count:files.length,files:files.map((f:string)=>f.slice(W.length+1))},null,0));}
 catch(e){console.log(JSON.stringify({compiled,error:String((e as Error).message).slice(0,500)}));}
}
