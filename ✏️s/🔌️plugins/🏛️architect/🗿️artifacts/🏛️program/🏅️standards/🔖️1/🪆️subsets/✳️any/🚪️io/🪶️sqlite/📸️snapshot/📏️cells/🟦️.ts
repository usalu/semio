/** 📏️ Scalar cell codecs and the row cursor of the Architect SQLite projection. */
import { architectProgramArtifactGuardBinary64, architectProgramArtifactGuardInteger, architectProgramArtifactGuardMember, architectProgramArtifactGuardUnsigned64 } from "../../../../🧬️schema/🟦️.ts";
import type { TaggedNote, TextField, TimestampMeta } from "../../../../🧬️schema/🪪️entity-foundation/🟦️.ts";
import type { SqliteRow, SqliteValue } from "@semio-tech/framework/io/sqlite-snapshot";
import { type Binary64, binary64Value } from "@semio-tech/framework/value/ieee754";

export type Cells=readonly SqliteValue[];
export const fail=(message:string):never=>{throw new Error("Architect SQLite "+message)};
export const text=(value:string):Cells=>typeof value==="string"?[value]:fail("requires TEXT");
export const boolean=(value:boolean):Cells=>typeof value==="boolean"?[value?1n:0n]:fail("requires Boolean");
export const uint32=(value:number):Cells=>[BigInt(architectProgramArtifactGuardInteger(value,"unsigned32",{minimum:0,maximum:4294967295}))];
export const uint64=(value:bigint):Cells=>{const word=architectProgramArtifactGuardUnsigned64(value,"unsigned64");return[word.toString(),BigInt.asIntN(64,word)]};
export function float(value:Binary64):Cells{const word=architectProgramArtifactGuardBinary64(value,"binary64").bits,query=binary64Value(value),kind=Number.isNaN(query)?"nan":query===Infinity?"positiveInfinity":query===-Infinity?"negativeInfinity":"finite";return[Number.isNaN(query)?null:query,BigInt.asIntN(64,word),kind]}
export const optional=<T>(value:T|null,width:number,encode:(value:T)=>Cells):Cells=>value===null?Array<null>(width).fill(null):encode(value);
export const textField=(value:TextField):Cells=>[...text(value.text),...optional(value.format??null,1,text)];
export const optionalTextField=(value:TextField|null):Cells=>value===null?[0n,null,null]:[1n,...textField(value)];
export const note=(value:TaggedNote):Cells=>[...text(value.tag),...text(value.text)];
export const symbol=<T extends string>(value:T,members:readonly T[]):Cells=>text(architectProgramArtifactGuardMember(value,"enum",members));
export const timestamp=(value:TimestampMeta):Cells=>[...text(value.created),...text(value.updated),...optional(value.createdBy??null,1,text),...optional(value.updatedBy??null,1,text)];
export class Cursor{
 private index:number;
 constructor(private readonly row:SqliteRow,index:number=1){this.index=index}
 next():SqliteValue{if(this.index>=this.row.values.length)return fail("scalar is absent");return this.row.values[this.index++]!}
 text():string{const value=this.next();return typeof value==="string"?value:fail("requires TEXT")}
 integer():bigint{const value=this.next();return typeof value==="bigint"?value:fail("requires INTEGER")}
 boolean():boolean{const value=this.integer();return value===0n?false:value===1n?true:fail("noncanonical Boolean")}
 uint32():number{const value=this.integer();return value>=0n&&value<=4294967295n?Number(value):fail("unsigned32 exceeds domain")}
 uint64():bigint{const decimal=this.text(),word=BigInt.asUintN(64,this.integer());return decimal===word.toString()?word:fail("unsigned64 decimal and word differ")}
 float():Binary64{const query=this.next(),word=BigInt.asUintN(64,this.integer()),kind=this.text(),value={bits:word},number=binary64Value(value),expected=Number.isNaN(number)?"nan":number===Infinity?"positiveInfinity":number===-Infinity?"negativeInfinity":"finite";if(kind!==expected)return fail("binary64 class differs");if(typeof query==="bigint"){if(!Number.isFinite(number)||!Number.isInteger(number)||BigInt(number)!==query)return fail("binary64 query INTEGER differs")}else if(typeof query==="number"){if(Number.isNaN(number)||query!==number)return fail("binary64 query REAL differs")}else if(query!==null||!Number.isNaN(number))return fail("binary64 query differs");return value}
 optional<T>(width:number,decode:()=>T):T|null{const end=this.index+width;if(end>this.row.values.length)return fail("optional scalar is absent");if(this.row.values.slice(this.index,end).every(value=>value===null)){this.index=end;return null}return decode()}
 textField():TextField{const value={text:this.text()},format=this.optional(1,()=>this.text());return format===null?value:{...value,format}}
 optionalTextField():TextField|null{if(this.boolean())return this.textField();if(this.next()!==null||this.next()!==null)return fail("absent text owns a payload");return null}
 note():TaggedNote{return{tag:this.text(),text:this.text()}}
 symbol<T extends string>(members:readonly T[]):T{return architectProgramArtifactGuardMember(this.text(),"enum",members)}
 timestamp():TimestampMeta{const created=this.text(),updated=this.text(),createdBy=this.optional(1,()=>this.text()),updatedBy=this.optional(1,()=>this.text());return{created,updated,...createdBy===null?{}:{createdBy},...updatedBy===null?{}:{updatedBy}}}
 end():void{if(this.index!==this.row.values.length)fail("unused scalar columns")}
}
