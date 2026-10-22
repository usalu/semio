import schema from "../📋️messages/🔣️.json" with {type:"json"};

/** 🧬️ One field of a message as the generated wire schema states it. */
export type ProtobufFieldV1=Readonly<{number:number;name:string;type:"bool"|"bytes"|"double"|"enum"|"float"|"int32"|"int64"|"message"|"string"|"uint32"|"uint64";repeated:boolean;message?:string;enum?:string}>;

/** 🧾️ A decoded or encodable message: plain objects keyed by the protocol's own field names. */
export type ProtobufMessageV1=Record<string,any>;

/** 🔌️ One method of a service, with its message names and streaming shape. */
export type ProtobufMethodV1=Readonly<{request:string;response:string;clientStreaming:boolean;serverStreaming:boolean}>;

const SCALARS=new Set(["bool","bytes","double","enum","float","int32","int64","string","uint32","uint64"]),VARINT=new Set(["bool","enum","int32","int64","uint32","uint64"]),PACKED=new Set([...VARINT,"double","float"]),
 MESSAGES=schema.messages as unknown as Readonly<Record<string,readonly ProtobufFieldV1[]>>,ENUMS=schema.enums as Readonly<Record<string,Readonly<Record<string,number>>>>,SERVICES=schema.services as unknown as Readonly<Record<string,Readonly<Record<string,ProtobufMethodV1>>>>,
 BY_NUMBER=new Map<string,Map<number,ProtobufFieldV1>>(),UTF8=new TextDecoder("utf-8",{fatal:true}),ENCODER=new TextEncoder(),MAXIMUM=Number.MAX_SAFE_INTEGER;
for(const [name,fields] of Object.entries(MESSAGES)){
 const numbers=new Map<number,ProtobufFieldV1>();
 for(const field of fields){
  if(numbers.has(field.number)||(field.type!=="message"&&!SCALARS.has(field.type)))throw Error(`Supported field required: ${name}.${field.name}`);
  numbers.set(field.number,field);
 }
 BY_NUMBER.set(name,numbers);
}

/** 📚️ The services of the wire schema by full name, each with its methods. */
export const PROTOBUF_SERVICES_V1=SERVICES;

/** 🔢️ The numeric value of an enum member by its name. */
export function protobufEnumV1(name:string,member:string):number{
 const value=ENUMS[name]?.[member];
 if(value===undefined)throw Error(`Declared enum member required: ${name}.${member}`);
 return value;
}

class Writer{
 bytes=new Uint8Array(256);length=0;
 reserve(extra:number):void{if(this.length+extra<=this.bytes.length)return;let size=this.bytes.length*2;while(size<this.length+extra)size*=2;const next=new Uint8Array(size);next.set(this.bytes.subarray(0,this.length));this.bytes=next;}
 varint(value:number):void{
  this.reserve(10);
  if(value<0){let rest=BigInt.asUintN(64,BigInt(value));while(rest>=128n){this.bytes[this.length++]=Number(rest&127n)|128;rest>>=7n;}this.bytes[this.length++]=Number(rest);return;}
  if(!Number.isSafeInteger(value))throw Error("Safe integer required");
  while(value>=128){this.bytes[this.length++]=(value%128)|128;value=Math.floor(value/128);}
  this.bytes[this.length++]=value;
 }
 raw(data:Uint8Array):void{this.reserve(data.length);this.bytes.set(data,this.length);this.length+=data.length;}
 view():Uint8Array{return this.bytes.subarray(0,this.length);}
}

const enumNumber=(field:ProtobufFieldV1,value:unknown):number=>typeof value==="string"?protobufEnumV1(field.enum!,value):Number(value);

function writeValue(writer:Writer,field:ProtobufFieldV1,value:unknown,tag:boolean):void{
 const key=(wire:number)=>{if(tag)writer.varint(field.number*8+wire);};
 switch(field.type){
  case "message":{const inner=encodeProtobufV1(field.message!,value as ProtobufMessageV1);key(2);writer.varint(inner.length);writer.raw(inner);return;}
  case "string":{const data=ENCODER.encode(String(value));key(2);writer.varint(data.length);writer.raw(data);return;}
  case "bytes":{const data=value as Uint8Array;key(2);writer.varint(data.length);writer.raw(data);return;}
  case "bool":key(0);writer.varint(value?1:0);return;
  case "enum":key(0);writer.varint(enumNumber(field,value));return;
  case "double":{key(1);writer.reserve(8);new DataView(writer.bytes.buffer).setFloat64(writer.length,Number(value),true);writer.length+=8;return;}
  case "float":{key(5);writer.reserve(4);new DataView(writer.bytes.buffer).setFloat32(writer.length,Number(value),true);writer.length+=4;return;}
  default:{
   const number=Number(value);
   if(!Number.isSafeInteger(number)||(field.type==="int32"&&(number<-2147483648||number>2147483647))||(field.type==="uint32"&&(number<0||number>4294967295))||((field.type==="uint64")&&number<0))throw Error(`Integer in range required: ${field.name}`);
   key(0);writer.varint(number);
  }
 }
}

const isDefault=(field:ProtobufFieldV1,value:unknown):boolean=>value===undefined||value===null||(field.type==="bytes"?(value as Uint8Array).length===0:field.type==="string"?value==="":field.type==="bool"?value===false:field.type==="message"?false:field.type==="enum"?enumNumber(field,value)===0:value===0);

/** 📦️ Encodes a message by its schema: fields in number order, defaults omitted, repeated scalars packed, enums by number or member name. */
export function encodeProtobufV1(name:string,value:ProtobufMessageV1):Uint8Array{
 const fields=MESSAGES[name];
 if(!fields)throw Error(`Declared message required: ${name}`);
 const writer=new Writer();
 for(const field of fields){
  const entry=value[field.name];
  if(field.repeated){
   const list=(entry??[]) as readonly unknown[];
   if(list.length===0)continue;
   if(PACKED.has(field.type)){const inner=new Writer();for(const item of list)writeValue(inner,field,item,false);writer.varint(field.number*8+2);writer.varint(inner.length);writer.raw(inner.view());}
   else for(const item of list)writeValue(writer,field,item,true);
  }else if(!isDefault(field,entry))writeValue(writer,field,entry,true);
 }
 return writer.view().slice();
}

class Reader{
 position=0;constructor(readonly bytes:Uint8Array,readonly end:number){}
 get done():boolean{return this.position>=this.end;}
 varint():number{
  let result=0,scale=1,count=0,big:bigint|undefined;
  for(;;){
   if(this.position>=this.end)throw Error("Truncated varint");
   const byte=this.bytes[this.position++]!;count++;
   if(count>10)throw Error("Varint too long");
   if(count<=7)result+=(byte&127)*scale,scale*=128;
   else{big??=BigInt(result);big|=BigInt(byte&127)<<BigInt(7*(count-1));}
   if(byte<128)break;
  }
  if(big===undefined)return result;
  const signed=BigInt.asIntN(64,big);
  if(signed<0n){if(signed<BigInt(-MAXIMUM))throw Error("Integer out of range");return Number(signed);}
  if(big>BigInt(MAXIMUM))throw Error("Integer out of range");
  return Number(big);
 }
 length():number{const size=this.varint();if(size<0||this.position+size>this.end)throw Error("Truncated field");return size;}
}

const defaultOf=(field:ProtobufFieldV1):unknown=>field.repeated?[]:field.type==="string"?"":field.type==="bytes"?new Uint8Array(0):field.type==="bool"?false:field.type==="message"?undefined:0;

function readValue(reader:Reader,field:ProtobufFieldV1,wire:number):unknown{
 switch(field.type){
  case "message":{const size=reader.length(),begin=reader.position;reader.position+=size;return decodeRange(field.message!,reader.bytes,[begin,begin+size]);}
  case "string":{const size=reader.length(),begin=reader.position;reader.position+=size;return UTF8.decode(reader.bytes.subarray(begin,begin+size));}
  case "bytes":{const size=reader.length(),begin=reader.position;reader.position+=size;return reader.bytes.slice(begin,begin+size);}
  case "bool":return reader.varint()!==0;
  case "double":{const view=new DataView(reader.bytes.buffer,reader.bytes.byteOffset+reader.position,8);reader.position+=8;return view.getFloat64(0,true);}
  case "float":{const view=new DataView(reader.bytes.buffer,reader.bytes.byteOffset+reader.position,4);reader.position+=4;return view.getFloat32(0,true);}
  default:return reader.varint();
 }
}

const expectedWire=(field:ProtobufFieldV1):number=>field.type==="double"?1:field.type==="float"?5:VARINT.has(field.type)?0:2;

function skip(reader:Reader,wire:number):void{
 if(wire===0)reader.varint();
 else if(wire===1){if(reader.position+8>reader.end)throw Error("Truncated field");reader.position+=8;}
 else if(wire===5){if(reader.position+4>reader.end)throw Error("Truncated field");reader.position+=4;}
 else if(wire===2){const size=reader.length();reader.position+=size;}
 else throw Error(`Unsupported wire type ${wire}`);
}

function decodeRange(name:string,bytes:Uint8Array,range:readonly [number,number]):ProtobufMessageV1{
 const numbers=BY_NUMBER.get(name);
 if(!numbers)throw Error(`Declared message required: ${name}`);
 const reader=new Reader(bytes,range[1]),out:ProtobufMessageV1={};
 reader.position=range[0];
 for(const field of numbers.values())if(field.repeated||field.type!=="message")out[field.name]=defaultOf(field);
 while(!reader.done){
  const key=reader.varint(),wire=key%8,number=Math.floor(key/8),field=numbers.get(number);
  if(number===0)throw Error("Field number required");
  if(!field){skip(reader,wire);continue;}
  if(field.repeated&&PACKED.has(field.type)&&wire===2){
   const size=reader.length(),inner=new Reader(bytes,reader.position+size);inner.position=reader.position;
   while(!inner.done)out[field.name].push(readValue(inner,field,expectedWire(field)));
   reader.position=inner.end;continue;
  }
  if(wire!==expectedWire(field)){skip(reader,wire);continue;}
  const value=readValue(reader,field,wire);
  if(field.repeated)out[field.name].push(value);else out[field.name]=value;
 }
 return out;
}

/** 📥️ Decodes a message by its schema into an object with every scalar defaulted and every repeated field a list; unknown fields are skipped, wrong wire types are treated as unknown, and truncated or malformed input throws. */
export function decodeProtobufV1(name:string,bytes:Uint8Array):ProtobufMessageV1{return decodeRange(name,bytes,[0,bytes.length]);}

/** 🏷️ The well-known messages of the protocol by short name. */
export const REAPI_V1=Object.freeze({
 digest:"build.bazel.remote.execution.v2.Digest",action:"build.bazel.remote.execution.v2.Action",command:"build.bazel.remote.execution.v2.Command",directory:"build.bazel.remote.execution.v2.Directory",
 tree:"build.bazel.remote.execution.v2.Tree",actionResult:"build.bazel.remote.execution.v2.ActionResult",executeRequest:"build.bazel.remote.execution.v2.ExecuteRequest",
 executeResponse:"build.bazel.remote.execution.v2.ExecuteResponse",executeMetadata:"build.bazel.remote.execution.v2.ExecuteOperationMetadata",operation:"google.longrunning.Operation",
 status:"google.rpc.Status",any:"google.protobuf.Any",preconditionFailure:"google.rpc.PreconditionFailure",capabilities:"build.bazel.remote.execution.v2.ServerCapabilities",
 stage:"build.bazel.remote.execution.v2.ExecutionStage.Value",
});
