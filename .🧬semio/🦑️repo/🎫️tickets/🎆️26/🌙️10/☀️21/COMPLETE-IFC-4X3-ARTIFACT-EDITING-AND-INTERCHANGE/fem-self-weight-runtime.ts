/** 🏋️ Preserve actual exported bytes for the owned external physical oracle. */
import {mkdir,writeFile} from 'node:fs/promises';
import {fileURLToPath} from 'node:url';
const repo=new URL('../../../../../../../',import.meta.url),base=new URL('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/',repo);
const {femConnectivityFixture}=await import(fileURLToPath(new URL('🧪️tests/fem-isolated-fixtures.ts',base))),{fem3dToIfc4x3,ifc4x3ToFem3d}=await import(fileURLToPath(new URL('fem.ts',base))),{writeIfc4x3Step}=await import(fileURLToPath(new URL('step-writer.ts',base))),{ifcReal}=await import(fileURLToPath(new URL('step.ts',base)));
const root=new URL('./🗑️generated/fem-self-weight/',import.meta.url);await mkdir(root,{recursive:true});const rows=[];
for(const id of ['false','true','only','mixed','zero','negative-zero','typed','factor']){
 const model=femConnectivityFixture({nodes:['a','b'],members:[{start:'a',end:'b'}],supports:['a'],loads:['b']});model.loadCases[0].selfWeight=!['false','zero','negative-zero'].includes(id);if(id==='only')model.loadCases[0].loads=[];if(id==='mixed')model.loadCases.push({...model.loadCases[0],id:'live',name:'Live',selfWeight:false,loads:[]});
 const result=await fem3dToIfc4x3(model);if(!result.artifact)throw Error('Export refused '+id);const graph=result.artifact,owner=graph.entities.find((row:{name:string})=>row.name==='IFCSTRUCTURALLOADCASE');
 if(['zero','negative-zero','typed'].includes(id))owner.args[10]={kind:'aggregate',value:[0,0,id==='typed'?-1:0].map(n=>id==='typed'?{kind:'typedValue',value:{name:'IFCRATIOMEASURE',items:[ifcReal(n)]}}:ifcReal(id==='negative-zero'?-0:n))};if(id==='factor')owner.args[8]=ifcReal(2);
 const projected=await ifc4x3ToFem3d(graph),text=await writeIfc4x3Step(graph);await writeFile(new URL(id+'.ifc',root),text);rows.push({id,selfWeights:model.loadCases.map((row:{selfWeight:boolean})=>row.selfWeight),actionCount:model.loadCases.reduce((sum:number,row:{loads:unknown[]})=>sum+row.loads.length,0),projected:projected.artifact?.loadCases.map((row:{selfWeight:boolean})=>row.selfWeight)??null,diagnostics:projected.diagnostics,coefficient:id==='factor'?2:1});
}
await writeFile(new URL('input.json',root),JSON.stringify(rows,null,2));console.log('[DEBUG] self-weight physical inputs '+rows.length);
