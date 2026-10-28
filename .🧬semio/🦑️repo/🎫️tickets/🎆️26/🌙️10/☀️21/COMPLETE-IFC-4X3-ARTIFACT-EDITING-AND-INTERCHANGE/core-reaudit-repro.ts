import {applyIfcDiffAsync,absorbIfcDiffAsync} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts';
import {IfcEditHistory} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
const graph=()=>({schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[{kind:'aggregate',value:[{kind:'string',value:'IFC4X3_ADD2'}]}]},entities:[{id:1n,name:'IFCTEST',args:[{kind:'integer',value:1n}],complex:[]}]});
const base=graph();let changed=false;
const result=await applyIfcDiffAsync(base as any,{}, {yieldEvery:1,onProgress(p){if(p.phase==='copyOwned'&&!changed){changed=true;(base.entities[0].args[0] as any).value='invalid';}}});
console.log('[DEBUG] validated result admits post-validation input mutation',result.entities[0].args[0]);
const history=await IfcEditHistory.create(graph() as any);const value:any={kind:'integer',value:2n};let mutationChanged=false;
const event=await history.executeAsync({mutation:'setEntityArg',id:1n,index:0,value},0,{yieldEvery:1,onProgress(p){if(p.phase==='copyOwned'&&!mutationChanged){mutationChanged=true;value.value='invalid';}}});
try{console.log('[DEBUG] published history value',history.snapshot.entities[0].args[0]);}catch(error){console.log('[DEBUG] published history becomes unreadable',{revision:history.revision,error:String(error),event});}
const malformed=await absorbIfcDiffAsync({entities:{modified:[{id:1n,diff:{args:{removed:[NaN]}}}]}} as any,{});
console.log('[DEBUG] absorption admits invalid position',malformed);
