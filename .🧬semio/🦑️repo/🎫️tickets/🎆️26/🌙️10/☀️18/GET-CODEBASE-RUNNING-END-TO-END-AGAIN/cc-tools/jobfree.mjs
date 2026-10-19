import {readFileSync,existsSync} from "node:fs";
import {loadGraph,closure} from "./graph.mjs";
const dir=process.argv[2];const batches=JSON.parse(readFileSync(dir+"/cc-tools/batches.json","utf8"));
const gr={"Cargo.toml":dir+"/cc-tools/meta-root.json","🧰️framework/Cargo.toml":dir+"/cc-tools/meta-framework.json"};
const gs={};for(const [k,v] of Object.entries(gr))if(existsSync(v))gs[k]=loadGraph(v);
const bad=new Set(process.argv.slice(3));
const out=[];
batches.forEach((b,i)=>{const a=b.cmd.split(/\s+/).slice(1);const mi=a.indexOf("--manifest-path");const man=mi>=0?a[mi+1]:"Cargo.toml";const g=gs[man];const ps=[];for(let j=0;j<a.length;j++)if(a[j]==="-p")ps.push(a[j+1]);
 const dev=a.includes("--tests")||a.includes("--all-targets");
 if(!g){out.push(`${i}\t${b.section}\tno-graph\t${ps.length}`);return;}
 const blocked=ps.filter(p=>[...closure(g,p,dev)].some(x=>bad.has(x)));
 out.push(`${i}\t${b.section}\t${ps.length-blocked.length}/${ps.length} free\t${man.slice(0,12)}`);});
console.log(out.join("\n"));
