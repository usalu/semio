import {test,expect} from "bun:test";
import {sqliteInventory} from "./🔮️oracle/🟦️.ts";
import {createRequire} from "node:module";
import {readFileSync} from "node:fs";
import fixture from "../🧫️fixtures/🔣️.json";
import fixtureSchema from "../🧬️schema/🔣️.json";
import wireSchema from "../../../🧬️schema/🔣️.json";
import policySchema from "../🧬️schema/🎛️policy/🔣️.json";
const require=createRequire(import.meta.url),Ajv=require("ajv/dist/2020");

test("closed mutation inventory fixtures agree with independent SQLite and schema admission",()=>{
 const ajv=new Ajv({strict:true});
 expect(ajv.compile(fixtureSchema)(fixture)).toBe(true);
 expect(ajv.compile(fixtureSchema)({...fixture,undeclared:true})).toBe(false);
 const validateWire=ajv.compile({$schema:"https://json-schema.org/draft/2020-12/schema",$defs:wireSchema.$defs,$ref:"#/$defs/RuntimeMutationInventory"});
 for(const row of fixture.cases){expect(sqliteInventory(row)).toEqual(row.expected);expect(validateWire(row.expected)).toBe(true);}
 console.log("[DEBUG] Eight inventory projections agree with independent SQLite");
});
test("resource policy requires finite positive work, owned bytes and time",()=>{
 const validate=new Ajv({strict:true}).compile(policySchema),valid={version:1,maximumUnits:2000000,maximumOwnedBytes:4194304,budgetMs:60000};
 expect(validate(valid)).toBe(true);
 for(const field of ["maximumUnits","maximumOwnedBytes","budgetMs"]){expect(validate({...valid,[field]:0})).toBe(false);const absent:any={...valid};delete absent[field];expect(validate(absent)).toBe(false);}
 expect(validate({...valid,signal:true})).toBe(false);
});

