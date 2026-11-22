F="⚡️caching/🚀️bootstrap/🛠️tools/"
TEST='''/** 🔖️ Contract: the installation key moves only with the Nx recipe (name, pins, lock, patches) and never with the build library, the authorizer bundle key moves with the library closures and with the installation, and both keys equal an independent Python computation over the very inputs the provisioning captures. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {captureNxToolingInputsV1} from "../../🟦️.ts";
import {nxAuthorizerBundleIdentityV1,nxToolingInstallIdentityV1} from "../../📦️sdk/🪪️identity/🟦️.ts";

setDefaultTimeout(120000);

const tools=join(import.meta.dir,"../.."),checkout=import.meta.dir.slice(0,import.meta.dir.indexOf("/🧰️framework")),recipe="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools";
const python=requireTestToolV1("python"),oracle=join(tools,"📦️sdk/🪪️identity/🧪️tests/🧮️keys/🐍️oracle/🐍️.py"),signal=new AbortController().signal,environment={platform:"linux",architecture:"x64",libc:"glibc"};
const material=Buffer.from(JSON.stringify({version:1,kind:"shared-pristine",snapshot:"a".repeat(64),sourceIndex:"b".repeat(64),ownedPackages:["nx"]})+"\\n");
const scratch=mkdtempSync(join(realpathSync(tmpdir()),"nxi-"));let counter=0;

function workspace(){
 const root=join(scratch,"w"+counter++),base=join(root,recipe),write=(path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
 mkdirSync(base,{recursive:true});for(const name of["package.json","bun.lock"])copyFileSync(join(tools,name),join(base,name));
 const patch=JSON.parse(readFileSync(join(tools,"package.json"),"utf8")).semio.toolPatches["nx@23.2.0"] as string;mkdirSync(dirname(join(root,patch)),{recursive:true});copyFileSync(join(checkout,patch),join(root,patch));
 write(recipe+"/🔏️authorizers/📦️runtime/🟦️.ts",'export {authorizerValue} from "./closure/🟦️.ts";\\n');write(recipe+"/🔏️authorizers/📦️runtime/closure/🟦️.ts","export const authorizerValue=1;\\n");
 write(recipe+"/🔏️authorizers/📜️script.ts",'export {compilerValue} from "./compiler/🟦️.ts";\\n');write(recipe+"/🔏️authorizers/compiler/🟦️.ts","export const compilerValue=1;\\n");
 write(recipe+"/🔏️authorizers/unrelated/🟦️.ts","export const unrelated=1;\\n");
 return{root,write,recipe:base,patch};
}
async function keys(root:string){
 const captured=await captureNxToolingInputsV1(root,signal),files=new Map<string,Uint8Array>(captured.files);files.set(".semio-sdk-material.json",material);
 const installation=nxToolingInstallIdentityV1(environment,files),bundle=nxAuthorizerBundleIdentityV1(installation,captured.authorizerFiles);
 return{installation,bundle,files,authorizerFiles:captured.authorizerFiles};
}
const text=(bytes:Uint8Array)=>Buffer.from(bytes).toString("utf8");

test("the captured installation inputs name no authorizer manifest and the bundle inputs name exactly the two manifests",async()=>{
 const row=await keys(workspace().root);
 expect([...row.files.keys()].filter(name=>name.includes("authorizer"))).toEqual([]);expect([...row.authorizerFiles.keys()]).toEqual([".semio-authorizer-source.json",".semio-authorizer-compiler.json"]);
 expect(row.installation).toMatch(/^[a-f0-9]{64}$/u);expect(row.bundle).toMatch(/^[a-f0-9]{64}$/u);expect(row.bundle).not.toBe(row.installation);
 expect((await keys(workspace().root)).installation).toBe(row.installation);
});

test("an unrelated library edit keeps the installation key and gives the authorizer bundle its own key",async()=>{
 const space=workspace(),base=await keys(space.root);
 space.write(recipe+"/🔏️authorizers/📦️runtime/closure/🟦️.ts","export const authorizerValue=2;\\n");
 const edited=await keys(space.root);expect(edited.installation).toBe(base.installation);expect(edited.bundle).not.toBe(base.bundle);
 space.write(recipe+"/🔏️authorizers/📦️runtime/closure/🟦️.ts","export const authorizerValue=1;\\n");expect((await keys(space.root)).bundle).toBe(base.bundle);
 space.write(recipe+"/🔏️authorizers/compiler/🟦️.ts","export const compilerValue=2;\\n");
 const compiled=await keys(space.root);expect(compiled.installation).toBe(base.installation);expect(compiled.bundle).not.toBe(base.bundle);expect(compiled.bundle).not.toBe(edited.bundle);
 space.write(recipe+"/🔏️authorizers/compiler/🟦️.ts","export const compilerValue=1;\\n");space.write(recipe+"/🔏️authorizers/unrelated/🟦️.ts","export const unrelated=2;\\n");
 const untouched=await keys(space.root);expect(untouched.installation).toBe(base.installation);expect(untouched.bundle).toBe(base.bundle);
});

test("the installation key is the same in every checkout while the bundle key binds the physical root",async()=>{
 const first=await keys(workspace().root),second=await keys(workspace().root);expect(second.installation).toBe(first.installation);expect(second.bundle).not.toBe(first.bundle);
});

test("an Nx recipe edit gives a new installation and with it a new authorizer bundle",async()=>{
 const base=await keys(workspace().root),renamed=workspace(),manifest=JSON.parse(readFileSync(join(renamed.recipe,"package.json"),"utf8"));manifest.name="@semio-tech/nx-bootstrap-tools-edited";writeFileSync(join(renamed.recipe,"package.json"),JSON.stringify(manifest,null,2)+"\\n");
 const edited=await keys(renamed.root);expect(edited.installation).not.toBe(base.installation);expect(edited.bundle).not.toBe(base.bundle);
});

test("both keys equal the Python oracle over the captured inputs",async()=>{
 const row=await keys(workspace().root),rows=[{environment,files:[...row.files].map(([name,bytes])=>[name,text(bytes)])},{dependencies:row.installation,files:[...row.authorizerFiles].map(([name,bytes])=>[name,text(bytes)])}];
 const run=spawnSync(python,["-I",oracle],{input:JSON.stringify({rows}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);expect(JSON.parse(run.stdout)).toEqual([row.installation,row.bundle]);
});

test("cleanup",()=>{rmSync(scratch,{recursive:true,force:true});});
'''
def plan(p,L):
    p.sub(L+F+"🟦️.ts",'async function captureNxToolingInputsV1(workspace:string,signal:AbortSignal){','export async function captureNxToolingInputsV1(workspace:string,signal:AbortSignal){')
    p.create(L+F+"🧪️tests/🔖️bundle-isolation/🟦️.ts",TEST)
