import {readFileSync} from "node:fs";
import {resolve,dirname} from "node:path";
import {fileURLToPath} from "node:url";
import {it,expect} from "vitest";
import Ajv from "ajv/dist/2020";

it("native material construction retains one canonical explicit value boundary",()=>{
 const root=resolve(dirname(fileURLToPath(import.meta.url)),"../../⚙️engine/🧱️material");const fixture=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/🔣️.json"),"utf8"));const schema=JSON.parse(readFileSync(resolve(root,"🧬️schema/🔣️.json"),"utf8"));const ajv=new Ajv({strict:true});expect(ajv.compile(schema)(fixture)).toBe(true);const valid=ajv.compile(schema.$defs.material);
 for(const test of fixture.accepted){expect(valid(test.row)).toBe(true);const numbers=Object.entries(test.row).filter(([,value])=>typeof value==="number");expect(numbers.every(([,value])=>Number.isFinite(value))).toBe(true);const actual=Object.fromEntries([...["id","name"].map(key=>[key,test.row[key]]),...numbers]);expect(actual).toEqual(test.row);}
 for(const test of fixture.refused)expect(valid(test.row)).toBe(false);
 console.log("[DEBUG] native material Ajv canonical accepted/refused vectors passed");
});
