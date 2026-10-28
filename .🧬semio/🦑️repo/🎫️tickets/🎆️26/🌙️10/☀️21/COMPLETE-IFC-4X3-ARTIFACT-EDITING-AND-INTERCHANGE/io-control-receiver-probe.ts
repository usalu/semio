/** 🕵️ Read-only genuine STEP ingress probe for original callback receiver and capture reads. */
import {parseIfc4x3Step} from "C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/step.ts";
import {captureIfcIoControl} from "C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/control.ts";
import {writeFile} from "node:fs/promises";
const original=new AbortController(),replacement=new AbortController();let receiverKeys:string[]=[],replacementCalls=0,calls=0,finished=false,error:unknown;
const source={signal:original.signal,batchSize:1,progress:function(this:any,_event:any){calls++;receiverKeys=Object.keys(this??{});Reflect.set(this,"signal",replacement.signal);Reflect.set(this,"progress",()=>replacementCalls++);original.abort();}};
try{await parseIfc4x3Step(await Bun.file("C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official/extruded-solid.ifc").text(),source);finished=true;}catch(value){error=value;}
let progressReads=0;const first=()=>{},third=()=>{};const getter=Object.defineProperty({},"progress",{get(){progressReads++;return progressReads===1?first:third;}});const captured=captureIfcIoControl(getter);
const result={finished,calls,replacementCalls,receiverKeys,error:error instanceof Error?{name:error.name,message:error.message}:error,originalAborted:original.signal.aborted,progressReads,capturedFirst:captured.progress===first,capturedLater:captured.progress===third};
await writeFile(new URL("./🗑️generated/io-control-receiver-before.json",import.meta.url),JSON.stringify(result,null,2));
console.log("[DEBUG] "+JSON.stringify(result));
