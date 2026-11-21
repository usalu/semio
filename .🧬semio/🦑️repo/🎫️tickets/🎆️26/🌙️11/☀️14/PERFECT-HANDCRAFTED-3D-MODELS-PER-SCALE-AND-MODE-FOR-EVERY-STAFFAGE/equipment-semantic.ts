/** 🧭 Check the live catalog contract, its environment and referenced alternatives one asset at a time. */
import {readFileSync} from 'node:fs';
import {validateAsset} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts';
import {loadEnvironments} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts';
import {referencedAssets} from './r3-validate.ts';
const {asset}=JSON.parse(readFileSync(process.argv[2]!,'utf8'));
const environments=new Set((await loadEnvironments()).presets.map(p=>p.id));
const issues=validateAsset(asset,{assets:referencedAssets(asset) as never,environments});
console.log(JSON.stringify({asset:asset.id,issues:issues.map(i=>[i.code,i.path,i.message.en])}));
