import sys
P=sys.argv[1]
s=open(P,encoding="utf8").read()
a='test("an unreachable cache endpoint is a quick, silent decline'
assert s.count(a)==1
new='''test("the repository's own layouts are mapped: an absolute source in the unit's own directory (registry crates) and a manifest below the directory that holds the source (module layout), both path-independent",()=>{
 const planned=(checkout:ReturnType<typeof chain>,mode:"registry"|"module")=>{
  const base=join(checkout.root,"pk"),manifest=mode==="registry"?join(base,"c"):join(base,"m","packages","rust");
  mkdirSync(join(manifest,"src"),{recursive:true});writeFileSync(join(manifest,"Cargo.toml"),"[package]\\nname = \\"c\\"\\n");
  const source=mode==="registry"?join(manifest,"src","lib.rs"):join(base,"m","lib.rs");writeFileSync(source,"pub fn c()->u32{b::b()+1}\\n");
  const cwd=mode==="registry"?manifest:checkout.root,args=checkout.arguments("c",["b"]).map(argument=>argument==="pk/c/src/lib.rs"?(mode==="registry"?source:"pk/m/lib.rs"):argument);
  return{cwd,result:planOffloadV1({args,environment:{...checkout.environment("c"),CARGO_MANIFEST_DIR:manifest},cwd})};
 };
 for(const mode of ["registry","module"] as const){
  const left=chain("layout-left-"+mode),right=chain("layout-right-"+mode),a=planned(left,mode),b=planned(right,mode);
  if("declined" in a.result||"declined" in b.result)throw Error(mode+" declined: "+JSON.stringify(a.result)+JSON.stringify(b.result));
  expect(a.result.arguments,mode).toEqual(b.result.arguments);expect(a.result.environment,mode).toEqual(b.result.environment);expect([...a.result.files.keys()],mode).toEqual([...b.result.files.keys()]);
  expect(a.result.arguments.some(argument=>argument.includes(left.base)),mode).toBe(false);
  if(mode==="registry")expect(a.result.arguments).toContain("src/lib.rs");
  else{expect([...a.result.files.keys()]).toContain("pk/m/lib.rs");expect([...a.result.files.keys()]).toContain("pk/m/packages/rust/Cargo.toml");expect(a.result.environment.SEMIO_REL_CARGO_MANIFEST_DIR).toBe("pk/m/packages/rust");}
 }
 const checkout=chain("layout-outside"),manifest=join(checkout.root,"pk","c"),result=planOffloadV1({args:checkout.arguments("c",["b"]).map(argument=>argument==="pk/c/src/lib.rs"?"/tmp/outside/lib.rs":argument),environment:{...checkout.environment("c"),CARGO_MANIFEST_DIR:manifest},cwd:checkout.root});
 expect("declined" in result&&result.declined).toBe("unmapped-path");
});

'''
s=s.replace(a,new+a)
open(P,"w",encoding="utf8").write(s)
