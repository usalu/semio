/** 🧪️ Current exporters produce physical observations through the real STEP writer interface. */
import {writeFile,mkdir} from 'node:fs/promises';
import {femConnectivityFixture} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/fem-isolated-fixtures.ts';
import {ifcBimAxisFixture} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/bim-axis-fixtures.ts';
import {fem3dToIfc4x3} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/fem.ts';
import {bimSpatialToIfc4x3} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/bim.ts';
import {writeIfc4x3Step} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/step-writer.ts';
const root=new URL('./🗑️generated/traversal-physical/',import.meta.url);await mkdir(root,{recursive:true});
const fem=femConnectivityFixture({nodes:['a','b'],members:[{start:'a',end:'b'}],supports:['a'],loads:['b']}),bim={...ifcBimAxisFixture(.3),storeys:{g:{name:'Ground',building:'building',level:0,height:3,cut_height:1.2},u:{name:'Upper',building:'building',level:1,height:3}}},rows=[];
for(const [name,outcome]of [['bim',await bimSpatialToIfc4x3(bim)],['fem',await fem3dToIfc4x3(fem)]]as const){if(!outcome.artifact)throw new Error('Export refused '+name);const step=await writeIfc4x3Step(outcome.artifact,{batchSize:4096});await writeFile(new URL(name+'.ifc',root),step);rows.push({name,entities:outcome.artifact.entities.length,fidelity:outcome.fidelity,diagnostics:outcome.diagnostics});}
await writeFile(new URL('input.json',root),JSON.stringify(rows,null,2));console.log('[DEBUG] '+JSON.stringify(rows.map(row=>({name:row.name,entities:row.entities}))));
