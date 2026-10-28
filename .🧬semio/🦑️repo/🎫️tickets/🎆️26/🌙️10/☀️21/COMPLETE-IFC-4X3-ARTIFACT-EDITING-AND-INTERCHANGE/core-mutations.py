from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts');s=p.read_text(encoding='utf-8');s=s.replace('function* transactionSteps(base:IfcSnapshot,diff:IfcDiff,options:IfcApplyOptions):IfcEditSteps<IfcDiffTransaction>{\n yield* validateIfcGraphSteps(base,"validateBase",options);yield* admitIfcDiffSteps(diff,options);','export function* prepareIfcDiffSteps(base:IfcSnapshot,plan:()=>IfcEditSteps<IfcDiff>,options:IfcApplyOptions):IfcEditSteps<IfcDiffTransaction>{\n yield* validateIfcGraphSteps(base,"validateBase",options);const diff=yield* plan();yield* admitIfcDiffSteps(diff,options);');s=s.replace('transactionSteps(base,diff,options)','prepareIfcDiffSteps(base,function*(){return diff;},options)');p.write_text(s,encoding='utf-8',newline='\n')
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts')
p.write_text(r'''/** 🧬️ Sparse commands and revision-checked events share one bounded transaction authority. */
import type {IfcSnapshot,IfcValue,IfcEntity} from "../📸️snapshot/🟦️.ts";
import type {IfcMutation} from "../../../../../4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts";
import {applyIfcDiff,applyIfcDiffAsync,applyIfcDiffTransaction,applyIfcDiffTransactionAsync,prepareIfcDiffSteps,validateIfcGraphSteps,admitIfcDiffSteps,copyIfcOwnedSteps,equalIfcValueSteps,runIfcEdit,runIfcEditAsync,checkIfcCancellation,IfcEditError,type IfcDiff,type IfcEntityDiff,type IfcApplyOptions,type IfcEditSteps,type IfcDiffTransaction} from "../🔺️diff/🟦️.ts";
export type Ifc4x3Mutation=IfcMutation|{mutation:"setEntityValue";id:bigint;index:number;path:readonly number[];value:IfcValue};
const refusal=(code:string,message:string):never=>{throw new IfcEditError(code,message);};
const position=(index:number,length:number,insertion=false):void=>{if(!Number.isSafeInteger(index)||index<0||index>length||(!insertion&&index===length))refusal("invalidIndex",`index ${index} is outside ${length} items`);};
function* findEntity(snapshot:IfcSnapshot,id:bigint):IfcEditSteps<{entity:IfcEntity;ordinal:number}>{for(let i=0;i<snapshot.entities.length;i++){if(snapshot.entities[i]!.id===id)return{entity:snapshot.entities[i]!,ordinal:i};yield{phase:"admitInput",completed:i+1,total:snapshot.entities.length};}return refusal("missingEntity",`entity #${id} does not exist`);}
function* replaceValue(root:IfcValue,path:readonly number[],replacement:IfcValue,options:IfcApplyOptions):IfcEditSteps<IfcValue>{
 if(path.length>(options.maxDepth??512))refusal("depthLimit","nested edit exceeds the value depth limit");const parents:{value:IfcValue;items:IfcValue[];index:number}[]=[];let value=root;
 for(let i=0;i<path.length;i++){const items=value.kind==="aggregate"?value.value:value.kind==="typedValue"?value.value.items:refusal("invalidPath","nested path traverses a scalar");position(path[i]!,items.length);parents.push({value,items,index:path[i]!});value=items[path[i]!]!;yield{phase:"admitInput",completed:i+1,total:path.length};}
 value=replacement;for(let i=parents.length-1;i>=0;i--){const parent=parents[i]!,items:IfcValue[]=[];for(let j=0;j<parent.items.length;j++){items.push(j===parent.index?value:parent.items[j]!);yield{phase:"admitInput",completed:j+1,total:parent.items.length};}value=parent.value.kind==="aggregate"?{kind:"aggregate",value:items}:{kind:"typedValue",value:{name:parent.value.kind==="typedValue"?parent.value.value.name:"",items}};}return value;
}
function* mutationPlan(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions):IfcEditSteps<IfcDiff>{
 const field=(id:bigint,diff:IfcEntityDiff):IfcDiff=>({entities:{modified:[{id,diff}]}});let diff:IfcDiff,before:unknown,after:unknown;const target="id"in mutation?yield* findEntity(snapshot,mutation.id):undefined;
 switch(mutation.mutation){
  case"setFileDescription":diff={fileDescription:mutation.values};before=snapshot.header.fileDescription;after=mutation.values;break;
  case"setFileName":diff={fileName:mutation.values};before=snapshot.header.fileName;after=mutation.values;break;
  case"setFileSchema":diff={fileSchema:mutation.values};before=snapshot.header.fileSchema;after=mutation.values;break;
  case"insertEntity":position(mutation.index,snapshot.entities.length,true);diff={entities:{added:[{index:mutation.index,entity:mutation.entity}]}};break;
  case"removeEntity":diff={entities:{removed:[{id:mutation.id,index:target!.ordinal}]}};break;
  case"setEntityName":diff=field(mutation.id,{name:mutation.name});before=target!.entity.name;after=mutation.name;break;
  case"setEntityComplex":diff=field(mutation.id,{complex:mutation.complex});before=target!.entity.complex;after=mutation.complex;break;
  case"setEntityArg":case"setEntityValue":{position(mutation.index,target!.entity.args.length);yield* admitIfcDiffSteps(field(mutation.id,{args:{modified:[{index:mutation.index,value:mutation.value}]}}),options);before=target!.entity.args[mutation.index]!;after=mutation.mutation==="setEntityValue"?yield* replaceValue(before as IfcValue,mutation.path,mutation.value,options):mutation.value;diff=field(mutation.id,{args:{modified:[{index:mutation.index,value:after as IfcValue}]}});break;}
  case"insertEntityArg":position(mutation.index,target!.entity.args.length,true);diff=field(mutation.id,{args:{added:[{index:mutation.index,value:mutation.value}]}});break;
  case"removeEntityArg":position(mutation.index,target!.entity.args.length);diff=field(mutation.id,{args:{removed:[mutation.index]}});break;
  default:return refusal("unknownMutation","unknown IFC mutation");
 }
 yield* admitIfcDiffSteps(diff,options);if(before!==undefined&&(yield* equalIfcValueSteps(before,after)))return{};return diff;
}
function* mutationDiffSteps(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions):IfcEditSteps<IfcDiff>{yield* validateIfcGraphSteps(snapshot,"validateBase",options);const diff=yield* mutationPlan(snapshot,mutation,options);return yield* copyIfcOwnedSteps(diff,options);}
/** 📐️ Plans an owned sparse diff directly from the command target. */
export function ifcMutationDiff(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions={}):IfcDiff{return runIfcEdit(mutationDiffSteps(snapshot,mutation,options),options);}
/** 📝️ Cooperatively plans and copies an admitted command. */
export function ifcMutationDiffAsync(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions={}):Promise<IfcDiff>{return runIfcEditAsync(mutationDiffSteps(snapshot,mutation,options),options);}
function commandSteps(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions):IfcEditSteps<IfcDiffTransaction>{return prepareIfcDiffSteps(snapshot,()=>mutationPlan(snapshot,mutation,options),options);}
/** ▶️ Executes one central graph transaction. */
export function applyIfcMutation(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions={}):IfcSnapshot{return runIfcEdit(commandSteps(snapshot,mutation,options),options).snapshot;}
/** 🧵️ Cooperatively executes one transaction without intermediate publication. */
export async function applyIfcMutationAsync(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions={}):Promise<IfcSnapshot>{return(await runIfcEditAsync(commandSteps(snapshot,mutation,options),options)).snapshot;}
function* inverseMutationSteps(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions):IfcEditSteps<Ifc4x3Mutation>{yield* validateIfcGraphSteps(snapshot,"validateBase",options);yield* mutationPlan(snapshot,mutation,options);const target="id"in mutation?yield* findEntity(snapshot,mutation.id):undefined;let result:Ifc4x3Mutation;
 switch(mutation.mutation){
  case"setFileDescription":result={mutation:mutation.mutation,values:snapshot.header.fileDescription};break;
  case"setFileName":result={mutation:mutation.mutation,values:snapshot.header.fileName};break;
  case"setFileSchema":result={mutation:mutation.mutation,values:snapshot.header.fileSchema};break;
  case"insertEntity":result={mutation:"removeEntity",id:mutation.entity.id};break;
  case"removeEntity":result={mutation:"insertEntity",index:target!.ordinal,entity:target!.entity};break;
  case"setEntityName":result={mutation:mutation.mutation,id:mutation.id,name:target!.entity.name};break;
  case"setEntityComplex":result={mutation:mutation.mutation,id:mutation.id,complex:target!.entity.complex};break;
  case"setEntityArg":case"setEntityValue":case"removeEntityArg":result={mutation:mutation.mutation==="removeEntityArg"?"insertEntityArg":"setEntityArg",id:mutation.id,index:mutation.index,value:target!.entity.args[mutation.index]!};break;
  case"insertEntityArg":result={mutation:"removeEntityArg",id:mutation.id,index:mutation.index};break;
  default:return refusal("unknownMutation","unknown IFC mutation");
 }return yield* copyIfcOwnedSteps(result,options);
}
/** 🔁️ Restores the original target through an admitted inverse command. */
export function inverseIfcMutation(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions={}):Ifc4x3Mutation{return runIfcEdit(inverseMutationSteps(snapshot,mutation,options),options);}
/** 🔙️ Cooperatively prepares an owned inverse command. */
export function inverseIfcMutationAsync(snapshot:IfcSnapshot,mutation:Ifc4x3Mutation,options:IfcApplyOptions={}):Promise<Ifc4x3Mutation>{return runIfcEditAsync(inverseMutationSteps(snapshot,mutation,options),options);}
export interface IfcEditEvent {revision:number;kind:"mutation"|"undo"|"redo";scope:"persistedLocal"|"persistedShared";diff:IfcDiff;inverse:IfcDiff;mutation?:Ifc4x3Mutation}
interface IfcHistoryEntry {diff:IfcDiff;inverse:IfcDiff}
/** ⏪️ Private owned state changes only after complete preparation and a final revision check. */
export class IfcEditHistory {
 #snapshot:IfcSnapshot;#events:IfcEditEvent[]=[];#undo:IfcHistoryEntry[]=[];#redo:IfcHistoryEntry[]=[];
 private constructor(snapshot:IfcSnapshot,readonly scope:IfcEditEvent["scope"]){this.#snapshot=snapshot;}
 static createSync(snapshot:IfcSnapshot,scope:IfcEditEvent["scope"]="persistedLocal",options:IfcApplyOptions={}):IfcEditHistory{return new IfcEditHistory(applyIfcDiff(snapshot,{},options),scope);}
 static async create(snapshot:IfcSnapshot,scope:IfcEditEvent["scope"]="persistedLocal",options:IfcApplyOptions={}):Promise<IfcEditHistory>{return new IfcEditHistory(await applyIfcDiffAsync(snapshot,{},options),scope);}
 get snapshot():IfcSnapshot{return this.readSnapshotSync();}
 get revision():number{return this.#events.length;}
 get events():readonly IfcEditEvent[]{return this.readEventsSync();}
 readSnapshotSync(options:IfcApplyOptions={}):IfcSnapshot{return runIfcEdit(copyIfcOwnedSteps(this.#snapshot,options),options);}
 readSnapshot(options:IfcApplyOptions={}):Promise<IfcSnapshot>{return runIfcEditAsync(copyIfcOwnedSteps(this.#snapshot,options),options);}
 readEventsSync(options:IfcApplyOptions={}):IfcEditEvent[]{return runIfcEdit(copyIfcOwnedSteps(this.#events,options),options);}
 readEvents(options:IfcApplyOptions={}):Promise<IfcEditEvent[]>{return runIfcEditAsync(copyIfcOwnedSteps(this.#events,options),options);}
 #check(revision:number):void{if(revision!==this.revision)refusal("staleRevision",`expected revision ${this.revision}, received ${revision}`);}
 *#prepare(kind:IfcEditEvent["kind"],revision:number,options:IfcApplyOptions,mutation?:Ifc4x3Mutation):IfcEditSteps<{transaction:IfcDiffTransaction;event:IfcEditEvent;returned:IfcEditEvent;entry:IfcHistoryEntry}|undefined>{
  this.#check(revision);const previous=kind==="undo"?this.#undo.at(-1):kind==="redo"?this.#redo.at(-1):undefined;if(kind!=="mutation"&&!previous)return undefined;const transaction=kind==="mutation"?yield* commandSteps(this.#snapshot,mutation!,options):yield* prepareIfcDiffSteps(this.#snapshot,function*(){return kind==="undo"?previous!.inverse:previous!.diff;},options);if(kind==="mutation"&&!Object.keys(transaction.diff).length)return undefined;
  const event:IfcEditEvent={revision:revision+1,kind,scope:this.scope,diff:transaction.diff,inverse:transaction.inverse,...(mutation?{mutation:yield* copyIfcOwnedSteps(mutation,options)}:{})};const returned=yield* copyIfcOwnedSteps(event,options);return{transaction,event,returned,entry:previous??{diff:transaction.diff,inverse:transaction.inverse}};
 }
 #publish(kind:IfcEditEvent["kind"],revision:number,prepared:ReturnType<IfcEditHistory["#prepare"]> extends never?never:{transaction:IfcDiffTransaction;event:IfcEditEvent;returned:IfcEditEvent;entry:IfcHistoryEntry}|undefined,options:IfcApplyOptions):IfcEditEvent|undefined{
  checkIfcCancellation(options);this.#check(revision);if(!prepared)return undefined;this.#snapshot=prepared.transaction.snapshot;this.#events.push(prepared.event);if(kind==="mutation"){this.#undo.push(prepared.entry);this.#redo=[];}else if(kind==="undo"){this.#undo.pop();this.#redo.push(prepared.entry);}else{this.#redo.pop();this.#undo.push(prepared.entry);}return prepared.returned;
 }
 execute(mutation:Ifc4x3Mutation,revision:number,options:IfcApplyOptions={}):IfcEditEvent|undefined{return this.#publish("mutation",revision,runIfcEdit(this.#prepare("mutation",revision,options,mutation),options),options);}
 async executeAsync(mutation:Ifc4x3Mutation,revision:number,options:IfcApplyOptions={}):Promise<IfcEditEvent|undefined>{return this.#publish("mutation",revision,await runIfcEditAsync(this.#prepare("mutation",revision,options,mutation),options),options);}
 undo(revision:number,options:IfcApplyOptions={}):IfcEditEvent|undefined{return this.#publish("undo",revision,runIfcEdit(this.#prepare("undo",revision,options),options),options);}
 async undoAsync(revision:number,options:IfcApplyOptions={}):Promise<IfcEditEvent|undefined>{return this.#publish("undo",revision,await runIfcEditAsync(this.#prepare("undo",revision,options),options),options);}
 redo(revision:number,options:IfcApplyOptions={}):IfcEditEvent|undefined{return this.#publish("redo",revision,runIfcEdit(this.#prepare("redo",revision,options),options),options);}
 async redoAsync(revision:number,options:IfcApplyOptions={}):Promise<IfcEditEvent|undefined>{return this.#publish("redo",revision,await runIfcEditAsync(this.#prepare("redo",revision,options),options),options);}
}
''',encoding='utf-8',newline='\n')
t=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️tests/🧬️editing/🟦️.ts');s=t.read_text(encoding='utf-8').replace('new IfcEditHistory(before)','IfcEditHistory.createSync(before)');t.write_text(s,encoding='utf-8',newline='\n')
