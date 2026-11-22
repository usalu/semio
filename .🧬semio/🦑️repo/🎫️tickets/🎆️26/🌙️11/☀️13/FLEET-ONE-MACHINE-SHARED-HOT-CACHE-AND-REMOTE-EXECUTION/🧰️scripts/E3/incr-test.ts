
test("an incremental compilation has other bytes on every run, one without a session has the same bytes every time, and the command line without the session is that compilation",()=>{
 expect(withoutIncrementalV1(["a","-C","incremental=/x","-Cincremental=/y","-C","opt-level=1","-Cdebuginfo=2","b"])).toEqual(["a","-C","opt-level=1","-Cdebuginfo=2","b"]);
 const root=directory();writeFileSync(join(root,"l.rs"),Array.from({length:120},(_,index)=>`pub fn f${index}(x:u64)->u64{ x.wrapping_mul(${index+3}).rotate_left(${index%13}) }\n`).join(""));
 const build=(label:string,session:boolean,strip=false)=>{const out=join(root,label);mkdirSync(out,{recursive:true});const args=["--crate-name","l","--edition=2021","l.rs","--crate-type","lib","--emit=link","-C","debuginfo=2","-C","metadata=a","-C","extra-filename=-a","--out-dir",out,...session?["-C","incremental="+join(root,"session-"+label)]:[]],run=spawnSync("rustc",strip?[...withoutIncrementalV1(args)]:args,{cwd:root,encoding:"utf8",env:toolchain});expect(run.status,run.stderr).toBe(0);return createHash("sha256").update(readFileSync(join(out,"libl-a.rlib"))).digest("hex");};
 const plain=[build("p1",false),build("p2",false)],stripped=build("s1",true,true),sessions=[build("i1",true),build("i2",true)];
 expect(plain[0]).toBe(plain[1]);expect(stripped,"the stripped command line is the compilation without a session").toBe(plain[0]);
 expect(sessions[0]).not.toBe(sessions[1]);
});
