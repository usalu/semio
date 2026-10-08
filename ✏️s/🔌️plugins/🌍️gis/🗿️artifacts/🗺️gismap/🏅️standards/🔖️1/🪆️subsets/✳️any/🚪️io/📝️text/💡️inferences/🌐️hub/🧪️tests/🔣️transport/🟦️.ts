/** 🧪️ Artifact-owned physical transport corpus and independent oracles. */
import {test,expect,afterAll} from "bun:test";
import Ajv from "ajv";
import {Buffer} from "node:buffer";
import fixture from "./🧫️fixtures/🔣️.json";
import {gisMapInferenceRequestToJson} from "./../../🟦️.ts";
import {sealGisMapInferenceJobRequestV1} from "./../../../../../../../../../../💡️inference/🧬️schema/🟦️.ts";
const oracle=new Ajv({strict:false});
test("GIS semantic requests acquire JSON byte admission only at transport IO",()=>{
 const row=fixture.gisInference,value=sealGisMapInferenceJobRequestV1(row.requestId,row.lifetimeMs),json=gisMapInferenceRequestToJson(value);
 expect(json).toBe(row.json);expect(Buffer.byteLength(json,"utf8")).toBe(row.bytes);
 expect(oracle.compile({const:JSON.parse(row.json)})(JSON.parse(json))).toBe(true);
 expect(()=>gisMapInferenceRequestToJson({...value,requestId:"x".repeat(10000)})).toThrow("oversized-request");
});
afterAll(()=>console.log("[DEBUG] Owned transport corpus completed: @semio-tech/gis-gismap-js"));
