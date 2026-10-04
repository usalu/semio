/** 🧾️ Direct FormDictionary entities keep unique question identities and ordered intrinsic children. */
import type{FormDictionary}from"../../../🟦️.ts";
import type{IntrinsicValue}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import{artifactSqliteInteger as integer,artifactSqliteText as text,artifactSqliteBoolean as boolean,artifactSqliteCheckpoint}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type{SqliteRow,SqliteValue,SqliteOperation}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
type Put=(name:string,cells:readonly SqliteValue[],id?:bigint)=>Promise<bigint>;
type Take=(name:string,id:bigint)=>Promise<SqliteRow>;
type List=(name:string,parent:bigint,index?:number,ordinal?:number)=>Promise<SqliteRow[]>;
/** 📤️ Assign each literal value one relational owner without serialization. */
export async function projectFormDictionary(dictionary:FormDictionary,put:Put,scalar:(word:{bits:bigint})=>Promise<bigint>):Promise<void>{
 await put("form_dictionary",[1n],1n);const identities=new Set<string>();
 const pending:{value:IntrinsicValue;attach:(id:bigint)=>Promise<unknown>}[]=[];
 for(let ordinal=dictionary.entries.length-1;ordinal>=0;ordinal--){const entry=dictionary.entries[ordinal]!;if(identities.has(entry.questionId))throw Error("duplicate FormDictionary question identity");identities.add(entry.questionId);pending.push({value:entry.value,attach:id=>put("dictionary_entry",[1n,BigInt(ordinal),entry.questionId,id])});}
 while(pending.length){const task=pending.pop()!,value=task.value,id=await put("dictionary_value",[value.kind]);await task.attach(id);
  switch(value.kind){
   case"null":break;
   case"boolean":await put("dictionary_boolean",[value.value?1n:0n],id);break;
   case"unsigned":if(value.value<0n||value.value>0xffffffffffffffffn)throw Error("FormDictionary UInt64 range");await put("dictionary_unsigned",[value.value.toString()],id);break;
   case"signed":if(value.value<-(1n<<63n)||value.value>=(1n<<63n))throw Error("FormDictionary Int64 range");await put("dictionary_signed",[value.value],id);break;
   case"float":await put("dictionary_float",[await scalar(value.value)],id);break;
   case"text":await put("dictionary_text",[value.value],id);break;
   case"bytes":await put("dictionary_bytes",[value.value],id);break;
   case"array":for(let ordinal=value.items.length-1;ordinal>=0;ordinal--)pending.push({value:value.items[ordinal]!,attach:child=>put("dictionary_array",[id,BigInt(ordinal),child])});break;
   case"object":for(let ordinal=value.members.length-1;ordinal>=0;ordinal--){const member=value.members[ordinal]!;pending.push({value:member.value,attach:child=>put("dictionary_member",[id,BigInt(ordinal),member.name,child])});}break;
  }
 }
}
/** 📥️ Consume exact relations once so cycles, shared ownership and orphan scalars refuse. */
export async function reconstructFormDictionary(take:Take,list:List,scalar:(row:SqliteRow,index:number)=>Promise<{bits:bigint}>,operation:SqliteOperation):Promise<FormDictionary>{
 const root=await take("form_dictionary",1n);if(integer(root,1)!==1n)throw Error("FormDictionary document owner");const entries:FormDictionary["entries"]=[],identities=new Set<string>();
 const pending:{id:bigint;attach:(value:IntrinsicValue)=>void}[]=[];
 for(const row of await list("dictionary_entry",1n)){const questionId=text(row,3);if(identities.has(questionId))throw Error("duplicate FormDictionary question identity");identities.add(questionId);const entry={questionId,value:{kind:"null"}as IntrinsicValue};entries.push(entry);pending.push({id:integer(row,4),attach:value=>{entry.value=value;}});}
 while(pending.length){const task=pending.pop()!,row=await take("dictionary_value",task.id),kind=text(row,1);let value:IntrinsicValue;
  switch(kind){
   case"null":value={kind};break;
   case"boolean":value={kind,value:boolean(await take("dictionary_boolean",task.id),1)};break;
   case"unsigned":{const magnitude=text(await take("dictionary_unsigned",task.id),1);if(magnitude.length>20||!/^(0|[1-9][0-9]*)$/.test(magnitude))throw Error("FormDictionary canonical UInt64 magnitude");const unsigned=BigInt(magnitude);if(unsigned>0xffffffffffffffffn)throw Error("FormDictionary UInt64 range");value={kind,value:unsigned};break;}
   case"signed":value={kind,value:integer(await take("dictionary_signed",task.id),1)};break;
   case"float":value={kind,value:await scalar(await take("dictionary_float",task.id),1)};break;
   case"text":value={kind,value:text(await take("dictionary_text",task.id),1)};break;
   case"bytes":{const row=await take("dictionary_bytes",task.id),bytes=row.values[1];if(!(bytes instanceof Uint8Array))throw Error("FormDictionary byte value required");await artifactSqliteCheckpoint(operation,"reconstructSnapshot",0,bytes.length);const backing=operation.allocateBytes(bytes.length);for(let offset=0;offset<bytes.length;){const end=Math.min(offset+16384,bytes.length);backing.set(bytes.subarray(offset,end),offset);offset=end;if(bytes.length>=65536)await artifactSqliteCheckpoint(operation,"reconstructSnapshot",offset,bytes.length);}value={kind,value:backing};break;}
   case"array":{const items:IntrinsicValue[]=[];for(const row of await list("dictionary_array",task.id)){const index=items.length;items.push({kind:"null"});pending.push({id:integer(row,3),attach:value=>{items[index]=value;}});}value={kind,items};break;}
   case"object":{const members:{name:string;value:IntrinsicValue}[]=[];for(const row of await list("dictionary_member",task.id)){const member={name:text(row,3),value:{kind:"null"}as IntrinsicValue};members.push(member);pending.push({id:integer(row,4),attach:value=>{member.value=value;}});}value={kind,members};break;}
   default:throw Error("unknown FormDictionary value kind");
  }
  task.attach(value);
 }
 return{entries};
}
