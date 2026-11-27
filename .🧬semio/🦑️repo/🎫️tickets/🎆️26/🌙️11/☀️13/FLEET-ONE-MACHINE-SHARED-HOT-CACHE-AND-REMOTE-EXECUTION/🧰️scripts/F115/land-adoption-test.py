import os,uuid
f="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧫️private-repository/🧰️adoption/🧪️tests/🟦️.ts"
s=open(f,encoding="utf8").read()
def sub(a,b):
    global s
    assert s.count(a)==1,a
    s=s.replace(a,b)
sub('import {existsSync,statSync} from "node:fs";\nimport {tmpdir} from "node:os";','import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,statSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";')
sub('import {adoptInstalledNxToolkitIdentityV1,','import {adoptBuildOwnerRegistryForV1,adoptInstalledNxToolkitIdentityV1,')
if not s.endswith("\n"): s+="\n"
s+='''
test("adopting the build owner registry copies a valid registry once whole, leaves an existing file and refuses an invalid registry",()=>{
 const base=mkdtempSync(join(tmpdir(),"adopt-owners-")),checkout=join(base,"checkout"),workspace=join(base,"workspace"),row=(name:string,packages:string[]=[])=>({name,root:name,manifest:name+"/📋️project.json",packages}),registry=(owners:unknown[])=>JSON.stringify({version:1,owners,catalogProviders:[]},null,2)+"\\n";
 try{
  mkdirSync(checkout);mkdirSync(workspace);
  adoptBuildOwnerRegistryForV1(workspace,checkout);expect(existsSync(join(workspace,"📋️build-owners.json"))).toBe(false);
  writeFileSync(join(checkout,"📋️build-owners.json"),registry([row("a"),row("a",["x"])]));
  expect(()=>adoptBuildOwnerRegistryForV1(workspace,checkout)).toThrow("Canonical build owner identity collision");expect(readdirSync(workspace)).toEqual([]);
  writeFileSync(join(checkout,"📋️build-owners.json"),registry([row("a")]));
  adoptBuildOwnerRegistryForV1(workspace,checkout);expect(readFileSync(join(workspace,"📋️build-owners.json"),"utf8")).toBe(registry([row("a")]));
  writeFileSync(join(checkout,"📋️build-owners.json"),registry([row("b")]));
  adoptBuildOwnerRegistryForV1(workspace,checkout);expect(readFileSync(join(workspace,"📋️build-owners.json"),"utf8")).toBe(registry([row("a")]));expect(readdirSync(workspace)).toEqual(["📋️build-owners.json"]);
 }finally{rmSync(base,{recursive:true,force:true});}
});
'''
t=f+"."+uuid.uuid4().hex+".tmp";open(t,"w",encoding="utf8").write(s);os.replace(t,f);print("landed")
