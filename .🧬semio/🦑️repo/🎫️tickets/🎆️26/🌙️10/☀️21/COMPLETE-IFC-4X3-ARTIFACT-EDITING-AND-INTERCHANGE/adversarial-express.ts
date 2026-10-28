import input from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/models.json';
import {parseIfc4x3Step} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/step.ts';
import {validateIfc4x3Express} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/index.ts';
const source=input.header+input.body.join('\n')+input.footer;
await Bun.write(new URL('./🗑️generated/adversarial.ifc',import.meta.url),source);
const result=await validateIfc4x3Express(await parseIfc4x3Step(source));
await Bun.write(new URL('./🗑️generated/express-adversarial.json',import.meta.url),JSON.stringify(result,(_,v)=>typeof v==='bigint'?String(v):v,2));console.log(result.complete,result.diagnostics);
