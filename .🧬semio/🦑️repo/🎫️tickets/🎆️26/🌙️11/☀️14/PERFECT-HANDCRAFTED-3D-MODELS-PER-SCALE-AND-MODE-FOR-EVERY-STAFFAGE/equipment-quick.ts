/** 📏 Check authored solid ownership, slender geometry and physical feature declarations without model export. */
import {readFileSync} from 'node:fs';
import {checkVolumes} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts';
const {asset}=JSON.parse(readFileSync(process.argv[2]!,'utf8'));
const rules=['volumes-availability','volumes-bones','volumes-feature','volumes-copied','volumes-slender','volumes-solids'] as const;
const issues=checkVolumes(asset,{rules});
console.log(JSON.stringify({asset:asset.id,rules,issues:issues.map(f=>[f.scale,f.path,f.message.en])}));
