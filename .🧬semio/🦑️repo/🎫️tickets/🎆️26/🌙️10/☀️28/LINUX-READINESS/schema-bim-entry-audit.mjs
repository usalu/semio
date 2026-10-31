import fs from "node:fs";
import Ajv from "ajv";
const owner="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any",schema=owner+"/🧬️schema",ajv=new Ajv({strict:false,allErrors:true});
for(const dir of [".","📸️snapshot","🔺️diff","💡️inferences"])ajv.addSchema(JSON.parse(fs.readFileSync(schema+"/"+dir+"/🔣️.json","utf8")));
const doc=JSON.parse(fs.readFileSync(schema+"/🔺️diff/🔣️.json","utf8")),validate=ajv.compile({$ref:doc.$id}),corpus=JSON.parse(fs.readFileSync(schema+"/🧪️tests/🚪️parsers/🔣️.json","utf8"));
const rows=[...corpus.diffCases.map(row=>({...row,ref:doc.$id+"#/$defs/"+row.exported})),...corpus.entryCases.map(row=>({...row,ref:doc.$id+"#/$defs/"+row.exported})),...corpus.collectionCases.map(row=>({...row,ref:doc.$id}))].map(row=>{const check=ajv.compile({$ref:row.ref});return{id:row.id,expected:row.valid,actual:check(row.value)}});
const failures=[];let fixtureCount=0;
function inspect(value,file){if(!value||typeof value!=="object")return;if(!Array.isArray(value)&&Object.hasOwn(value,"diff")){fixtureCount++;if(!validate(value.diff))failures.push({file,errors:validate.errors});}for(const child of Object.values(value))inspect(child,file);}
function walk(dir){for(const entry of fs.readdirSync(dir,{withFileTypes:true})){const file=dir+"/"+entry.name;if(entry.isDirectory())walk(file);else if(entry.name==="🔣️.json"){const value=JSON.parse(fs.readFileSync(file,"utf8"));if(file.includes("/🔺️diff/")){fixtureCount++;if(!validate(value))failures.push({file,errors:validate.errors});}else inspect(value,file);}}}
walk(owner+"/🧪️tests/🧫️fixtures");
const result={at:new Date().toISOString(),rows,fixtureCount,failures};fs.writeFileSync(new URL("🗑️generated/schema-bim-entry-audit.json",import.meta.url),JSON.stringify(result,null,2)+"\n");console.log(JSON.stringify({rows:rows.length,mismatches:rows.filter(row=>row.actual!==row.expected).length,fixtureCount,failures:failures.length}));if(failures.length||rows.some(row=>row.actual!==row.expected))process.exitCode=1;
