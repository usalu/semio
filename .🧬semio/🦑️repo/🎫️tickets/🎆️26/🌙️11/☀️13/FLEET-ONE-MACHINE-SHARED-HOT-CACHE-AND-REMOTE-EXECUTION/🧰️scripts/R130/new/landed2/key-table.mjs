import{createHash}from"node:crypto";

const WORDS=5;

/**
 * 🔑️ A table from texts to unsigned numbers that keeps a fixed number of bytes per entry however long the texts are: a text is known by the first 128 bits of its SHA-256 (a clash of two different texts needs about 2^64 of them), the entries live in one typed array (`getDigest` and `setDigest` take a SHA-256 digest the caller already holds) with open addressing, and the table doubles at half load. `get` answers the number or `undefined`, `set` stores a number below 2^32 - 1, `intern` answers the number a text already has or stores the given one and answers it.
 * @see ../🗃️row-log/🟨️.mjs the row log that names its rows by it
 */
export function createKeyTableV1(capacity=1024){
 let size=1;while(size<capacity)size*=2;
 let mask=size-1,slots=new Uint32Array(size*WORDS),count=0;
 const words=new Uint32Array(4);
 const hash=text=>hashDigest(createHash("sha256").update(text).digest());
 const hashDigest=bytes=>{for(let at=0;at<4;at++)words[at]=bytes.readUInt32LE(at*4);};
 const find=()=>{
  let at=words[0]&mask;
  for(;;){
   const base=at*WORDS;
   if(slots[base+4]===0||(slots[base]===words[0]&&slots[base+1]===words[1]&&slots[base+2]===words[2]&&slots[base+3]===words[3]))return base;
   at=(at+1)&mask;
  }
 };
 const grow=()=>{
  const next={mask:mask*2+1,slots:new Uint32Array((mask+1)*2*WORDS)};
  for(let base=0;base<slots.length;base+=WORDS){
   if(slots[base+4]===0)continue;
   let at=slots[base]&next.mask;
   while(next.slots[at*WORDS+4]!==0)at=(at+1)&next.mask;
   next.slots.set(slots.subarray(base,base+WORDS),at*WORDS);
  }
  mask=next.mask;slots=next.slots;
 };
 const read=()=>{const stored=slots[find()+4];return stored===0?undefined:stored-1;};
 const write=value=>{
   if(!Number.isSafeInteger(value)||value<0||value>=0xffffffff)throw Error("Unsigned 32 bit number below the maximum required");
   let base=find();
   if(slots[base+4]===0){
    if((count+1)*2>mask+1){grow();base=find();}
    count++;
   }
   slots[base]=words[0];slots[base+1]=words[1];slots[base+2]=words[2];slots[base+3]=words[3];slots[base+4]=value+1;
 };
 return Object.freeze({
  get(text){hash(text);return read();},
  set(text,value){hash(text);write(value);},
  intern(text,value){hash(text);const known=read();if(known!==undefined)return known;write(value);return value;},
  getDigest(bytes){hashDigest(bytes);return read();},
  setDigest(bytes,value){hashDigest(bytes);write(value);},
  size:()=>count,
  bytes:()=>slots.byteLength,
  clear(){count=0;mask=1023;slots=new Uint32Array(1024*WORDS);},
 });
}
