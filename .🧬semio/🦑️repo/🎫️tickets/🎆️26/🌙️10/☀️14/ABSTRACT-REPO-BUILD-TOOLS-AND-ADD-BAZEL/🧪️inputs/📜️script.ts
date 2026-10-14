import { test, expect } from "bun:test";
import { readFileSync, mkdirSync, mkdtempSync, writeFileSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import { createRequire } from "node:module";
const root=process.cwd(),library=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),require=createRequire(join(root,"package.json"));
const fixture=JSON.parse(readFileSync(join(library,"⚡️caching/🧪️tests/🧫️fixtures/nx-contract/🔣️.json"),"utf8"));
const inputs=await import(join(library,"🏗️build/📥️inputs/🟨️.mjs")),locks=await import(join(library,"🏗️build/📥️inputs/🔒️lock/🟨️.mjs")),digests=await import(join(library,"🏗️build/📥️inputs/🧮️digest/🟨️.mjs"));
const Ajv=require("ajv/dist/2020").default,minimatch=require("minimatch").minimatch,graphlib=require("graphlib");
const sourceSchema=JSON.parse(readFileSync(join(library,"🏗️build/📥️inputs/🧮️digest/🔣️schema.json"),"utf8")),lockSchema=JSON.parse(readFileSync(join(library,"🏗️build/📥️inputs/🔒️lock/🔣️schema.json"),"utf8"));
const ajv=new Ajv({strict:false}),sourceValid=ajv.compile(sourceSchema),lockValid=ajv.compile(lockSchema.$defs.CargoLockInventory);

test("neutral closure schemas and compaction preserve independent glob membership",()=>{
 expect(sourceValid({version:1,files:["owner/source.ts"]})).toBe(true);expect(lockValid({version:1,manifest:"owner/Cargo.toml"})).toBe(true);
 for(const path of ["../escape","/absolute","owner/../escape","C:/absolute","owner\\source",""]){expect(sourceValid({version:1,files:[path]})).toBe(false);expect(lockValid({version:1,manifest:path})).toBe(false);}
 const row=fixture.sourceInputCompaction,compacted=inputs.compactSourceInputs(row.inputs,root,row.root),key=compacted.find(input=>input?.env?.startsWith("SEMIO_SOURCE_CLOSURE_")).env.slice(21),inventory=JSON.parse(readFileSync(join(root,".🧬semio/🦑️repo/⚡️cache/build-input-closures",key+".json"),"utf8"));
 const expected=row.inputs.filter(input=>!input.startsWith("!")).map(input=>input.replace("{workspaceRoot}/","")).filter(path=>!row.inputs.filter(input=>input.startsWith("!")).some(input=>minimatch(path,input.slice(1).replace("{projectRoot}",row.root))));
 expect(expected).toEqual(row.expected);expect(sourceValid(inventory)).toBe(true);expect(inventory.files.filter(path=>path.startsWith(row.root+"/"))).toEqual(expected);expect(inventory.files.some(path=>path.includes("🔌️nx-plugin"))).toBe(false);
 expect(inputs.deduplicateProjectInputs(fixture.inputIdentity.inputs,fixture.inputIdentity.root)).toEqual(fixture.inputIdentity.expected);
 const sdk=createRequire(resolve(root,".nx/installation/package.json")),hasher=sdk("nx/src/hasher/task-hasher"),groups=inputs.internProjectInputGroups(fixture.sharedInputGroups);
 for(const name of Object.keys(fixture.sharedInputGroups))expect(hasher.expandNamedInput(name,groups).map(JSON.stringify).sort()).toEqual(hasher.expandNamedInput(name,fixture.sharedInputGroups).map(JSON.stringify).sort());
 console.log("[DEBUG] Neutral source inventories, safe paths and installed Nx group expansion agree");
});

test("neutral lock closure matches independent graph reachability",()=>{
 const row=fixture.cargoLockClosure,projection=locks.cargoLockClosure({version:4,package:row.packages},row.roots),graph=new graphlib.Graph({directed:true});
 for(const entry of row.packages)graph.setNode(locks.cargoLockIdentity(entry));
 for(const entry of row.packages)for(const dependency of entry.dependencies??[]){const[name,version]=dependency.split(" "),target=row.packages.find(candidate=>candidate.name===name&&(version===undefined||candidate.version===version));graph.setEdge(locks.cargoLockIdentity(entry),locks.cargoLockIdentity(target));}
 expect(projection.packages.map(locks.cargoLockIdentity)).toEqual(row.expected);expect(projection.packages.map(locks.cargoLockIdentity)).toEqual(graphlib.alg.preorder(graph,[locks.cargoLockIdentity(row.packages[0])]).sort());
 console.log("[DEBUG] Neutral Cargo lock closure agrees with graphlib");
});

test("neutral source digest matches independent Bun SHA-256 over fixture bytes",()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Ticket artifact directory required");mkdirSync(output,{recursive:true});const directory=mkdtempSync(join(output,"dg-")),row=fixture.sourceDigest;
 try{
  for(const[path,bytes]of Object.entries(row.files)){mkdirSync(join(directory,path,".."),{recursive:true});writeFileSync(join(directory,path),String(bytes));}
  const expected=new Bun.CryptoHasher("sha256");for(const path of [...row.paths].sort()){expected.update(path+"\0");const bytes=row.files[path];expected.update(bytes===undefined?"missing\0":"file\0"+new Bun.CryptoHasher("sha256").update(bytes).digest("hex")+"\0");}
  const contract={version:1,files:row.paths};expect(sourceValid(contract)).toBe(true);expect(digests.sourceClosureDigest(directory,contract)).toBe(expected.digest("hex"));
  console.log("[DEBUG] Neutral source closure digest agrees with Bun SHA-256");
 }finally{rmSync(directory,{recursive:true,force:true});}
});
