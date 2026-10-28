import Ajv from 'ajv/dist/2020';
import {readFileSync} from 'node:fs';
import {join} from 'node:path';
const read=(name:string)=>JSON.parse(readFileSync(join(import.meta.dir,name),'utf8').replace(/^\uFEFF/,''));
const check=new Ajv({strict:true}).compile(read('tfs-admission.schema.json'));
const data=read('tfs-admission-cases.json');
if(!check(data))throw new Error(JSON.stringify(check.errors));
console.log(JSON.stringify({cases:data.cases.length,valid:true}));
