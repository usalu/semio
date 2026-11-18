import sys,re
def sub(t,old,new,count=1):
    assert t.count(old)==count,(old[:60],t.count(old))
    return t.replace(old,new)
def patch_ts(t):
    t=sub(t,"cpu:number;loopLagMs:number|null}>;","cpu:number;loopLagMs:number|null;activity:string}>;")
    t=sub(t,"/** 🔖️ One sample of one machine","""/** 🧭️ What the resident service was doing, read from the end of its log: the last line that names a tool download or install (`tools`), a command of the build script (`build:source-admission`, `build:selected-declaration`, ...), a missing module (`module-missing`) or a capacity admission (`capacity`); `-` when the tail names none. */
export function sliceLabActivityV1(tail:string):string{
 const lines=tail.split("\\n");
 for(let at=lines.length-1;at>=0;at--){
  const line=lines[at]!,command=/"command":"(build:[a-z-]+)"/u.exec(line);
  if(/^Cannot find module /u.test(line))return"module-missing";
  if(/^(?:Downloading the pinned|info: |warn: |Rust is installed)|rustup|toolchain/u.test(line))return"tools";
  if(command!==null)return command[1]!;
  if(/^\\[capacity\\] admitted /u.test(line))return"capacity";
 }
 return"-";
}

/** 🔖️ One sample of one machine""")
    t=sub(t,"leftoverImages:number;admitted:number|null;elapsedSeconds:number;load:number}>;","leftoverImages:number;admitted:number|null;elapsedSeconds:number;load:number;activitySeconds:Readonly<Record<string,number>>;loopLagMaxMs:number|null;lagWithin:boolean;busyShare:number|null;published:readonly string[]}>;")
    t=sub(t,"""  if(sample.phase!==null)phaseSeconds""","""  activitySeconds[sample.activity]=Math.round(((activitySeconds[sample.activity]??0)+step/Math.max(1,input.machines.length))*10)/10;
  if(sample.phase!==null)phaseSeconds""")
    t=sub(t,"stageSeconds:Record<string,number>={},peak","stageSeconds:Record<string,number>={},activitySeconds:Record<string,number>={},peak")
    t=sub(t," const admitted=Math.max(0,"," const lags=input.samples.map(sample=>sample.loopLagMs).filter((value):value is number=>value!==null),lagMax=lags.length===0?null:Math.max(...lags),lagWithin=lagMax===null||lagMax<P.loopLagLimitMilliseconds,counted=input.samples.filter(sample=>sample.processes>0),busyShare=counted.length===0?null:Math.round(counted.reduce((sum,sample)=>sum+sample.busy/sample.processes,0)/counted.length*100);\n const admitted=Math.max(0,")
    t=sub(t,"leftoverImages:number;elapsedSeconds:number;load:number;timedOut:boolean}>)","leftoverImages:number;elapsedSeconds:number;load:number;timedOut:boolean;published?:readonly string[]}>)")
    t=sub(t,"pass:allReady&&within&&","pass:allReady&&within&&lagWithin&&")
    t=sub(t,"elapsedSeconds:input.elapsedSeconds,load:input.load});","elapsedSeconds:input.elapsedSeconds,load:input.load,activitySeconds,loopLagMaxMs:lagMax,lagWithin,busyShare,published:input.published??[]});")
    t=sub(t,"""`- ${t("nice",{count:verdict.lowered})}`,""","""`- ${t("nice",{count:verdict.lowered})}`,`- ${t("lag",{lag:verdict.loopLagMaxMs??"-",limit:P.loopLagLimitMilliseconds})}`,`- ${t("busy",{share:verdict.busyShare??"-"})}`,`- seconds per activity: ${JSON.stringify(verdict.activitySeconds)}`,`- ${t("published",{count:verdict.published.length})}${verdict.published.length>0?": "+verdict.published.join(", "):""}`,""")
    t=sub(t,"cpu:Math.round(mine.reduce((sum,row)=>sum+row.cpu,0)*10)/10,loopLagMs:report?.sources?.loop?.lagMilliseconds??null});","cpu:Math.round(mine.reduce((sum,row)=>sum+row.cpu,0)*10)/10,loopLagMs:report?.sources?.loop?.lagMilliseconds??null,activity:hostActivity(plan)});")
    t=sub(t,"leaks=new Set<string>(),niced=new Set<number>();let tick=0;","leaks=new Set<string>(),niced=new Set<number>(),published=new Set<string>();let tick=0;")
    t=sub(t,"   if(tick++%3===0)for(const path of openFiles","   if(tick%5===0)for(const plan of plans)for(const name of selectedPublications(plan.state))published.add(name);\n   if(tick++%3===0)for(const path of openFiles")
    t=sub(t,"leftoverImages,elapsedSeconds:elapsed,load,timedOut});","leftoverImages,elapsedSeconds:elapsed,load,timedOut,published:[...published]});")
    t=sub(t,"const readJson=<T>","""const hostActivity=(plan:SliceLabPlanV1):string=>{
 try{
  const user=join(plan.runtime,"semio-repo"),found=readdirSync(user).flatMap(name=>readdirSync(join(user,name)).filter(file=>/^host-.*\\.log$/u.test(file)).map(file=>join(user,name,file)))[0];
  if(found===undefined)return"-";
  const size=statSync(found).size,length=Math.min(size,16384),buffer=Buffer.alloc(length),fd=openSync(found,"r");
  try{readSync(fd,buffer,0,length,size-length);}finally{closeSync(fd);}
  return sliceLabActivityV1(buffer.toString("utf8"));
 }catch{return"-";}
};
const selectedPublications=(state:string):readonly string[]=>{
 const found:string[]=[],walk=(directory:string,depth:number):void=>{
  if(depth>7)return;
  let entries;try{entries=readdirSync(directory,{withFileTypes:true});}catch{return;}
  for(const entry of entries){
   const path=join(directory,entry.name);
   if(entry.isDirectory()){if(entry.name==="selected"){for(const file of readdirSync(path))if(/^[0-9a-f]{64}\\.json$/u.test(file))found.push(file);}else walk(path,depth+1);}
  }
 };
 walk(join(state,"b"),0);return found;
};
const readJson=<T>""")
    t=sub(t,'import {existsSync,mkdirSync,mkdtempSync,openSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";','import {closeSync,existsSync,mkdirSync,mkdtempSync,openSync,readdirSync,readFileSync,readSync,realpathSync,rmSync,statSync,writeFileSync} from "node:fs";')
    return t
def patch_schema(t):
    t=sub(t,'          "@semio-tech/stdio-ifc-rs",\n          "@semio-tech/norm-din4108-rs",\n          "@semio-tech/norm-artifact-contract-rs",\n          "@semio-tech/stdio-ifc",\n          "@semio-tech/norm-din4108",\n          "@semio-tech/norm-din16798",\n          "@semio-tech/energy-oracle-py"\n','          "@semio-tech/norm-din4108",\n          "@semio-tech/norm-din4108-rs",\n          "repo-mcp"\n')
    t=sub(t,'"loadWarning": 30','"loadWarning": 30,\n        "loopLagLimitMilliseconds": 200')
    t=sub(t,'"load": "load average at start {load}",','"load": "load average at start {load}",\n          "lag": "parent event-loop lag, maximum {lag} ms (limit {limit} ms)",\n          "busy": "processes of the machines busy in {share} percent of the samples",\n          "published": "selected graph publications sealed on the machines: {count}",')
    t=sub(t,'"load": "Lastmittel zu Beginn {load}",','"load": "Lastmittel zu Beginn {load}",\n          "lag": "Schleifenverzug des Elternprozesses, Maximum {lag} ms (Grenze {limit} ms)",\n          "busy": "Prozesse der Maschinen waren in {share} Prozent der Messpunkte beschäftigt",\n          "published": "auf den Maschinen versiegelte ausgewählte Graph-Veröffentlichungen: {count}",')
    return t
def patch_test(t):
    t=sub(t,"sliceLabLimitMegabytesV1,","sliceLabActivityV1,sliceLabLimitMegabytesV1,")
    t=sub(t,"loopLagMs:null,...patch});",'loopLagMs:null,activity:"-",...patch});')
    t=sub(t,'"nice","notExercised"','"nice","lag","busy","published","notExercised"')
    new="""
 test("the activity is read from the end of the service log: a tool install, a command of the build script, a missing module",()=>{
  expect(sliceLabActivityV1(["[script] {\\"owner\\":\\"repository:build\\",\\"command\\":\\"build:source-admission\\",\\"stage\\":\\"running\\"}","Downloading the pinned cargo-nextest 0.9.140 for darwin-arm64 (1 bytes).","[repository:command] still running after 30 s"].join("\\n"))).toBe("tools");
  expect(sliceLabActivityV1(["info: downloading 8 components","[script] {\\"owner\\":\\"repository:build\\",\\"command\\":\\"build:selected-declaration\\",\\"stage\\":\\"running\\"}","[repository:command] still running after 30 s"].join("\\n"))).toBe("build:selected-declaration");
  expect(sliceLabActivityV1("Cannot find module '/x/render.ts'\\n[capacity] admitted graph-plan: 1 job(s)")).toBe("capacity");
  expect(sliceLabActivityV1("[capacity] admitted graph-plan: 1 job(s)\\nCannot find module '/x/render.ts'")).toBe("module-missing");
  expect(sliceLabActivityV1("")).toBe("-");
 });
 test("the verdict fails on a parent loop lag of 200 ms and names the seconds per activity and the busy share",()=>{
  const ok={options:{...options,machines:1},machines:["m1"],samples:[sample({activity:"tools",loopLagMs:20,processes:4,busy:1}),sample({activity:"build:selected-declaration",loopLagMs:150,processes:4,busy:3})],results:{m1:{ready:true,failure:null,timings:[]}},duplicateOwners:0,leftoverProcesses:0,leftoverImages:0,elapsedSeconds:9,load:3,timedOut:false};
  expect(sliceLabVerdictV1({...ok,published:["a".repeat(64)+".json"]})).toMatchObject({pass:true,lagWithin:true,loopLagMaxMs:150,busyShare:50,activitySeconds:{tools:2,"build:selected-declaration":2},published:["a".repeat(64)+".json"]});
  const slow=sliceLabVerdictV1({...ok,samples:[sample({loopLagMs:200})]});
  expect(slow).toMatchObject({lagWithin:false,pass:false});
  expect(sliceLabReportV1("en",options,slow,[sample()])).toContain("maximum 200 ms (limit 200 ms)");
  expect(sliceLabReportV1("de",options,slow,[sample()])).toContain("Maximum 200 ms (Grenze 200 ms)");
 });
"""
    i=t.rstrip().rfind("});")
    return t[:i].rstrip("\n")+"\n"+new.rstrip("\n")+"\n});\n"
if __name__=="__main__":
    kind,src,dst=sys.argv[1:4]
    t=open(src,encoding="utf8").read()
    open(dst,"w",encoding="utf8").write({"ts":patch_ts,"schema":patch_schema,"test":patch_test}[kind](t))
