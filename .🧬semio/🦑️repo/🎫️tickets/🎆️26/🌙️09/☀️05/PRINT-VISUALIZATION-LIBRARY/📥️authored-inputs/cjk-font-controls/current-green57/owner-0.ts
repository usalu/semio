import { BundleScript, ScriptRouter } from "../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { stagePrintFonts } from "./🟦️.ts";

class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Print font preparation accepts no arguments");
    const result = await stagePrintFonts(this.repoRoot);
    console.log(`Print fonts staged: ${result.total} authored OpenType assets`);
  }
}


import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {join} from "node:path";
import {createHash} from "node:crypto";
import type {PrintFontMetrics,PrintFontPositionTable} from "./📏️metrics/🟦️.ts";

/** 🔎️ Derives only the tracked Anta Latin default shaping and advance tables. */
export function derivePrintSansMetrics(workspace:string):PrintFontMetrics {
  const b=readFileSync(join(workspace,"🧰️framework/🛍️products/📓️print/🖼️assets/🔤️font/🅰️anta/🅰️Anta-Regular.ttf")),tables=new Map<string,number>();
  const u=(p:number)=>b.readUInt16BE(p),s=(p:number)=>b.readInt16BE(p);
  for(let i=0;i<u(4);i++){const p=12+16*i;tables.set(b.toString("ascii",p,p+4),b.readUInt32BE(p+8));}
  const count=u(tables.get("maxp")!+4),unitsPerEm=u(tables.get("head")!+18),horizontal=u(tables.get("hhea")!+34),hmtx=tables.get("hmtx")!;
  const coverage=(p:number):number[]=>{
    if(u(p)===1)return Array.from({length:u(p+2)},(_,i)=>u(p+4+2*i));
    if(u(p)!==2)throw Error("unsupported Anta coverage");
    return Array.from({length:u(p+2)},(_,i)=>{const q=p+4+6*i;return Array.from({length:u(q+2)-u(q)+1},(_,j)=>u(q)+j);}).flat();
  };
  const classes=(p:number):number[]=>{
    const result=Array<number>(count).fill(0);
    if(u(p)===1)for(let i=0;i<u(p+4);i++)result[u(p+2)+i]=u(p+6+2*i);
    else if(u(p)===2)for(let i=0;i<u(p+2);i++){const q=p+4+6*i;for(let gid=u(q);gid<=u(q+2);gid++)result[gid]=u(q+4);}
    else throw Error("unsupported Anta glyph classes");
    return result;
  };
  const cmap=tables.get("cmap")!,encoding=Array.from({length:u(cmap+2)},(_,i)=>cmap+4+i*8).find(p=>u(p)===3&&u(p+2)===1)!;
  const cm=cmap+b.readUInt32BE(encoding+4),segments=u(cm+6)/2,end=cm+14,start=end+2*segments+2,delta=start+2*segments,offset=delta+2*segments;
  if(u(cm)!==4)throw Error("Anta Latin metrics require Unicode cmap 4");
  const characters:Record<string,number>={};
  for(let i=0;i<segments;i++)for(let cp=u(start+2*i);cp<=u(end+2*i)&&cp<65535;cp++){
    const address=offset+2*i,range=u(address),raw=range===0?cp:u(address+range+2*(cp-u(start+2*i))),glyph=(raw+s(delta+2*i))&65535;
    if(raw!==0&&glyph!==0)characters[String(cp)]=glyph;
  }
  const lookup=(name:string,index:number):{base:number;ignoreMarks:boolean;subtables:number[]}=>{
    const table=tables.get(name)!,list=table+u(table+8),base=list+u(list+2+2*index),flags=u(base+2);
    if(flags!==0&&flags!==8)throw Error("unsupported Anta lookup flags");
    return {base,ignoreMarks:flags===8,subtables:Array.from({length:u(base+4)},(_,i)=>base+u(base+6+2*i))};
  };
  const substitution=(index:number):PrintFontMetrics["substitutions"][number]=>{
    const record=lookup("GSUB",index),kind=u(record.base),rules:{input:number[];output:number[]}[]=[];
    for(const p of record.subtables){
      if(u(p)!==1)throw Error("unsupported Anta substitution");
      const glyphs=coverage(p+u(p+2));
      if(kind===2)glyphs.forEach((glyph,i)=>{const q=p+u(p+6+2*i);rules.push({input:[glyph],output:Array.from({length:u(q)},(_,j)=>u(q+2+2*j))});});
      else if(kind===4)glyphs.forEach((glyph,i)=>{const set=p+u(p+6+2*i);for(let j=0;j<u(set);j++){const q=set+u(set+2+2*j);rules.push({input:[glyph,...Array.from({length:u(q+2)-1},(_,k)=>u(q+4+2*k))],output:[u(q)]});}});
      else throw Error("unsupported Anta Latin substitution lookup");
    }
    return {kind:kind===2?"multiple":"ligature",ignoreMarks:record.ignoreMarks,rules};
  };
  const value=(p:number,format:number):{advance:number;bytes:number}=>{
    let advance=0,bytes=0;for(let bit=0;bit<8;bit++)if(format&(1<<bit)){if(bit===2)advance=s(p+bytes);bytes+=2;}
    return {advance,bytes};
  };
  const positioning=Array.from({length:4},(_,index)=>{
    const record=lookup("GPOS",index);
    if(u(record.base)!==2)throw Error("Anta kerning requires pair positioning");
    const pairs=record.subtables.map((p):PrintFontPositionTable=>{
      const covered=coverage(p+u(p+2)),firstFormat=u(p+4),secondFormat=u(p+6);
      if(u(p)===1){
        const result:number[][]=[];
        covered.forEach((first,i)=>{const set=p+u(p+10+2*i);let q=set+2;for(let j=0;j<u(set);j++){const second=u(q),a=value(q+2,firstFormat),z=value(q+2+a.bytes,secondFormat);result.push([first,second,a.advance+z.advance]);q+=2+a.bytes+z.bytes;}});
        return {coverage:covered,pairs:result};
      }
      if(u(p)!==2)throw Error("unsupported Anta pair format");
      let q=p+16;
      const adjustments=Array.from({length:u(p+12)},()=>Array.from({length:u(p+14)},()=>{const a=value(q,firstFormat),z=value(q+a.bytes,secondFormat);q+=a.bytes+z.bytes;return a.advance+z.advance;}));
      return {coverage:covered,firstClasses:classes(p+u(p+8)),secondClasses:classes(p+u(p+10)),adjustments};
    });
    return {ignoreMarks:record.ignoreMarks,tables:pairs};
  });
  const gdef=tables.get("GDEF")!;
  return {family:"Anta",sha256:createHash("sha256").update(b).digest("hex"),unitsPerEm,characters,advances:Array.from({length:count},(_,gid)=>u(hmtx+4*Math.min(gid,horizontal-1))),classes:classes(gdef+u(gdef+4)),substitutions:[substitution(2),substitution(13)],positioning};
}
/** 🌏️ Derives checked Unicode coverage and horizontal advances from the tracked CJK OpenType font. */
export function derivePrintCjkMetrics(workspace:string):PrintFontMetrics {
  const bytes=readFileSync(join(workspace,"🧰️framework/🛍️products/📓️print/🖼️assets/🔤️font/🌏️noto-sans-cjk-sc/🌏️NotoSansCJKsc-Regular.otf")),tables=new Map<string,{offset:number;length:number}>();
  const check=(offset:number,length:number,limit=bytes.length)=>{if(!Number.isSafeInteger(offset)||!Number.isSafeInteger(length)||offset<0||length<0||offset+length>limit)throw Error("CJK font table bounds");};
  const u16=(offset:number,limit=bytes.length)=>{check(offset,2,limit);return bytes.readUInt16BE(offset);},u32=(offset:number,limit=bytes.length)=>{check(offset,4,limit);return bytes.readUInt32BE(offset);};
  if(u32(0)!==0x4f54544f)throw Error("CJK metrics require tracked OpenType CFF");
  check(12,u16(4)*16);
  for(let index=0;index<u16(4);index++){const at=12+index*16,name=bytes.toString("ascii",at,at+4),offset=u32(at+8),length=u32(at+12);check(offset,length);if(tables.has(name))throw Error("Duplicate CJK font table");tables.set(name,{offset,length});}
  const table=(name:string,minimum:number)=>{const value=tables.get(name);if(!value||value.length<minimum)throw Error("Missing CJK font table "+name);return value;};
  const maxp=table("maxp",6),head=table("head",20),hhea=table("hhea",36),count=u16(maxp.offset+4),unitsPerEm=u16(head.offset+18),horizontal=u16(hhea.offset+34),hmtx=table("hmtx",4*horizontal+2*(count-horizontal));
  if(!count||!unitsPerEm||!horizontal||horizontal>count)throw Error("Invalid CJK horizontal metrics");
  const cmap=table("cmap",4),limit=cmap.offset+cmap.length,records=u16(cmap.offset+2);check(cmap.offset+4,records*8,limit);
  const mappings=Array.from({length:records},(_,index)=>{const at=cmap.offset+4+index*8,platform=u16(at,limit),encoding=u16(at+2,limit),offset=cmap.offset+u32(at+4,limit);return{platform,encoding,offset,format:u16(offset,limit)};}).filter(record=>(record.platform===0||record.platform===3&&[1,10].includes(record.encoding))&&[4,12].includes(record.format)).sort((a,b)=>b.format-a.format),mapping=mappings[0],characters:Record<string,number>={};
  if(!mapping)throw Error("Missing CJK Unicode cmap");
  const assign=(codepoint:number,glyph:number)=>{if(codepoint>0x10ffff||glyph>=count)throw Error("Invalid CJK Unicode glyph");if(glyph)characters[String(codepoint)]=glyph;};
  if(mapping.format===12){const at=mapping.offset,length=u32(at+4,limit),end=at+length,groups=u32(at+12,limit);check(at,length,limit);check(at+16,groups*12,end);let previous=-1;for(let index=0;index<groups;index++){const p=at+16+index*12,start=u32(p,end),stop=u32(p+4,end),glyph=u32(p+8,end);if(start<=previous||stop<start||stop>0x10ffff||glyph+stop-start>=count)throw Error("Invalid CJK cmap group");for(let cp=start;cp<=stop;cp++)assign(cp,glyph+cp-start);previous=stop;}}
  else {const at=mapping.offset,length=u16(at+2,limit),end=at+length,twice=u16(at+6,limit),segments=twice/2;check(at,length,limit);if(twice%2||!segments)throw Error("Invalid CJK cmap segments");check(at+14,segments*8+2,end);const ends=at+14,starts=ends+2*segments+2,deltas=starts+2*segments,ranges=deltas+2*segments;for(let index=0;index<segments;index++){const start=u16(starts+2*index,end),stop=u16(ends+2*index,end),delta=u16(deltas+2*index,end),address=ranges+2*index,range=u16(address,end);if(stop<start)throw Error("Invalid CJK cmap range");for(let cp=start;cp<=stop&&cp<0xffff;cp++){const raw=range?u16(address+range+2*(cp-start),end):cp;assign(cp,range&&raw===0?0:(raw+delta)&0xffff);}}}
  return {family:"Noto Sans CJK SC",sha256:createHash("sha256").update(bytes).digest("hex"),unitsPerEm,unicodeRanges:[[4352,4607],[11904,12351],[12352,12799],[13312,19903],[19968,40959],[43360,43391],[44032,55295],[63744,64255],[65072,65103],[65280,65519],[131072,195103],[196608,205743]],characters,advances:Array.from({length:count},(_,glyph)=>u16(hmtx.offset+4*Math.min(glyph,horizontal-1),hmtx.offset+hmtx.length)),classes:[],substitutions:[],positioning:[]};
}
class MetricsScript extends BundleScript {
  async run(args:string[]):Promise<void>{
    if(args.length)throw Error("Font metrics derivation accepts no arguments");
    const output=join(import.meta.dir,"📏️metrics","🔣️.json");mkdirSync(join(import.meta.dir,"📏️metrics"),{recursive:true});
    writeFileSync(output,JSON.stringify(derivePrintSansMetrics(this.repoRoot),null,2)+"\n");
    writeFileSync(join(import.meta.dir,"📏️metrics","🌏️.json"),JSON.stringify(derivePrintCjkMetrics(this.repoRoot))+"\n");
    console.log("[print-fonts] source-bound primary/CJK metrics derived");
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("metrics", MetricsScript);
if (import.meta.main) await router.run(process.argv.slice(2));
