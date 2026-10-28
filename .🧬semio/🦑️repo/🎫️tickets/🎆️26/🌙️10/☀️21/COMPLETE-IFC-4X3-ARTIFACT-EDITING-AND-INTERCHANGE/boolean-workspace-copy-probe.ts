import{SolidBooleanEvaluation,type SolidMesh,type SolidSteps}from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🟦️.ts';
import law from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🧪️tests/owned-workspace-cases.json';
function drain<T>(steps:SolidSteps<T>):T{try{for(;;){const step=steps.next();if(step.done)return step.value;}}finally{steps.return(undefined as T);}}
const mesh=structuredClone(law.source)as SolidMesh;let changed=false,owner:SolidBooleanEvaluation;owner=new SolidBooleanEvaluation({tick:()=>{if(!changed&&owner.peakValues>500&&owner.usedValues===60){changed=true;(mesh.triangles[0]as number[]).push(7);}}});
let output:unknown,error:unknown,failed=false;try{output=drain(owner.booleanSteps('difference',mesh,{vertices:[],triangles:[]}));}catch(value){failed=true;error=String(value);}finally{owner.close();}process.stdout.write(JSON.stringify({changed,failed,error,output},null,2)+'\n');
