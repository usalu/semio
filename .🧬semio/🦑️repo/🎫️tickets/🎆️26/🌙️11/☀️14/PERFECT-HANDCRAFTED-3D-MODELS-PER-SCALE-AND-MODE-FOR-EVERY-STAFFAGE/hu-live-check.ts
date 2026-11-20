/** 📋 Checks every current human document against the public schema and the catalog validator. */
import Ajv from 'ajv';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { STAFFAGE, catalogEntries, loadAsset } from './volumes-lib.ts';
import { validateAsset } from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts';
import { loadEnvironments } from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts';
const schema=JSON.parse(readFileSync(join(STAFFAGE,'🧬️schema/🔣️.json'),'utf8'));
const ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(schema);
const check=ajv.compile({$ref:schema.$id+'#/$defs/AssetDocument'});
const entries=catalogEntries();
const assets=new Map(entries.map(e=>{const a=loadAsset(e.path);return [a.id,a]}));
const environments=new Set((await loadEnvironments()).presets.map(p=>p.id));
for(const e of entries.filter(e=>e.category==='humans')){
 const d=JSON.parse(readFileSync(e.path,'utf8'));
 const schemaValid=check(d);
 const issues=validateAsset(d.asset,{assets,environments});
 console.log(d.asset.id,schemaValid?'schema OK':'schema '+check.errors?.length,issues.length===0?'references OK':issues.map(i=>i.code+' '+i.path).join(', '));
}
