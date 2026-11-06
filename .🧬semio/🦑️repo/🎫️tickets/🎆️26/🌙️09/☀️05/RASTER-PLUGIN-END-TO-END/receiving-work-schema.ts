import {readFileSync,writeFileSync} from 'node:fs';
const file='🧰️framework/🔨️modules/🚪️io/⏱️control/🧾️failure/schema.json';
const schema=JSON.parse(readFileSync(file,'utf8'));let count=0;
function visit(value:any){if(!value||typeof value!=='object')return;if(value.properties?.allocationRefusal&&Array.isArray(value.required)&&value.required.includes('allocationRefusal')){value.properties.workRefusal=structuredClone(value.properties.allocationRefusal);if(!value.required.includes('workRefusal'))value.required.push('workRefusal');count++}for(const child of Object.values(value))visit(child)}
visit(schema);if(count!==1)throw new Error(`expected one receiving schema, found ${count}`);writeFileSync(file,JSON.stringify(schema,null,2)+'\n');
