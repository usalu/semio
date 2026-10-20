import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {existsSync,readFileSync} from "node:fs";
import {createRequire} from "node:module";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import fixture from "./🔣️.json" with {type:"json"};
import policy from "../../../🔣️policy.json" with {type:"json"};
import installSchema from "../../../../⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json" with {type:"json"};
import {buildEnvironmentPlanV1,buildParserScopeV1,javascriptInstallModeV1,javascriptScopeOwnersV1} from "../../🟦️.ts";
import {parserRequireV1} from "../../📦️parsers/🟨️.mjs";

const root=join(import.meta.dir,"../../../../../../../../..").replace(/[\\/]+$/u,"");

test("project roots map to the deepest declared dependency scope exactly as a SQL longest-prefix query does",()=>{
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE scope(name TEXT PRIMARY KEY)");
  for(const row of fixture.owners){
   database.exec("DELETE FROM scope");for(const name of row.scopes)database.query("INSERT INTO scope VALUES(?1)").run(name);
   const normalised=row.roots.map(value=>value.replaceAll("\\","/").replace(/^(\.\/)+/u,"").replace(/\/+$/u,""));
   const sql=[...new Set(normalised.map(path=>(database.query("SELECT COALESCE((SELECT name FROM scope WHERE name <> '.' AND (?1 = name OR substr(?1, 1, length(name) + 1) = name || '/') ORDER BY length(name) DESC LIMIT 1), '.') AS owner").get(path) as {owner:string}).owner))].sort();
   expect(javascriptScopeOwnersV1(row.scopes,row.roots),row.name).toEqual(sql);
   expect(javascriptScopeOwnersV1(row.scopes,row.roots),row.name).toEqual(row.expected);
  }
 }finally{database.close();}
});

test("the install mode is all or closure, the environment wins over the policy and the policy ships all",()=>{
 expect(policy.javascriptInstall).toBe("all");expect(policy.javascriptInstallRoots).toEqual([]);
 expect(javascriptInstallModeV1({})).toBe("all");
 for(const value of fixture.invalidModes)expect(()=>javascriptInstallModeV1({SEMIO_JS_INSTALL:value}),JSON.stringify(value)).toThrow("Unknown JavaScript install mode");
 for(const row of fixture.plans){
  const plan=buildEnvironmentPlanV1(root,row.tool as "nx"|"bazel",row.environment);
  expect(plan.closure,row.name).toBe(row.closure);
  expect(plan.scope!==undefined&&typeof plan.install==="function",row.name).toBe(row.closure);
  if(row.closure)expect(plan.scope).toBe(buildParserScopeV1(root));
 }
});

test("the closure bootstrap scope is the parser package, a workspace member of the exact parsers the bootstrap witness checks",()=>{
 const directory=dirname(fileURLToPath(new URL("../../📦️parsers/🟨️.mjs",import.meta.url))),scope=buildParserScopeV1(root);
 expect(join(root,...scope.split("/"))).toBe(directory);
 if(!existsSync(join(directory,"package.json")))return;
 const manifest=JSON.parse(readFileSync(join(directory,"package.json"),"utf8")),workspace=JSON.parse(readFileSync(join(root,"package.json"),"utf8"));
 expect(Object.keys(manifest.dependencies).sort()).toEqual(["@iarna/toml","typescript"]);
 for(const version of Object.values(manifest.dependencies) as string[])expect(version).toMatch(/^\d+\.\d+\.\d+$/u);
 expect(Bun.semver.satisfies(manifest.dependencies.typescript,workspace.devDependencies.typescript)).toBe(true);
 expect(manifest.dependencies["@iarna/toml"]).toBe(workspace.devDependencies["@iarna/toml"]);
 expect(workspace.workspaces).toContain(scope);
});

test("the store selection schema admits a distinct non-empty owner list and nothing else",()=>{
 const validate=new Ajv({strict:false}).compile(installSchema.$defs.SourceOwnerStoreSelection);
 for(const row of fixture.selections)expect(validate(row.value),JSON.stringify(row.value)).toBe(row.valid);
});

test("the parser require loads the parser package first and falls back to the requiring file",()=>{
 const load=parserRequireV1(import.meta.url),direct=createRequire(import.meta.url);
 expect(load("typescript").version).toBe(direct("typescript").version);
 expect(load.resolve("typescript/package.json")).toMatch(/typescript[\\/]package\.json$/u);
 expect(()=>load("absent-parser-package-v1")).toThrow();
});
