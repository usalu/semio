import {closeSync,existsSync,openSync,readFileSync,readSync,readdirSync,statSync} from "node:fs";
import {join} from "node:path";
import policy from "./🔣️.json" with {type:"json"};

/** 📐️ Where the replica journal lives in the fleet store and how it is read: the origins folder, the segment folder and extension, the byte budget per read and the look-back of an editor query. */
export type JournalPolicyV1=Readonly<typeof policy>;
/** ✍️ The newest recorded change of a path: the machine whose journal wrote it. */
export type JournalEditorV1=Readonly<{origin:string;atMs:number;path:string}>;
/** 🧭️ How far the journal of every origin has been read: the segment and the byte offset. */
export type JournalCursorV1=Readonly<Record<string,Readonly<{segment:number;offset:number}>>>;
/** 📜️ One change of the replica journal. */
export type JournalEventV1=Readonly<{origin:string;atMs:number;path:string;operation:string;sha256?:string}>;

const rules=policy as JournalPolicyV1;

/** 🕰️ Splits a journal clock `<ms>-<counter>-<origin>`: the millisecond and the origin the entry was written on. */
export function journalClockV1(hlc:string):Readonly<{atMs:number;origin:string}>|undefined{
 const match=/^(\d+)-(\d+)-(.+)$/u.exec(hlc);
 return match===null?undefined:{atMs:Number(match[1]),origin:match[3]!};
}

const segmentNumber=(name:string,limits:JournalPolicyV1):number=>name.endsWith(limits.journal.extension)?Number(name.slice(0,-limits.journal.extension.length)):NaN;

/** 📖️ Reads what the replica journal gained since a cursor: per origin the segments from the cursor on, from the saved byte offset, only complete lines, bounded per call. An empty cursor starts every origin at the end of its newest segment (nothing to replay, the sweep then starts with the whole workspace); an origin that appears later is read from the start of its newest segment. Returns the events and the advanced cursor. */
export function journalReadV1(store:string,cursor:JournalCursorV1,limits:JournalPolicyV1=rules):Readonly<{events:readonly JournalEventV1[];cursor:JournalCursorV1;fresh:readonly string[]}>{
 const origins=join(store,...limits.store.directory),events:JournalEventV1[]=[],next:Record<string,{segment:number;offset:number}>={},fresh:string[]=[];
 let names:string[];try{names=readdirSync(origins);}catch{return {events,cursor,fresh};}
 for(const origin of names.sort()){
  const directory=join(origins,origin,limits.journal.segmentsDirectory);let files:number[];
  try{files=readdirSync(directory).map(name=>segmentNumber(name,limits)).filter(Number.isInteger).sort((a,b)=>a-b);}catch{continue;}
  if(!files.length)continue;
  const known=cursor[origin];
  if(known===undefined&&!Object.keys(cursor).length){const last=files[files.length-1]!;let size=0;try{size=statSync(join(directory,String(last).padStart(12,"0")+limits.journal.extension)).size;}catch{}next[origin]={segment:last,offset:size};fresh.push(origin);continue;}
  const known2=known??{segment:files[files.length-1]!,offset:0};
  let budget=limits.journal.maximumBytesPerRead,position={...known2};
  for(const segment of files.filter(number=>number>=known2.segment)){
   const file=join(directory,String(segment).padStart(12,"0")+limits.journal.extension);let size:number;
   try{size=statSync(file).size;}catch{continue;}
   const start=segment===position.segment?position.offset:0;
   if(size<=start){position={segment,offset:Math.max(start,size)};continue;}
   const length=Math.min(size-start,budget);
   if(length<=0)break;
   const buffer=Buffer.alloc(length),handle=openSync(file,"r");
   try{readSync(handle,buffer,0,length,start);}finally{closeSync(handle);}
   const text=buffer.toString("utf8"),cut=text.lastIndexOf("\n");
   if(cut<0){position={segment,offset:start};break;}
   for(const line of text.slice(0,cut).split("\n")){
    if(line==="")continue;
    try{
     const row=JSON.parse(line) as {hlc?:string;path?:string;operation?:string;sha256?:string},clock=row.hlc===undefined?undefined:journalClockV1(row.hlc);
     if(clock!==undefined&&typeof row.path==="string")events.push({origin:clock.origin,atMs:clock.atMs,path:row.path,operation:row.operation??"put",...typeof row.sha256==="string"?{sha256:row.sha256}:{}});
    }catch{}
   }
   position={segment,offset:start+Buffer.byteLength(text.slice(0,cut+1))};
   budget-=length;
   if(budget<=0)break;
  }
  next[origin]=position;
 }
 return {events,cursor:{...cursor,...next},fresh};
}

/** 🔎️ The newest journal change of one path, looking back over the last segments of every origin (the last editor of a failing file). */
export function journalFindEditorV1(store:string,path:string,limits:JournalPolicyV1=rules):JournalEditorV1|undefined{
 const origins=join(store,...limits.store.directory);let best:JournalEditorV1|undefined,names:string[];
 try{names=readdirSync(origins);}catch{return undefined;}
 for(const origin of names){
  const directory=join(origins,origin,limits.journal.segmentsDirectory);let files:number[];
  try{files=readdirSync(directory).map(name=>segmentNumber(name,limits)).filter(Number.isInteger).sort((a,b)=>b-a).slice(0,limits.journal.lookbackSegments);}catch{continue;}
  for(const segment of files){
   let text:string;try{text=readFileSync(join(directory,String(segment).padStart(12,"0")+limits.journal.extension),"utf8");}catch{continue;}
   for(const line of text.split("\n")){
    if(!line.includes(path))continue;
    try{
     const row=JSON.parse(line) as {hlc?:string;path?:string},clock=row.hlc===undefined?undefined:journalClockV1(row.hlc);
     if(clock!==undefined&&row.path===path&&(best===undefined||clock.atMs>best.atMs))best={origin:clock.origin,atMs:clock.atMs,path};
    }catch{}
   }
   if(best!==undefined)break;
  }
 }
 return best;
}

/** 🏠️ The origin id of this machine (`originId` of the machine record of the checkout) and the fleet store root it names, for the sweep to publish under. */
export function journalMachineV1(root:string,environment:NodeJS.ProcessEnv,limits:JournalPolicyV1=rules):Readonly<{origin:string;store:string}>|undefined{
 try{
  const record=JSON.parse(readFileSync(join(root,limits.store.machineFile),"utf8")) as {originId?:string;storeRoot?:string};
  const store=environment[limits.store.environment]??record.storeRoot;
  if(typeof record.originId==="string"&&record.originId!==""&&typeof store==="string"&&store!==""&&existsSync(store))return {origin:record.originId,store};
 }catch{}
 return undefined;
}

