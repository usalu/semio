import {mkdirSync,rmSync,writeFileSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {bazel,flagsFor,shutdown} from "./probe.ts";
const tools=import.meta.dir,S="/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/bz",[source,key,label]=process.argv.slice(2) as [string,string,string];
const out=join(tools,"..","🗑️generated","bzsrv-g");mkdirSync(out,{recursive:true});
const texts:string[]=[];
for(const slot of ["s1","s2"]){
 const root=join(S,"aq-"+slot),pkg=join(root,"ws",slot,key);rmSync(root,{recursive:true,force:true});mkdirSync(join(root,"ws",slot),{recursive:true});
 if(Bun.spawnSync(["cp","-cR",join(source,key),pkg]).exitCode!==0)throw Error("clone");
 const flags=flagsFor(tools,join(root,"p"),slot,1024),run=await bazel(flags,[],pkg,["aquery",...flags.command,"--output=jsonproto","--include_commandline","--include_artifacts=false",`deps(//:${label})`]);
 if(run.code!==0)throw Error(run.log.slice(-800));
 const base=join(root,"p");texts.push(run.stdout.replaceAll(pkg,"<WS>").replaceAll(root,"<ROOT>").replaceAll(slot,"<SLOT>"));
 shutdown(flags,pkg);rmSync(root,{recursive:true,force:true});
}
writeFileSync(join(out,"aquery-s1.json"),texts[0]!);writeFileSync(join(out,"aquery-s2.json"),texts[1]!);
console.log(JSON.stringify({equal:texts[0]===texts[1],bytes:texts.map(text=>text.length)}));
