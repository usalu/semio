/** 🧬 Independently validate actual-source polygon interchange and portable negative contracts. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const read=p=>JSON.parse(fs.readFileSync(path.join(__dirname,p),'utf8'));
const validate=new Ajv({strict:true}).compile(read('pixel-source-export.schema.json'));
for(const test of read('pixel-ray-cases.json').cases)assert.equal(validate(test.items),true,test.name);
assert.equal(validate(read('🗑️generated/coverage-paint-order-diagnostic/coverage-items.json')),true,JSON.stringify(validate.errors));
for(const mutation of ['opacity','normal','pixel']){
 const rows=structuredClone(read('pixel-ray-cases.json').cases[0].items);
 if(mutation==='opacity')rows[0].opacity=2;
 if(mutation==='normal')rows[0].normal=[0,1];
 if(mutation==='pixel')rows[0].pixel_polygons[0][0]=[0,0,0];
 assert.equal(validate(rows),false,mutation);
}
console.log('[DEBUG] Exact source export+7portable cases accepted;3malformed exports rejected.');
