import{closeSync,constants,fsyncSync,lstatSync,mkdirSync,openSync,readFileSync,renameSync,unlinkSync,writeSync}from"node:fs";
import{dirname,join}from"node:path";
import{createHash}from"node:crypto";
import{flushNativeDirectoryV1}from"../../../../📁️directory/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json" with{type:"json"};
const protocol=Object.freeze({...schema.definitions.Protocol.const}),ledger=protocol.ledger;

/** 📒️ One closed append-only recovery journal event. */
export type NativeBootLedgerEventV1=Readonly<Record<string,unknown>&{v:1;type:"begin"|"step"|"commit"|"abort";boot:string}>;

/** 📑️ Summarizes a ledger: whether the last recovery of its scope finished, and in which boot. */
export type NativeBootLedgerStateV1=Readonly<{state:"empty"|"committed"|"incomplete";boot:string|null;corrupt:boolean;tornTail:boolean}>;

const empty=(corrupt:boolean,tornTail:boolean):NativeBootLedgerStateV1=>Object.freeze({state:"empty",boot:null,corrupt,tornTail});
function complete(text:string):Readonly<{rows:readonly string[];tornTail:boolean}>{const lines=text.split("\n"),tail=lines.pop()!;return Object.freeze({rows:lines,tornTail:tail!==""});}

/** 🔎️ Reads ledger text without trusting it: a torn final line is ignored, any other unknown line makes the whole ledger advisory-corrupt. */
export function readNativeBootLedgerV1(text:string):NativeBootLedgerStateV1{
 if(Buffer.byteLength(text)>ledger.maximumBytes)return empty(true,false);
 const{rows,tornTail}=complete(text);let last:NativeBootLedgerEventV1|undefined,begin:string|null=null;
 for(const line of rows){
  let event:unknown;try{event=JSON.parse(line);}catch{return empty(true,tornTail);}
  if(Buffer.byteLength(line)>ledger.maximumLineBytes||validateJsonSchemaSubset(schema.definitions.LedgerEvent,event,schema).length)return empty(true,tornTail);
  last=event as NativeBootLedgerEventV1;if(last.type==="begin")begin=last.boot;
 }
 if(!last)return empty(false,tornTail);
 return Object.freeze({state:last.type==="commit"?"committed" as const:"incomplete" as const,boot:last.type==="commit"?last.boot:begin??last.boot,corrupt:false,tornTail});
}

/** 🏷️ Names the ledger file of one scope path, outside every partition so no image inventory ever sees it. */
export function nativeBootLedgerPathV1(directory:string,scopePath:string):string{return join(directory,createHash("sha256").update(scopePath).digest("hex")+ledger.extension);}

function replaceAtomically(path:string,text:string):void{
 const temporary=path+".new";try{unlinkSync(temporary);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
 const fd=openSync(temporary,constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|(constants.O_NOFOLLOW??0),0o600);
 try{writeSync(fd,text);fsyncSync(fd);}finally{closeSync(fd);}
 renameSync(temporary,path);flushNativeDirectoryV1(dirname(path));
}

/** 📥️ Loads one ledger file; a missing file is an empty ledger and an unsafe or oversized file is a corrupt one. */
export function loadNativeBootLedgerV1(path:string):Readonly<{text:string;state:NativeBootLedgerStateV1}>{
 try{const row=lstatSync(path);if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1||row.size>ledger.maximumBytes)return Object.freeze({text:"",state:empty(true,false)});const text=readFileSync(path,"utf8");return Object.freeze({text,state:readNativeBootLedgerV1(text)});}
 catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;return Object.freeze({text:"",state:empty(false,false)});}
}

/** 🧾️ Appends and fsyncs one event; a full, torn or corrupt ledger is first replaced atomically by its surviving complete lines. */
export function appendNativeBootLedgerV1(path:string,event:unknown):void{
 if(validateJsonSchemaSubset(schema.definitions.LedgerEvent,event,schema).length)throw Error("Closed boot recovery ledger event required");
 const line=JSON.stringify(event)+"\n";if(Buffer.byteLength(line)>ledger.maximumLineBytes)throw Error("Bounded boot recovery ledger line required");
 mkdirSync(dirname(path),{recursive:true,mode:0o700});
 const{text,state}=loadNativeBootLedgerV1(path),full=Buffer.byteLength(text)+Buffer.byteLength(line)>ledger.maximumBytes;
 if(state.corrupt||state.tornTail||full){
  const rows=state.corrupt?[]:complete(text).rows.filter(row=>row!=="");
  const kept=full||state.corrupt?rows.filter(row=>JSON.parse(row).type==="commit").slice(-ledger.retainedCommits):rows;
  replaceAtomically(path,kept.length?kept.join("\n")+"\n":"");
 }
 const fd=openSync(path,constants.O_WRONLY|constants.O_CREAT|constants.O_APPEND|(constants.O_NOFOLLOW??0),0o600);
 try{writeSync(fd,line);fsyncSync(fd);}finally{closeSync(fd);}
}

/** 🚦️ Tells whether this scope already finished its recovery in this very boot session. */
export function nativeBootLedgerCommittedV1(path:string,bootSession:string):boolean{const{state}=loadNativeBootLedgerV1(path);return state.state==="committed"&&state.boot===bootSession&&!state.corrupt&&!state.tornTail;}
