import {readFileSync,writeFileSync} from "node:fs";
import {loadGraph} from "./graph.mjs";
const [dir,file,start,...flags]=process.argv.slice(2);
const fw=loadGraph(dir+"/cc-tools/meta-framework.json"),rt=loadGraph(dir+"/cc-tools/meta-root.json");
const names=[...new Set(readFileSync(file,"utf8").split("\n").map(x=>x.trim()).filter(x=>/^semio[\w-]*$/.test(x)))];
const g={framework:[],root:[],none:[]};
for(const n of names){if(fw.memberNames.has(n))g.framework.push(n);else if(rt.memberNames.has(n))g.root.push(n);else g.none.push(n);}
console.log(Object.fromEntries(Object.entries(g).map(([k,v])=>[k,v.length])),"none:",g.none.join(" "));
const batches=JSON.parse(readFileSync(dir+"/cc-tools/batches.json","utf8"));
let idx=Number(start);
for(const [ws,man] of [["framework","🧰️framework/Cargo.toml"],["root","Cargo.toml"]]){
 for(let i=0;i<g[ws].length;i+=20){const chunk=g[ws].slice(i,i+20);
  for(const flag of flags){batches[idx]={section:"list "+ws,cmd:`pl check --manifest-path ${man} ${chunk.map(c=>"-p "+c).join(" ")} ${flag} --keep-going --message-format=short`};console.log(idx,ws,flag,chunk.length);idx++;}}}
writeFileSync(dir+"/cc-tools/batches.json",JSON.stringify(batches));
writeFileSync(dir+"/cc-tools/none-"+start+".txt",g.none.join("\n"));
