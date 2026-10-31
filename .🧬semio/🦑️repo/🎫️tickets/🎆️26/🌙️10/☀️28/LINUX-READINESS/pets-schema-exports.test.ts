const{rustSurfaceRowsV1}=require("./rust-surface-oracle.cjs");
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {declaresSchemaExport,schemaExportCompletenessDiagnostics} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📐️schema-contracts/📤️export-completeness/🟦️.ts";
import {readSchemaCatalog} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📐️schema-contracts/🟦️.ts";
import vectors from "./pets-schema-export-vectors.json";
const root=process.cwd(),directory=resolve(root,"🧰️framework/🛍️products/🐾️pets/🧬️schema"),source=readFileSync(resolve(directory,"🦀️.rs"),"utf8"),normative=JSON.parse(readFileSync(resolve(directory,"🔣️.json"),"utf8")) as {$defs:Record<string,unknown>};
const valid=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["modules"],properties:{modules:{type:"array",minItems:5,maxItems:5,items:{type:"object",additionalProperties:false,required:["module","directory","names","restricted"],properties:{module:{type:"string",pattern:"^[a-z_]+$"},directory:{type:"string"},restricted:{type:"array",items:{type:"object",additionalProperties:false,required:["name","visibility"],properties:{name:{type:"string"},visibility:{enum:["pub(crate)","pub(super)"]}}}},names:{type:"array",minItems:1,uniqueItems:true,items:{type:"string",pattern:"^[A-Za-z_][A-Za-z0-9_]*$"}}}}}}});expect(valid(vectors)).toBe(true);
for(const row of vectors.modules)test("explicit public exports "+row.module,async()=>{
 const child=readFileSync(resolve(directory,row.directory,"🦀️.rs"),"utf8"),items=await rustSurfaceRowsV1(child),actual=items.filter((x:any)=>x.visibility==="pub").map((x:any)=>x.name),names=[...source.matchAll(new RegExp("^pub\\s+use\\s+"+row.module+"::([A-Za-z_][A-Za-z0-9_]*)\\s*;","gmu"))].map(x=>x[1]!);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE expected(name TEXT PRIMARY KEY);CREATE TABLE actual(name TEXT PRIMARY KEY)");for(const name of row.names)db.query("INSERT INTO expected VALUES(?)").run(name);for(const name of names)db.query("INSERT INTO actual VALUES(?)").run(name);expect(db.query("SELECT name FROM expected EXCEPT SELECT name FROM actual").all()).toEqual([]);expect(db.query("SELECT name FROM actual EXCEPT SELECT name FROM expected").all()).toEqual([])}finally{db.close()}
 expect(actual).toEqual(row.names);expect(items.filter((x:any)=>x.conditional)).toEqual([]);expect(items.filter((x:any)=>x.visibility!=="pub").map(({name,visibility}:any)=>({name,visibility}))).toEqual(row.restricted);for(const helper of row.restricted)expect(source.includes("pub use "+row.module+"::"+helper.name+";")).toBe(false);for(const name of row.names.filter(name=>Object.hasOwn(normative.$defs,name)))expect(declaresSchemaExport("🦀️rust",source,name)).toBe(true);
});
test("every normative public type is explicit",()=>{const names=vectors.modules.flatMap(x=>x.names);expect(Object.keys(normative.$defs).filter(name=>!names.includes(name))).toEqual([]);expect(source).not.toMatch(/^pub\s+use\s+[A-Za-z_][A-Za-z0-9_]*::\*;/mu)});

test("actual catalog scope publishes every Rust export",()=>{const {catalog}=readSchemaCatalog(root);expect(catalog).not.toBeNull();const scope=Object.entries(catalog!.scopes).find(([,scope])=>resolve(root,scope.path)===directory);expect(scope).toBeDefined();const found=schemaExportCompletenessDiagnostics(root,scope![0],scope![1]);expect(found.filter(row=>row.format==="🦀️rust")).toEqual([])});
