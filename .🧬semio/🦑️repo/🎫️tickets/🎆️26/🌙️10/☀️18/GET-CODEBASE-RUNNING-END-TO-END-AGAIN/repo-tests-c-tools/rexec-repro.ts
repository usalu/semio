import {startClusterV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/🧰️cluster/🟦️.ts";
import {existsSync,mkdtempSync,readFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
const alive=(p:string)=>{try{process.kill(Number(readFileSync(p,"utf8").trim()),0);return true;}catch{return false;}};
async function round(label:string,warm:boolean){
  const cluster=await startClusterV1({workers:1,worker:{sandbox:"isolated-root"} as any});
  const dir=mkdtempSync(join(tmpdir(),"rx-"));
  const slow=(name:string)=>cluster.action({arguments:["/bin/sh","-c","(sleep 60 & echo $! > "+dir+"/"+name+".child; wait) & echo $! > "+dir+"/"+name+".pid; wait"],salt:name});
  if(warm){const quick=await cluster.action({arguments:["/bin/true"],salt:"q"+label});await cluster.execute(quick);}
  const abort=new AbortController();
  const running=cluster.execute(await slow(label),{signal:abort.signal}).catch(e=>e);
  for(let i=0;i<160&&!existsSync(dir+"/"+label+".child");i++)await Bun.sleep(50);
  abort.abort();await running;
  let gone=false;for(let i=0;i<100;i++){if(!alive(dir+"/"+label+".pid")&&!alive(dir+"/"+label+".child")){gone=true;break;}await Bun.sleep(100);}
  console.log("[rx]",label,"warm="+warm,"processes gone:",gone);
  if(!gone){try{process.kill(Number(readFileSync(dir+"/"+label+".child","utf8")),9);process.kill(Number(readFileSync(dir+"/"+label+".pid","utf8")),9);}catch{}}
  await cluster.stop();
}
await round("a",false);await round("b",true);await round("c",true);await round("d",false);
process.exit(0);
