import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {pathToFileURL} from "node:url";
import {resolve,dirname,join} from "node:path";
const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations";
const out=process.argv[2],variant=process.argv[3]??"base";
mkdirSync(out,{recursive:true});
const absolute=(text,dir)=>text.replace(/(from\s*|import\s*\(\s*|import\s*)(["'])(\.{1,2}\/[^"']+)\2/gu,(all,head,quote,spec)=>head+quote+pathToFileURL(resolve(dir,spec)).href+quote);
const edits={
 base:[],
 release:[
  ['await profilePhaseV1("graph.build.fragments",()=>mapLimit(stored,P.readConcurrency,job=>job(),signal));mark("fragments");','await profilePhaseV1("graph.build.fragments",()=>mapLimit(stored,P.readConcurrency,job=>job(),signal));mark("fragments");for(const owner of result.owners){owner.node=undefined;owner.edges=undefined;}'],
 ],
 gc:[
  ['await profilePhaseV1("graph.build.fragments",()=>mapLimit(stored,P.readConcurrency,job=>job(),signal));mark("fragments");','await profilePhaseV1("graph.build.fragments",()=>mapLimit(stored,P.readConcurrency,job=>job(),signal));mark("fragments");for(const owner of result.owners){owner.node=undefined;owner.edges=undefined;}globalThis.Bun?.gc?.(true);'],
  ['mark("patch");','mark("patch");if(cold){upserts.clear();append.clear();for(const owner of result.owners)owner.rows=undefined;}globalThis.Bun?.gc?.(true);'],
 ],
};
let store=absolute(readFileSync(join(G,"🟨️.mjs"),"utf8"),G);
for(const[from,to]of edits[variant]){if(!store.includes(from))throw Error("anchor missing: "+from.slice(0,60));store=store.replace(from,()=>to);}
writeFileSync(join(out,"store.mjs"),store);
let load=absolute(readFileSync(join(G,"🔌️load/🟨️.mjs"),"utf8"),join(G,"🔌️load"));
load=load.replace(pathToFileURL(join(G,"🟨️.mjs")).href,pathToFileURL(join(out,"store.mjs")).href);
writeFileSync(join(out,"load.mjs"),load);
console.log("staged",variant,out);
