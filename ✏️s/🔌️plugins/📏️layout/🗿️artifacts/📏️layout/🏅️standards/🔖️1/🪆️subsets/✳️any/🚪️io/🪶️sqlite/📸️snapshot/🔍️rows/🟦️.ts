/** 🔍️ Full bigint relation lookup uses concrete caller-owned sorted row positions. */
import {ValueError,type SqliteOperation,type SqliteRow} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {artifactSqliteCheckpoint} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
function invalid(message:string):never{throw new ValueError("invalidValue","Layout "+message)}
function words(operation:SqliteOperation,length:number):Uint32Array{if(!Number.isSafeInteger(length)||length<0||length>0xffffffff)throw new ValueError("workLimit","Layout row lookup extent");const bytes=operation.allocateBytes(length*4);return new Uint32Array(bytes.buffer,bytes.byteOffset,length)}
/** 📍️ Sorts the actual row positions and resolves complete literal identities without deletion maps. */
export class LayoutRows{
 private constructor(private readonly names:readonly string[],private readonly rows:readonly (readonly SqliteRow[])[],private readonly offsets:Uint32Array,private readonly positions:Uint32Array){}
 static async create(names:readonly string[],rows:readonly (readonly SqliteRow[])[],widths:readonly number[],operation:SqliteOperation):Promise<LayoutRows>{
  if(names.length!==rows.length||names.length!==widths.length)throw new ValueError("invariantViolated","Layout row table roles");
  let total=0;for(let table=0;table<rows.length;table++){total+=rows[table]!.length;if(!Number.isSafeInteger(total)||total>0xffffffff||total>(operation.maxRows??1000000))throw new ValueError("workLimit","Layout row lookup limit")}
  const offsets=words(operation,rows.length+1),positions=words(operation,total),lookup=new LayoutRows(names,rows,offsets,positions);let start=0;
  for(let table=0;table<rows.length;table++){
   const entries=rows[table]!,length=entries.length;offsets[table]=start;const forecast=4*(length+1)*Math.max(1,Math.ceil(Math.log2(length+1)));let completed=0;
   const step=async()=>{if(++completed%256===0)await artifactSqliteCheckpoint(operation,"reconstructSnapshot",completed,forecast)};
   const greater=(left:number,right:number)=>entries[positions[start+left]!]!.rowid>entries[positions[start+right]!]!.rowid;
   const swap=(left:number,right:number)=>{const value=positions[start+left]!;positions[start+left]=positions[start+right]!;positions[start+right]=value};
   const sift=async(root:number,end:number)=>{while(root*2+1<end){let child=root*2+1;if(child+1<end&&greater(child+1,child))child++;if(!greater(child,root))break;swap(child,root);root=child;await step()}};
   for(let at=0;at<length;at++){const row=entries[at]!;if(row.rowid<=0n||row.values.length!==widths[table]||row.values[0]!==row.rowid)invalid("entity identity or fields");positions[start+at]=at;await step()}
   for(let root=Math.floor(length/2)-1;root>=0;root--)await sift(root,length);
   for(let end=length-1;end>0;end--){swap(0,end);await sift(0,end);await step()}
   for(let at=1;at<length;at++){if(entries[positions[start+at-1]!]!.rowid===entries[positions[start+at]!]!.rowid)invalid("duplicate entity identity");await step()}
   await artifactSqliteCheckpoint(operation,"reconstructSnapshot",completed,completed);start+=length;
  }
  offsets[rows.length]=total;return lookup;
 }
 /** 🔗️ Binary search compares full authored INTEGER identity, preserving the captured row. */
 peek(name:string,id:bigint):SqliteRow|undefined{
  const table=this.names.indexOf(name);if(table<0)invalid("unknown table");let low=this.offsets[table]!,high=this.offsets[table+1]!;while(low<high){const middle=Math.floor((low+high)/2),row=this.rows[table]![this.positions[middle]!]!;if(row.rowid<id)low=middle+1;else high=middle}if(low===this.offsets[table+1])return;const row=this.rows[table]![this.positions[low]!]!;return row.rowid===id?row:undefined;
 }
}
