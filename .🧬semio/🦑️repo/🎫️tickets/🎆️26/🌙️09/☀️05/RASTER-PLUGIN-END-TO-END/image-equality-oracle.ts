import {readFileSync} from "node:fs";
import Ajv from "ajv";
const file="✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔑️content-hash/🧪️tests/🟰️equality.json";
const fixture=JSON.parse(readFileSync(file,"utf8"));
const ajv=new Ajv({strict:false});
for(const row of fixture.cases){const actual=ajv.compile({const:row.left})(row.right);if(actual!==row.equal)throw new Error(`Independent equality oracle disagrees: ${row.name}`);}
console.log(JSON.stringify({oracle:"Ajv const deep equality",vectors:fixture.cases.length,matched:true}));
