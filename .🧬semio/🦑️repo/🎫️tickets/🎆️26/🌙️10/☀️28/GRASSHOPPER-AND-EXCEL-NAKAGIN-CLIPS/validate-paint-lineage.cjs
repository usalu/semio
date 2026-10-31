/** 🧬 Validate the portable original-carrier and positive-fragment witness. */
const fs=require('node:fs'),path=require('node:path'),assert=require('node:assert/strict'),Ajv=require('ajv/dist/2020').default;
const cases=JSON.parse(fs.readFileSync(path.join(__dirname,'pixel-plane-lineage-cases.json'),'utf8'));
const schema=JSON.parse(fs.readFileSync(path.join(__dirname,'pixel-plane-lineage.schema.json'),'utf8'));
const validate=new Ajv({strict:true}).compile(schema);
assert.equal(validate(cases),true,JSON.stringify(validate.errors));
for(const mutate of [x=>x.expected_triangles=0,x=>x.fragment.vertices[0]=[0,0],x=>x.ulp_multiplier=65]){const bad=structuredClone(cases);mutate(bad);assert.equal(validate(bad),false);}
console.log('[DEBUG] Original carrier / positive triangle contract accepted; three invalid controls rejected.');
