/** 📚 Validate the single native registry dependency manifest. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
const read=name=>JSON.parse(fs.readFileSync(path.join(__dirname,name),'utf8'));
const contract=read('scene-provenance.json').registry_inputs;
const values=read(contract.manifest);
const validate=new Ajv({strict:true}).compile(read(contract.schema));
assert.equal(validate(values),true,JSON.stringify(validate.errors));
assert.equal(values.length,contract.count);
for(const invalid of [[],[...values,values[0]],['../outside.json'],['/outside.json'],[3]]) assert.equal(validate(invalid),false);
console.log('[DEBUG] Registry manifest accepted; five malformed contracts rejected.');
