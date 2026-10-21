import {expect,test} from "bun:test";
import yargsParser from "yargs-parser";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {buildDoorCommandV1} from "../../📜️script.ts";
import fixture from "./🔣️.json" with {type:"json"};

const script=fileURLToPath(new URL("../../📜️script.ts",import.meta.url));

test("The command is named after the global options, for every spelling of them",async()=>{
 for(const row of fixture.cases){
  const door=await buildDoorCommandV1(row.args,{});
  expect(door.lead??null,row.name).toBe(row.lead);
  const independent=yargsParser([...row.args],{string:["tool","workers","graph-policy"],configuration:{"populate--":true}})._[0];
  expect(independent===undefined?null:String(independent),row.name).toBe(row.lead);
 }
});

test("Light commands behind a global option print their help without capacity, images or graph work",()=>{
 for(const command of ["leases","capacity"]){
  const run=spawnSync(process.execPath,[script,"repo","--tool=bazel",command,"--help"],{encoding:"utf8",env:{...process.env,SEMIO_LANGUAGE:"en"},timeout:60000});
  expect(run.status,run.stderr).toBe(0);
  expect(run.stdout).toContain(command);
 }
});
