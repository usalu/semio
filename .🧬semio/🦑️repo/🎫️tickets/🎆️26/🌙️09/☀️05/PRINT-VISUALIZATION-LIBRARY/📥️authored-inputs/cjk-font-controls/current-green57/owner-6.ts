/** 🧪️ Verifies source-bound font schema, shaping and advances using independent native Canvas. */
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {createCanvas,GlobalFonts} from "@napi-rs/canvas";
import Ajv2020 from "ajv/dist/2020.js";
import {measurePrintSans,printSansGlyphs,printSansMetrics,printSansRuns,printFontFamily,printFontTexSelector} from "../🟦️.ts";
import fallback from "../🌏️.json";
import schema from "../🧬️schema/🔣️.json";
import vectors from "./🔣️.json";
export type FontMetricsCheck={name:string;subject:()=>unknown;oracle:()=>unknown;tolerance?:number};
const fontPath=join(import.meta.dir,"../../../../🖼️assets/🔤️font/🅰️anta/🅰️Anta-Regular.ttf");
const fallbackPath=join(import.meta.dir,"../../../../🖼️assets/🔤️font/🌏️noto-sans-cjk-sc/🌏️NotoSansCJKsc-Regular.otf");
let context:ReturnType<ReturnType<typeof createCanvas>["getContext"]>|undefined;
function canvasWidth(text:string,size:number,cjk=false):number{
  if(context===undefined){if(!GlobalFonts.registerFromPath(fontPath,"SemioSansMetricsOracle")||!GlobalFonts.registerFromPath(fallbackPath,"SemioCjkMetricsOracle"))throw new Error("tracked font registration failed");context=createCanvas(1,1).getContext("2d");}
  context.font="2048px "+(cjk?"SemioCjkMetricsOracle":"SemioSansMetricsOracle");
  return context.measureText(text).width/2048*size*25.4/72.27;
}
/** 🌏️ Independently segments the neutral Unicode blocks before third-party font shaping. */
function canvasFallbackWidth(text:string,size:number):number{
  const runs:{text:string;cjk:boolean}[]=[];
  for(const character of text){const cp=character.codePointAt(0)!,cjk=vectors.fallback.unicodeRanges.some(([start,end])=>cp>=start!&&cp<=end!),last=runs.at(-1);if(last?.cjk===cjk)last.text+=character;else runs.push({text:character,cjk});}
  return runs.reduce((sum,run)=>sum+canvasWidth(run.text,size,run.cjk),0);
}
/** 📏️ Provides language-neutral advance vectors and schema/digest assertions. */
export function printFontMetricsChecks():FontMetricsCheck[]{
  const metrics=printSansMetrics();
  const checks:FontMetricsCheck[]=[
    {name:"font-metrics/source-digest",subject:()=>metrics.sha256,oracle:()=>createHash("sha256").update(readFileSync(fontPath)).digest("hex")},
    {name:"font-metrics/schema",subject:()=>new Ajv2020({strict:false}).compile(schema)(metrics),oracle:()=>true},
    {name:"font-metrics/ligature",subject:()=>printSansGlyphs("fi").length,oracle:()=>1},
    {name:"font-metrics/cjk-source-digest",subject:()=>fallback.sha256,oracle:()=>createHash("sha256").update(readFileSync(fallbackPath)).digest("hex")},
    {name:"font-metrics/cjk-schema",subject:()=>new Ajv2020({strict:false}).compile(schema)(fallback),oracle:()=>true},
    {name:"font-metrics/cjk-ranges",subject:()=>fallback.unicodeRanges,oracle:()=>vectors.fallback.unicodeRanges},
    {name:"font-metrics/cjk-run-payload",subject:()=>printSansRuns("  Root 根 / Leaf 葉  "),oracle:()=>[{family:"Anta",text:"  Root "},{family:"Noto Sans CJK SC",text:"根"},{family:"Anta",text:" / Leaf "},{family:"Noto Sans CJK SC",text:"葉"},{family:"Anta",text:"  "}]},
    {name:"font-metrics/cjk-family",subject:()=>printFontFamily("SemioCJK"),oracle:()=>fallback.family},
    {name:"font-metrics/cjk-selector",subject:()=>printFontTexSelector(fallback.family),oracle:()=>"SemioCJK"},
    ...vectors.fallback.sizes.flatMap(size=>vectors.fallback.cases.map(entry=>({name:"font-metrics/cjk/"+entry.language+"/"+entry.text+"/"+size,subject:()=>[measurePrintSans(entry.text,size)],oracle:()=>[canvasFallbackWidth(entry.text,size)],tolerance:1e-4}))),
    ...vectors.sizes.flatMap(size=>vectors.cases.map(entry=>({name:"font-metrics/"+entry.language+"/"+entry.text+"/"+size,subject:()=>[measurePrintSans(entry.text,size)],oracle:()=>[canvasWidth(entry.text,size)],tolerance:1e-4})))
  ];
  return checks;
}
