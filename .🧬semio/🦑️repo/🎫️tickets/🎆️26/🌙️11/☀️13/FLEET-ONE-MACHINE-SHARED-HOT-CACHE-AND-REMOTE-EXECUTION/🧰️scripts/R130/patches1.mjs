import {readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
const here=dirname(fileURLToPath(import.meta.url));
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const G=B+"🕸️graph/🕰️generations/";
export const PATCHES=[
 {step:1,file:G+"🔌️load/🟨️.mjs",edits:[
  ['import{closureMissingErrorV1}from"../../🩹️closure-repair/🟨️.mjs";','import{closureMissingErrorV1}from"../../🩹️closure-repair/🟨️.mjs";\nimport{runtimeCollectGarbageV1}from"../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";'],
  {between:["/**\n * 🧩️ Shapes what one partial inference produced","\n/** 🔍️ Names the exact input behind one stale reason"],to:readFileSync(join(here,"shape-new.mjs"),"utf8")},
 ]},
 {step:1,file:B+"🕸️graph/🧭️discovery/🟦️.ts",edits:[
  ['return {...captured.value,rows:[...captured.observations] as BuildGraphRowV1[],dispose:()=>rowLog.close()};','return {...captured.value,rows:captured.observations as unknown as RowLogListV1<BuildGraphRowV1>,dispose:()=>rowLog.close()};'],
 ]},
 {step:1,file:B+"🕸️graph/📋️model/🟦️.ts",edits:[
  ['parseUnavailable:BuildUnavailableProjectV1[];rows:BuildGraphRowV1[];env:','parseUnavailable:BuildUnavailableProjectV1[];rows:BuildGraphRowV1[]|RowLogListV1<BuildGraphRowV1>;env:'],
 ]},
];
export const NEWS=[
 {step:2,from:"new/key-table.mjs",to:G+"🔑️key-table/🟨️.mjs"},
 {step:2,from:"new/key-table.d.mts",to:G+"🔑️key-table/🟨️.d.mts"},
 {step:2,from:"new/key-table.test.ts",to:G+"🧪️tests/🔑️key-table/🟦️.ts"},
 {step:2,from:"new/key-table.py",to:G+"🧪️tests/🔑️key-table/🐍️.py"},
 {step:1,from:"new/streamed-shape.test.ts",to:G+"🔌️load/🧪️tests/🌊️streamed-shape/🟦️.ts"},
];
export const NEWS_ROOT=here;
export function applyEdits(text,edits,file){
 for(const edit of edits){
  if(Array.isArray(edit)){const [from,to]=edit,count=text.split(from).length-1;if(count!==1)throw Error("anchor count "+count+" in "+file+": "+from.slice(0,70));text=text.replace(from,()=>to);}
  else{const [a,b]=edit.between,i=text.indexOf(a),j=text.indexOf(b,i);if(i<0||j<0||text.indexOf(a,i+1)>=0)throw Error("between anchors in "+file+": "+a.slice(0,60));text=text.slice(0,i)+edit.to+text.slice(j);}
 }
 return text;
}
