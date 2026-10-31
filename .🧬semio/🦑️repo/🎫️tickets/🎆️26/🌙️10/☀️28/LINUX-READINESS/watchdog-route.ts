import assert from "node:assert/strict";
import {readFileSync,writeFileSync} from "node:fs";
import {resolve,relative,join} from "node:path";
import {pathToFileURL} from "node:url";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
const root=process.cwd(),cache="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching",ticket=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/LINUX-READINESS",input=JSON.parse(readFileSync(join(ticket,"watchdog-route.json"),"utf8")),table=JSON.parse(readFileSync(join(cache,"🗂️suites/🔣️.json"),"utf8")),project=JSON.parse(readFileSync(join(cache,"📋️project.json"),"utf8")),budget=JSON.parse(readFileSync(join(cache,"🗂️suites/🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));
const {readCachingTestTableV1}=await import(pathToFileURL(resolve(cache,"🗂️suites/🟦️.ts")).href),{resolveCanonicalTestTargetsV1,deriveCanonicalTestAggregateV1}=await import(pathToFileURL(resolve(cache,"🗂️suites/🧮️aggregate/🟦️.ts")).href);
readCachingTestTableV1(table);
const suite=table.suites.find(row=>row.id===input.id);assert.ok(suite,"New watchdog suite must be declared");
const file=relative(cache,input.file).replaceAll("\\","/");assert.deepEqual(suite.files,[file]);assert.deepEqual(suite.platforms,input.platforms);assert.equal(suite.budgetMs,budget.levels[input.level]);assert.equal(suite.typecheck,"none");assert.deepEqual(suite.tools,["node"]);assert.equal(suite.capabilities,undefined);assert.equal(suite.scratch,undefined);assert.equal(suite.fileLaunchers,undefined);assert.ok(table.suites.some(row=>row.id===input.preserveSuite));
const schema=JSON.parse(readFileSync("🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🧬️schema/🔣️.json","utf8")),ajv=new Ajv({strict:false});assert.equal(ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/Table"})(table),true);
assert.equal(project.targets[input.target].cache,false);assert.equal(project.targets[input.target].metadata.semio.sourceInputObservation,"freshness");assert.equal(project.targets[input.target].metadata.semio.cache.reason,"exercises-host-resources");
const derived=resolveCanonicalTestTargetsV1({...project,root:cache});assert.ok(derived.includes(input.target));assert.deepEqual(project.targets.test,deriveCanonicalTestAggregateV1({...project,root:cache}));
const text=readFileSync(input.file,"utf8"),tokens=budget.spawnTokens.filter(token=>text.includes(token));assert.ok(tokens.includes("spawn"));
const db=new Database(":memory:");try{db.run("create table routes(id text,target text,file text)");for(const row of table.suites)for(const name of row.files??[])db.run("insert into routes values(?,?,?)",[row.id,"test-"+row.id,name]);assert.deepEqual(db.query("select id,target,file from routes where file=?1").all(file),[{id:input.id,target:input.target,file}]);}finally{db.close();}
const result={suite:input.id,file,target:input.target,budgetMs:suite.budgetMs,class:input.level,classificationTokens:tokens,aggregateTargets:derived.length,preservedCpu:true,platforms:suite.platforms};writeFileSync(join(ticket,"🗑️generated/watchdog-route-result.json"),JSON.stringify(result,null,2)+"\n");console.log(JSON.stringify(result));
