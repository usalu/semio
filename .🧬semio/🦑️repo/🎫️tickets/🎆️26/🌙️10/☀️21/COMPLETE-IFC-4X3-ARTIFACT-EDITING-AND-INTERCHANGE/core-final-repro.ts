import {applyIfcDiffAsync} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts';
import {IfcEditHistory} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
const graph=()=>({schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[{kind:'aggregate',value:[{kind:'string',value:'IFC4X3_ADD2'}]}]},entities:[{id:1n,name:'IFCTEST',args:[{kind:'integer',value:1n}]}]});
for(const phase of ['copyOwned','validateBase','admitInput','applyEntities','validateResult']){
 const base=graph();const diff:any={entities:{modified:[{id:1n,diff:{args:{modified:[{index:0,value:{kind:'integer',value:2n}}]}}}]}};let changed=false;
 try{const after=await applyIfcDiffAsync(base as any,diff,{yieldEvery:1,onProgress(p){if(p.phase===phase&&!changed){changed=true;diff.entities.modified[0].diff.args.modified[0].value.value='invalid';(base.entities[0].args[0] as any).value='invalid';}}});if(typeof (after.entities[0].args[0] as any).value!=='bigint')throw Error('unsafe result');console.log('[DEBUG] phase safe',phase);}catch(error){console.log('[DEBUG] phase refused',phase,String(error));}
}
const history=await IfcEditHistory.create(graph() as any);
try{const event=await history.executeAsync({mutation:'setEntityName',id:1n} as any,0);console.log('[DEBUG] missing required command name accepted',{revision:history.revision,event,unchanged:history.snapshot.entities[0].name});}catch(error){console.log('[DEBUG] missing command name refused',String(error));}
try{await applyIfcDiffAsync(graph() as any,{fileDescription:null} as any);console.log('[DEBUG] null array lane accepted');}catch(error){console.log('[DEBUG] null array lane refused',String(error));}
