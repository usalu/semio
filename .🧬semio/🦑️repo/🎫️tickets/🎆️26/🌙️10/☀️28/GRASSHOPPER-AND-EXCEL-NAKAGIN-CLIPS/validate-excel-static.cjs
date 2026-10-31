/** 🧮 Retain independent Excel JavaScript/AJV oracle input endpoints and completed output. */
const fs=require('node:fs');
const path=require('node:path');
const crypto=require('node:crypto');
const ticket=__dirname;
let root=ticket;while(!fs.existsSync(path.join(root,'🧰️framework')))root=path.dirname(root);
const clip=path.join(root,'🎓️teaching/🏛️architecture/📊️excel/🎬️clip');
const out=path.join(ticket,'🗑️generated/excel-static/node');fs.mkdirSync(out,{recursive:true});
const inputs=[__filename,path.join(clip,'🧪️tests/🟨️.cjs'),path.join(clip,'🧱️model/🔣️.json'),path.join(clip,'🧱️model/🧬️schema/🔣️.schema.json'),path.join(clip,'🧪️cases/🔣️.json'),path.join(root,'🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json')];
inputs.push(path.join(clip,'🧪️tests/⚡️anchors/🟨️.cjs'),path.join(clip,'🧪️cases/⚡️anchors/🔣️.json'),path.join(clip,'🧪️cases/⚡️anchors/🔣️.schema.json'));
const canonical=x=>Array.isArray(x)?x.map(canonical):x&&typeof x==='object'?Object.fromEntries(Object.keys(x).sort().map(k=>[k,canonical(x[k])])):x;
const fingerprint=()=>Object.fromEntries(inputs.map(p=>[path.relative(root,p),crypto.createHash('sha256').update(p.endsWith('.json')?JSON.stringify(canonical(JSON.parse(fs.readFileSync(p,'utf8')))):fs.readFileSync(p)).digest('hex')]));
const before=fingerprint(),started=new Date().toISOString();
fs.writeFileSync(path.join(out,'tests.log'),'');
const write=process.stdout.write.bind(process.stdout);
process.stdout.write=(chunk,...args)=>{fs.appendFileSync(path.join(out,'tests.log'),chunk);return write(chunk,...args);};
let passed=false,error=null;
try{require(path.join(clip,'🧪️tests/🟨️.cjs'));require(path.join(clip,'🧪️tests/⚡️anchors/🟨️.cjs'));passed=true;}catch(e){error=String(e.stack);console.error(error);}
const after=fingerprint();
const result={started_at:started,ended_at:new Date().toISOString(),assertions_passed:passed,source_fingerprints:before,source_fingerprints_end:after,source_unchanged:JSON.stringify(before)===JSON.stringify(after),runtime_identity:{node:process.version,versions:process.versions,platform:process.platform,ajv:require('ajv/package.json').version},error};
result.success=passed&&result.source_unchanged;
fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(result,null,2));
console.log('[DEBUG] Excel static Node '+JSON.stringify({success:result.success,source_unchanged:result.source_unchanged}));
process.exitCode=result.success?0:1;
