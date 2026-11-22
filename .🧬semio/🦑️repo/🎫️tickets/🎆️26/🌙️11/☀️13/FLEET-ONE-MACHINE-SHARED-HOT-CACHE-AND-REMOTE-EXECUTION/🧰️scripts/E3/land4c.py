import sys
U=sys.argv[1].rstrip('/')+'/'
def edit(path,pairs):
    s=open(path).read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:80],s.count(a))
        s=s.replace(a,b)
    open(path,'w').write(s)
edit(U+'🧭️remap/🟦️.ts',[('/** 🧷️ Everything the wrapper adds to a local link','''/** 🦀️ The remap that keeps the standard library's source out of a unit: with the `rust-src` component installed rustc names the code of `core` that it inlines by its real path below the sysroot (`<sysroot>/lib/rustlib/src/rust/library/...`) in the debug information of the unit, without it by the virtual `/rustc/<commit>/library/...`, so the same unit gave other bytes (and a path of the machine in them) on the machines that have the component, and the leak check cannot see it where the sysroot is the default rustup home. Cargo's `trim-paths` gives this remap only for some profiles; this adds it when the command line has none for that directory. It is placed by the caller before Cargo's remaps (the last matching one wins) and names a path that does not exist where the component is missing, where it changes nothing. `[]` when the sysroot or the commit is unknown. */
export function rustSourceRemapArgumentsV1(args:readonly string[],sysroot:string|undefined,commit:string|undefined):readonly string[]{
 if(sysroot===undefined||sysroot===""||commit===undefined)return[];
 const trimmed=sysroot.replace(/[\\\\/]+$/u,""),from=trimmed+(trimmed.includes("\\\\")?"\\\\":"/")+["lib","rustlib","src","rust"].join(trimmed.includes("\\\\")?"\\\\":"/");
 return remapsOfV1(args).some(row=>row.from.replace(/[\\\\/]+$/u,"")===from)?[]:[`--remap-path-prefix=${from}=/rustc/${commit}`];
}

/** 🧷️ Everything the wrapper adds to a local link''')])
edit(U+'🔏️key/🟦️.ts',[('/** 📨️ What the build broker is told about a unit:','''/** 📍️ The sysroot of the compiler behind `inner` (`--print sysroot`), memoised for one minute like {@link toolchainIdentityV1}; undefined when the probe fails. */
export function rustSysrootV1(root:string,inner:readonly string[],environment:Readonly<Record<string,string|undefined>>):string|undefined{
 const memo=join(root,"sr-"+sha256V1(inner.join("\\0")+"\\0"+(environment.RUSTUP_TOOLCHAIN??"")).slice(0,16));
 try{if(Date.now()-statSync(memo).mtimeMs<60000)return readFileSync(memo,"utf8")||undefined;}catch{}
 const result=spawnSync(inner[0]!,[...inner.slice(1),"--print","sysroot"],{encoding:"utf8",env:environment as NodeJS.ProcessEnv});
 if(result.status!==0)return undefined;
 const sysroot=result.stdout.trim();mkdirSync(root,{recursive:true});writeFileSync(memo,sysroot);return sysroot||undefined;
}

/** 📨️ What the build broker is told about a unit:''')])
edit(U+'📜️script.ts',[
('sha256V1,toolchainIdentityV1,','rustSysrootV1,sha256V1,toolchainIdentityV1,'),
(' argv=(await moduleRemapped())??argv;\n',''' argv=(await moduleRemapped())??argv;
 const sourceRemapped=():readonly string[]=>{
  if(!argv.some(argument=>argument.startsWith("--remap-path-scope")))return argv;
  try{
   const commit=/^commit-hash: ([0-9a-f]{40})$/mu.exec(toolchainIdentityV1(settings.root,settings.inner,environment))?.[1],extra=rustSourceRemapArgumentsV1(expandRustcArgumentsV1(argv,cwd),rustSysrootV1(settings.root,settings.inner,environment),commit);
   if(extra.length===0)return argv;
   const first=argv.findIndex(argument=>argument==="--remap-path-prefix"||argument.startsWith("--remap-path-prefix=")),at=first<0?argv.length:first;
   return Object.freeze([...argv.slice(0,at),...extra,...argv.slice(at)]);
  }catch{return argv;}
 };
 argv=sourceRemapped();
''')])
edit(U+'📜️script.ts',[('import {linkRootArgumentsV1,remappedNameV1,','import {linkRootArgumentsV1,remappedNameV1,rustSourceRemapArgumentsV1,')])
test='''
test("the standard library source is remapped for every unit: the rule is stated once, a remap that exists is not repeated, and the compiler itself records no sysroot path with the flag (oracle: rustc)",()=>{
 const sysroot="/Users/a/.rustup/toolchains/t",commit="9f36de775bc636c8e88c31a173c2bcb6995956a0",from=sysroot+"/lib/rustlib/src/rust";
 expect(rustSourceRemapArgumentsV1(["--crate-name","x"],sysroot,commit)).toEqual([`--remap-path-prefix=${from}=/rustc/${commit}`]);
 expect(rustSourceRemapArgumentsV1(["--remap-path-prefix="+from+"=/rustc/"+commit],sysroot+"/",commit)).toEqual([]);
 expect(rustSourceRemapArgumentsV1([],undefined,commit)).toEqual([]);expect(rustSourceRemapArgumentsV1([],sysroot,undefined)).toEqual([]);
 expect(machineNeutralRemapV1(rustSourceRemapArgumentsV1([],sysroot,commit)[0]!)).toBe(`--remap-path-prefix=⟨rust-src⟩=/rustc/${commit}`);
 const environment={...process.env,RUSTUP_TOOLCHAIN:"nightly-2026-07-20"},probe=spawnSync("rustc",["--print","sysroot"],{encoding:"utf8",env:environment}),real=probe.stdout.trim(),hash=/^commit-hash: ([0-9a-f]{40})$/mu.exec(spawnSync("rustc",["-vV"],{encoding:"utf8",env:environment}).stdout)?.[1];
 if(probe.status!==0||hash===undefined||!existsSync(join(real,"lib","rustlib","src","rust","library")))return;
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"rust-src-")));
 try{
  writeFileSync(join(base,"l.rs"),"pub fn f(x:u32)->u32{ x.wrapping_pow(3).rotate_left(5).saturating_add(x.pow(2)) }\\npub fn g(v:&[u32])->u32{ v.iter().map(|x|x.wrapping_mul(3)).sum() }\\n");
  const build=(extra:readonly string[],out:string)=>{mkdirSync(join(base,out));const run=spawnSync("rustc",["--crate-name","l","--edition=2021","l.rs","--crate-type","lib","--emit=link","-C","debuginfo=2","-C","opt-level=1","-C","metadata=a","-C","extra-filename=-a","--remap-path-scope=all",`--remap-path-prefix=${base}=.`,...extra,"--out-dir",out],{cwd:base,encoding:"utf8",env:environment});expect(run.status,run.stderr).toBe(0);return readFileSync(join(base,out,"libl-a.rlib"));};
  const plain=build([],"plain"),mapped=build(rustSourceRemapArgumentsV1([],real,hash),"mapped");
  expect(plain.includes(Buffer.from(real)),"without the flag the sysroot path is in the unit").toBe(true);expect(mapped.includes(Buffer.from(real)),"with the flag no path of the machine is left").toBe(false);
 }finally{rmSync(base,{recursive:true,force:true});}
});
'''
edit(U+'🧭️remap/🧪️tests/🟦️.ts',[('import {mkdirSync,mkdtempSync,','import {existsSync,mkdirSync,mkdtempSync,'),('remappedNameV1,remapsOfV1}','remappedNameV1,remapsOfV1,rustSourceRemapArgumentsV1}')])
open(U+'🧭️remap/🧪️tests/🟦️.ts','a').write(test)
print("ok")
