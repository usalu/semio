/** 🪧 Contract: repository Cargo tests and lint bind their named packages through the literal members the caller's workspace declares to Cargo, admitted exactly as the whole-owner walk admits them and without listing one directory; every other request defers to the whole-repository inventory, so the selected rows, or the refusal, never differ from it. Cargo's own `metadata` is the third-party oracle for which manifest `-p <name>` means.
 * @see ../../🟦️.ts
 * @see ../../../../🏃️process/🦀️cargo/🎟️admission/🟦️.ts
 * @see https://doc.rust-lang.org/cargo/reference/workspaces.html */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import schema from "../../🧬️schema/🪧️declared-selection/🔣️.json" with {type:"json"};
import fixture from "../🧫️fixtures/🪧️declared-selection/🔣️.json" with {type:"json"};
import {cargoDeclaredPackageSelectionsV1,cargoPackageSelectionsNearV1,cargoRepositoryPackageSelections,cargoRepositoryPackages,type CargoWorkspacePackage} from "../../🟦️.ts";
import type {CargoPhysicalObservationsV1} from "../../🛠️preparation/👁️witness/🟦️.ts";

const answer=(run:()=>readonly CargoWorkspacePackage[]):string=>{try{return JSON.stringify(run());}catch(error){return "refused: "+(error instanceof Error?error.message:String(error));}};
const workspaceSource=(row:typeof fixture.workspaces[number]):string=>'[workspace]\nresolver="2"\nmembers='+JSON.stringify(row.members)+"\nexclude="+JSON.stringify(row.exclude)+'\n[workspace.metadata.semio.repository]\nschema-version=1\n'+(row.directory==="."?'owner-manifests=["[!.]*/**/Cargo.toml"]\n':"")+"member-manifests="+JSON.stringify(row.memberManifests)+"\nexclude-patterns="+JSON.stringify(row.excludePatterns)+"\n";
function corpus():string{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("SEMIO_TEST_ARTIFACT_DIR is required");mkdirSync(output,{recursive:true});
 const root=mkdtempSync(join(output,"declared-selection-")),put=(path:string,body:string)=>{mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),body);};
 for(const row of fixture.workspaces)put(join(row.directory,"Cargo.toml"),workspaceSource(row));
 for(const row of fixture.packages){put(join(row.directory,"Cargo.toml"),'[package]\nname="'+row.name+'"\nversion="0.1.0"\nedition="2021"\n[lib]\npath="🦀️.rs"\n');put(join(row.directory,"🦀️.rs"),"pub fn observed() {}\n");}
 return root;
}
function cargoManifest(root:string,workspace:string,name:string):string{
 const result=spawnSync("cargo",["metadata","--no-deps","--format-version","1","--offline","--manifest-path",join(root,workspace,"Cargo.toml")],{encoding:"utf8",maxBuffer:1<<26});
 if(result.status!==0)throw Error("cargo metadata failed: "+result.stderr);
 const row=(JSON.parse(result.stdout).packages as {name:string;manifest_path:string}[]).filter(row=>row.name===name);expect(row.length).toBe(1);return realpathSync(row[0]!.manifest_path);
}

test("the corpus is schema-valid",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
});

test("declared requests equal the repository inventory and Cargo's own member, list no directory, and every other request defers to the inventory",()=>{
 const root=corpus();
 for(const row of fixture.cases){
  const observations:CargoPhysicalObservationsV1=new Map(),declared=cargoDeclaredPackageSelectionsV1(root,row.near,row.names,observations),inventory=answer(()=>cargoRepositoryPackageSelections(root,row.names));
  expect([row.id,answer(()=>cargoPackageSelectionsNearV1(root,row.near,row.names))]).toEqual([row.id,inventory]);
  if(row.expect==="deferred"){expect([row.id,declared]).toEqual([row.id,undefined]);continue;}
  expect([row.id,JSON.stringify(declared)]).toEqual([row.id,inventory]);
  expect([row.id,[...observations.values()].filter(step=>step.operation==="list").length]).toEqual([row.id,0]);
  for(const selected of declared!)expect([row.id,realpathSync(join(root,selected.manifest))]).toEqual([row.id,cargoManifest(root,selected.workspace,selected.name)]);
 }
});

test("for every inventoried package, near its own manifest and near its workspace manifest, the declared answer is the inventory row or a deferral, and declared admitted members are always answered",()=>{
 const root=corpus(),declaredDirectories=new Set(fixture.workspaces.flatMap(row=>row.members.map(member=>join(row.directory,member))));
 for(const row of cargoRepositoryPackages(root))for(const near of [row.manifest,join(row.workspace,"Cargo.toml")]){
  const got=cargoDeclaredPackageSelectionsV1(root,near,[row.name]);
  if(got!==undefined)expect([row.name,near,got]).toEqual([row.name,near,[row]]);
  else expect([row.name,near,declaredDirectories.has(row.directory)&&!/[*?]/u.test(row.directory)]).toEqual([row.name,near,false]);
 }
});

test("a declared member whose directory vanished defers with the inventory's own refusal while its sibling stays declared",()=>{
 const root=corpus(),{stale}=fixture;rmSync(join(root,stale.remove),{recursive:true,force:true});
 expect(cargoDeclaredPackageSelectionsV1(root,stale.near,[stale.gone])).toBeUndefined();
 expect(answer(()=>cargoPackageSelectionsNearV1(root,stale.near,[stale.gone]))).toBe(answer(()=>cargoRepositoryPackageSelections(root,[stale.gone])));
 expect(answer(()=>cargoPackageSelectionsNearV1(root,stale.near,[stale.gone]))).toStartWith("refused: Unknown current repository Cargo package");
 expect(JSON.stringify(cargoDeclaredPackageSelectionsV1(root,stale.near,[stale.kept]))).toBe(answer(()=>cargoRepositoryPackageSelections(root,[stale.kept])));
});

test("repository Cargo tests and lint select through the declaring workspace, never through the synchronous whole-repository inventory",()=>{
 for(const relative of ["../../../../🏃️process/🦀️cargo/🎟️admission/🟦️.ts","../../../../🏃️process/🦀️cargo/🧹️lint/🟦️.ts"]){
  const source=readFileSync(new URL(relative,import.meta.url),"utf8");
  expect([relative,/cargoPackageSelectionsNearV1\(repository, ?join\(cwd, ?"Cargo\.toml"\)/u.test(source),source.includes("cargoRepositoryPackageSelections")]).toEqual([relative,true,false]);
 }
});
