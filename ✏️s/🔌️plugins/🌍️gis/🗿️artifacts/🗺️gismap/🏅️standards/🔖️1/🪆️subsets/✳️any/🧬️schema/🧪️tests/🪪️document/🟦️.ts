/** 🧪️ Map document facets preserve every durable handle and dynamic feature value. */
import artifactReferenceSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json";
import {test} from "bun:test";
import {binary64,binary64Value} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {applyGisMapFeaturePatch,parseGisMapFeature,parseGisMapFeaturePatch,type GisMapValue} from "../../📍️feature/🟦️.ts";
import {readFileSync,readdirSync} from "node:fs";
import {applyPatch,type Operation} from "fast-json-patch";
import assert from "node:assert/strict";
import { testSchemaRecordOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🧪️tests/🔬️unit/🟦️.ts";
import { join } from "node:path";
import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/🪪️document/🟦️.ts";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import valueSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json" with { type: "json" };
import featureSchema from "../../📍️feature/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import { parseGisMapArtifact } from "../../🟦️.ts";
import { parseGisMapSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parseGisMapDiff } from "../../🔺️diff/🟦️.ts";
import vectors from "../../🧫️fixtures/🪪️document/🔣️.json" with { type: "json" };


function foreignValue(value:unknown):GisMapValue{if(value===null)return{kind:"null"};if(typeof value==="boolean")return{kind:"boolean",value};if(typeof value==="string")return{kind:"text",value};if(typeof value==="number")return Number.isSafeInteger(value)?value>=0?{kind:"unsigned",value:BigInt(value)}:{kind:"signed",value:BigInt(value)}:{kind:"float",value:binary64(value)};if(Array.isArray(value))return{kind:"array",items:value.map(foreignValue)};if(typeof value==="object")return{kind:"object",members:Object.entries(value).map(([name,value])=>({name,value:foreignValue(value)}))};throw Error("foreign JSON value required");}
function foreignFeature(value:unknown):unknown{if(value===null||typeof value!=="object")return value;const row=value as Record<string,unknown>;return{...row,...("data"in row?{data:foreignValue(row.data)}:{})};}
function foreignDocument(value:unknown):unknown{if(value===null||typeof value!=="object")return value;const row=value as Record<string,unknown>;return{...row,...(Array.isArray(row.positions)?{positions:row.positions.map(foreignFeature)}:{}),...(Array.isArray(row.routes)?{routes:row.routes.map(foreignFeature)}:{}),...(Array.isArray(row.regions)?{regions:row.regions.map(foreignFeature)}:{})};}
function foreignPatch(value:unknown):unknown{const row=value as Record<string,unknown>;return{...row,...(row.data==null?{}:{data:foreignValue(row.data)}),...(Array.isArray(row.properties)?{properties:row.properties.map((item:Record<string,unknown>)=>({...item,...(item.set===undefined?{}:{set:{value:foreignValue((item.set as {value:unknown}).value)}})}))}:{})};}
function foreignDelta(value:unknown):unknown{if(value===null||typeof value!=="object")return value;const row=value as Record<string,unknown>;return{...row,...(Array.isArray(row.inserted)?{inserted:row.inserted.map((item:Record<string,unknown>)=>({...item,row:foreignFeature(item.row)}))}:{}),...(Array.isArray(row.modified)?{modified:row.modified.map((item:Record<string,unknown>)=>({...item,patch:foreignPatch(item.patch)}))}:{})};}
function foreignDiff(value:unknown):unknown{if(value===null||typeof value!=="object")return value;const row=value as Record<string,unknown>;return{...row,...(row.positions==null?{}:{positions:foreignDelta(row.positions)}),...(row.routes==null?{}:{routes:foreignDelta(row.routes)}),...(row.regions==null?{}:{regions:foreignDelta(row.regions)})};}
function foreignJsonValue(value:GisMapValue):unknown{switch(value.kind){case"null":return null;case"boolean":case"text":return value.value;case"unsigned":case"signed":return Number(value.value);case"float":return binary64Value(value.value);case"array":return value.items.map(foreignJsonValue);case"object":return Object.fromEntries(value.members.map(m=>[m.name,foreignJsonValue(m.value)]));case"bytes":throw Error("octets are outside foreign JSON fixture domain");}}
function nativeJson(value:unknown):unknown{if(value===null||typeof value!=="object")return value;if(Array.isArray(value))return value.map(nativeJson);const row=value as Record<string,unknown>;return Object.fromEntries(Object.entries(row).filter(([name,value])=>name!=="properties"||!Array.isArray(value)||value.length>0).map(([name,value])=>[name,(name==="data"||name==="value")&&value!==null&&typeof value==="object"&&"kind"in value?foreignJsonValue(value as GisMapValue):nativeJson(value)]));}

/** 🗺️ Matches first-party parsing to Ajv and all committed Map mutation fixtures. */
export function testMapDocumentContractOracle(): void {
  testSchemaRecordOracle();
  assertDocumentContractOracle({
    name: "GIS Map", dependencies: [ioSchema, childSchema, valueSchema, featureSchema,artifactReferenceSchema],
    artifact: { schema: artifactSchema, parse: value=>parseGisMapArtifact(foreignDocument(value)), nativeJson },
    snapshot: { schema: snapshotSchema, parse: value=>parseGisMapSnapshot(foreignDocument(value)), nativeJson },
    diff: { schema: diffSchema, parse: value=>parseGisMapDiff(foreignDiff(value)), nativeJson },
    validDocuments: [{ input: vectors.document, output: vectors.document }],
    invalidDocuments: vectors.invalidDocuments, invalidDiffs: vectors.invalidDiffs,
    mutationRoots: [join(import.meta.dir, "../../../🧫️fixtures/🧬️mutations")],
    committed: { snapshots: 36, diffs: 18 },
  });
  assert.deepEqual(nativeJson(parseGisMapDiff(foreignDiff(vectors.diff))), vectors.diff);
  assert.deepEqual(parseGisMapDiff({}), { positions: null, routes: null, regions: null });
}

test("GIS map foreign JSON contract matches typed intrinsic feature meaning",testMapDocumentContractOracle);


test("GIS sparse feature patches match all nine neutral payload/property mutations and independent JSON Patch",()=>{
 const root=join(import.meta.dir,"../../../🧫️fixtures/🧬️mutations");
 const read=(file:string)=>JSON.parse(readFileSync(file,"utf8"));
 let count=0;
 for(const path of readdirSync(root,{recursive:true}).map(String).filter(path=>path.endsWith("/🔺️diff/🔣️.json"))){
  const caseRoot=join(root,path.slice(0,-"/🔺️diff/🔣️.json".length));
  const diff=read(join(root,path)),before=read(join(caseRoot,"📸️snapshot/⬅️before/🔣️.json")),after=read(join(caseRoot,"📸️snapshot/➡️after/🔣️.json"));
  for(const field of ["positions","routes","regions"]){
   for(const entry of diff[field]?.modified??[]){
    const original=before[field].find((feature:{id:string})=>feature.id===entry.id),expected=after[field].find((feature:{id:string})=>feature.id===entry.id);
    const actual=applyGisMapFeaturePatch(parseGisMapFeature(foreignFeature(original)),parseGisMapFeaturePatch(foreignPatch(entry.patch)));
    assert.deepEqual(actual,parseGisMapFeature(foreignFeature(expected)),path);
    const operations:Operation[]=entry.patch.data==null?[]:[{op:"replace",path:"",value:entry.patch.data}];
    for(const edit of entry.patch.properties??[]){const pointer="/"+edit.key.replaceAll("~","~0").replaceAll("/","~1");operations.push(edit.set===undefined?{op:"remove",path:pointer}:{op:"add",path:pointer,value:edit.set.value});}
    const independent=applyPatch(structuredClone(original.data),operations,true,false).newDocument;
    assert.deepEqual(foreignJsonValue(actual.data),independent,path);
    assert.deepEqual(independent,expected.data,path);
    count++;
   }
  }
 }
 assert.equal(count,9);
 console.log(`[DEBUG] GIS sparse feature patches executed ${count} independent JSON Patch mutation witnesses`);
});
