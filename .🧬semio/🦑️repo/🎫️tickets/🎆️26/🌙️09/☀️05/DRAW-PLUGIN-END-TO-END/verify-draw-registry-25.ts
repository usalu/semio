/** 🛂️ Checks the real Draw row against the published descriptor and component receipt. */
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { validateDescriptors } from '../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🟦️.ts';
const root=process.cwd(), rows=JSON.parse(readFileSync(resolve(root,'✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json'),'utf8'));
const draw=rows.filter((row:{pluginId:string})=>row.pluginId==='draw');
if(draw.length!==1)throw Error('Exactly one Draw registry row required');
const result=validateDescriptors(draw,root);
console.log(JSON.stringify({draw:draw[0],...result}));
if(result.errors.length)process.exitCode=1;
