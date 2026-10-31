import { expect, test } from "bun:test";
import Ajv from "ajv";
import { spawn } from "node:child_process";
import { readFileSync } from "node:fs";
import { resolve, join } from "node:path";
import law from "./typing-logger-outcomes.json";
import schema from "./typing-logger-outcomes.schema.json";
const root=process.cwd(),ticket=import.meta.dir,repo=process.env.SEMIO_REPO_BIN??"repo",ajv=new Ajv({strict:true,allErrors:true}),validateLaw=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Law"}),validateOutcome=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Outcome"});
for(const row of law.cases)test(row.id,async()=>{
 expect(validateLaw(law),JSON.stringify(validateLaw.errors)).toBe(true);
 const name="typing-logger-law-"+row.id,args=["tools","exec","--light","node","--","-e",row.payload],output=join(ticket,"🗑️generated",name+".outcome.json"),logger=join(ticket,"run-repo-command.cjs"),child=spawn(repo,["tools","exec","--light","node","--",logger,name,...args],{cwd:root,env:{...process.env,SEMIO_LANGUAGE:"en"},stdio:["ignore","pipe","pipe"]});let stdout="",stderr="";child.stdout.on("data",chunk=>stdout+=chunk);child.stderr.on("data",chunk=>stderr+=chunk);const frontend=await new Promise<{status:number|null;signal:string|null}>((complete,fail)=>{child.once("error",fail);child.once("close",(status,signal)=>complete({status,signal}));});
 expect(frontend,JSON.stringify({stdout,stderr})).toEqual({status:row.frontend,signal:null});const outcome=JSON.parse(readFileSync(output,"utf8"));expect(validateOutcome(outcome),JSON.stringify(validateOutcome.errors)).toBe(true);expect({status:outcome.status,signal:outcome.signal}).toEqual({status:row.status,signal:row.signal});expect(outcome.args).toEqual(args);expect(resolve(outcome.logPath)).toBe(join(ticket,"🗑️generated",name+".log"));expect(outcome.pid).toBeGreaterThan(0);expect(Number.isFinite(Date.parse(outcome.endedAt))).toBe(true);expect(stdout).toContain(JSON.stringify(outcome));expect(validateOutcome({...outcome,status:null,signal:null})).toBe(false);expect(validateOutcome({...outcome,status:0,signal:"SIGTERM"})).toBe(false);
},30000);
