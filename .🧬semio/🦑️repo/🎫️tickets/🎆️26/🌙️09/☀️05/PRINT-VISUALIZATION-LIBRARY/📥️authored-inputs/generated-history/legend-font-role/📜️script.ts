import {readFileSync,writeFileSync,renameSync} from 'node:fs';
const product='C:/git/semio/🧰️framework/🛍️products/📓️print/',temporary=import.meta.dir+'/source.tmp';
function edit(path:string,change:(source:string)=>string){const file=product+path;writeFileSync(temporary,change(readFileSync(file,'utf8')));renameSync(temporary,file);}
edit('🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts',source=>source.replace('import authored from "./🔣️.json";','import authored from "./🔣️.json";\nimport catalog from "../🔣️.json";')+`
/** 🔤️ Resolves native print selectors through the tracked pure font catalog. */
export function printFontFamily(selector:string):string {const filename={SemioSans:"Anta-Regular.ttf",SemioSerif:"Anta-Regular.ttf",SemioMono:"ShareTechMono-Regular.ttf",SemioEmoji:"NotoEmoji-Regular.ttf"}[selector as "SemioSans"|"SemioSerif"|"SemioMono"|"SemioEmoji"];const font=catalog.find(entry=>entry.texFilename===filename);if(font===undefined)throw new Error("unknown print font selector "+selector);return font.family;}
/** 🖋️ Resolves tracked families to native selectors, retaining explicit authored family names. */
export function printFontTexSelector(family:string):string|undefined {return {"Anta":"SemioSans","Share Tech Mono":"SemioMono","Noto Emoji":"SemioEmoji"}[family as "Anta"|"Share Tech Mono"|"Noto Emoji"];}
`);
edit('🧬️schema/💡️inferences/🖼️render/🟦️.ts',source=>{
  source=source.replace('import { measurePrintSans }','import { measurePrintSans, printFontFamily, printFontTexSelector }');
  source=source.replace('readonly content: string; readonly size: number; readonly fill?','readonly content: string; readonly size: number; readonly font?: string; readonly fill?');
  source=source.replace('size:Number(o.titleSize??d.titleSize),fill:textFill','size:Number(o.titleSize??d.titleSize),font:printFontFamily(d.titleFont),fill:textFill');
  source=source.replaceAll('content:label(value),size,fill:textFill','content:label(value),size,font:printFontFamily(d.labelFont),fill:textFill').replaceAll('content,size,fill:textFill','content,size,font:printFontFamily(d.labelFont),fill:textFill');
  source=source.replace('content:item.content,size:item.size*25.4/72.27,anchor:','content:item.content,size:item.size*25.4/72.27,...(item.font===undefined?{}:{font:item.font}),anchor:');
  source=source.replace('font=\\\\fontsize{${tikzNumber(item.size)}}','font=${item.font===undefined?"":printFontTexSelector(item.font)?"\\\\"+printFontTexSelector(item.font):"\\\\fontspec{"+tikzText(item.font)+"}"}\\\\fontsize{${tikzNumber(item.size)}}');
  source=source.replace('theme:VizTheme,palette:readonly string[]):VizRenderItem[]','theme:VizTheme,palette:readonly string[],checkpoint:()=>void):VizRenderItem[]').replace('for(let slice=0;slice<48;slice++){const fraction','for(let slice=0;slice<48;slice++){checkpoint();const fraction').replace('    const value=values[index],content=label(value)','    checkpoint();const value=values[index],content=label(value)').replace('legendLabel,title,theme,palette));','legendLabel,title,theme,palette,checkpoint));');
  return source;
});
edit('🧪️tests/🎬️render-scene/🧭️legend/🟦️.ts',source=>source.replace('content:item.content,size:item.size,anchor:','content:item.content,size:item.size,font:item.font,anchor:').replace('size:number;anchor:string;baseline:string','size:number;font:string;anchor:string;baseline:string').replace('content:title,size:Number(o.titleSize??7.2),anchor:','content:title,size:Number(o.titleSize??7.2),font:"Share Tech Mono",anchor:').replaceAll('content:label(value),size,anchor:','content:label(value),size,font:"Anta",anchor:').replaceAll('content,size,anchor:','content,size,font:"Anta",anchor:'));
