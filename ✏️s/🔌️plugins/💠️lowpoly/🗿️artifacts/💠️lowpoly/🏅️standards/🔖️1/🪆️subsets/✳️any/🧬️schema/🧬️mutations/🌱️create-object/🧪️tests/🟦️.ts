import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import{semioSchemaAjvV1}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🟦️typescript/🔮️oracles/✅️validator/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{parseLowpolyMeshState}from"../../../🕸️mesh/🟦️.ts";
import laws from"./🔣️.json";
const root=resolve(import.meta.dirname,"../../../../../../../../../../../../.."),schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));
const model="✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema";
const paths=[model+"/🔣️.json",model+"/🕸️mesh/🔣️.json","🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🪆️child/🧬️schema/🔣️.json","🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json","🧰️framework/🔨️modules/🌱️value/🧬️schema/🌳️intrinsic/🔣️.json","🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json"],documents=paths.map(path=>JSON.parse(readFileSync(resolve(root,path),"utf8")));
const fixture=JSON.parse(readFileSync(resolve(root,model+"/../🧪️tests/🧫️fixtures/🧬️mutations/🌱️create-object/⛵️inserts/🦠️mutation/🔣️.json"),"utf8")).CreateObject;
function oracle(){const ajv=semioSchemaAjvV1({allErrors:true});for(const document of documents)ajv.addSchema(document);return ajv.compile(schema);}
test("actual mutation payload schema is a valid draft07 resource",()=>{expect(semioSchemaAjvV1().validateSchema(schema)).toBe(true);});
test("the unchanged committed create-object payload remains accepted",()=>{expect(oracle()(fixture)).toBe(true);expect(validateJsonSchemaSubset(schema,fixture,schema,documents)).toEqual([]);});
for(const row of laws.meshStates)test(row.name,()=>{const payload=structuredClone(fixture);payload.object.meshState=row.value;expect(oracle()(payload)).toBe(row.valid);expect(validateJsonSchemaSubset(schema,payload,schema,documents).length===0).toBe(row.valid);if(row.valid&&row.value!==null)expect<unknown>(parseLowpolyMeshState(row.value)).toEqual(row.value);else if(!row.valid)expect(()=>parseLowpolyMeshState(row.value)).toThrow();});
for(const field of laws.requiredObjectFields)test("canonical object requires "+field,()=>{const payload=structuredClone(fixture);delete payload.object[field];expect(oracle()(payload)).toBe(false);expect(validateJsonSchemaSubset(schema,payload,schema,documents).length>0).toBe(true);});

test("mutation preserves the public canonical object fieldset",()=>{const actual=schema.properties.object,declared=documents[0].$defs.LowpolyObject;expect([...actual.required].sort()).toEqual([...declared.required].sort());expect(Object.keys(actual.properties).sort()).toEqual(Object.keys(declared.properties).sort());});
