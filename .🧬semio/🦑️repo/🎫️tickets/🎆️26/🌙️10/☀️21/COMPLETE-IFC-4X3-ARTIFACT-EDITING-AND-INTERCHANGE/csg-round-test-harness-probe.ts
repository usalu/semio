/** 🔬️ Isolate actual fixture cloning and neutral schema validation before attributing producer failures. */
import Ajv from'ajv/dist/2020';import{readFile}from'node:fs/promises';
import{ifcRoundFixture}from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/csg-round-fixtures.ts';
const tests=new URL('../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/',import.meta.url),data=JSON.parse(await readFile(new URL('csg-round-cases.json',tests),'utf8')),supplement=JSON.parse(await readFile(new URL('csg-round-supplemental-cases.json',tests),'utf8'));
for(const[name,input]of[['csg-round',data],['csg-round-supplemental',supplement]]){const definition=JSON.parse(await readFile(new URL(name+'.schema.json',tests),'utf8')),validate=new Ajv({strict:true}).compile(definition);console.log('[DEBUG]',name,validate(input),JSON.stringify(validate.errors));}
const row=data.cases[0],fixture=ifcRoundFixture(row),snapshot=fixture.snapshot,copy=structuredClone(snapshot);
console.log('[DEBUG]',JSON.stringify({same:JSON.stringify(snapshot,(_key,v)=>typeof v==='bigint'?String(v):v)===JSON.stringify(copy,(_key,v)=>typeof v==='bigint'?String(v):v),entities:snapshot.entities.length,first:snapshot.entities[1],copied:copy.entities[1]},(_key,v)=>typeof v==='bigint'?String(v):v));
