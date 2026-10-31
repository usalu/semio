import {readFile,writeFile} from "node:fs/promises";
import {resolve} from "node:path";
const input=process.argv[2];if(!input)throw Error("actual native log required");
const original=await readFile(input,"utf8"),prefix="[DEBUG] [text-splice-native] ",lines=original.split("\n"),indices=lines.flatMap((line,index)=>line.startsWith(prefix)?[index]:[]);
const oracle=resolve(import.meta.dirname,"text-splice-native-oracle.ts");
async function check(path:string,accepted:boolean){const child=Bun.spawn([process.execPath,oracle,path],{stdout:"pipe",stderr:"pipe"});await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text()]);const code=await child.exited;if((code===0)!==accepted)throw Error("oracle corpus result mismatch: "+path);}
await check(input,true);
const select=(source:string)=>indices.filter(i=>JSON.parse(lines[i].slice(prefix.length)).source===source);
const fixture=select("fixture"),typing=select("typing"),generated=select("generated");if(fixture.length<2||typing.length<2||generated.length<2)throw Error("actual coverage insufficient for hostile proof");
const corpora=[
 {name:"fixture-duplicate",mutate:(copy:string[])=>{copy[fixture[1]]=copy[fixture[0]]}},
 {name:"typing-missing",mutate:(copy:string[])=>{copy[typing[0]]=""}},
 {name:"typing-duplicate",mutate:(copy:string[])=>{copy[typing[1]]=copy[typing[0]]}},
 {name:"generated-duplicate",mutate:(copy:string[])=>{copy[generated[1]]=copy[generated[0]]}},
 {name:"expiry-missing",mutate:(copy:string[])=>{const i=copy.findIndex(s=>s.startsWith("[DEBUG] [text-splice-expiry] "));if(i<0)throw Error("actual expiry missing");copy[i]=""}}
];
for(const corpus of corpora){const copy=[...lines];corpus.mutate(copy);const path=resolve(import.meta.dirname,"🗑️generated","text-splice-"+corpus.name+".log");await writeFile(path,copy.join("\n"));await check(path,false);}
console.log(JSON.stringify({hostile:corpora.length,actualBaseline:true,rejected:true}));
