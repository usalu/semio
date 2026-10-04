/** 🎨️ Explicit canonical indices or exact authored Unicode literals. */
import {artifactSqliteInteger,artifactSqliteText,artifactSqliteCheckpoint,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions,type ArtifactSqliteProjection} from "../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type {SqliteRow} from "../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
const alphabet="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
/** 🔤️ Recognizes the exact canonical padded standard alphabet without normalizing native text. */
export async function canonicalIndexCount(text:string,options:ArtifactSqliteOptions={}):Promise<number|undefined>{
 if(text.length%4!==0)return undefined;
 if(text.length===0)return 0;
 const padding=text.endsWith("==")?2:text.endsWith("=")?1:0,end=text.length-padding;
 for(let i=0;i<text.length;i++){if(i%1024===0)await artifactSqliteCheckpoint(options,"projectSnapshot",i,text.length);if(i<end?alphabet.indexOf(text[i]!)<0:text[i]!=="=")return undefined;}
 const last=alphabet.indexOf(text[end-1]!);if(padding===2&&(last&15)!==0||padding===1&&(last&3)!==0)return undefined;
 return text.length/4*3-padding;
}
type Tables={bitmap:string;pixel:string;literal:string};
/** 📤️ Projects byte occurrences directly and keeps intermediate text in a separate literal branch. */
export async function projectBitmapPixels(p:ArtifactSqliteProjection,t:Tables,id:bigint,width:bigint,height:bigint,text:string,o:ArtifactSqliteOptions):Promise<void>{
 const count=await canonicalIndexCount(text,o);
 await p.insert(t.bitmap,[width,height,count===undefined?"literal":"indices"],id);
 if(count===undefined){let ordinal=0;for(const scalar of text){await p.insert(t.literal,[id,BigInt(ordinal++),scalar]);}return;}
 p.checkRowsAdditional(count);
 let ordinal=0;
 for(let offset=0;offset<text.length;offset+=4){
  const a=alphabet.indexOf(text[offset]!),b=alphabet.indexOf(text[offset+1]!),c=text[offset+2]==="="?0:alphabet.indexOf(text[offset+2]!),d=text[offset+3]==="="?0:alphabet.indexOf(text[offset+3]!);
  const word=a*262144+b*4096+c*64+d;
  for(let shift=16;shift>=0;shift-=8){if(ordinal===count)break;await p.insert(t.pixel,[id,BigInt(ordinal++),BigInt((word>>>shift)&255)]);}
 }
}
/** 📥️ One semantic branch is the sole authority for each bitmap's native pixel field. */
export async function reconstructBitmapPixels(bitmap:SqliteRow,pixels:readonly SqliteRow[],literals:readonly SqliteRow[],o:ArtifactSqliteOptions):Promise<string>{
 const mode=artifactSqliteText(bitmap,3),rows=await artifactSqliteOrderedRowsControlled(mode==="indices"?pixels:literals,2,o);
 if(mode!=="indices"&&mode!=="literal")throw Error("bitmap storage branch differs");
 if(mode==="indices"&&literals.length||mode==="literal"&&pixels.length)throw Error("bitmap storage branches conflict");
 let result="";
 if(mode==="literal"){for(let i=0;i<rows.length;i++){const scalar=artifactSqliteText(rows[i]!,3);const first=scalar.codePointAt(0);if(first===undefined||scalar.length!==(first>65535?2:1))throw Error("bitmap literal requires exactly one Unicode scalar");result+=scalar;if(i%256===0)await artifactSqliteCheckpoint(o,"reconstructSnapshot",i,rows.length);}return result;}
 for(let offset=0;offset<rows.length;offset+=3){
  const count=Math.min(3,rows.length-offset);let word=0;
  for(let j=0;j<count;j++){const value=artifactSqliteInteger(rows[offset+j]!,3);if(value<0n||value>255n)throw Error("bitmap palette index exceeds unsigned byte");word|=Number(value)<<(16-j*8);}
  result+=alphabet[(word>>>18)&63]!+alphabet[(word>>>12)&63]!+(count>1?alphabet[(word>>>6)&63]!:"=")+(count>2?alphabet[word&63]!:"=");
  if(offset%256===0)await artifactSqliteCheckpoint(o,"reconstructSnapshot",offset,rows.length);
 }return result;
}
