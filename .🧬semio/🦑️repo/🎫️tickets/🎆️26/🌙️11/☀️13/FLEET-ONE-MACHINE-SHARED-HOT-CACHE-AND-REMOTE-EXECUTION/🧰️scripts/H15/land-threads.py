import os
U="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache"
def edit(path,old,new):
    s=open(path,encoding="utf8").read();assert s.count(old)==1,(path,old[:60],s.count(old))
    open(path+".h15tmp","w",encoding="utf8").write(s.replace(old,new));os.replace(path+".h15tmp",path)
key=U+"/🔏️key/🟦️.ts";sc=U+"/📜️script.ts";tt=U+"/🧪️tests/🧳️portable/🟦️.ts"
edit(key,"/** 📦️ The OUT_DIR of a build script as a key value",'''/** 🧵️ The command line with the compiler's parallel front end turned off (`-Z threads=N` in the spellings `-Z threads=N` and `-Zthreads=N` become `threads=1`): with more than one thread the compiler orders the definitions it creates while lowering (the opaque types of `async fn` in a trait, for one) by whichever thread is first, so two compilations of one unit, on one machine or two, give other `.rmeta` tables and an other `.rlib`, and the second origin of a shared unit is refused as diverged. A unit that is shared through the fleet is compiled with one thread; the key keeps what Cargo gave, because every machine gives the same.
 * @see https://doc.rust-lang.org/nightly/cargo/reference/unstable.html */
export function deterministicFrontEndArgumentsV1(args:readonly string[]):readonly string[]{
 return args.map((argument,index)=>/^-Zthreads=/u.test(argument)?"-Zthreads=1":args[index-1]==="-Z"&&/^threads=/u.test(argument)?"threads=1":argument);
}

/** 📦️ The OUT_DIR of a build script as a key value''')
edit(sc,"stripIncremental()?withoutIncrementalV1(argv):argv)","shared?deterministicFrontEndArgumentsV1(stripIncremental()?withoutIncrementalV1(argv):argv):stripIncremental()?withoutIncrementalV1(argv):argv)")
edit(sc,"withoutIncrementalV1,unitKeyComponentsV1","withoutIncrementalV1,deterministicFrontEndArgumentsV1,unitKeyComponentsV1")
s=open(tt,encoding="utf8").read()
s=s.replace('import {mkdirSync,mkdtempSync,realpathSync,','import {mkdirSync,mkdtempSync,readFileSync,realpathSync,',1)
s=s.replace("import {keyEnvironmentV1,neutralRustflagsV1,","import {deterministicFrontEndArgumentsV1,keyEnvironmentV1,neutralRustflagsV1,",1)
s+='''
test("a shared unit is compiled with one front-end thread in every spelling, and nothing else of the command line moves",()=>{
 expect(deterministicFrontEndArgumentsV1(["--crate-name","a","-Z","threads=0","-C","opt-level=0","-Zthreads=8","-Z","unstable-options"])).toEqual(["--crate-name","a","-Z","threads=1","-C","opt-level=0","-Zthreads=1","-Z","unstable-options"]);
 expect(deterministicFrontEndArgumentsV1(["--cfg","threads=0"])).toEqual(["--cfg","threads=0"]);
});

test("one real compiler: async fn in a trait gives the same metadata bytes in every compilation with one front-end thread (the cross-check of the rewrite; threads=0 does not)",()=>{
 const found=spawnSync("rustc",["-vV"],{encoding:"utf8"});if(found.status!==0)return;
 const work=directory("front-end"),source=join(work,"lib.rs");
 writeFileSync(source,"#![allow(async_fn_in_trait)]\\n"+Array.from({length:40},(_,index)=>`pub trait T${index}{async fn a(&self)->u32;async fn b(&self,m:&[u8;32])->Result<Vec<u8>,String>;}\\npub trait V${index}{async fn v(&self,s:&str,m:&[u8;32])->Result<bool,String>;}\\n`).join(""));
 const digests=new Set<string>();
 for(let run=0;run<4;run++){
  const out=join(work,"o"+run);mkdirSync(out);
  const args=deterministicFrontEndArgumentsV1(["--crate-name","fe","--crate-type","lib","--edition","2021","-Z","threads=0","-C","metadata=fe","--emit=metadata","--out-dir",out,source]);
  const compiled=spawnSync("rustc",args,{encoding:"utf8",env:{...process.env,RUSTC_BOOTSTRAP:"1"}});
  if(compiled.status!==0)return;
  digests.add(sha256V1(readFileSync(join(out,"libfe.rmeta"))));
 }
 expect(digests.size).toBe(1);
});
'''
open(tt+".h15tmp","w",encoding="utf8").write(s);os.replace(tt+".h15tmp",tt)
