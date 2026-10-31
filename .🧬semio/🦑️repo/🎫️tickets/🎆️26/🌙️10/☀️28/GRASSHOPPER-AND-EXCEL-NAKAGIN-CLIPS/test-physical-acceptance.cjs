/** 🧬 Independently reproduce complete physical domain decisions with AJV. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const read=p=>JSON.parse(fs.readFileSync(path.join(__dirname,p),'utf8'));
const validate=new Ajv({strict:true}).compile(read('physical-acceptance-domains.schema.json'));
const examples=read('🗑️generated/physical-acceptance-examples.json');
for(const [name,rows,expected] of examples)assert.equal(validate(rows),expected,name);
console.log('[DEBUG] Independent AJV domain completeness: '+examples.length+' positive/negative contracts PASS.');
