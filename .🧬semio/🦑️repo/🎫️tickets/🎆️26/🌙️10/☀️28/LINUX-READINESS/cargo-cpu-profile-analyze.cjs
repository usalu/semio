const fs=require("fs"),path=require("path"),directory=process.argv[2]||path.join(__dirname,"🗑️generated/cargo-cpu");
const profiles=fs.readdirSync(directory).filter(name=>name.endsWith(".cpuprofile")||name.endsWith(".jsc.json")).map(name=>{
 const data=JSON.parse(fs.readFileSync(path.join(directory,name),"utf8"));
 if(data.runtime==="bun"){
  const traces=data.stackTraces.traces,first=traces[0]?.timestamp??0,last=traces.at(-1)?.timestamp??first,all=new Map(),tail=new Map(),stacks=new Map();
  const frame=value=>({name:value.name,url:value.sourceURL,line:value.line,column:value.column,category:value.category});
  for(const trace of traces){const key=JSON.stringify(frame(trace.frames[0]||{}));all.set(key,(all.get(key)||0)+1);if(trace.timestamp>=first+(last-first)*2/3){tail.set(key,(tail.get(key)||0)+1);const stack=JSON.stringify(trace.frames.slice(0,12).map(frame));stacks.set(stack,(stacks.get(stack)||0)+1);}}
  const rank=table=>[...table].sort((a,b)=>b[1]-a[1]).slice(0,20).map(([key,count])=>({count,frame:JSON.parse(key)}));
  return{name,pid:data.pid,bytes:fs.statSync(path.join(directory,name)).size,samples:traces.length,durationMilliseconds:(last-first)*1000,functions:data.functions,all:rank(all),lastThird:rank(tail),stacks:rank(stacks)};
 }
 const nodes=new Map(data.nodes.map(row=>[row.id,row])),samples=data.samples||[],deltas=data.timeDeltas||[],elapsed=deltas.reduce((sum,value)=>sum+value,0),all=new Map(),tail=new Map();let time=0;
 for(let i=0;i<samples.length;i++){const weight=deltas[i]||1,id=samples[i];time+=weight;all.set(id,(all.get(id)||0)+weight);if(time>=elapsed*2/3)tail.set(id,(tail.get(id)||0)+weight);}
 const rank=table=>[...table].sort((a,b)=>b[1]-a[1]).slice(0,20).map(([id,count])=>({count,frame:nodes.get(id)?.callFrame}));
 return{name,bytes:fs.statSync(path.join(directory,name)).size,samples:samples.length,durationMilliseconds:(data.endTime-data.startTime)/1000,all:rank(all),lastThird:rank(tail)};
});
const output=path.join(__dirname,"🗑️generated/cargo-cpu-analysis.json");fs.writeFileSync(output,JSON.stringify({at:new Date().toISOString(),profiles},null,2)+"\n");
console.log(JSON.stringify({profiles:profiles.map(row=>({name:row.name,pid:row.pid,bytes:row.bytes,samples:row.samples,durationMilliseconds:row.durationMilliseconds,lastThird:row.lastThird.slice(0,6)})),output}));
