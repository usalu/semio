/** 🧪️ Actual optional-force STEP outputs feed a separate owned physical oracle. */
import {mkdir,writeFile} from 'node:fs/promises';
import cases from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/fem/nodal/cases.json';
import {femConnectivityFixture} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/fem-isolated-fixtures.ts';
import {fem3dToIfc4x3} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/fem.ts';
import {writeIfc4x3Step} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/step-writer.ts';
const root=new URL('./🗑️generated/fem-nodal-physical/',import.meta.url);await mkdir(root,{recursive:true});const rows=[];
for(const dof of cases.dofs)for(const value of cases.values){const source=femConnectivityFixture({nodes:['a','b'],members:[{start:'a',end:'b'}],supports:['a'],loads:['b']}),load=source.loadCases[0]!.loads[0]!;if(load.kind!=='nodal')throw Error('Nodal');load.dof=dof as typeof load.dof;load.value={bits:BigInt(value.bits)};const result=await fem3dToIfc4x3(source);if(!result.artifact)throw Error('Export refused');const id=dof+'-'+value.id;await writeFile(new URL(id+'.ifc',root),await writeIfc4x3Step(result.artifact));rows.push({id,dof,bits:value.bits,node:'b',loadId:load.id});}
await writeFile(new URL('input.json',root),JSON.stringify(rows,null,2));console.log('[DEBUG] optional component files '+rows.length);

