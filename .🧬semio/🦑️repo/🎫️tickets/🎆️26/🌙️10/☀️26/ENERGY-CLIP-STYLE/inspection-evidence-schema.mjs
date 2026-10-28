/** 🧬 Check portable evidence coverage with the existing independent Ajv implementation. */
import fs from 'node:fs';
import path from 'node:path';
import {createRequire} from 'node:module';
import {fileURLToPath} from 'node:url';
const require=createRequire(import.meta.url),Ajv=require('ajv/dist/2020');
const ticket=path.dirname(fileURLToPath(import.meta.url));
const read=name=>JSON.parse(fs.readFileSync(path.join(ticket,name),'utf8'));
const validate=new Ajv({strict:true,allErrors:true}).compile(read('inspection-evidence.schema.json'));
const data=read('inspection-evidence-cases.json');
if(!validate(data))throw Error(JSON.stringify(validate.errors));
for(const bad of [{...data,clauses:112},{...data,optimization:{...data.optimization,phases:['early','early','late']}}]){
  if(validate(bad))throw Error('Invalid evidence coverage accepted');
}
console.log('[DEBUG] Evidence schema and two negative controls passed');
