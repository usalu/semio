import{readFileSync,readdirSync,statSync}from"node:fs";
import{join}from"node:path";
import{readyImageGuardV1,readyImageShortV1,readyImageNeedBytesV1,readyImageFloorBytesV1,READY_IMAGE_POLICY_V1 as P}from"./🖼️image-guard/🟦️.ts";
const S="/Users/Shared/semio-store/v1/origins",MB=1048576;
const stageOf=(letter:string)=>Object.entries(P.stages).filter(([,l])=>(l as readonly string[]).includes(letter)).map(([k])=>k);
for(const origin of readdirSync(S).filter(n=>n.startsWith("g1-m")).sort())for(const f of readdirSync(join(S,origin)).filter(n=>n.startsWith("ready-log-")&&n.endsWith(".txt"))){
 const text=readFileSync(join(S,origin,f),"utf8"),i=text.lastIndexOf("disk facts (free of total, MB)");if(i<0)continue;
 const best={t:0,block:text.slice(i).split("\n").slice(1,14).join("\n")};
 const host=/Documents\/semio: (\d+) of/.exec(best.block);
 const images=[...best.block.matchAll(/\.cache\/b\/(\w)\/m: (\d+) of (\d+)/g)].map(m=>({partition:m[1]!,path:"/w/.cache/b/"+m[1]+"/m",freeBytes:Number(m[2])*MB,totalBytes:Number(m[3])*MB,hostFreeBytes:host?Number(host[1])*MB:null}));
 const out:string[]=[];
 for(const stage of Object.keys(P.stages)){
  const rows=images.filter(i=>(P.stages as Record<string,string[]>)[stage]!.includes(i.partition));
  const s=readyImageShortV1(rows),g=readyImageGuardV1(rows);
  if(s)out.push(`${stage}: SHORT ${s.partition} ${s.available}/${s.total} need ${s.needed}${s.limitedByHost?" host-limited":""}`);
  if(g)out.push(`${stage}: GUARD ${g.partition} ${g.available} < ${g.floor}`);
 }
 if(out.length)console.log(origin,f,out.join(" | "));
}
