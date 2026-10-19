import {readFileSync,writeFileSync} from "node:fs";
import {loadGraph} from "./graph.mjs";
const dir=process.argv[2];
const L=f=>readFileSync(`${dir}/uicarve-tools/lists/${f}`,"utf8").split("\n").filter(Boolean);
const fw=loadGraph(dir+"/cc-tools/meta-framework.json"),rt=loadGraph(dir+"/cc-tools/meta-root.json");
const names=[...L("crates-new.txt"),"semio-framework-ui",...L("crates-consumers.txt")];
const uniq=[...new Set(names)];
const g={framework:[],root:[],none:[]};
for(const n of uniq){if(fw.memberNames.has(n))g.framework.push(n);else if(rt.memberNames.has(n))g.root.push(n);else g.none.push(n);}
console.log(Object.fromEntries(Object.entries(g).map(([k,v])=>[k,v.length])),"none:",g.none.slice(0,10));
const batches=JSON.parse(readFileSync(dir+"/cc-tools/batches.json","utf8"));
let idx=300;const out=[];
for(const [ws,man] of [["framework","🧰️framework/Cargo.toml"],["root","Cargo.toml"]]){
 for(let i=0;i<g[ws].length;i+=20){const chunk=g[ws].slice(i,i+20);
  for(const flag of ["--lib","--tests"]){batches[idx]={section:"uicarve "+ws,cmd:`pl check --manifest-path ${man} ${chunk.map(c=>"-p "+c).join(" ")} ${flag} --keep-going --message-format=short`};out.push(`${idx} ${ws} ${flag} ${chunk.length}`);idx++;}}}
writeFileSync(dir+"/cc-tools/batches.json",JSON.stringify(batches));
console.log(out.join("\n"));
