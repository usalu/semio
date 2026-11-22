/** 🔬 Read semantic findings before the independent presentation and per-scale geometry passes. */
import {readFileSync} from 'node:fs';
import {validateAsset} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts';
const asset=JSON.parse(readFileSync(process.argv[2]!,'utf8')).asset;
for(const i of validateAsset(asset)) console.log(i.code,i.path,i.message.en);
