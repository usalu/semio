/** 🌍️ Contract: the derivation of the environment tells its operations (`fields` once per batch of projects, `manifests` per workspace manifest, `lock`, `taxonomy`) to the caller before it does them, so a guard can stop it at any of them, and a caller that throws there aborts the derivation; the answer is the same whether an operation hook is given or not.
 * @see ../../🟨️.mjs */
import {expect,test} from "bun:test";
import {mkdtempSync,mkdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {deriveEnvironmentV1} from "../../🟨️.mjs";

const make=()=>{
 const dir=mkdtempSync(join(tmpdir(),"p12-env-")),write=(path:string,text:string)=>{mkdirSync(join(dir,path,".."),{recursive:true});writeFileSync(join(dir,path),text);};
 for(let at=0;at<150;at++){write(`p${at}/package.json`,JSON.stringify({name:"@x/p"+at}));}
 write("Cargo.toml","[workspace]\nmembers=[]\n");write("c/Cargo.toml","[package]\nname=\"c\"\n");write("bun.lock","{}");write("taxonomy.json","{}");
 const read=async(path:string)=>{try{return(await Bun.file(join(dir,path)).text());}catch{return undefined;}};
 const stat=async(path:string)=>{try{const status=await Bun.file(join(dir,path)).stat();return{size:status.size,mtimeMs:status.mtimeMs};}catch{return undefined;}};
 return{dir,read,stat,projects:Array.from({length:150},(_unused,at)=>({name:"@x/p"+at,root:"p"+at})),workspaceManifests:["Cargo.toml","c/Cargo.toml"]};
};

test("the operations are told in order, once per batch of 64 projects for the fields, and the answer does not depend on the hook",async()=>{
 const f=make();try{
  const told:string[]=[],input={projects:f.projects,workspaceManifests:f.workspaceManifests,lock:"bun.lock",taxonomy:"taxonomy.json",read:f.read,stat:f.stat};
  const watched=await deriveEnvironmentV1({...input,operation:(name:string)=>{told.push(name);}}),plain=await deriveEnvironmentV1(input);
  expect(told).toEqual(["fields","fields","fields","manifests","manifests","lock","taxonomy"]);
  expect(watched.digest).toBe(plain.digest);expect(watched.rows).toEqual(plain.rows);
 }finally{rmSync(f.dir,{recursive:true,force:true});}
});

test("a hook that throws aborts the derivation at that operation",async()=>{
 const f=make();try{
  const told:string[]=[];
  const error=await deriveEnvironmentV1({projects:f.projects,workspaceManifests:f.workspaceManifests,lock:"bun.lock",taxonomy:"taxonomy.json",read:f.read,stat:f.stat,operation:(name:string)=>{told.push(name);if(told.length===2)throw new Error("stopped in "+name);}}).then(()=>undefined,reason=>reason);
  expect(error?.message).toBe("stopped in fields");expect(told).toEqual(["fields","fields"]);
 }finally{rmSync(f.dir,{recursive:true,force:true});}
});
