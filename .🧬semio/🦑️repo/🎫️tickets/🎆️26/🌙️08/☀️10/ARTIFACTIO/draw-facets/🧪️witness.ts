import {test,expect} from "bun:test";
import {PNG} from "pngjs";
import {writeFileSync,mkdirSync} from "node:fs";
import {join} from "node:path";
import inputs from "./asset-inputs.json";
const component=(text:string):Uint8Array=>{const parts:number[]=[];for(let i=0;i<text.length;i++){if(text[i]==="%"){const hex=text.slice(i+1,i+3);if(!/^[0-9a-fA-F]{2}$/.test(hex))throw Error("Invalid percent octet");parts.push(parseInt(hex,16));i+=2;}else{const code=text.charCodeAt(i);if(code>127)throw Error("Non-ASCII octet");parts.push(code);}}return new Uint8Array(parts)};
test("independent pngjs observes canonical sample inputs",()=>{
 const output=[];
 for(const item of inputs){try{
  let payload=item.asset.data;let bytes:Uint8Array;
  if(payload.toLowerCase().startsWith("data:")){const comma=payload.indexOf(",");if(comma<0)throw Error("Missing data delimiter");const header=payload.slice(0,comma),octets=component(payload.slice(comma+1));bytes=header.toLowerCase().endsWith(";base64")?new Uint8Array(Buffer.from(Buffer.from(octets).toString("ascii"),"base64")):octets;}
  else bytes=new Uint8Array(Buffer.from(payload,"base64"));
  const png=PNG.sync.read(Buffer.from(bytes)),samples:[number,number,number,number][]=[];for(let index=0;index<png.data.length;index+=4)samples.push([png.data[index]!,png.data[index+1]!,png.data[index+2]!,png.data[index+3]!]);
  expect(samples.length).toBe(png.width*png.height);output.push({...item,observed:{width:png.width,height:png.height,samples}});
 }catch(error){output.push({...item,refusal:String(error)});}}
 const generated=join(import.meta.dir,"../🗑️generated/draw-facets");mkdirSync(generated,{recursive:true});writeFileSync(join(generated,"png-observations.json"),JSON.stringify(output));
 const observed=output.filter(item=>"observed"in item);expect(observed.length).toBeGreaterThan(20);expect(observed.some(item=>item.dsl)).toBe(true);console.log("[DEBUG] independent PNG assets",observed.length,"refused",output.length-observed.length);
});

test("canonical fixture samples equal observed independent PNG pixels",()=>{
 const observed=JSON.parse(require("node:fs").readFileSync(join(import.meta.dir,"../🗑️generated/draw-facets/png-observations.json"),"utf8"));let actual=0;
 for(const row of observed){if(!row.observed||row.dsl||row.file.includes("⚠️invalid")||row.file.includes("🚪️io/"))continue;
  const document=JSON.parse(require("node:fs").readFileSync(row.file,"utf8"));let value=document;for(const key of row.pointer)value=value[key];
  const model=row.observed,expected=value.image?{id:row.asset.id,image:{width:model.width,height:model.height,pixels:model.samples.flat()}}:model;
  expect(value).toEqual(expected);actual++;
 }
 const emblem=observed.find((row:any)=>row.dsl).observed,text=require("node:fs").readFileSync(observed.find((row:any)=>row.dsl).file,"utf8");
 expect(text).not.toContain("mime=");expect(text).toContain("width="+emblem.width+" height="+emblem.height+" samples=[");
 const sampleText=text.slice(text.indexOf("samples=[ ")+10,text.lastIndexOf(" ]}")),parts=sampleText.match(/\d+/g)!.map(Number);expect(parts).toEqual(emblem.samples.flat());
 console.log("[DEBUG] canonical PNG fixture tuples",actual,"DSL emblem samples",emblem.samples.length);
});
