/** 🔎️ Inspect unchanged represented faceted diagnostics through actual public conversion. */
import {readFile} from 'node:fs/promises';
import {parseIfc4x3Step} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/step.ts';
import {extractIfc4x3Triangles} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/geometry.ts';
const source=await parseIfc4x3Step(await readFile(new URL('../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official/basin-faceted-brep.ifc',import.meta.url),'utf8'),{batchSize:4096});
const outcome=await extractIfc4x3Triangles(source,{batchSize:4096});
console.log(JSON.stringify(outcome,(_key,value)=>typeof value==='bigint'?value.toString():value,2));
