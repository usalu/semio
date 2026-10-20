/** 🕰️ Contract: the repository glue of the graph generation keeps its protocol. Shared readers never take a writer, the builder queues on one generation scope, the facts resource is only the bounded lease handed to the store, documents stay inside the original store, and the partial inference keeps its order. The model is the fixture's ordered text laws, checked here against the graph source and in Python by `🐍️oracle.py`. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";

const python=Bun.which("python3"),here=dirname(fileURLToPath(import.meta.url)),graphPath=fileURLToPath(new URL("../../🟦️.ts",import.meta.url)),source=readFileSync(graphPath,"utf8");
const body=(name:string):string=>{
 const row=fixture.functions[name as keyof typeof fixture.functions],start=source.indexOf(row.start),end=source.indexOf(row.end,start+row.start.length);
 expect(start,name+" start").toBeGreaterThan(-1);expect(source.indexOf(row.start,start+1),name+" start unique").toBe(-1);expect(end,name+" end").toBeGreaterThan(start);
 return source.slice(start,end);
};

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});
for(const row of fixture.order)test("order: "+row.name,()=>{const text=body(row.function),before=text.indexOf(row.before),after=text.indexOf(row.after);expect(before,"before text").toBeGreaterThan(-1);expect(after,"after text").toBeGreaterThan(-1);expect(before).toBeLessThan(after);});
for(const row of fixture.present)test("present: "+row.name,()=>{expect(body(row.function).includes(row.text)).toBe(true);});
for(const row of fixture.absent)test("absent: "+row.name,()=>{expect(body(row.function).includes(row.text)).toBe(false);});
test("the independent text model reaches the same verdict",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json"),graphPath]);
 expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
