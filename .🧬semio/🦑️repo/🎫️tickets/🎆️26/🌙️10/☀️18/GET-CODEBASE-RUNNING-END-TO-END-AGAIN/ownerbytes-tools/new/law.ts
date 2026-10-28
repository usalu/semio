/** 📏️ Contract: the bootstrap owner ceiling never exceeds the Source revision text ceiling and every fixed recovery owner manifest (frontend, maintenance, reset) stays inside the authored headroom, in agreement with an independent Python and SQLite reading, so a growing manifest fails this law before it refuses any recovery route. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {statSync} from "node:fs";
import {join} from "node:path";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {findWorkspaceRoot} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {REVISION_POLICY_V1} from "../../../../📥️inputs/🔏️revision/🟨️.mjs";
import {readNativeRepoRecoveryPlanV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py"),protocol=schema.$defs.Protocol.const,root=findWorkspaceRoot();
const ask=(mode:string[],input:unknown)=>spawnSync(python,["-I",oracleFile,...mode],{input:JSON.stringify(input),encoding:"utf8"});

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[0],headroomPercent:101}]})).toBe(false);
 expect(new Set(corpus.cases.map(row=>row.accepted)).size).toBe(2);
});

test("every corpus case is decided as the corpus, the Python reading and SQLite say",()=>{
 const run=ask(["rows"],{rows:corpus.cases}),answers=JSON.parse(run.stdout) as boolean[],db=new Database(":memory:");expect(run.status,run.stderr).toBe(0);
 try{for(const[index,row]of corpus.cases.entries()){
  const sql=db.query<{accepted:number},[number,number,number,number]>("select (?1<=?2 and ?3*100<=?1*(100-?4)) accepted").get(row.maximumOwnerBytes,row.maximumTextBytes,row.ownerBytes,row.headroomPercent);
  expect(answers[index],row.name).toBe(row.accepted);expect(sql?.accepted===1,row.name).toBe(row.accepted);
 }}finally{db.close();}
});

test("the checkout keeps the bootstrap ceiling under the revision text ceiling, every owner inside its headroom, and every recovery route entering",()=>{
 const owners=[...new Set([protocol.owner,protocol.maintenance.owner,protocol.reset.owner])].sort(),rows=owners.map(owner=>({owner,ownerBytes:statSync(join(root,owner,"📋️project.json")).size,maximumOwnerBytes:protocol.maximumOwnerBytes,maximumTextBytes:REVISION_POLICY_V1.maximumTextBytes,headroomPercent:protocol.ownerHeadroomPercent}));
 const tree=ask(["tree",root],""),found=JSON.parse(tree.stdout) as (typeof rows[number]&{accepted:boolean})[];
 expect(found.map(({accepted:_accepted,...row})=>row),"the Python reading of the checkout agrees with the module readings").toEqual(rows);
 for(const row of found)expect(row.accepted,`${row.owner}: ${row.ownerBytes} B of ${row.maximumOwnerBytes} B bootstrap ceiling (${row.headroomPercent}% headroom) under the ${row.maximumTextBytes} B revision text ceiling`).toBe(true);
 expect(tree.status,tree.stderr).toBe(0);
 const routes=[...protocol.targets.map(target=>"repo-mcp:"+target),...Object.keys(protocol.maintenance.targets).map(target=>"repo:"+target),...Object.keys(protocol.reset.targets).map(target=>"repo:"+target)];
 expect(routes.length).toBe(protocol.targets.length+Object.keys(protocol.maintenance.targets).length+Object.keys(protocol.reset.targets).length);
 for(const route of routes)expect(readNativeRepoRecoveryPlanV1(root,["run",route])?.target,route).toBeDefined();
});
