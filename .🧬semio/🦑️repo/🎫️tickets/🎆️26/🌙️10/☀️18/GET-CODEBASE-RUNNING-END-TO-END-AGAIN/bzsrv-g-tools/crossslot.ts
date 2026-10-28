import {cpSync,mkdirSync,rmSync,writeFileSync,appendFileSync} from "node:fs";
import {join} from "node:path";
import {bazel,flagsFor,shutdown} from "./probe.ts";
const tools=import.meta.dir,S="/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/bz",count=Number(process.argv[2]??30);
const root=join(S,"cross");rmSync(root,{recursive:true,force:true});mkdirSync(root,{recursive:true});
const build=Array.from({length:count},(_,index)=>`genrule(name="g${index}",srcs=["src${index}.txt"],outs=["o${index}.txt"],cmd="cat $< > $@ && echo ${index} >> $@",tags=[])`).join("\n");
const make=(dir:string)=>{mkdirSync(dir,{recursive:true});writeFileSync(join(dir,"MODULE.bazel"),'module(name="fx", version="0")\n');writeFileSync(join(dir,"BUILD.bazel"),build+"\n");for(let index=0;index<count;index++)writeFileSync(join(dir,`src${index}.txt`),"content "+index+"\n");};
const rows:Record<string,unknown>[]=[];
const summary=(log:string)=>/INFO: (\d+) processes?(?:: ([^.]+))?\./.exec(log)?.[0]??"?";
for(const [slot,dir] of [["s0","w/s0/k"],["s1","w/s1/k"],["s2","w/s2/k"]] as const){
 const pkg=join(root,dir);make(pkg);
 const flags=flagsFor(tools,join(root,"p"),slot,1024),run=await bazel(flags,[],pkg,["build",...flags.command,"//:all"]);
 rows.push({slot,wall:run.wall,code:run.code,summary:summary(run.log),peak:run.peakRssMb});
}
const flags0=flagsFor(tools,join(root,"p"),"s0",1024),pkg0=join(root,"w/s0/k"),again=await bazel(flags0,[],pkg0,["build",...flags0.command,"//:all"]);rows.push({slot:"s0 again (hot server)",wall:again.wall,summary:summary(again.log)});
for(const slot of ["s0","s1","s2"])shutdown(flagsFor(tools,join(root,"p"),slot,1024),join(root,"w",slot,"k"));
const out=join(tools,"..","🗑️generated","bzsrv-g");mkdirSync(out,{recursive:true});appendFileSync(join(out,"crossslot.jsonl"),JSON.stringify(rows)+"\n");console.log(JSON.stringify(rows,null,1));
rmSync(root,{recursive:true,force:true});
