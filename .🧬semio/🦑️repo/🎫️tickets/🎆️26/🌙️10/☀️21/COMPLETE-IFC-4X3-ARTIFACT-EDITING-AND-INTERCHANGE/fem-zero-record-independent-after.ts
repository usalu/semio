/** 🧪️ Read-only actual endpoint observations distinguish authored zero nodal records from absent loads. */
import {mkdir,writeFile,readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {femConnectivityFixture} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/fem-isolated-fixtures.ts';
import {fem3dToIfc4x3,ifc4x3ToFem3d} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/fem.ts';
import {ifcReal,parseIfc4x3Step} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/step.ts';
import {writeIfc4x3Step} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/step-writer.ts';
import {ifcMutationDiffAsync,applyIfcMutationAsync} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import {inverseIfcDiffAsync,applyIfcDiffAsync} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts';
import {isDeepStrictEqual} from 'node:util';
const root=new URL('./🗑️generated/fem-zero-record-after/',import.meta.url);await mkdir(root,{recursive:true});
const word=(n:number)=>{const value=ifcReal(n);if(value.kind!=='real')throw Error('real');return value.value;},rows=[];
for(const [id,value]of [['positive-zero',0],['negative-zero',-0],['nonzero',3]]as const){
 const source=femConnectivityFixture({nodes:['a','b'],members:[{start:'a',end:'b'}],supports:['a'],loads:['b']});source.loadCases[0]!.loads[0]={kind:'nodal',id:'explicit-load',nodeId:'b',dof:'Tz',value:word(value)};
 const graph=(await fem3dToIfc4x3(source)).artifact;if(!graph)throw Error('No export');
 const step=await writeIfc4x3Step(graph),parsed=await parseIfc4x3Step(step),imported=await ifc4x3ToFem3d(parsed),force=graph.entities.find(row=>row.name==='IFCSTRUCTURALLOADSINGLEFORCE')!,mutation={mutation:'setEntityArg' as const,id:force.id,index:3,value:ifcReal(5)},diff=await ifcMutationDiffAsync(graph,mutation),edited=await applyIfcMutationAsync(graph,mutation),editedProjection=await ifc4x3ToFem3d(edited),restored=await applyIfcDiffAsync(edited,await inverseIfcDiffAsync(graph,diff));
 if(id==='positive-zero')await writeFile(new URL('zero.ifc',root),step);
 rows.push({id,inputValue:source.loadCases[0]!.loads[0]!.value,sourceLoads:source.loadCases[0]!.loads.length,actualForceArgs:force.args,importedLoads:imported.artifact?.loadCases[0]?.loads,importDiagnostics:imported.diagnostics,editedLoads:editedProjection.artifact?.loadCases[0]?.loads,inverseGraphExact:isDeepStrictEqual(restored,graph),inverseProjectionExact:isDeepStrictEqual(await ifc4x3ToFem3d(restored),await ifc4x3ToFem3d(graph))});
}
const sourceHashes=[];for(const path of ['C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/fem.ts','C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/geometry.ts','C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/control.ts'])sourceHashes.push({path,sha256:createHash('sha256').update(await readFile(path)).digest('hex')});
await writeFile(new URL('after.json',root),JSON.stringify({sourceHashes,rows},(_,value)=>typeof value==='bigint'?value.toString():value,2),{flag:'wx'});console.log('[DEBUG] '+JSON.stringify(rows.map(row=>({id:row.id,source:row.sourceLoads,imported:row.importedLoads?.length,edited:row.editedLoads?.length,inverseGraphExact:row.inverseGraphExact,inverseProjectionExact:row.inverseProjectionExact}))));

