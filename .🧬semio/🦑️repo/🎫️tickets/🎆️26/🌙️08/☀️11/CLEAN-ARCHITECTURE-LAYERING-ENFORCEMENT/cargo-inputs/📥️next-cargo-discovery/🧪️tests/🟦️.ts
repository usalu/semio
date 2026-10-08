import {test,expect} from "bun:test";
import Ajv from "ajv";
import TOML from "@iarna/toml";
import glob from "fast-glob";
import {mkdtempSync,mkdirSync,writeFileSync,readFileSync} from "node:fs";
import {join,dirname} from "node:path";
import {discoverCargoWorkspaces,cargoRepositoryPackages} from "../../🟦️.ts";
import {observeCargoProbe} from "../../../../🧪️tests/🎛️cargo-owner/🟦️.ts";
const vector=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8")),schema=JSON.parse(readFileSync(new URL("../../🧬️schema/📇️discovery/🔣️.json",import.meta.url),"utf8")),artifact=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!artifact)throw Error("SEMIO_TEST_ARTIFACT_DIR required");
const put=(root:string,path:string,source:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),source);};
test("schema-first owner exclusions prune malformed and over-depth sources before physical observation",async()=>{
 expect(new Ajv({strict:true}).compile(schema)((TOML.parse(vector.workspace).workspace as any).metadata.semio.repository)).toBe(true);const root=mkdtempSync(join(artifact,"cargo-owner-exclusion-"));put(root,"Cargo.toml",vector.workspace);for(const [path,source]of Object.entries(vector.files))put(root,path,String(source));put(root,"standards/"+Array.from({length:vector.blockedDepth},(_,index)=>"d"+index).join("/")+"/Cargo.toml","invalid = [ unfinished\n");
 const rootDocument=TOML.parse(vector.workspace),workspace=rootDocument.workspace as Record<string,unknown>,metadata=workspace.metadata as Record<string,unknown>,semio=metadata.semio as Record<string,unknown>,admission=semio.repository as Record<string,unknown>,physical=glob.sync(admission["owner-manifests"] as string[],{cwd:root,onlyFiles:true,followSymbolicLinks:false,ignore:admission["exclude-patterns"] as string[]}).sort(),independentOwners:string[]=[],independentPackages:string[]=[];for(const path of physical){const document=TOML.parse(readFileSync(join(root,path),"utf8"));if(document.workspace!==undefined)independentOwners.push(path);else if(document.package!==undefined)independentPackages.push(path);}expect(independentOwners).toEqual(vector.owners);expect(independentPackages).toEqual(vector.packages);
 const observation=await observeCargoProbe(root,async owner=>{const scopes=await discoverCargoWorkspaces(root,owner.operation),packages=await cargoRepositoryPackages(root,owner.operation);return structuredClone({owners:scopes.map(row=>row.manifest).sort(),packages:packages.map(row=>row.manifest).sort(),opened:owner.workspace.directories.map(row=>row.path)});});expect(observation.value.owners).toEqual(independentOwners);expect(observation.value.packages).toEqual(independentPackages);expect(observation.manifests.some(row=>row.path.split("/").includes("standards"))).toBe(false);expect(observation.value.opened.some(path=>path.split("/").includes("standards"))).toBe(false);expect(observation.completed).toBeLessThan(vector.discovery.maximumUnits);expect(observation.ownedBytes).toBeLessThan(vector.discovery.maximumOwnedBytes);
});
