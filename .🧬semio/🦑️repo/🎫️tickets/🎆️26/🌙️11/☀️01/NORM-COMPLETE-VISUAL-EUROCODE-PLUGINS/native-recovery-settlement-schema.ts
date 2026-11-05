import fs from 'node:fs';
const p='🧰️framework/🔨️modules/🚪️io/⏱️control/🧾️failure/schema.json';const schema=JSON.parse(fs.readFileSync(p,'utf8'));
if(!schema.required.includes('settlementRefusal'))schema.required.push('settlementRefusal');
schema.properties.settlementRefusal=structuredClone(schema.properties.contextRefusal);
fs.writeFileSync(p,JSON.stringify(schema,null,2)+'\n');
