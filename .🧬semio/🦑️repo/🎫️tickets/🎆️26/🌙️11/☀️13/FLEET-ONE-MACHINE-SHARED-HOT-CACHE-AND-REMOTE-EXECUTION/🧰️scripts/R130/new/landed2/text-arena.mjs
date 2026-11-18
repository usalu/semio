import{createHash}from"node:crypto";
import{createKeyTableV1}from"../🔑️key-table/🟨️.mjs";

/** 🧠️ The short texts remembered by their number so that the keys and kinds every fact repeats are not hashed again: at most this many, each at most this long. */
const RECENT_TEXTS=8192,RECENT_TEXT_LENGTH=48;

/**
 * 🔤️ A set of distinct texts kept as UTF-8 bytes in one growing buffer with a start offset per text (about the length of the text plus 28 bytes each, where a `Set` of strings and a buffer per string costs several times that): `add` answers the number of the text (its place in order of arrival), `id` finds it again, `sorted` answers the numbers in the byte order of the texts and `bytes` the bytes of one. `validate` is called once for every new text and may refuse it by throwing.
 * @see ../🔑️key-table/🟨️.mjs the table that finds a text by its digest
 */
export function createTextArenaV1({validate}={}){
 const table=createKeyTableV1(),recent=new Map();
 let arena=Buffer.allocUnsafe(1<<16),used=0,starts=new Uint32Array(1025),count=0;
 const digest=span=>createHash("sha256").update(span).digest();
 const encode=text=>{
  const room=text.length*3;
  if(used+room>arena.length){const next=Buffer.allocUnsafe(Math.max(arena.length*2,used+room));arena.copy(next,0,0,used);arena=next;}
  return arena.write(text,used,"utf8");
 };
 const remember=(text,number)=>{if(text.length<=RECENT_TEXT_LENGTH){if(recent.size>=RECENT_TEXTS)recent.clear();recent.set(text,number);}return number;};
 const api={
  add(text){
   const near=recent.get(text);if(near!==undefined)return near;
   const length=encode(text),known=table.getDigest(digest(arena.subarray(used,used+length)));
   if(known!==undefined)return remember(text,known);
   validate?.(text);
   if(count+2>starts.length){const next=new Uint32Array(starts.length*2);next.set(starts);starts=next;}
   starts[count]=used;used+=length;starts[count+1]=used;
   table.setDigest(digest(arena.subarray(starts[count],used)),count);
   return remember(text,count++);
  },
  id(text){
   const near=recent.get(text);if(near!==undefined)return near;
   const length=encode(text),known=table.getDigest(digest(arena.subarray(used,used+length)));
   return known===undefined?undefined:remember(text,known);
  },
  count:()=>count,
  bytes:number=>arena.subarray(starts[number],starts[number+1]),
  sorted(){
   const order=Uint32Array.from({length:count},(_unused,at)=>at);
   return order.sort((a,b)=>arena.compare(arena,starts[b],starts[b+1],starts[a],starts[a+1]));
  },
  size:()=>used+starts.byteLength+table.bytes(),
 };
 return Object.freeze(api);
}
