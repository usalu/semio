import {resolve,join} from "node:path";
const root=resolve(import.meta.dir,"../../../../../../../..");
const N=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph");
const owner=await import(join(N,"🟦️.ts"));
const fixture=(await import(join(N,"🧪️tests/🔣️.json"),{with:{type:"json"}})).default as any;
const [list,fn,only]=process.argv.slice(2);
for(const row of fixture[list]){
  if(only&&!row.name.includes(only))continue;
  let msg="ok";
  try{(owner as any)[fn](row.root,row.intent,structuredClone(row.observation));}catch(error){msg=String((error as Error).message).slice(0,160);}
  console.log((row.accepted?"ACC ":"REF ")+row.name.padEnd(46)+" -> "+msg);
}
