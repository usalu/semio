import {test,expect} from "bun:test";
import Ajv from "ajv";
import corpus from "/home/deck/semio/🧰️framework/🔨️modules/🧬️schema/✅️validator/🧪️tests/🧫️fixtures/🔣️.json";
import {compileJsonSchemaSubset,validateJsonSchemaSubset} from "/home/deck/semio/🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
for(const row of corpus.cases)test(row.id,()=>{const docs=("documents" in row.input?row.input.documents:[])??[];const oracle=new Ajv({strict:false,schemas:docs}).compile(row.input.schema);expect(oracle(row.value)).toBe(row.valid);expect(compileJsonSchemaSubset(row.input.schema,row.input.schema,docs).validate(row.value).length===0).toBe(row.valid);expect(validateJsonSchemaSubset(row.input.schema,row.value,row.input.schema,docs).length===0).toBe(row.valid);});

import {readFileSync} from "node:fs";
import {createHash} from "node:crypto";
import regressions from "./schema-definitions-validator-cases.json";
for(const row of regressions.cases)test("actual conditional refusal "+row.path,()=>{const bytes=readFileSync(row.path);expect(createHash("sha256").update(bytes).digest("hex")).toBe(row.sha256);const root=JSON.parse(bytes.toString()),schema=row.pointer?row.pointer.slice(1).split("/").reduce((node,key)=>node[key.replaceAll("~1","/").replaceAll("~0","~")],root):root;expect(root.$schema).toBe("http://json-schema.org/draft-07/schema#");const oracle=new Ajv({strict:false}).addSchema(root,"actual").getSchema("actual#"+row.pointer)!;expect(oracle(row.value)).toBe(false);expect(validateJsonSchemaSubset(schema,row.value,root).length>0).toBe(true);});

for(const row of corpus.compilationRefusals)test("schema compilation refusal "+row.id,()=>{expect(()=>new Ajv({strict:false}).compile(row.input.schema)).toThrow();expect(()=>compileJsonSchemaSubset(row.input.schema)).toThrow();expect(()=>validateJsonSchemaSubset(row.input.schema,null)).toThrow();});
