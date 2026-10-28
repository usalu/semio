import {base,application} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️tests/template-fixtures.ts';
import {applyIfcMutationAsync} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
for(const kind of ['nonEnumerableAccessor','arrayUnknown','arrayAccessor','symbolUnknown']){
const app=application();let invoked=false;
if(kind==='nonEnumerableAccessor')Object.defineProperty(app,'description',{get(){invoked=true;return 'hidden';}});
if(kind==='arrayUnknown')(app.entries as any).unknown=true;
if(kind==='arrayAccessor')Object.defineProperty(app.entries,'unknown',{enumerable:true,get(){invoked=true;return true;}});
if(kind==='symbolUnknown')(app as any)[Symbol('unknown')]=true;
try {const next=await applyIfcMutationAsync(base(),{mutation:'applyTemplate',application:app});console.log(kind,'ACCEPTED',next.entities.length,'invoked',invoked);}catch(e){console.log(kind,'REFUSED',String(e));}
}
