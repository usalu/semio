/** 🧪️ Artifact-owned physical transport corpus and independent oracles. */
import {test,expect,afterAll} from "bun:test";
import Ajv from "ajv";
import {Buffer} from "node:buffer";
import fixture from "./🧫️fixtures/🔣️.json";
import {parseFormsJsonValue,formsValueJson} from "./../../🟦️.ts";
import {testFormsResponseExport} from "./../../../📨️response/📤️export/🧪️tests/🟦️.ts";
const oracle=new Ajv({strict:false});
test("Forms JSON and response exports follow the neutral transport corpus",()=>{
 expect(formsValueJson(parseFormsJsonValue(fixture.forms.value))).toBe(fixture.forms.json);
 expect(oracle.compile({const:fixture.forms.value})(JSON.parse(formsValueJson(parseFormsJsonValue(fixture.forms.value))))).toBe(true);
 testFormsResponseExport();
});
afterAll(()=>console.log("[DEBUG] Owned transport corpus completed: @semio-tech/forms-js"));
