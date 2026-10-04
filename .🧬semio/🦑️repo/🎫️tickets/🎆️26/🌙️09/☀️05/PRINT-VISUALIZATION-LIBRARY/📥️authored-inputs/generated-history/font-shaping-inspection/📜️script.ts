import {readFileSync} from 'node:fs';
import {join} from 'node:path';
const path=join(process.cwd(),'🧰️framework/🛍️products/📓️print/🖼️assets/🔤️font/🅰️anta/🅰️Anta-Regular.ttf');
const b=readFileSync(path),tables=new Map<string,number>();
for(let i=0;i<b.readUInt16BE(4);i++){const p=12+16*i;tables.set(b.toString('ascii',p,p+4),b.readUInt32BE(p+8));}
for(const name of ['GPOS','GSUB']){
  const p=tables.get(name)!,scripts=p+b.readUInt16BE(p+4),features=p+b.readUInt16BE(p+6),lookups=p+b.readUInt16BE(p+8);
  const scriptData=Array.from({length:b.readUInt16BE(scripts)},(_,i)=>{const at=scripts+2+i*6,base=scripts+b.readUInt16BE(at+4),lang=base+b.readUInt16BE(base);return {script:b.toString('ascii',at,at+4),required:b.readUInt16BE(lang+2),features:Array.from({length:b.readUInt16BE(lang+4)},(_,j)=>b.readUInt16BE(lang+6+2*j))};});
  const featureData=Array.from({length:b.readUInt16BE(features)},(_,i)=>{const at=features+2+i*6,base=features+b.readUInt16BE(at+4);return {tag:b.toString('ascii',at,at+4),lookups:Array.from({length:b.readUInt16BE(base+2)},(_,j)=>b.readUInt16BE(base+4+j*2))};});
  const lookupData=Array.from({length:b.readUInt16BE(lookups)},(_,i)=>{const base=lookups+b.readUInt16BE(lookups+2+i*2);return {index:i,type:b.readUInt16BE(base),flags:b.readUInt16BE(base+2),subtables:Array.from({length:b.readUInt16BE(base+4)},(_,j)=>{const sub=base+b.readUInt16BE(base+6+j*2);return {format:b.readUInt16BE(sub),header:Array.from({length:10},(_,k)=>b.readUInt16BE(sub+2*k))};})};});
  console.log('[DEBUG] '+JSON.stringify({table:name,scripts:scriptData,features:featureData,lookups:lookupData}));
}
