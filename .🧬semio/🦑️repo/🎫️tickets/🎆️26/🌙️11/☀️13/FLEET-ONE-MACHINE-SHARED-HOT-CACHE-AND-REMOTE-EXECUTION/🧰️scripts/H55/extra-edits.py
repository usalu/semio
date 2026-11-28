def every(path, old, new, count):
    text = load(path)
    if text.count(old) != count:
        sys.exit("anchor count %d != %d in %s: %s" % (text.count(old), count, path, old[:80]))
    files[path] = text.replace(old, new)

portable = os.path.join(U, "🧪️tests", "🧳️portable", "🟦️.ts")
every(portable, "d['environment']],separators=", "d['environment'],['determinism',d['determinism']]],separators=", 2)
once(portable, "source:fixture.sourceSha256,environment:key.environment})", "source:fixture.sourceSha256,environment:key.environment,determinism:UNIT_CACHE_POLICY_V1.key.outputDeterminism.version})")
once(portable, 'source:sha256V1("serde source"),environment:two.environment})', 'source:sha256V1("serde source"),environment:two.environment,determinism:UNIT_CACHE_POLICY_V1.key.outputDeterminism.version})')
once(portable, "import {deterministicFrontEndArgumentsV1,frontEndThreadsV1,", "import {deterministicFrontEndArgumentsV1,extraFilenameV1,frontEndThreadsV1,")
once(portable, '''  digests.add(sha256V1(readFileSync(join(out,"libfe.rmeta"))));
 }
 expect(digests.size).toBe(1);
});
''', '''  digests.add(sha256V1(readFileSync(join(out,"libfe.rmeta"))));
 }
 expect(digests.size).toBe(1);
});

test("two machines compile one unit under the determinism contract: one key and one set of output bytes; another Cargo identity or another contract version is another key",()=>{
 const found=spawnSync("rustc",["-vV"],{encoding:"utf8"});if(found.status!==0)return;
 const source="#![allow(async_fn_in_trait)]\\n"+Array.from({length:40},(_,index)=>`pub trait T${index}{async fn a(&self)->u32;async fn b(&self,m:&[u8;32])->Result<Vec<u8>,String>;}\\n`).join("");
 const raw=(out:string,extra:string)=>["--crate-name","two","--crate-type","lib","--edition","2021","-Z","threads=0","-C","metadata=two","-C","extra-filename="+extra,"--emit=metadata,link","--out-dir",out,"lib.rs"];
 const keyOf=(extra:string,version=UNIT_CACHE_POLICY_V1.key.outputDeterminism.version)=>{
  const arguments_=raw("/o",extra),input={toolchain:found.stdout,normalized:normalizeRustcArgumentsV1(arguments_,new Map(),undefined),sourceSha256:sha256V1(source),externs:[],environment:[] as Array<[string,string]>,cargoIdentity:extraFilenameV1(arguments_)};
  return version===UNIT_CACHE_POLICY_V1.key.outputDeterminism.version?unitKeyV1(input):sha256V1(JSON.stringify([UNIT_CACHE_POLICY_V1.namespace,input.toolchain,input.normalized,input.sourceSha256,[],[],["determinism",version],["cargo",input.cargoIdentity]]));
 };
 const outputs=["a","bbbbbbbbbbbb"].map(label=>{
  const work=directory("two-"+label),out=join(work,"o");mkdirSync(out);writeFileSync(join(work,"lib.rs"),source);
  const compiled=spawnSync("rustc",[...deterministicFrontEndArgumentsV1(raw(out,"-3c9971acb8b0a698")),"--remap-path-prefix="+work+"=/w"],{cwd:work,encoding:"utf8",env:{...process.env,RUSTC_BOOTSTRAP:"1"}});
  if(compiled.status!==0)return undefined;
  return["libtwo-3c9971acb8b0a698.rmeta","libtwo-3c9971acb8b0a698.rlib"].map(name=>sha256V1(readFileSync(join(out,name))));
 });
 if(outputs.some(row=>row===undefined))return;
 expect(outputs[0]).toEqual(outputs[1]);
 expect(keyOf("-3c9971acb8b0a698")).toBe(keyOf("-3c9971acb8b0a698"));
 expect(keyOf("-4fe346c9fadf2a8a")).not.toBe(keyOf("-3c9971acb8b0a698"));
 expect(keyOf("-3c9971acb8b0a698","1")).not.toBe(keyOf("-3c9971acb8b0a698"));
});
''')

determinism = os.path.join(root, "🏗️build", "🌐️remote-execution", "🚀️offload", "🧪️tests", "🎯️determinism", "🟦️.ts")
once(determinism, "import {moduleRemapArgumentsV1,planOffloadV1,type OffloadPlanV1} from", "import {moduleRemapArgumentsV1,moduleRemapV1,planOffloadV1,type OffloadPlanV1} from")
once(determinism, 'import {copyFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";', 'import {copyFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";')
files[determinism] = load(determinism) + '''
test("the module remap of a unit says what it came to: a remap of its directory, nothing needed, or the guard that declined it (a declined unit is never shared)",()=>{
 try{
  const {base,root}=checkout("remap",{label:"lib",type:"lib"}),manifest=join(root,"t","📦️packages","🦀️rust"),shipped=join(root,"t"),environment={CARGO_MANIFEST_DIR:manifest};
  const args=["--crate-name","t","--edition=2021","🦀️.rs","--crate-type","lib","--emit=metadata","--out-dir","/o"],remap=`--remap-path-prefix=${shipped}=t`;
  const made=moduleRemapV1(args,environment,manifest,root);
  expect("arguments" in made&&made.arguments.includes(remap)&&made.arguments.includes(join(manifest,"🦀️.rs"))).toBe(true);
  expect(moduleRemapArgumentsV1(args,environment,manifest,root)).toEqual("arguments" in made?made.arguments:undefined);
  expect(moduleRemapV1(args,environment,manifest,undefined)).toEqual({declined:"no-workspace"});
  expect(moduleRemapV1(args,environment,manifest,join(base,"elsewhere"))).toEqual({declined:"outside-workspace"});
  const link=join(base,"link");symlinkSync(root,link);
  const linked=moduleRemapV1(args,environment,manifest,link);
  expect("arguments" in linked&&linked.arguments.includes(remap),"a workspace spelled through a link names the directory like the manifest").toBe(true);
  expect(moduleRemapV1([...args,`--remap-path-prefix=${root}=⟨root⟩`],environment,manifest,root)).toEqual({needless:true});
  writeFileSync(join(manifest,"local.rs"),"pub fn f()->u32{1}\\n");
  expect(moduleRemapV1(["--crate-name","t","local.rs"],environment,manifest,root)).toEqual({needless:true});
  expect(moduleRemapV1(args,{},manifest,root)).toEqual({needless:true});
 }finally{cleanup();}
});
'''
