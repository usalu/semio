import {foldNameV1} from "../../🚧️boundary/🟦️.ts";

/** 🧩️ One live non-directory entry of the folded manifest, as the structural conflict rule sees it. */
export type ClashEntryV1=Readonly<{path:string;hlc:string}>;

/** 🔑️ The fleet-wide comparison key of a path: the fold of the boundary, so that paths which collide on the most restrictive file system share it. */
export const clashKeyV1=(path:string):string=>foldNameV1(path);

/** ⚔️ Tells whether two live entries cannot coexist on the most restrictive file system: their keys are equal (case collision) or the key of one is a directory prefix of the other (a file or link with entries below it). Equal paths are the same entry and never clash. */
export function clashV1(left:string,right:string):boolean{
 if(left===right)return false;
 const one=clashKeyV1(left),two=clashKeyV1(right);
 return one===two||two.startsWith(`${one}/`)||one.startsWith(`${two}/`);
}

/** 🏆️ The structural conflict rule as a pure function of the live entries: entries are taken newest hlc first and an entry is kept unless it clashes with one that was kept already; the result maps every retired path to the kept path that retired it. The rule reads nothing but the entries, so every machine that folds the same manifest retires the same paths. */
export function retiredV1(entries:readonly ClashEntryV1[]):ReadonlyMap<string,string>{
 const order=[...entries].sort((left,right)=>left.hlc<right.hlc?1:left.hlc>right.hlc?-1:left.path<right.path?-1:1),kept=new Map<string,string>(),below=new Map<string,string>(),retired=new Map<string,string>();
 for(const {path} of order){
  const key=clashKeyV1(path);
  let winner=kept.get(key)??below.get(key);
  for(let cut=key.lastIndexOf("/");winner===undefined&&cut>0;cut=key.lastIndexOf("/",cut-1))winner=kept.get(key.slice(0,cut));
  if(winner!==undefined){retired.set(path,winner);continue;}
  kept.set(key,path);
  for(let cut=key.lastIndexOf("/");cut>0;cut=key.lastIndexOf("/",cut-1)){const parent=key.slice(0,cut);if(!below.has(parent))below.set(parent,path);}
 }
 return retired;
}

/** 📜️ One content event of one path as the stale rule sees it: its hlc, the content it leaves (`sha256`, `null` for a delete) and the content the sender's own register held when it wrote it (`base`). */
export type ContentEventV1=Readonly<{hlc:string;sha256:string|null;base:string|null}>;

/** 🧓 The stale-republish rule as a pure function of the events of one path: the event with `hlc` is a republish of old content, not an edit, when it leaves content, differs from its predecessor (the event with the greatest smaller hlc), was not written on top of that predecessor (`base` differs from the predecessor's content) and leaves content that an earlier event had left already. The answer is the content of the predecessor, which the fleet keeps. The rule reads nothing but the events, so every machine that holds the same events decides alike; an event whose predecessor is unknown is never stale. */
export function staleRepublishV1(events:readonly ContentEventV1[],hlc:string):Readonly<{winner:string|null}>|undefined{
 const order=[...events].sort((left,right)=>left.hlc<right.hlc?-1:left.hlc>right.hlc?1:0),index=order.findIndex(event=>event.hlc===hlc);
 if(index<=0)return undefined;
 const event=order[index]!,predecessor=order[index-1]!;
 if(event.sha256===null||event.sha256===predecessor.sha256||event.base===predecessor.sha256)return undefined;
 return order.slice(0,index-1).some(older=>older.sha256===event.sha256)?{winner:predecessor.sha256}:undefined;
}
