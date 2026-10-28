import fs from 'node:fs/promises';
import path from 'node:path';
import {createHash} from 'node:crypto';
export async function build(root,ticket){
 const classes=["AI","AK","AL","AP","AS","B2","BA","BB","BK","CB","CJ","CL","CM","CP","CR","EB","EM","EX","GL","H2","H3","HL","HY","ID","IN","IS","JL","JT","JV","LF","NL","NS","NU","OP","PO","PR","QU","RI","SA","SG","SP","SY","VF","VI","WJ","XX","ZW","ZWJ"];
 const ids=new Map(classes.map((name,index)=>[name,index]));
 const primary=await fs.readFile(path.join(ticket,'LineBreak-16.0.0.txt'),'utf8'),east=await fs.readFile(path.join(ticket,'EastAsianWidth-16.0.0.txt'),'utf8');
 const values=new Uint8Array(0x110000).fill(ids.get('XX')),widths=new Uint8Array(0x110000);
 const ideographs=[[0x3400,0x4dbf],[0x4e00,0x9fff],[0xf900,0xfaff],[0x20000,0x2fffd],[0x30000,0x3fffd]];
 for(const[first,last]of ideographs){values.fill(ids.get('ID'),first,last+1);widths.fill(1,first,last+1);}
 for(const[first,last]of[[0x1f000,0x1faff],[0x1fc00,0x1fffd]])values.fill(ids.get('ID'),first,last+1);
 values.fill(ids.get('PR'),0x20a0,0x20d0);
 const apply=(source,target,convert)=>{for(const line of source.split('\n')){const body=line.split('#')[0].trim();if(!body)continue;const[range,property]=body.split(';').map(value=>value.trim());const[firstText,lastText]=range.split('..'),first=parseInt(firstText,16),last=parseInt(lastText??firstText,16);target.fill(convert(property),first,last+1);}};
 apply(primary,values,name=>{if(!ids.has(name))throw Error('Unknown line class '+name);return ids.get(name);});
 apply(east,widths,name=>['F','W','H'].includes(name)?1:0);
 const ranges=[];let start=0;for(let at=1;at<=values.length;at++){if(at<values.length&&values[at]===values[start]&&widths[at]===widths[start])continue;ranges.push([start,at-1,values[start],widths[start]]);start=at;}
 const bytes=Buffer.alloc(24+ranges.length*12);bytes.writeUInt32BE(0x554c424b,0);bytes.writeUInt32BE(0x00100000,4);bytes.writeUInt32BE(bytes.length,8);bytes.writeUInt32BE(ranges.length,12);bytes.writeUInt32BE(12,16);
 ranges.forEach(([first,last,kind,wide],index)=>{const at=24+index*12;bytes.writeUInt32BE(first,at);bytes.writeUInt32BE(last,at+4);bytes[at+8]=kind;bytes[at+9]=wide;});
 const owner=path.join(root,'🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/📦️data');await fs.mkdir(owner,{recursive:true});await fs.writeFile(path.join(owner,'🔢️line-break.bin'),bytes);
 const sources=[['LineBreak-16.0.0.txt','https://www.unicode.org/Public/16.0.0/ucd/LineBreak.txt',primary],['EastAsianWidth-16.0.0.txt','https://www.unicode.org/Public/16.0.0/ucd/EastAsianWidth.txt',east]].map(([name,url,text])=>({url,sha256:createHash('sha256').update(text).digest('hex'),bytes:Buffer.byteLength(text)}));
 await fs.writeFile(path.join(owner,'🔣️provenance.json'),JSON.stringify({license:'https://www.unicode.org/license.txt',version:'16.0.0',byteOrder:'big-endian',headerBytes:24,rangeBytes:12,classes,sources},null,2)+'\n');
 return{ranges:ranges.length,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};
}
