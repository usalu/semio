/** 🎬 Validate the complete bilingual native replay plan with AJV. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const read=name=>JSON.parse(fs.readFileSync(path.join(__dirname,name),'utf8'));
const plan=read('native-pairs.json');
const validate=new Ajv({strict:true}).compile(read('native-pairs.schema.json'));
assert.equal(validate(plan),true,JSON.stringify(validate.errors));
const malformed=[{...plan,languages:['en']},{...plan,series:{...plan.series,excel:{...plan.series.excel,scenes:plan.series.excel.scenes.slice(1)}}},{...plan,series:{...plan.series,grasshopper:{...plan.series.grasshopper,contracts:11}}}];
for(const row of malformed)assert.equal(validate(row),false);
console.log('[DEBUG] Complete native plan AJV PASS; '+malformed.length+' incomplete plans rejected.');
