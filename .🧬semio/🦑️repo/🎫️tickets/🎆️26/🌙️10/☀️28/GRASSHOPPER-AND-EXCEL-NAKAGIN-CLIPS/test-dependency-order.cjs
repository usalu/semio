/** 🪢 Validate portable painter-dependency graphs with AJV. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
const Ajv=require('ajv/dist/2020').default;
let root=__dirname;
while(!fs.existsSync(path.join(root,'🧰️framework'))){const parent=path.dirname(root);if(parent===root)throw Error('Repository root missing');root=parent;}
const cases=JSON.parse(fs.readFileSync(path.join(root,'🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/👁️occlusion/🔣️.json'),'utf8')).dependency_order;
const schema=JSON.parse(fs.readFileSync(path.join(__dirname,'dependency-order.schema.json'),'utf8'));
const validate=new Ajv({strict:true}).compile(schema);
assert.equal(validate(cases),true,JSON.stringify(validate.errors));
for(const row of cases){assert.equal(row.edges.length,row.depths.length);assert.deepEqual([...row.expected].sort((a,b)=>a-b),row.edges.map((_,index)=>index));for(const targets of row.edges)for(const target of targets)assert.ok(target<row.edges.length);}
const invalid=structuredClone(cases);delete invalid[0].depths;assert.equal(validate(invalid),false);
const negative=structuredClone(cases);negative[0].edges[0]=[-1];assert.equal(validate(negative),false);
const duplicate=structuredClone(cases);duplicate[0].expected=[0,0,1];assert.equal(validate(duplicate),false);
console.log('[DEBUG] '+cases.length+' portable dependency graphs valid; 3 malformed contracts rejected.');
