import fs from "node:fs";import path from "node:path";import {createRequire} from "node:module";
const req=createRequire(path.join(process.cwd(),"package.json")),Ajv=req("ajv/dist/2020"),map=req("lodash/map"),equal=req("lodash/isEqual"),base="🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛬️receive/🧪️tests/",law=JSON.parse(fs.readFileSync(base+"🧫️fixtures/📦️bytes.json","utf8")),schema=JSON.parse(fs.readFileSync(base+"🧬️schema/📦️bytes.json","utf8"));
if(!new Ajv({strict:true}).compile(schema)(law))throw Error("Octet fixture invalid");
const actual=map(Buffer.from(law.bytes),value=>value);
if(!equal(actual,law.expected))throw Error("Independent octet sequence mismatch");
console.log("[DEBUG] "+JSON.stringify({octets:actual.length,expected:actual}));
