import sys,os,re;sys.path.insert(0,os.path.dirname(__file__))
from f72lib import *
assert have()
s=open(TEST,encoding="utf8").read()
def sub(a,b,count=1):
    global s
    n=s.count(a)
    if n!=count:print("COUNT",n,"expected",count,a[:90]);raise SystemExit(1)
    s=s.replace(a,b)
sub('const repository=await privateRepositoryWorkspaceV1();','const repository=await privateRepositoryWorkspaceV1("graph");')
sub('import{privateRepositoryWorkspaceV1,testCheckoutRootV1,testNxToolkitDirectoryV1}from"../../../../🧪️testing/🧫️private-repository/🟦️.ts";','import{privateRepositoryWorkspaceV1,testCheckoutRootV1,testNxToolkitDirectoryV1}from"../../../../🧪️testing/🧫️private-repository/🟦️.ts";\nimport{adoptAuthorizerSourcesForV1}from"../../../../🧪️testing/🧫️private-repository/🧰️adoption/🟦️.ts";\nimport rustCodePolicy from"../🏗️compiler/🔏️code/🔣️policy.json";')
sub('const invoke=<T,>(body','await adoptAuthorizerSourcesForV1(repository.root,testCheckoutRootV1(),rustCodePolicy.entry);\nconst invoke=<T,>(body')
s=re.sub(r'(\D)(60000|120000)(\))',lambda m:m.group(1)+"300000"+m.group(3),s)
s=s.replace("maximumElapsedMilliseconds:240000","maximumElapsedMilliseconds:300000").replace("},240000);","},300000);").replace('"rust-unit:ordinary-nx",240000','"rust-unit:ordinary-nx",300000')
# 77
sub('await invoke(original=>withUnitDescriptorFixture(original,scriptCompilerAdmissionControlV1(original,"rust-unit:owner-plan",300000),async({file,units,outputRoot,control})=>{','await invoke(original=>withRepositoryRustUnitCodeV1(repository.root,original,scriptCompilerAdmissionControlV1(original,"rust:fixture",0),(_code,codePhase)=>withUnitDescriptorFixture(original,createRequestedIssuedResourceControlV1(codePhase,300000),async({file,units,outputRoot,control})=>{')
sub('expect(refused.stderr).toContain("Original selected Rust unit identity required");}));},300000);','expect(refused.stderr).toContain("Original selected Rust unit identity required");})));},300000);')
# 102
sub('for(const unit of materialized.units)unit.execution=rustCompilerExecutionV2(mode==="prototype"?process.cwd():root,mode);observed[phase]=[];for(const originalPlan of planRustUnitsV1(materialized.units,"unit-artifacts")){','const executed=materialized.units.map(unit=>({...unit,execution:rustCompilerExecutionV2(mode==="prototype"?process.cwd():root,mode)}));observed[phase]=[];for(const originalPlan of planRustUnitsV1(executed,"unit-artifacts")){')
# 194
sub('const root=mkdtempSync(join(artifacts,"ca-")),source=join(root,"lib.rs"),manifest=join(root,"Cargo.toml"),workspace=process.cwd();','const root=await repository.scratch("ca-"),source=join(root,"lib.rs"),manifest=join(root,"Cargo.toml"),workspace=repository.root;')
# 79
sub('throw Error("Original descriptor mode required");await invoke(async original=>{const root=process.cwd(),','throw Error("Original descriptor mode required");await adoptIoBase64WorkspaceV1();await invoke(async original=>{const root=process.cwd(),')
helper='''/** 🧫️ Gives the private repository the io-base64 crate and a one-member Cargo workspace derived from the checkout's own workspace manifest, lock entry and Cargo configuration, so the actual crate is planned without the live store of the checkout. */
async function adoptIoBase64WorkspaceV1():Promise<void>{
 const fs=await import("node:fs"),checkout=testCheckoutRootV1(),crate="🧰️framework/🔨️modules/🚪️io/🔤️base64",member=crate+"/📦️packages/🦀️rust",root=repository.root,put=(path:string,text:string)=>{try{fs.writeFileSync(join(root,path),text,{flag:"wx"});}catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;}};
 fs.cpSync(join(checkout,crate),join(root,crate),{recursive:true,force:false,errorOnExist:false,filter:source=>!/[/\\\\](🧪️tests|node_modules|dist)$/u.test(source)});
 const lock=fs.readFileSync(join(checkout,"Cargo.lock"),"utf8"),entry=lock.match(/\\[\\[package\\]\\]\\nname = "semio-framework-io-base64"\\nversion = "[^"]+"\\n/u);if(!entry)throw Error("Checkout lock entry of io-base64 required");
 put("Cargo.toml",fs.readFileSync(join(checkout,"Cargo.toml"),"utf8").replace(/members = \\[[\\s\\S]*?\\n\\]/u,'members = ["'+member+'"]'));put("Cargo.lock",lock.slice(0,lock.indexOf("[[package]]"))+entry[0]);
 fs.mkdirSync(join(root,".cargo"),{recursive:true});put(".cargo/config.toml",fs.readFileSync(join(checkout,".cargo/config.toml"),"utf8"));
}
'''
sub('const clone=<T>(value:T):T=>JSON.parse(JSON.stringify(value));\n','const clone=<T>(value:T):T=>JSON.parse(JSON.stringify(value));\n'+helper)
write(TEST,s)
print("ok")
