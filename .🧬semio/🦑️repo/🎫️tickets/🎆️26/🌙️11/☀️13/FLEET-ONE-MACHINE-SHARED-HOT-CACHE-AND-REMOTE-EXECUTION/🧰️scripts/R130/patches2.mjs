import {readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
const here=dirname(fileURLToPath(import.meta.url));
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const G=B+"🕸️graph/🕰️generations/";
export const PATCHES=[
 {step:2,file:G+"🔌️load/🟨️.mjs",edits:[
  ['import{closureMissingErrorV1}from"../../🩹️closure-repair/🟨️.mjs";','import{closureMissingErrorV1}from"../../🩹️closure-repair/🟨️.mjs";\nimport{runtimeCollectGarbageV1}from"../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";'],
  {between:["/** 🗃️ Whether rows are a row log list","\n/** 🔍️ Names the exact input behind one stale reason"],to:readFileSync(join(here,"shape-new.mjs"),"utf8")},
 ]},
 {step:2,file:G+"🌊️stream-patch/🟨️.mjs",edits:[
  ["import{createHash}from\"node:crypto\";","import{createHash}from\"node:crypto\";\nimport{createKeyTableV1}from\"../🔑️key-table/🟨️.mjs\";"],
  ["import{createKeyTableV1}from\"../🔑️key-table/🟨️.mjs\";","import{createKeyTableV1}from\"../🔑️key-table/🟨️.mjs\";\nimport{createTextArenaV1}from\"../🔤️text-arena/🟨️.mjs\";"],
  ["freshText=new Set(),freshFacts","freshText=createTextArenaV1({validate:text=>{if(LONE_SURROGATE.test(text))throw fail(\"Well formed text required in a fact index: \"+JSON.stringify(text.slice(0,64)));}}),freshFacts"],
  [" const sortedFresh=[...freshText].map(text=>{if(LONE_SURROGATE.test(text))throw fail(\"Well formed text required in a fact index: \"+JSON.stringify(text.slice(0,64)));return{text,bytes:Buffer.from(text,\"utf8\")};}).sort((a,b)=>Buffer.compare(a.bytes,b.bytes));"," const sortedFresh=freshText.sorted(),freshRank=new Uint32Array(freshText.count());"],
  ["freshId=new Map(),stringSink","stringSink"],
  ["  while(next<sortedFresh.length&&Buffer.compare(sortedFresh[next].bytes,bytes)<0){freshId.set(sortedFresh[next].text,strings);emit(sortedFresh[next].bytes);next++;}","  while(next<sortedFresh.length&&Buffer.compare(freshText.bytes(sortedFresh[next]),bytes)<0){freshRank[sortedFresh[next]]=strings;emit(freshText.bytes(sortedFresh[next]));next++;}"],
  ["  if(next<sortedFresh.length&&Buffer.compare(sortedFresh[next].bytes,bytes)===0){keep=true;freshId.set(sortedFresh[next].text,strings);next++;}","  if(next<sortedFresh.length&&Buffer.compare(freshText.bytes(sortedFresh[next]),bytes)===0){keep=true;freshRank[sortedFresh[next]]=strings;next++;}"],
  [" while(next<sortedFresh.length){freshId.set(sortedFresh[next].text,strings);emit(sortedFresh[next].bytes);next++;}"," while(next<sortedFresh.length){freshRank[sortedFresh[next]]=strings;emit(freshText.bytes(sortedFresh[next]));next++;}"],
  ["textId=text=>freshId.get(text);","textId=text=>freshRank[freshText.id(text)];"],
  [",spans=[],byBytes=new Map(),oldFact",",byBytes=createKeyTableV1(),oldFact"],
  ["  const span=factSink.result().subarray(start,factSink.size()),key=createHash(\"sha256\").update(span).digest(\"latin1\"),known=byBytes.get(key);\n  if(known!==undefined)for(const id of known){const[from,to]=spans[id];if(Buffer.compare(factSink.result().subarray(from,to),span)===0){factSink.truncate(start);return id;}}\n  const id=spans.length;spans.push([start,factSink.size()]);if(known===undefined)byBytes.set(key,[id]);else known.push(id);return id;","  const result=factSink.result(),span=result.subarray(start,factSink.size()),key=createHash(\"sha256\").update(span).digest(),known=byBytes.getDigest(key);\n  if(known!==undefined&&Buffer.compare(result.subarray(spanFrom[known],spanTo[known]),span)===0){factSink.truncate(start);return known;}\n  const id=spanCount++;\n  if(id>=spanFrom.length){const from=new Uint32Array(spanFrom.length*2),to=new Uint32Array(spanTo.length*2);from.set(spanFrom);to.set(spanTo);spanFrom=from;spanTo=to;}\n  spanFrom[id]=start;spanTo[id]=factSink.size();if(known===undefined)byBytes.setDigest(key,id);return id;"],
  [" const factSink=createFactSinkV1(maximumBytes),"," let spanFrom=new Uint32Array(1024),spanTo=new Uint32Array(1024),spanCount=0;\n const factSink=createFactSinkV1(maximumBytes),"],
  ["for(const count of[strings,kindStrings.length,ownersOut.length,spans.length,rowsOut,compact.length])","for(const count of[strings,kindStrings.length,ownersOut.length,spanCount,rowsOut,compact.length])"],
  ["  yield mapped;await pause?.();","  yield mapped;await pause?.();if((at/step)%32===31)runtimeCollectGarbageV1(true);"],
  ["memo[unknown[item]]=idOfRow(found[item]);","memo[unknown[item]]=idOfRow(found[item],unknown[item],log);"],
  [" const fresh=[],added=new Map(),byObject=new Map();"," const fresh=[],added=createKeyTableV1(),byObject=new Map(),FRESH_WINDOW=2048;let refs=new Uint32Array(1024),refLog;"],
  ["   const row=fresh[id-oldRows];freshText.add(row.kind);freshText.add(row.path);\n   if(typeof row.fact!==\"object\"||row.fact===null||!freshFacts.has(row.fact)){if(typeof row.fact===\"object\"&&row.fact!==null)freshFacts.add(row.fact);collectFactStringsV1(row.fact,freshText);}","   const row=freshRow(id-oldRows),logged=fresh[id-oldRows]===undefined;freshText.add(row.kind);freshText.add(row.path);\n   if(typeof row.fact!==\"object\"||row.fact===null||logged||!freshFacts.has(row.fact)){if(!logged&&typeof row.fact===\"object\"&&row.fact!==null)freshFacts.add(row.fact);collectFactStringsV1(row.fact,freshText);}"],
  ["   const row=fresh[id-oldRows];kind=row.kind;kindString=textId(kind);path=textId(row.path);\n   fact=typeof row.fact===\"object\"&&row.fact!==null?newFact.get(row.fact):undefined;\n   if(fact===undefined){const start=factSink.size();writeFactValueV1(factSink,row.fact,textId,1);fact=commit(start);if(typeof row.fact===\"object\"&&row.fact!==null)newFact.set(row.fact,fact);}","   const row=freshRow(id-oldRows),logged=fresh[id-oldRows]===undefined;kind=row.kind;kindString=textId(kind);path=textId(row.path);\n   fact=!logged&&typeof row.fact===\"object\"&&row.fact!==null?newFact.get(row.fact):undefined;\n   if(fact===undefined){const start=factSink.size();writeFactValueV1(factSink,row.fact,textId,1);fact=commit(start);if(!logged&&typeof row.fact===\"object\"&&row.fact!==null)newFact.set(row.fact,fact);}"],
  ["freshFacts=new Set(),mark=","freshFacts=new Set(),kindNo=new Uint32Array(fresh.length),pathNo=new Uint32Array(fresh.length),mark="],
  ["freshText.add(row.kind);freshText.add(row.path);","kindNo[id-oldRows]=freshText.add(row.kind);pathNo[id-oldRows]=freshText.add(row.path);"],
  ["kindString=textId(kind);path=textId(row.path);","kindString=freshRank[kindNo[id-oldRows]];path=freshRank[pathNo[id-oldRows]];"],
  {between:[" const idOfRow=row=>{"," const lengthOf=source=>"],to:readFileSync(join(here,"patch-new.mjs"),"utf8")},
 ]},
 {step:3,file:G+"🗃️row-log/🟨️.mjs",edits:[
  ["import{rowKeyV1}from\"../🧮️fact-rows/🟨️.mjs\";","import{rowKeyV1}from\"../🧮️fact-rows/🟨️.mjs\";\nimport{createKeyTableV1}from\"../🔑️key-table/🟨️.mjs\";"],
  [" const descriptor=openSync(path,\"w+\"),byKey=new Map(),exit=","const descriptor=openSync(path,\"w+\"),table=createKeyTableV1(),exit="],
  [" let api,keys=[],offsets="," let api,count=0,scratch=Buffer.allocUnsafe(0),offsets="],
  ["  const key=rowKeyV1(row);let id=byKey.get(key);\n  if(id!==undefined)return id;\n  id=keys.length;byKey.set(key,id);keys.push(key);\n","  const key=rowKeyV1(row),next=count,id=table.intern(key,next);\n  if(id!==next)return id;\n  count++;\n"],
  ["   const buffer=Buffer.allocUnsafe(end-start);let got=0;","   if(scratch.length<end-start)scratch=Buffer.allocUnsafe(Math.max(end-start,policy.spanBytes));const buffer=scratch.subarray(0,end-start);let got=0;"],
  ["   keys:function*(){for(let at=0;at<ids.length;at++)yield keys[ids[at]];for(const tail of tails)for(const row of tail)yield rowKeyV1(row);},","   keys:function*(){for(const row of self.rows())yield rowKeyV1(row);},"],
  ["marks=new Uint8Array(Math.max(1,Math.ceil(keys.length/8)))","marks=new Uint8Array(Math.max(1,Math.ceil(count/8)))"],
  ["  size:()=>keys.length,","  size:()=>count,"],
  ["stats:()=>({rows:keys.length,","stats:()=>({rows:count,"],
  ["byKey.clear();keys=[];","table.clear();count=0;"],
 ]},
 {step:2,file:G+"🧪️tests/🗃️row-log/🟦️.ts",edits:[
  ["the patch learns the index row of a log row once: 300 owners that share a 3000-row closure read each distinct row from the log once, and the patched bytes equal the patch from arrays","the patch learns the index row of a log row once: 300 owners that share a 3000-row closure read each distinct row from the log to number it once and at most twice more to write it, and the patched bytes equal the patch from arrays"],
  ["expect(log.stats().readBytes).toBeLessThan(2*log.stats().bytes);","expect(log.stats().readBytes).toBeLessThan(4*log.stats().bytes);"],
 ]},
];
export const NEWS=[
 {step:2,from:"new/text-arena.mjs",to:G+"🔤️text-arena/🟨️.mjs"},
 {step:2,from:"new/text-arena.d.mts",to:G+"🔤️text-arena/🟨️.d.mts"},
 {step:2,from:"new/text-arena.test.ts",to:G+"🧪️tests/🔤️text-arena/🟦️.ts"},
 {step:2,from:"new/key-table.mjs",to:G+"🔑️key-table/🟨️.mjs"},
 {step:2,from:"new/key-table.d.mts",to:G+"🔑️key-table/🟨️.d.mts"},
 {step:2,from:"new/key-table.test.ts",to:G+"🧪️tests/🔑️key-table/🟦️.ts"},
 {step:2,from:"new/key-table.py",to:G+"🧪️tests/🔑️key-table/🐍️.py"},
 ];
export const NEWS_ROOT=here;
export function applyEdits(text,edits,file){
 for(const edit of edits){
  if(Array.isArray(edit)){const [from,to]=edit,count=text.split(from).length-1;if(count!==1)throw Error("anchor count "+count+" in "+file+": "+from.slice(0,70));text=text.replace(from,()=>to);}
  else{const [a,b]=edit.between,i=text.indexOf(a),j=text.indexOf(b,i);if(i<0||j<0||text.indexOf(a,i+1)>=0)throw Error("between anchors in "+file+": "+a.slice(0,60));text=text.slice(0,i)+edit.to+text.slice(j);}
 }
 return text;
}
