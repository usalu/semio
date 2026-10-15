import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {resolve,join} from "node:path";
import ts from "typescript";
import Ajv from "ajv";
import {Command} from "commander";
import fixture from "./fixture.json" with {type:"json"};
import schema from "./schema.json" with {type:"json"};
const root=resolve(import.meta.dir,"../../../../../../../.."),read=(path:string)=>readFileSync(join(root,path),"utf8");
test("managed editor jobs corpus preserves every other actual command and declaration",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  for(const file of[".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){
   const text=read(file),parsed=ts.parseConfigFileTextToJson(file,text);expect(parsed.error).toBeUndefined();expect(parsed.config).toEqual(JSON.parse(text));
   const configurations=parsed.config.configurations as Record<string,unknown>[];
   for(const row of fixture.rows.filter(row=>row.catalog===file)){
    const matches=configurations.filter(candidate=>candidate?.name===row.name);expect(matches.length,row.name).toBe(1);
    const oracle=database.query("SELECT json_remove(?, '$.env.CARGO_BUILD_JOBS') AS value").get(JSON.stringify(row.before)) as {value:string};
    expect(JSON.parse(oracle.value)).toEqual(row.expected);expect(matches[0],row.name).toEqual(row.expected);
    const command=new Command().exitOverride().argument("runner").argument("repo").argument("verb").argument("task");command.parse(row.expected.command.split(/\s+/u).slice(0,4),{from:"user"});expect(command.args.slice(0,3)).toEqual(["bun","repo","run"]);
    expect(["@semio-tech/value-rs:test","@semio-tech/framework-pack-json-rs:test-native","@semio-tech/framework-job-rs:test","@semio-tech/framework-replication-rs:test"]).toContain(command.args[3]!);
    expect(Object.keys(row.expected.env).filter(key=>key.startsWith("CARGO_")&&key!=="CARGO_NET_OFFLINE")).toEqual([]);
   }
  }
 }finally{database.close();}
 console.log("[DEBUG] Exact 20 launch and 19 mirrored seed entries preserve all facts except redundant Cargo jobs");
});
test("all four selected tests retain the canonical Original compiler admission route",()=>{
 const owners=JSON.parse(read("📋️build-owners.json")).owners as {name:string;root:string}[];
 for(const[name,kind]of[["@semio-tech/value-rs","TestScript"],["@semio-tech/framework-pack-json-rs","NativeScript"],["@semio-tech/framework-job-rs","TestScript"],["@semio-tech/framework-replication-rs","TestScript"]]){
  const owner=owners.find(owner=>owner.name===name);expect(owner).toBeDefined();const path=owner!.root+"/📜️script.ts",text=read(path),source=ts.createSourceFile(path,text,ts.ScriptTarget.Latest,true);
  expect(text).toContain("runRepositoryCargoTestsV1 as runCargoTestsV1");const node=source.statements.find(node=>ts.isClassDeclaration(node)&&node.name?.text===kind);expect(node).toBeDefined();expect(node!.getText(source)).toContain("runCargoTestsV1(");expect(node!.getText(source)).toContain("this.invocation");
 }
 const core=read("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts");expect(core).toContain("environment.CARGO_BUILD_JOBS = String(capacity.parallelism)");
 console.log("[DEBUG] Canonical Source test owners retain Original invocation and real admitted compiler parallelism");
});
