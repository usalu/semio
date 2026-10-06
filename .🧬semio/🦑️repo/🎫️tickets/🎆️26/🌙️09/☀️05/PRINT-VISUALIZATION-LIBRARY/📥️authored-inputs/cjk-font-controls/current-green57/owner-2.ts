/** 📏️ Source-bound SemioSans plain-text advances, independent of filesystem and drawing backends. */
import authored from "./🔣️.json";
import cjkAuthored from "./🌏️.json";
import catalog from "../🔣️.json";
export type PrintFontPositionTable = {readonly coverage:readonly number[];readonly pairs:readonly (readonly number[])[]} | {readonly coverage:readonly number[];readonly firstClasses:readonly number[];readonly secondClasses:readonly number[];readonly adjustments:readonly (readonly number[])[]};
export type PrintFontMetrics = {readonly family:string;readonly sha256:string;readonly unitsPerEm:number;readonly unicodeRanges?:readonly (readonly [number,number])[];readonly characters:Readonly<Record<string,number>>;readonly advances:readonly number[];readonly classes:readonly number[];readonly substitutions:readonly {readonly kind:"multiple"|"ligature";readonly ignoreMarks:boolean;readonly rules:readonly {readonly input:readonly number[];readonly output:readonly number[]}[]}[];readonly positioning:readonly {readonly ignoreMarks:boolean;readonly tables:readonly PrintFontPositionTable[]}[]};
const metrics=authored as unknown as PrintFontMetrics;
const cjkMetrics=cjkAuthored as unknown as PrintFontMetrics;
export type PrintFontRun={readonly family:string;readonly text:string};
const positioning=metrics.positioning.map(lookup=>({ignoreMarks:lookup.ignoreMarks,tables:lookup.tables.map(table=>({table,coverage:new Set(table.coverage),pairs:"pairs" in table?new Map(table.pairs.map(([first,second,advance])=>[first+"/"+second,advance!])):undefined}))}));

/** 🔤️ Returns the owned immutable metrics contract and tracked font digest. */
export function printSansMetrics():PrintFontMetrics{return metrics;}
/** 🌏️ Returns source-bound CJK coverage and horizontal advances. */
export function printCjkMetrics():PrintFontMetrics{return cjkMetrics;}
/** 🧵️ Selects contiguous font runs without changing authored Unicode or spaces. */
export function printSansRuns(text:string):readonly PrintFontRun[]{
  const runs:{family:string;text:string}[]=[];
  for(const character of text){const cp=character.codePointAt(0)!,family=cjkMetrics.unicodeRanges!.some(([start,end])=>cp>=start&&cp<=end)?cjkMetrics.family:metrics.family,last=runs.at(-1);if(last?.family===family)last.text+=character;else runs.push({family,text:character});}
  return runs;
}

/** 🧩️ Shapes en/de Latin default ccmp/liga substitutions with mark-aware matching. */
export function printSansGlyphs(text:string):readonly number[]{
  const glyphs=Array.from(text,character=>metrics.characters[String(character.codePointAt(0))]??0);
  for(const lookup of metrics.substitutions){
    for(let index=0;index<glyphs.length;index++){
      if(lookup.ignoreMarks&&metrics.classes[glyphs[index]!]===3)continue;
      for(const rule of lookup.rules){
        if(glyphs[index]!==rule.input[0])continue;
        const matched=[index];let next=index+1;
        for(const component of rule.input.slice(1)){
          while(lookup.ignoreMarks&&metrics.classes[glyphs[next]!]===3)next++;
          if(glyphs[next]!==component)break;matched.push(next++);
        }
        if(matched.length!==rule.input.length)continue;
        for(const position of matched.slice(1).reverse())glyphs.splice(position,1);
        glyphs.splice(index,1,...rule.output);index+=rule.output.length-1;break;
      }
    }
  }
  return glyphs;
}

/** 📐️ Measures plain SemioSans text in millimetres at an authored TeX-point size. */
function measurePrintPrimary(text:string,size:number):number{
  if(!Number.isFinite(size)||size<0)throw new Error("font size requires nonnegative finite TeX points");
  const glyphs=printSansGlyphs(text);let advance=glyphs.reduce((sum,glyph)=>sum+(metrics.advances[glyph]??metrics.advances[0]??0),0);
  for(const lookup of positioning){
    for(let index=0;index<glyphs.length-1;index++){
      const first=glyphs[index]!;if(lookup.ignoreMarks&&metrics.classes[first]===3)continue;
      let next=index+1;while(lookup.ignoreMarks&&metrics.classes[glyphs[next]!]===3)next++;
      const second=glyphs[next];if(second===undefined)continue;
      for(const {table,coverage,pairs} of lookup.tables){
        if(!coverage.has(first))continue;
        if(pairs){const value=pairs.get(first+"/"+second);if(value===undefined)continue;advance+=value;}
        else if("adjustments" in table)advance+=table.adjustments[table.firstClasses[first]??0]?.[table.secondClasses[second]??0]??0;
        break;
      }
    }
  }
  return advance/metrics.unitsPerEm*size*25.4/72.27;
}

/** 📐️ Measures primary shaping and source-bound CJK runs at authored TeX-point sizes. */
export function measurePrintSans(text:string,size:number):number{
  if(!Number.isFinite(size)||size<0)throw new Error("font size requires nonnegative finite TeX points");
  return printSansRuns(text).reduce((sum,run)=>sum+(run.family===metrics.family?measurePrintPrimary(run.text,size):Array.from(run.text,character=>cjkMetrics.advances[cjkMetrics.characters[String(character.codePointAt(0))]??0]??0).reduce((total,advance)=>total+advance,0)/cjkMetrics.unitsPerEm*size*25.4/72.27),0);
}

/** 🔤️ Resolves native print selectors through the tracked pure font catalog. */
export function printFontFamily(selector:string):string {const filename={SemioSans:"Anta-Regular.ttf",SemioSerif:"Anta-Regular.ttf",SemioMono:"ShareTechMono-Regular.ttf",SemioEmoji:"NotoEmoji-Regular.ttf",SemioCJK:"NotoSansCJKsc-Regular.otf"}[selector as "SemioSans"|"SemioSerif"|"SemioMono"|"SemioEmoji"|"SemioCJK"];const font=catalog.find(entry=>entry.texFilename===filename);if(font===undefined)throw new Error("unknown print font selector "+selector);return font.family;}
/** 🖋️ Resolves tracked families to native selectors, retaining explicit authored family names. */
export function printFontTexSelector(family:string):string|undefined {return {"Anta":"SemioSans","Share Tech Mono":"SemioMono","Noto Emoji":"SemioEmoji","Noto Sans CJK SC":"SemioCJK"}[family as "Anta"|"Share Tech Mono"|"Noto Emoji"|"Noto Sans CJK SC"];}
