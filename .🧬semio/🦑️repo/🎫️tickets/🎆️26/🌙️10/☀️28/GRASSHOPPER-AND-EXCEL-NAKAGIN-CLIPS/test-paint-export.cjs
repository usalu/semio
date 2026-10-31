/** 🎨 Validate explicit clipped-paint rows independently of immutable physical Item geometry. */
const fs=require('node:fs');
const path=require('node:path');
const assert=require('node:assert/strict');
let root=__dirname;
while(!fs.existsSync(path.join(root,'🧰️framework'))){const parent=path.dirname(root);if(parent===root)throw Error('Repository root missing');root=parent;}
const Ajv=require('ajv/dist/2020').default;
const ticket=__dirname;
const cases=JSON.parse(fs.readFileSync(path.join(ticket,'pixel-ray-cases.json'),'utf8'));
const validate=new Ajv({strict:true}).compile(JSON.parse(fs.readFileSync(path.join(ticket,'pixel-paint-fragments.schema.json'),'utf8')));
const physical=new Ajv({strict:true}).compile(JSON.parse(fs.readFileSync(path.join(ticket,'pixel-source-export.schema.json'),'utf8')));
for(const row of cases.fragment_cases){assert.equal(validate(row.fragments),true,JSON.stringify(validate.errors));assert.equal(physical(row.items),true,JSON.stringify(physical.errors));}
for(const change of [rows=>delete rows[0].piece,rows=>rows[0].stage='inferred',rows=>rows[0].world_vertices[0]=[0,0],rows=>rows[0].source=-1,rows=>delete rows[0].normal,rows=>rows[0].normal=[0,0,0]]){
 const bad=structuredClone(cases.fragment_cases[0].fragments);change(bad);assert.equal(validate(bad),false);
}
for(const argument of process.argv.slice(2))assert.equal(validate(JSON.parse(fs.readFileSync(argument,'utf8'))),true,JSON.stringify(validate.errors));
const warpCases=JSON.parse(fs.readFileSync(path.join(ticket,'pixel-warped-mesh-cases.json'),'utf8'));
const warpValidate=new Ajv({strict:true}).compile(JSON.parse(fs.readFileSync(path.join(ticket,'pixel-warped-mesh.schema.json'),'utf8')));
assert.equal(warpValidate(warpCases),true,JSON.stringify(warpValidate.errors));
for(const mutate of [value=>value.tie_diagonal=[1,3],value=>value.roundoff_ulps=65]){
 const invalid=structuredClone(warpCases);mutate(invalid);assert.equal(warpValidate(invalid),false);
}
const fanCases=JSON.parse(fs.readFileSync(path.join(ticket,'pixel-convex-fan-cases.json'),'utf8'));
const fanValidate=new Ajv({strict:true}).compile(JSON.parse(fs.readFileSync(path.join(ticket,'pixel-convex-fan.schema.json'),'utf8')));
assert.equal(fanValidate(fanCases),true,JSON.stringify(fanValidate.errors));
for(const mutate of [value=>value.cases[0].vertices[0]=[0,0],value=>value.tiny_triangle.nearest=[1]]){
 const invalid=structuredClone(fanCases);mutate(invalid);assert.equal(fanValidate(invalid),false);
}
console.log('[DEBUG] Six paint/five mesh/four convex controls accepted; ten malformed contracts rejected.');
