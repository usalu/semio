/** 🧭 Observe actual intent flattening; this is not a staging authorization implementation. */
import {writeFile}from'node:fs/promises';
const {uiIntentToActionDescriptor}=await import('file:///C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx');
const original={surface:'actual-form-surface',revision:12,node:8,nodeKey:'declared-field',trigger:'change',action:{scope:'framework',name:'stageForm',version:1},args:{token:{action:'applyIfcTemplate',generation:'4',revision:'rev:1',window:'real-window'},path:['entries','0','value']},input:'draft',seq:1n};
const normal=uiIntentToActionDescriptor(original),forged=uiIntentToActionDescriptor({...original,input:{path:['identities','0','id'],token:{action:'other',generation:'0',revision:'old',window:'other'},value:'999'}});
const result={producer:'actual uiIntentToActionDescriptor',mountedForm:false,originalFields:Object.keys(original),normal,forged,descriptorPreservesOrigin:['surface','revision','node','nodeKey','trigger','seq'].filter(key=>Object.hasOwn(normal,key))};
await writeFile(new URL('./🗑️generated/staged-intent-transport-probe.json',import.meta.url),JSON.stringify(result,null,2));console.log('[DEBUG] '+JSON.stringify(result));
