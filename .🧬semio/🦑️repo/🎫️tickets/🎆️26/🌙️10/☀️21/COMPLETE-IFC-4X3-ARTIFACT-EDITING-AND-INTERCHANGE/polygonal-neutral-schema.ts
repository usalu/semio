import Ajv from 'ajv/dist/2020.js';
import {readFileSync} from 'node:fs';
const p=import.meta.dir;const ajv=new Ajv({strict:true,allErrors:true});const schema=JSON.parse(readFileSync(p+'/polygonal-neutral.schema.json','utf8').replace(/^\uFEFF/,''));const validate=ajv.compile(schema);for(const name of ['polygonal-neutral-cases.json','polygonal-identity-cases.json']){const data=JSON.parse(readFileSync(p+'/'+name,'utf8').replace(/^\uFEFF/,''));if(!validate(data))throw Error(JSON.stringify(validate.errors));console.log('[DEBUG] '+name+' '+data.cases.length+' valid');}
