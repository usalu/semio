import {runCargoProbe,publishCargoProbeOwners} from "../../../../🧪️tests/🎛️cargo-owner/🟦️.ts";
import { test, expect } from "bun:test";
import Ajv from "ajv";
import TOML from "@iarna/toml";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { discoverCargoWorkspaces, cargoRepositoryPackages, cargoWorkspaceForManifest, publishCargoWorkspaceMembership } from "../../🟦️.ts";
const owner=resolve(import.meta.dir,"../.."),fixture=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🪆️nested-owners/🔣️.json"),"utf8")),artifactRoot=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!artifactRoot)throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
const put=(root:string,path:string,text:string):void=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
const native=(root:string,manifest:string)=>Bun.spawnSync(["cargo","metadata","--offline","--no-deps","--format-version","1","--manifest-path",join(root,manifest)],{cwd:root,stdout:"pipe",stderr:"pipe"});
const pkg=(name:string,dependencies="")=>'[package]\nname="'+name+'"\nversion="0.1.0"\nedition="2021"\n[lib]\npath="🦀️.rs"\n'+dependencies;
test("nested owner corpus is closed and admits self-package manifest authority",()=>{
 const admission=new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(owner,"🧬️schema/📇️discovery/🔣️.json"),"utf8")));
 expect(fixture["schemaVersion"]).toEqual(1);
 expect(admission({"schema-version":1,"member-manifests":["Cargo.toml"],"exclude-patterns":[]})).toBe(true);
 expect(admission({"schema-version":1,"member-manifests":["../Cargo.toml"]})).toBe(false);
});
test("authored nested native owners survive deletion without concealing retained broken consumers",async()=>{
 for(const row of fixture.cases){
  mkdirSync(artifactRoot!,{recursive:true});const root=mkdtempSync(join(artifactRoot!,"nested-owner-"));
  put(root,"Cargo.toml",'[workspace]\nresolver="2"\nmembers=["framework/kernel"]\nexclude=["owners/**"]\n[workspace.metadata.semio.repository]\nschema-version=1\nexclude-patterns=[]\nmember-manifests=["framework/*/Cargo.toml"]\nowner-manifests=["owners/*/Cargo.toml","owners/**/native/Cargo.toml"]\n');
  put(root,"framework/kernel/Cargo.toml",pkg("neutral-kernel"));put(root,"framework/kernel/🦀️.rs","pub fn kernel() {}\n");
  if(row.removed!=="application"){
   put(root,"owners/application/Cargo.toml",'[workspace]\nresolver="2"\nmembers=["app"]\nexclude=["extension/**"]\n[workspace.metadata.semio.repository]\nschema-version=1\nexclude-patterns='+JSON.stringify(row.admitted?[]:["extension/artifact/oracle/native"])+'\nmember-manifests=["app/Cargo.toml"]\n');
   put(root,"owners/application/app/Cargo.toml",pkg("application",'[dev-dependencies]\nindependent-oracle={path="../extension/artifact/oracle/native"}\n'));put(root,"owners/application/app/🦀️.rs","pub fn app() {}\n");
   if(row.removed!=="artifact"){
    const admission=row.admitted?'[workspace.metadata.semio.repository]\nschema-version=1\nexclude-patterns=[]\nmember-manifests=["Cargo.toml"]\n':'';
    put(root,"owners/application/extension/artifact/oracle/native/Cargo.toml",'[workspace]\nresolver="2"\nmembers=["."]\n'+admission+pkg("independent-oracle"));put(root,"owners/application/extension/artifact/oracle/native/🦀️.rs","pub fn oracle() {}\n");
   }
  }
  const {scopes,packages}=await runCargoProbe(root,async owner=>({scopes:await discoverCargoWorkspaces(root,owner.operation),packages:await cargoRepositoryPackages(root,owner.operation)}));await publishCargoProbeOwners(root);expect(scopes.map(s=>s.directory).sort()).toEqual([...row.expectedOwnerDirectories].sort());expect(packages.length).toBe(scopes.length);
  for(const scope of scopes){const source=readFileSync(join(root,scope.manifest),"utf8");expect(Bun.TOML.parse(source)).toEqual(TOML.parse(source));const actual=native(root,scope.manifest);expect(actual.exitCode===0,actual.stderr.toString()).toBe(scope.directory==="owners/application"?row.applicationMetadataValid:true);if(actual.exitCode===0){const metadata=JSON.parse(actual.stdout.toString());expect(metadata.workspace_members.length).toBe(packages.filter(p=>p.workspace===scope.directory).length);}}
  if(row.removed==="none")expect((await runCargoProbe(root,async owner=>cargoWorkspaceForManifest(root,"owners/application/extension/artifact/oracle/native/Cargo.toml",owner.operation))).directory).toBe("owners/application/extension/artifact/oracle/native");
  console.log("[DEBUG] Nested Cargo owner "+JSON.stringify({id:row.id,owners:scopes.map(s=>s.directory),packages:packages.map(p=>p.name)}));
 }
});
