const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto'),assert=require('node:assert/strict');
const Ajv07=require('ajv').default,Ajv2020=require('ajv/dist/2020').default;
const directory=__dirname,cohort=JSON.parse(fs.readFileSync(path.join(directory,'🗑️generated/schema-dialect-candidates.json'),'utf8')),vectors=JSON.parse(fs.readFileSync(path.join(directory,'schema-dialect-vectors.json'),'utf8')),dialect='http://json-schema.org/draft-07/schema#';
const hash=bytes=>crypto.createHash('sha256').update(bytes).digest('hex'),options={strict:false,allErrors:false,validateFormats:false,logger:false};
const annotations=new Set(['$schema','$id','$comment','title','description','default','examples','readOnly','writeOnly','deprecated']);
const keywords=new Set([...annotations,'$ref','definitions','type','enum','const','allOf','anyOf','oneOf','not','if','then','else','properties','patternProperties','additionalProperties','propertyNames','required','minProperties','maxProperties','items','contains','minItems','maxItems','uniqueItems','minLength','maxLength','pattern','minimum','maximum','exclusiveMinimum','exclusiveMaximum','multipleOf']);
function closedVocabulary(node,location=''){
 if(typeof node==='boolean')return;if(!node||typeof node!=='object'||Array.isArray(node))throw Error('Schema object required at '+location);
 for(const key of Object.keys(node))if(!keywords.has(key)&&!key.startsWith('x-'))throw Error('Unproved keyword '+key+' at '+location);
 if(location&&(Object.hasOwn(node,'$id')||Object.hasOwn(node,'$schema')))throw Error('Embedded schema resource refused at '+location);
 if(node.$ref!==undefined){if(typeof node.$ref!=='string'||!(node.$ref==='#'||node.$ref.startsWith('#/')))throw Error('Nonlocal reference refused at '+location);for(const key of Object.keys(node))if(key!=='$ref'&&key!=='definitions'&&!annotations.has(key)&&!key.startsWith('x-'))throw Error('Reference assertion sibling refused at '+location);}
 if(Array.isArray(node.items))throw Error('Legacy tuple vocabulary refused at '+location);
 for(const key of ['definitions','properties','patternProperties'])if(node[key]!==undefined){if(!node[key]||typeof node[key]!=='object'||Array.isArray(node[key]))throw Error('Schema map required at '+location+'/'+key);for(const[name,child]of Object.entries(node[key]))closedVocabulary(child,location+'/'+key+'/'+name);}
 for(const key of ['additionalProperties','propertyNames','items','contains','not','if','then','else'])if(node[key]!==undefined)closedVocabulary(node[key],location+'/'+key);
 for(const key of ['allOf','anyOf','oneOf'])if(node[key]!==undefined){if(!Array.isArray(node[key]))throw Error('Schema sequence required at '+location+'/'+key);node[key].forEach((child,index)=>closedVocabulary(child,location+'/'+key+'/'+index));}
}
function pointer(root,value){if(!value.startsWith('#/'))throw Error('Local schema pointer required');return value.slice(2).split('/').reduce((node,key)=>node[key.replaceAll('~1','/').replaceAll('~0','~')],root);}
function merge(values){if(values.every(value=>value&&typeof value==='object'&&!Array.isArray(value)))return Object.assign({},...values);return values[0];}
function witnesses(node,root,depth=0){
 if(depth>12||node===false)return [];if(node===true)return [null];if(!node||typeof node!=='object')throw Error('Schema node required');
 if(Object.hasOwn(node,'const'))return [structuredClone(node.const)];if(node.enum)return node.enum.slice(0,16).map(value=>structuredClone(value));
 if(node.$ref)return witnesses(pointer(root,node.$ref),root,depth+1);
 const alternatives=[...(node.anyOf??[]),...(node.oneOf??[])].flatMap(child=>witnesses(child,root,depth+1));
 const conjunction=node.allOf?.length?[merge(node.allOf.map(child=>witnesses(child,root,depth+1)[0]))]:[];
 const types=Array.isArray(node.type)?node.type:[node.type??(node.properties?'object':node.items?'array':undefined)],values=[];
 for(const type of types){
  if(type==='null')values.push(null);
  else if(type==='boolean')values.push(false,true);
  else if(type==='number'||type==='integer'){let low=node.minimum??0;if(node.exclusiveMinimum!==undefined)low=Math.max(low,node.exclusiveMinimum+(type==='integer'?1:0.5));if(type==='integer')low=Math.ceil(low);if(node.multipleOf)low=Math.ceil(low/node.multipleOf)*node.multipleOf;values.push(low,...[node.minimum,node.maximum,node.exclusiveMinimum,node.exclusiveMaximum].filter(value=>typeof value==='number'));}
  else if(type==='string'){const length=Math.min(1024,node.minLength??1);values.push('a'.repeat(length),'0'.repeat(length),'x'.repeat(length),...(typeof node.default==='string'?[node.default]:[]));}
  else if(type==='array'){const length=node.minItems??0;if(length<=128){const child=witnesses(node.items??true,root,depth+1)[0];values.push(Array.from({length},()=>structuredClone(child??null)));}}
  else if(type==='object'){const properties=node.properties??{},make=all=>{const value={};for(const key of all?Object.keys(properties):node.required??[])Object.defineProperty(value,key,{value:witnesses(properties[key]??true,root,depth+1)[0]??null,enumerable:true,writable:true,configurable:true});return value;};values.push(make(false),make(true));}
 }
 if(!types[0])values.push(null,{},[],0,'a');return [...alternatives,...conjunction,...values].slice(0,64);
}
const results=[];
for(const candidate of cohort.candidates){
 const result={path:candidate.path,sha256:candidate.sha256,eligible:false};
 try{
  const bytes=fs.readFileSync(candidate.path);assert.equal(hash(bytes),candidate.sha256,'Current exact schema bytes required');const source=JSON.parse(bytes),next={...source,$schema:dialect};closedVocabulary(source);result.sharedVocabulary=true;assert.equal(source.$schema,'https://json-schema.org/draft/2020-12/schema');
  const text=bytes.toString('utf8').replace(JSON.stringify(source.$schema),JSON.stringify(dialect));assert.deepEqual(JSON.parse(text),next,'Only the root dialect declaration may change');
  const oldValidator=new Ajv2020(options),newValidator=new Ajv07(options);assert.equal(oldValidator.validateSchema(source),true,JSON.stringify(oldValidator.errors));assert.equal(newValidator.validateSchema(next),true,JSON.stringify(newValidator.errors));oldValidator.addSchema(source,'proof');newValidator.addSchema(next,'proof');
  const targets=[{pointer:'',schema:source},...Object.entries(source.definitions??{}).map(([name,schema])=>({pointer:'/definitions/'+name.replaceAll('~','~0').replaceAll('/','~1'),schema}))],coverage=[];
  for(const target of targets){
   const old=oldValidator.getSchema('proof#'+target.pointer),current=newValidator.getSchema('proof#'+target.pointer);assert.equal(typeof old,'function');assert.equal(typeof current,'function');
   const samples=[...vectors.values,...witnesses(target.schema,source)],seen=new Set();let valid=0,invalid=0;
   for(const value of [...samples])if(value&&typeof value==='object'&&!Array.isArray(value)){samples.push({...value,__unexpected__:true});for(const key of Object.keys(value)){const missing={...value};delete missing[key];samples.push(missing,{...value,[key]:null},{...value,[key]:[]});}}
   for(const value of samples){const key=JSON.stringify(value);if(seen.has(key))continue;seen.add(key);const before=old(value),after=current(value);assert.equal(after,before,'Dialect validators disagree on '+target.pointer+' witness '+hash(key));if(before)valid++;else invalid++;}
   const unconstrained=target.schema===true||target.schema&&typeof target.schema==='object'&&Object.keys(target.schema).every(key=>annotations.has(key)||key==='definitions'||key.startsWith('x-'));
   coverage.push({pointer:target.pointer,valid,invalid,samples:seen.size,unconstrained:!!unconstrained,nonvacuous:valid>0&&(invalid>0||unconstrained)});
  }
  Object.assign(result,{valid:coverage.reduce((sum,row)=>sum+row.valid,0),invalid:coverage.reduce((sum,row)=>sum+row.invalid,0),samples:coverage.reduce((sum,row)=>sum+row.samples,0),coverage,sampleAgreement:true,nextSha256:hash(text),eligible:coverage.every(row=>row.nonvacuous)});if(!result.eligible)result.reason='Positive and constraint-negative witnesses required for every constrained root or named definition';
 }catch(error){result.reason=String(error.message).slice(0,1200);}
 results.push(result);
}
const summary={version:1,candidates:results.length,eligible:results.filter(row=>row.eligible).length,excluded:results.filter(row=>!row.eligible).length,samples:results.reduce((sum,row)=>sum+(row.samples??0),0),results};
fs.writeFileSync(path.join(directory,'🗑️generated/schema-dialect-proof.json'),JSON.stringify(summary,null,2)+'\n');console.log(JSON.stringify({candidates:summary.candidates,eligible:summary.eligible,excluded:summary.excluded,samples:summary.samples,reasons:results.filter(row=>!row.eligible).slice(0,8)}));
