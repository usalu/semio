import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {createRequire} from "node:module";
const [overlay,repo]=process.argv.slice(2);
const rel="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const fixture=JSON.parse(readFileSync(join(overlay!,rel,"📥️caller/🧪️tests/📥️birth/🔣️.json"),"utf8"));
const {createBuildBootstrapCapabilitiesV1}=await import(join(repo!,rel,"📥️caller/📤️child/🟦️.ts"));
const Ajv=createRequire(join(repo!,"package.json"))("ajv").default,schema=JSON.parse(readFileSync(join(repo!,rel,"📥️caller/🧪️tests/📥️birth/📋️schema.json"),"utf8")),db=new Database(":memory:"),failures:string[]=[];let checked=0;
if(!new Ajv({strict:true}).compile(schema)(fixture))failures.push("fixture schema");
for(const row of fixture.cases){
 const defaults=row.name.startsWith("default-bazel")?["bazel"]:["nx","bazel"];
 for(const tool of defaults){
  const grant=await createBuildBootstrapCapabilitiesV1(repo,["repo",...row.args],{SEMIO_BUILD_MODE:row.mode,SEMIO_BUILD_TOOL:tool}) as {buildMode:string;launches:string[][]};
  const oracle=db.query(`with rows(value,ordinal) as (
 select json(?1),0 where json_type(?1)!='null' union all
 select json_array('nx','run','workspace:deps-javascript','--output-style=stream'),1 where ?2 union all
 select json_array('nx','watch','--all','--includeGlobalWorkspaceFiles','--verbose','--','bun','nx','run',?3,'--output-style=stream'),2 where ?3 is not null union all
 select json_array('nx','run',?3,'--output-style=stream'),3 where ?3 is not null union all
 select json_array('nx','daemon','--start'),4 where ?3 is not null)
 select json_group_array(json(value)) as value from (select value from rows order by ordinal)`).get(JSON.stringify(row.main),Number(row.javascript),row.watch??null) as {value:string};
  checked++;
  if(grant.buildMode!==row.mode||JSON.stringify(grant.launches.filter(args=>args[0]==="nx"))!==JSON.stringify(JSON.parse(oracle.value)))failures.push(row.name+"@"+tool);
 }
}
console.log(JSON.stringify({rows:fixture.cases.length,checked,failures}));
process.exit(failures.length?1:0);
