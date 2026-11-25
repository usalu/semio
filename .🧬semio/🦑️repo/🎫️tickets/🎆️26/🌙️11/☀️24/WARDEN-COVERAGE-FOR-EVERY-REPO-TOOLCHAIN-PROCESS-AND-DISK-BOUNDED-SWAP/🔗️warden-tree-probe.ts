/** 🔗️ Waits for a linker (`ld`) with a resident set of at least 500 MB below a `rustc --crate-name <crate>` process, then prints what the warden's own tree observation (`observeTreeMegabytesV1`, the footprint of the whole subtree of the compiler subject) reports for that compiler, next to the footprint of the linker alone, and the chain of processes between them. Usage: bun 🔗️warden-tree-probe.ts <crate-name> */
import {observeTreeMegabytesV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
const crate=process.argv[2]!;
const table=()=>Bun.spawnSync(["ps","-axo","pid=,ppid=,rss=,args="]).stdout.toString().split("\n").filter(Boolean).map(line=>{const m=line.trim().match(/^(\d+)\s+(\d+)\s+(\d+)\s+((\S+)\s*.*)$/u)!;return{pid:+m[1]!,ppid:+m[2]!,rss:+m[3]!,comm:m[5]!,args:m[4]!};});
for(;;){
 const rows=table(),byPid=new Map(rows.map(row=>[row.pid,row]));
 for(const linker of rows.filter(row=>/\/ld$/u.test(row.comm)&&row.rss>500000)){
  let up=byPid.get(linker.ppid),compiler;while(up){if(up.comm.endsWith("rustc")&&up.args.includes("--crate-name "+crate)){compiler=up;break;}up=byPid.get(up.ppid);}
  if(compiler){
   const tree=await observeTreeMegabytesV1(compiler.pid),alone=await observeTreeMegabytesV1(linker.pid),chain:string[]=[];for(let row=linker;row&&row.pid!==compiler.pid;row=byPid.get(row.ppid)!)chain.push(row.pid+":"+row.comm.split("/").pop());
   console.log(JSON.stringify({crate,compiler:compiler.pid,linker:linker.pid,chain:chain.reverse(),linkerAloneMegabytes:alone,compilerTreeMegabytes:tree,linkerResidentMegabytes:Math.floor(linker.rss/1024),compilerResidentMegabytes:Math.floor(compiler.rss/1024)}));
   process.exit(0);
  }
 }
 await Bun.sleep(500);
}
