import {readFileSync} from "node:fs";
const events:{path:string;loadMs:number;endAt:number}[]=[];
(globalThis as any).__mtEnd=(path:string,loadMs:number)=>{events.push({path,loadMs,endAt:performance.now()});};
Bun.plugin({name:"module-trace",setup(build){
 build.onLoad({filter:/\.(ts|mjs)$/},args=>{
  if(args.path.includes("/node_modules/")||args.path.includes("module-trace-preload"))return undefined;
  const s=performance.now(),text=readFileSync(args.path,"utf8"),loadMs=performance.now()-s;
  const trailer=`\n;globalThis.__mtEnd(${JSON.stringify(args.path)},${loadMs});\n`;
  return {contents:text+trailer,loader:args.path.endsWith(".ts")?"ts":"js"};
 });
}});
process.on("exit",()=>{
 const rows=events.map((e,i)=>({...e,self:e.endAt-(i?events[i-1]!.endAt:0)})),total=events.at(-1)?.endAt??0;
 const short=(p:string)=>p.split("/").slice(-3).join("/");
 const lines=rows.slice().sort((a,b)=>b.self-a.self).slice(0,Number(process.env.MT_TOP??22)).map(r=>`${r.self.toFixed(1).padStart(8)} ms  ${short(r.path)}`);
 process.stderr.write(`#module-trace modules=${rows.length} last-end=${total.toFixed(0)}ms sum-self=${rows.reduce((a,r)=>a+r.self,0).toFixed(0)}ms\n`+lines.join("\n")+"\n");
});
