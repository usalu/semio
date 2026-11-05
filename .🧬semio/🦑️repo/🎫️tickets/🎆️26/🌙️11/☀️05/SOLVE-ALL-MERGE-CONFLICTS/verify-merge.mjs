import fs from 'node:fs';
import assert from 'node:assert/strict';
import {stripTypeScriptTypes} from 'node:module';
import {DatabaseSync} from 'node:sqlite';
const storage='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/';
const source=fs.readFileSync(storage+'🟦️.ts','utf8'),tests=fs.readFileSync(storage+'🧪️tests/🚧️fence/🟦️.ts','utf8'),corpus=JSON.parse(fs.readFileSync(storage+'🧪️tests/🚧️fence/🔣️.json','utf8')),schema=JSON.parse(fs.readFileSync(storage+'🧬️schema/📥️install.json','utf8'));
for(const text of[source,tests]){assert(!/^<<<<<<< |^>>>>>>> /m.test(text));stripTypeScriptTypes(text,{mode:'transform'});}
const names=['sameJavascriptDependencyParentIdentityV1','javascriptDependencyPreservationMismatchV1','javascriptDependencyPreservationSourceMismatchV1','requireJavascriptDependencyPreservationSourceUnchangedV1','javascriptDependencyStoreWorkV1'];
const extracted=names.map(name=>{const start=source.indexOf('function '+name+'(');assert(start>=0,name);const line=source.indexOf('\n',start),end=source.slice(start,line).trimEnd().endsWith('}')?line-2:source.indexOf('\n}',start);assert(end>=0);return source.slice(start,end+2);}).join('\n');
const validate=(spec,row)=>{if(!row||typeof row.path==='string'&&row.path.length>spec.properties.path.maxLength)return['invalid'];return[];};
const api=new Function('installSchema','validateJsonSchemaSubset',stripTypeScriptTypes(extracted,{mode:'transform'}).replace(/^export /gm,'')+';return {'+names.join(',')+'};')(schema,validate);
const maps=o=>[new Map(o.rows.map(r=>[r.sourceDirectory,r])),new Map(o.parents.map(r=>[r.path,r.identity])),new Map(o.ancestors.map(r=>[r.path,r.identity]))];
const db=new DatabaseSync(':memory:');
let checked=0;
for(const row of corpus.diagnostic){
 const original={rows:[{sourceDirectory:'/p/node_modules',kind:'link',stamp:'old',target:'/old'},{sourceDirectory:'/q/node_modules',kind:'link',stamp:'old',target:'/old'}],parents:[{path:'/p',identity:JSON.stringify(['1','2','3','4','5','6','7'])}],ancestors:[{path:'/',identity:JSON.stringify(['1','2','3'])}]},current=structuredClone(original);
 if(row.section==='rows'){if(row.field==='long-target')current.rows[0].target='x'.repeat(4096);else if(row.field==='missing')current.rows.pop();else if(row.field==='reorder')current.rows.reverse();else if(row.field==='serialization')current.rows[0]=Object.fromEntries(Object.entries(current.rows[0]).reverse());else if(row.field!=='none')Object.assign(current.rows[0],{[row.field]:row.field==='stamp'?'new':'/new'});}
 if(row.section==='parents'){const v=JSON.parse(current.parents[0].identity);v[row.field==='mtime'?5:1]='9';current.parents[0].identity=JSON.stringify(v);}
 if(row.section==='ancestors')current.ancestors[0].identity=JSON.stringify(['1','2','9']);
 const actual=api.javascriptDependencyPreservationSourceMismatchV1(current,...maps(original));assert.deepEqual(actual,row.expected,row.id);
 let error;try{api.requireJavascriptDependencyPreservationSourceUnchangedV1(current,...maps(original));}catch(e){error=e;}
 assert.equal(error===undefined,actual===null,row.id);
 if(error){assert(error.message.startsWith('Exact current Store Source fence changed'));assert.equal(error.cause.purpose,'current-store-source-fence-mismatch');}
 const oracle=db.prepare('select json_extract(?1,?) as value').get(JSON.stringify(row), '$.expected.field').value;assert.equal(actual?.field??null,oracle,row.id);checked++;
}
for(const row of corpus.neutralDiagnostic){
 const stamp=JSON.stringify(['1','2','16832','1','0','10','11']),original={rows:[{sourceDirectory:'/workspace/p/node_modules',kind:'link',stamp,target:'/store/current/node_modules'}],parents:[{path:'/workspace/p',identity:stamp}],ancestors:[{path:'/workspace',identity:JSON.stringify(['1','3','16832'])}]},current=structuredClone(original);
 if(row.mode==='endpoint-time'){const s=JSON.parse(current.rows[0].stamp);s[5]='99';current.rows[0].stamp=JSON.stringify(s);}
 if(row.mode==='endpoint-target')current.rows[0].target='/store/peer/node_modules';
 if(row.mode==='parent-device'){const s=JSON.parse(current.parents[0].identity);s[0]='9';current.parents[0].identity=JSON.stringify(s);}
 if(row.mode==='ancestor-inode'){const s=JSON.parse(current.ancestors[0].identity);s[1]='9';current.ancestors[0].identity=JSON.stringify(s);}
 if(row.mode==='endpoint-count')current.rows=[];
 if(row.mode==='endpoint-order')current.rows[0].sourceDirectory='/workspace/peer/node_modules';
 if(row.mode==='bounded-path')current.rows[0].sourceDirectory='/workspace/'+'p'.repeat(300);
 if(row.mode==='all-fields')current.rows[0]={sourceDirectory:'/workspace/peer/node_modules',kind:'file',target:'/store/peer/node_modules',stamp:JSON.stringify(['9','9','9','9','9','99','99'])};
 let error;try{api.requireJavascriptDependencyPreservationSourceUnchangedV1(current,...maps(original));}catch(e){error=e;}
 assert(error,row.id);assert.deepEqual(error.cause,{version:1,purpose:'current-store-source-fence-mismatch',section:row.section,index:0,path:row.path,fields:row.fields},row.id);
 assert(!JSON.stringify(error.cause).includes('/store/'));checked++;
}
let calls=0;const work=api.javascriptDependencyStoreWorkV1(2,()=>calls++);for(let i=0;i<100;i++)work.check();assert.equal(work.completed,0);work.charge();work.charge();assert.equal(work.completed,2);assert.throws(()=>work.charge());assert(calls>=103);
for(const name of ['verifyJavascriptDependencyDescriptorGroupV1','captureJavascriptDependencyDescriptorInputsV1']){
 const at=source.indexOf('function '+name+'(');assert(at>=0);const body=source.slice(at,source.indexOf('\n}',at)+2);assert(body.includes('withJavascriptDependencyInputParentsV1'));assert(body.includes('javascriptDependencyStoreWorkV1'));assert(body.includes('charge();await advance();'));assert(body.includes('if(stamp(file.stat())!=='));checked++;
}
db.close();
console.log(JSON.stringify({typescriptFilesParsed:2,languageNeutralDiagnosticCases:checked,workAccounting:'passed',traversalAndProgress:'passed',sqlite:'passed'}));
