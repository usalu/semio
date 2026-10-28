from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts')
s=p.read_text(encoding='utf-8');tail=s[s.index('const forwardPosition='):]
core=r'''/** 🔺️ Bounded, owned IFC graph transactions shared by commands, history and replay. */
import type {IfcSnapshot, IfcEntity, IfcValue} from "../📸️snapshot/🟦️.ts";
import {IFC4X3_DOCUMENT_SCHEMA, IFC4X3_SCHEMA_IDENTIFIER} from "../📸️snapshot/🟦️.ts";
import type {IfcDiff, IfcArgsDiff, IfcEntityDiff} from "../../../../../4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";
export type {IfcDiff, IfcArgsDiff, IfcEntityDiff};
export interface IfcEditProgress {phase:"validateBase"|"applyEntities"|"validateResult"|"admitInput"|"copyOwned"|"compare";completed:number;total:number}
export interface IfcApplyOptions {signal?:AbortSignal;onProgress?:(progress:IfcEditProgress)=>void;maxEntities?:number;maxValues?:number;maxDepth?:number;yieldEvery?:number}
export type IfcEditSteps<T>=Generator<IfcEditProgress,T>;
export interface IfcDiffTransaction {snapshot:IfcSnapshot;inverse:IfcDiff;diff:IfcDiff}
/** 🛂️ Refusals leave graph ownership and publication unchanged. */
export class IfcEditError extends Error {constructor(readonly code:string,message:string,readonly target:readonly(string|number)[]=[]){super(`${code}: ${message}`);this.name="IfcEditError";}}
const fail=(code:string,message:string):never=>{throw new IfcEditError(code,message);};
const identifier=(value:unknown):value is string=>typeof value==="string"&&/^[A-Z][A-Z0-9_]*$/.test(value);
const index=(value:number,length:number,insertion=false):void=>{if(!Number.isSafeInteger(value)||value<0||value>length||(!insertion&&value===length))fail("invalidIndex",`index ${value} is outside ${length} items`);};
const limit=(value:number|undefined,fallback:number):number=>{const n=value??fallback;if(!Number.isSafeInteger(n)||n<0)fail("invalidLimit","limits must be nonnegative safe integers");return n;};
export const checkIfcCancellation=(options:IfcApplyOptions):void=>{if(options.signal?.aborted)fail("cancelled","IFC edit was cancelled");};
const tick=(phase:IfcEditProgress["phase"],completed:number,total=completed):IfcEditProgress=>({phase,completed,total});
/** ▶️ Drives the same bounded authority without asynchronous scheduling. */
export function runIfcEdit<T>(steps:IfcEditSteps<T>,options:IfcApplyOptions={}):T {try{for(;;){checkIfcCancellation(options);const step=steps.next();if(step.done){checkIfcCancellation(options);return step.value;}options.onProgress?.(step.value);}}finally{steps.return(undefined as T);}}
/** ⏳️ Drives bounded steps with cancellation checks on both sides of every yield. */
export async function runIfcEditAsync<T>(steps:IfcEditSteps<T>,options:IfcApplyOptions={}):Promise<T>{const every=limit(options.yieldEvery,256);if(every===0)fail("invalidLimit","yieldEvery must be positive");let count=0;try{for(;;){checkIfcCancellation(options);const step=steps.next();if(step.done){checkIfcCancellation(options);return step.value;}options.onProgress?.(step.value);checkIfcCancellation(options);if(++count%every===0)await new Promise<void>(resolve=>setTimeout(resolve,0));}}finally{steps.return(undefined as T);}}

function* graphRoots(snapshot:IfcSnapshot):Generator<IfcValue> {
 for(const lane of[snapshot.header.fileDescription,snapshot.header.fileName,snapshot.header.fileSchema])for(let i=0;i<lane.length;i++)yield lane[i]!;
 for(let i=0;i<snapshot.entities.length;i++){const entity=snapshot.entities[i]!;for(let j=0;j<entity.args.length;j++)yield entity.args[j]!;for(let j=0;j<entity.complex.length;j++){const part=entity.complex[j]!;for(let k=0;k<part.args.length;k++)yield part.args[k]!;}}
}
function* values(roots:Iterator<IfcValue>,ids:ReadonlySet<bigint>|undefined,phase:IfcEditProgress["phase"],options:IfcApplyOptions):IfcEditSteps<void>{
 const maximum=limit(options.maxValues,10_000_000),depthLimit=limit(options.maxDepth,512),active=new Set<object>();let visited=0;
 const stack:{items:Iterator<IfcValue>;depth:number;owner?:object}[]=[{items:roots,depth:0}];
 while(stack.length){const frame=stack.at(-1)!,next=frame.items.next();if(next.done){stack.pop();if(frame.owner)active.delete(frame.owner);continue;}
  if(++visited>maximum)fail("valueLimit","IFC value limit exceeded");if(frame.depth>depthLimit)fail("depthLimit","IFC value nesting exceeds limit");const value=next.value;
  if(value===null||typeof value!=="object"||active.has(value))fail("invalidValue","invalid or cyclic IFC value");
  switch(value.kind){
   case"unset":case"derived":break;
   case"integer":if(typeof value.value!=="bigint"||value.value< -9223372036854775808n||value.value>9223372036854775807n)fail("invalidValue","integer must be an i64");break;
   case"real":if(typeof value.value?.bits!=="bigint"||value.value.bits<0n||value.value.bits>18446744073709551615n||(value.value.bits&0x7ff0000000000000n)===0x7ff0000000000000n)fail("invalidValue","STEP real must be finite binary64");break;
   case"string":if(typeof value.value!=="string")fail("invalidValue","string value is required");break;
   case"binary":if(typeof value.value!=="string"||!/^(?:0[0-9A-F]*|1[0-7][0-9A-F]*|2[0-3][0-9A-F]*|3[01][0-9A-F]*)$/.test(value.value))fail("invalidValue","binary content requires canonical zero-padded STEP bits");break;
   case"enum":if(!identifier(value.value))fail("invalidValue","enumeration must be an EXPRESS identifier");break;
   case"reference":if(typeof value.value!=="bigint"||value.value<=0n||value.value>18446744073709551615n)fail("invalidValue","reference must be a positive u64");if(ids&&!ids.has(value.value))fail("danglingReference",`reference #${value.value} has no target`);break;
   case"aggregate":case"typedValue":{if(value.kind==="typedValue"&&!identifier(value.value?.name))fail("invalidValue","typed value must have an EXPRESS identifier");const children=value.kind==="aggregate"?value.value:value.value.items;if(!Array.isArray(children))fail("invalidValue","aggregate items must be an array");if(children.length>maximum-visited)fail("valueLimit","IFC aggregate exceeds remaining value admission");active.add(value);stack.push({items:children.values(),depth:frame.depth+1,owner:value});break;}
   default:fail("invalidValue","unknown IFC value kind");
  }
  yield tick(phase,visited);
 }
}
/** 🔎️ Validates identities before lazily traversing every owned value root. */
export function* validateIfcGraphSteps(snapshot:IfcSnapshot,phase:IfcEditProgress["phase"],options:IfcApplyOptions):IfcEditSteps<void>{
 if(snapshot.schema!==IFC4X3_DOCUMENT_SCHEMA)fail("invalidSchema","unexpected IFC document identity");if(snapshot.entities.length>limit(options.maxEntities,1_000_000))fail("entityLimit","IFC entity limit exceeded");
 const schemas=snapshot.header.fileSchema;if(schemas.length!==1||schemas[0]?.kind!=="aggregate"||schemas[0].value.length!==1||schemas[0].value[0]?.kind!=="string"||schemas[0].value[0].value!==IFC4X3_SCHEMA_IDENTIFIER)fail("invalidSchema",`FILE_SCHEMA must identify ${IFC4X3_SCHEMA_IDENTIFIER}`);
 const ids=new Set<bigint>();for(let i=0;i<snapshot.entities.length;i++){const entity=snapshot.entities[i]!;if(typeof entity.id!=="bigint"||entity.id<=0n||entity.id>18446744073709551615n)fail("invalidIdentity","entity id must be a positive u64");if(ids.has(entity.id))fail("duplicateTarget",`entity #${entity.id} occurs twice`);if(!identifier(entity.name))fail("invalidName","entity type must be an EXPRESS identifier");ids.add(entity.id);for(let j=0;j<entity.complex.length;j++){if(!identifier(entity.complex[j]!.name))fail("invalidName","complex type must be an EXPRESS identifier");yield tick(phase,j+1,entity.complex.length);}yield tick(phase,i+1,snapshot.entities.length);}
 yield* values(graphRoots(snapshot),ids,phase,options);
}
function* diffRoots(diff:IfcDiff):Generator<IfcValue>{for(const lane of[diff.fileDescription,diff.fileName,diff.fileSchema])if(lane)for(let i=0;i<lane.length;i++)yield lane[i]!;for(const addition of diff.entities?.added??[]){for(const value of addition.entity.args)yield value;for(const part of addition.entity.complex)for(const value of part.args)yield value;}for(const edit of diff.entities?.modified??[]){for(const item of edit.diff.args?.modified??[])yield item.value;for(const item of edit.diff.args?.added??[])yield item.value;for(const part of edit.diff.complex??[])for(const value of part.args)yield value;}}
/** 🧾️ Admits proposed typed values before allocating their owned copies. */
export function* admitIfcDiffSteps(diff:IfcDiff,options:IfcApplyOptions):IfcEditSteps<void>{yield* values(diffRoots(diff),undefined,"admitInput",options);}
/** 📋️ Copies admitted data through lazy cursor frames; no eager array frontier or native clone. */
export function* copyIfcOwnedSteps<T>(value:T,options:IfcApplyOptions={}):IfcEditSteps<T>{
 let result:unknown;const active=new Set<object>();type Frame={source:object;target:Record<string,unknown>|unknown[];keys:Iterator<string>;depth:number};const stack:Frame[]=[];let copied=0;
 function* keys(source:object):Generator<string>{if(Array.isArray(source)){for(let i=0;i<source.length;i++)yield String(i);}else for(const key in source)if(Object.hasOwn(source,key))yield key;}
 const put=(source:unknown,set:(value:unknown)=>void,depth:number):void=>{if(source===null||typeof source!=="object"){set(source);return;}if(depth>limit(options.maxDepth,512)*4+32)fail("depthLimit","owned data nesting exceeds limit");if(active.has(source))fail("invalidValue","cyclic owned IFC data");const target=Array.isArray(source)?[]:{};set(target);active.add(source);stack.push({source,target,keys:keys(source),depth});};
 put(value,item=>{result=item;},0);while(stack.length){const frame=stack.at(-1)!,next=frame.keys.next();if(next.done){active.delete(frame.source);stack.pop();continue;}const key=next.value;put((frame.source as Record<string,unknown>)[key],item=>{(frame.target as Record<string,unknown>)[key]=item;},frame.depth+1);yield tick("copyOwned",++copied);}return result as T;
}
const clone=<T>(value:T):T=>runIfcEdit(copyIfcOwnedSteps(value));
function* uniqueSteps<T,K>(items:readonly T[],key:(item:T)=>K,label:string,maximum:number):IfcEditSteps<Map<K,T>>{if(items.length>maximum)fail("valueLimit",`${label} exceeds admission`);const map=new Map<K,T>();for(let i=0;i<items.length;i++){const item=items[i]!,id=key(item);if(map.has(id))fail("duplicateTarget",`${label} must be unique`);map.set(id,item);yield tick("applyEntities",i+1,items.length);}return map;}
function* applyArgsSteps(base:IfcValue[],diff:IfcArgsDiff,options:IfcApplyOptions):IfcEditSteps<{next:IfcValue[];inverse:IfcArgsDiff}>{
 const maximum=limit(options.maxValues,10_000_000),removed=yield* uniqueSteps(diff.removed??[],i=>i,"argument removals",maximum),modified=yield* uniqueSteps(diff.modified??[],i=>i.index,"argument modifications",maximum),added=yield* uniqueSteps(diff.added??[],i=>i.index,"argument insertions",maximum);
 for(const target of removed.keys()){index(target,base.length);if(modified.has(target))fail("overlappingTarget","removed arguments cannot also be modified");yield tick("applyEntities",target);}
 for(const target of modified.keys()){index(target,base.length);yield tick("applyEntities",target);}
 const length=base.length-removed.size+added.size;if(length>maximum)fail("valueLimit","result arguments exceed value admission");for(const target of added.keys()){index(target,length);yield tick("applyEntities",target);}
 const next:IfcValue[]=[],inverse:IfcArgsDiff={},undoRemoved:number[]=[],undoAdded:{index:number;value:IfcValue}[]=[],undoModified:{index:number;value:IfcValue}[]=[];let original=0;
 for(let final=0;final<length;final++){const insertion=added.get(final);if(insertion){next.push(insertion.value);undoRemoved.push(final);}else{while(removed.has(original)){undoAdded.push({index:original,value:base[original]!});original++;yield tick("applyEntities",original,base.length);}const replacement=modified.get(original);if(replacement)undoModified.push({index:final,value:base[original]!});next.push(replacement?.value??base[original]!);original++;}yield tick("applyEntities",final+1,length);}
 while(original<base.length){if(!removed.has(original))fail("invalidIndex","argument coordinate transport differs");undoAdded.push({index:original,value:base[original]!});original++;yield tick("applyEntities",original,base.length);}
 if(undoRemoved.length)inverse.removed=undoRemoved;if(undoAdded.length)inverse.added=undoAdded;if(undoModified.length)inverse.modified=undoModified;return{next,inverse};
}
function* applyEntitySteps(base:IfcEntity,diff:IfcEntityDiff,options:IfcApplyOptions):IfcEditSteps<{next:IfcEntity;inverse:IfcEntityDiff}>{const inverse:IfcEntityDiff={};if(diff.name!==undefined)inverse.name=base.name;if(diff.complex!==undefined)inverse.complex=base.complex;const args=diff.args?yield* applyArgsSteps(base.args,diff.args,options):undefined;if(args)inverse.args=args.inverse;return{next:{id:base.id,name:diff.name??base.name,args:args?.next??base.args,complex:diff.complex??base.complex},inverse};}
function applyEntity(base:IfcEntity,diff:IfcEntityDiff):IfcEntity{return runIfcEdit(applyEntitySteps(base,diff,{})).next;}
function* transactionSteps(base:IfcSnapshot,diff:IfcDiff,options:IfcApplyOptions):IfcEditSteps<IfcDiffTransaction>{
 yield* validateIfcGraphSteps(base,"validateBase",options);yield* admitIfcDiffSteps(diff,options);const changes=diff.entities,maximum=limit(options.maxEntities,1_000_000),removed=yield* uniqueSteps(changes?.removed??[],i=>i.id,"entity removals",maximum),modified=yield* uniqueSteps(changes?.modified??[],i=>i.id,"entity modifications",maximum),added=yield* uniqueSteps(changes?.added??[],i=>i.index,"entity insertion positions",maximum);yield* uniqueSteps(changes?.added??[],i=>i.entity.id,"inserted entity identities",maximum);
 const positions=new Map<bigint,number>();for(let i=0;i<base.entities.length;i++){positions.set(base.entities[i]!.id,i);yield tick("applyEntities",i+1,base.entities.length);}
 for(const item of removed.values()){if(!positions.has(item.id))fail("missingEntity",`entity #${item.id} does not exist`);if(positions.get(item.id)!==item.index)fail("invalidIndex","removal position must match original identity");yield tick("applyEntities",item.index);}
 for(const item of modified.values()){if(!positions.has(item.id))fail("missingEntity",`entity #${item.id} does not exist`);if(removed.has(item.id))fail("overlappingTarget","removed entities cannot also be modified");yield tick("applyEntities",positions.get(item.id)!);}
 const length=base.entities.length-removed.size+added.size;if(length>maximum)fail("entityLimit","result entity count exceeds admission");for(const item of added.values()){index(item.index,length);if(positions.has(item.entity.id)&&!removed.has(item.entity.id))fail("duplicateTarget",`entity #${item.entity.id} already exists`);yield tick("applyEntities",item.index,length);}
 const entities:IfcEntity[]=[],undoRemoved:{id:bigint;index:number}[]=[],undoAdded:{index:number;entity:IfcEntity}[]=[],undoModified:{id:bigint;diff:IfcEntityDiff}[]=[];let original=0;
 for(let final=0;final<length;final++){const insertion=added.get(final);if(insertion){entities.push(insertion.entity);undoRemoved.push({id:insertion.entity.id,index:final});}else{while(removed.has(base.entities[original]!.id)){undoAdded.push({index:original,entity:base.entities[original]!});original++;yield tick("applyEntities",original,base.entities.length);}const entity=base.entities[original++]!,edit=modified.get(entity.id);if(edit){const changed=yield* applyEntitySteps(entity,edit.diff,options);entities.push(changed.next);undoModified.push({id:entity.id,diff:changed.inverse});}else entities.push(entity);}yield tick("applyEntities",final+1,length);}
 while(original<base.entities.length){undoAdded.push({index:original,entity:base.entities[original]!});original++;yield tick("applyEntities",original,base.entities.length);}
 const inverse:IfcDiff={};if(diff.fileDescription!==undefined)inverse.fileDescription=base.header.fileDescription;if(diff.fileName!==undefined)inverse.fileName=base.header.fileName;if(diff.fileSchema!==undefined)inverse.fileSchema=base.header.fileSchema;if(changes)inverse.entities={...(undoRemoved.length?{removed:undoRemoved}:{}),...(undoAdded.length?{added:undoAdded}:{}),...(undoModified.length?{modified:undoModified}:{})};
 const next:IfcSnapshot={schema:base.schema,header:{fileDescription:diff.fileDescription??base.header.fileDescription,fileName:diff.fileName??base.header.fileName,fileSchema:diff.fileSchema??base.header.fileSchema},entities};yield* validateIfcGraphSteps(next,"validateResult",options);return yield* copyIfcOwnedSteps({snapshot:next,inverse,diff},options);
}
/** 🧮️ One validated transaction computes the next graph and original-coordinate inverse together. */
export function applyIfcDiffTransaction(base:IfcSnapshot,diff:IfcDiff,options:IfcApplyOptions={}):IfcDiffTransaction{return runIfcEdit(transactionSteps(base,diff,options),options);}
/** 🧵️ Cooperatively prepares one fully owned atomic transaction. */
export function applyIfcDiffTransactionAsync(base:IfcSnapshot,diff:IfcDiff,options:IfcApplyOptions={}):Promise<IfcDiffTransaction>{return runIfcEditAsync(transactionSteps(base,diff,options),options);}
/** ▶️ Applies a sparse diff and returns a completely owned mutable graph. */
export function applyIfcDiff(base:IfcSnapshot,diff:IfcDiff,options:IfcApplyOptions={}):IfcSnapshot{return applyIfcDiffTransaction(base,diff,options).snapshot;}
/** ⏳️ Applies through bounded validation, transport and copying steps. */
export async function applyIfcDiffAsync(base:IfcSnapshot,diff:IfcDiff,options:IfcApplyOptions={}):Promise<IfcSnapshot>{return(await applyIfcDiffTransactionAsync(base,diff,options)).snapshot;}
/** ↩️ Returns the original-coordinate inverse computed by the central transaction. */
export function inverseIfcDiff(base:IfcSnapshot,diff:IfcDiff,options:IfcApplyOptions={}):IfcDiff{return applyIfcDiffTransaction(base,diff,options).inverse;}
/** 🔙️ Cooperatively prepares an inverse with the same validation and cancellation authority. */
export async function inverseIfcDiffAsync(base:IfcSnapshot,diff:IfcDiff,options:IfcApplyOptions={}):Promise<IfcDiff>{return(await applyIfcDiffTransactionAsync(base,diff,options)).inverse;}
/** ⚖️ Lazily compares owned values without a wide array frontier. */
export function* equalIfcValueSteps(left:unknown,right:unknown):IfcEditSteps<boolean>{type Frame={a:Record<string,unknown>;b:Record<string,unknown>;keys:Iterator<string>};const stack:Frame[]=[];const active=new WeakMap<object,WeakSet<object>>();let count=0;function* keys(value:object):Generator<string>{if(Array.isArray(value)){for(let i=0;i<value.length;i++)yield String(i);}else for(const key in value)if(Object.hasOwn(value,key))yield key;}const enter=(a:unknown,b:unknown):boolean=>{if(Object.is(a,b))return true;if(a===null||b===null||typeof a!=="object"||typeof b!=="object"||Array.isArray(a)!==Array.isArray(b))return false;if(Array.isArray(a)&&a.length!==(b as unknown[]).length)return false;if(!Array.isArray(a)&&Object.keys(a).length!==Object.keys(b).length)return false;if(active.get(a)?.has(b))return true;const seen=active.get(a)??new WeakSet<object>();seen.add(b);active.set(a,seen);stack.push({a:a as Record<string,unknown>,b:b as Record<string,unknown>,keys:keys(a)});return true;};if(!enter(left,right))return false;while(stack.length){const frame=stack.at(-1)!,next=frame.keys.next();if(next.done){stack.pop();continue;}if(!Object.hasOwn(frame.b,next.value)||!enter(frame.a[next.value],frame.b[next.value]))return false;yield tick("compare",++count);}return true;}
/** ⚖️ Compares native bigint and binary64 words exactly. */
export function equalIfcValue(left:unknown,right:unknown):boolean{return runIfcEdit(equalIfcValueSteps(left,right));}
'''
p.write_text(core+'\n'+tail,encoding='utf-8',newline='\n')
t=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️tests/🧬️editing/🟦️.ts');t.write_text(t.read_text(encoding='utf-8').replace(r'/^\\d+$/',r'/^\d+$/'),encoding='utf-8',newline='\n')
