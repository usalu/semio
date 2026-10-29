/** 🔮️ Ajv and Lodash independently check native field identity and dynamic map laws. */
import fs from "node:fs";import path from "node:path";import {createRequire} from "node:module";
const req=createRequire(path.join(process.cwd(),"package.json")),Ajv=req("ajv/dist/2020"),filter=req("lodash/filter"),equal=req("lodash/isEqual"),base="🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/",read=p=>JSON.parse(fs.readFileSync(base+p,"utf8"));
const record=read("🧩️record/🧪️tests/🧫️fixtures/🔣️.json"),map=read("🗂️map/🔣️.json");for(const [schema,data]of [[read("🧩️record/🧬️schema/🔣️.json"),record],[read("🗂️map/🧬️.json"),map]])if(!new Ajv({strict:false}).compile(schema)(data))throw Error("Portable native key fixture refused");
for(const row of record.keys){const matching=filter(record.fields,field=>Object.hasOwn(row.key,"name")?field.name===row.key.name:field.ordinal===row.key.ordinal).map(field=>field.name),bytes=Object.hasOwn(row.key,"name")?Buffer.byteLength(row.key.name):Buffer.alloc(2).length;if(!equal(matching,row.matches)||bytes!==row.bytes)throw Error("Declared field identity differs")}
for(const row of map.keyCases)if(Object.hasOwn(row.key,"name")!==row.accepted)throw Error("Dynamic map must require an actual name");
for(const row of map.cases){const actual=[...new Map(row.entries)].sort((a,b)=>Buffer.compare(Buffer.from(a[0]),Buffer.from(b[0])));if(!equal(actual,row.expected))throw Error("Independent map replacement/order differs")}
console.log("[DEBUG] "+JSON.stringify({fieldKeys:record.keys.length,dynamicKeys:map.keyCases.length,maps:map.cases.length}));
