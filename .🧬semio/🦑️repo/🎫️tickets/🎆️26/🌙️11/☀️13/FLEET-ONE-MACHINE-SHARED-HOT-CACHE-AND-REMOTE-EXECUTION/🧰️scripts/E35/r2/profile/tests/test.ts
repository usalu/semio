import {test,expect} from "bun:test";
import {spawnSync} from "node:child_process";
import Ajv from "ajv";
import {manifestProfileSha256V1,manifestProfilePolicyV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import installSchema from "../../../💾️storage/🧬️schema/📥️install.json";
import cases from "./🔣️.json";

const profile=(name:string)=>manifestProfileSha256V1(cases.cases.find(row=>row.name===name)!.path,cases.cases.find(row=>row.name===name)!.text);

test("the profile equals the python oracle on every case and satisfies its schema",()=>{
 const oracle=spawnSync("python3",["-I",new URL("./🐍️.py",import.meta.url).pathname],{input:JSON.stringify({policy:manifestProfilePolicyV1,cases:cases.cases}),encoding:"utf8"});expect(oracle.status).toBe(0);
 const expected=JSON.parse(oracle.stdout)as (string|null)[],validate=new Ajv({strict:false}).compile({...schema.$defs.Profile,$defs:schema.$defs});
 cases.cases.forEach((row,at)=>{const actual=manifestProfileSha256V1(row.path,row.text);expect(actual,row.name).toBe(expected[at]);expect(validate(actual)).toBe(true);expect(manifestProfileSha256V1(row.path,Buffer.from(row.text))).toBe(actual);});
});

test("inert edits keep the profile and receipt-relevant edits change it",()=>{
 const base=profile("plain manifest");
 for(const name of ["scripts edited is inert","key order is inert","whitespace is inert"])expect(profile(name),name).toBe(base);
 for(const name of ["exports edited counts","dependency edited counts","version edited counts","semio edited counts"])expect(profile(name),name).not.toBe(base);
 expect(profile("exports nested order is inert")).toBe(profile("exports nested reordered"));
 expect(profile("array order counts")).not.toBe(profile("array order swapped counts"));
 for(const name of ["not an object","malformed","not a manifest"])expect(profile(name),name).toBeNull();
});

test("the excluded fields are a subset of the install projection's, and the workspace topology field stays covered",()=>{
 for(const field of manifestProfilePolicyV1.excludedTopLevelFields)expect(installSchema.properties.excludedTopLevelFields.const).toContain(field);
 expect(manifestProfilePolicyV1.excludedTopLevelFields).not.toContain("semio");
});
