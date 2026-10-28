import Ajv from'ajv';
import{readFileSync,writeFileSync}from'node:fs';
const load=(name:string)=>JSON.parse(readFileSync(new URL(name,import.meta.url),'utf8').replace(/^\uFEFF/u,''));
const schema=load('./boolean-owned-mesh-programs.schema.json'),law=load('./boolean-owned-mesh-programs.json'),validate=new Ajv({strict:true}).compile(schema);
if(!validate(law))throw Error(JSON.stringify(validate.errors));
if(validate({...law,physicalHeapBound:true})||validate({...law,hiddenAuthority:true}))throw Error('Closed neutral capacity policy');
writeFileSync(new URL('./🗑️generated/boolean-owned-mesh-schema.json',import.meta.url),JSON.stringify({valid:true,closed:true,programs:law.programs.length,steps:law.programs.reduce((sum:number,row:{steps:unknown[]})=>sum+row.steps.length,0),productionExecuted:false},null,2)+'\n');
console.log('[DEBUG] Closed neutral owned-mesh schema validated; production not executed');
