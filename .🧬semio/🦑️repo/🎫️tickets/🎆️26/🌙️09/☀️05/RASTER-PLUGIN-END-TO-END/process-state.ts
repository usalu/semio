import {readdirSync,readFileSync,readlinkSync} from "node:fs";
import {basename} from "node:path";
if(process.platform!=="linux"){console.log("[DEBUG] "+JSON.stringify({platform:process.platform,procInspection:false}));}else{
  const selected=new Set(process.argv.slice(2));for(const id of readdirSync("/proc")){if(!/^\d+$/.test(id)||selected.size>0&&!selected.has(id))continue;try{
    const executable=readlinkSync(`/proc/${id}/exe`);if(!/^(bun|node|cargo|rustc|repo|cargo-nextest|nextest)(-|$)/.test(basename(executable)))continue;
    const argv=readFileSync(`/proc/${id}/cmdline`,"utf8").split("\0");const relevant=argv.some(arg=>/raster|source-loader|auxiliary-probe|semio/.test(arg));if(!relevant)continue;
    const stat=readFileSync(`/proc/${id}/stat`,"utf8");const fields=stat.slice(stat.lastIndexOf(")")+2).split(" ");
    const kind=argv.some(arg=>arg.includes("auxiliary-probe.ts"))?"owned-probe":argv.some(arg=>arg.includes("raster-js:test"))?"raster-js":argv.some(arg=>arg.includes("semio-s-artifact-raster-raster-host"))?"raster-host":argv.some(arg=>arg.includes("repo"))?"repo-runtime":"workspace-runtime";
    const commands=argv.filter(arg=>/^(run|cargo|doctor|show|project|test|check|-p|--lib|--store=shared|--configuration=prototype|@semio-tech\/[A-Za-z0-9-]+:[A-Za-z-]+|semio-s-artifact-raster-[A-Za-z0-9-]+)$/.test(arg));const wait=readFileSync(`/proc/${id}/wchan`,"utf8").trim();const io=Object.fromEntries(readFileSync(`/proc/${id}/io`,"utf8").trim().split("\n").map(row=>row.split(": ")));
    console.log("[DEBUG] "+JSON.stringify({pid:Number(id),ppid:Number(fields[1]),state:fields[0],cpuUserTicks:fields[11],cpuSystemTicks:fields[12],startTicks:fields[19],executable,kind,commands,wait,io}));
  }catch{}}
}
