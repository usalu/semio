import sys,os,json,hashlib
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab"
FILES={"drv":L+"/👷️machine/🟦️.ts","hs":L+"/🏠️home/🧬️schema/🔣️.json","lab":L+"/🟦️.ts"}
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
def one(text,old,new):
    assert text.count(old)==1,("anchor count",text.count(old),old[:90])
    return text.replace(old,new)
OLD=t_old='''const warmed:SliceLabWarmRowV1[]=[],warmSeconds:Record<string,number>={};
const wrapped={...ports,...config.warm===undefined?{}:{run:async(stage:ReadyStageV1,options:Parameters<typeof ports.run>[1])=>{const started=performance.now(),rows=sliceLabWarmV1({state:config.state,source:config.warm!.source,stage:stage.name,tools:config.warm!.tools});warmed.push(...rows);warmSeconds[stage.name]=(performance.now()-started)/1000;return ports.run(stage,options);}},timing:(run:ReadyTimingV1)=>{const spent=warmSeconds[run.stage]??0;const row=spent>0?{...run,seconds:Math.max(0,run.seconds-spent)}:run;timings.push(row);timing?.(run);}};'''
NEW='''const warmed:SliceLabWarmRowV1[]=[];
if(config.warm!==undefined){
 const prime=await runReadyStageV1({root:config.root,environment:{...process.env,HTTPS_PROXY:"http://127.0.0.1:9",HTTP_PROXY:"http://127.0.0.1:9",NO_PROXY:"127.0.0.1,localhost"},stage:{name:"prime",kind:"repo",arguments:["tools","setup","toolchains",...config.warm.prime],timeoutMilliseconds:600000,inputs:[],bootstrapInput:false},signal:AbortSignal.timeout(600000),jobs:1,...process.env.HOME===undefined?{}:{home:process.env.HOME},onLine:line=>process.stdout.write(`${new Date().toISOString()} prime: ${line}\\n`)});
 process.stdout.write(`${new Date().toISOString()} prime exit ${prime.exit}\\n`);
 for(const stage of["dependencies","tools"])warmed.push(...sliceLabWarmV1({state:config.state,source:config.warm.source,stage,tools:config.warm.tools}));
}
const wrapped={...ports,timing:(run:ReadyTimingV1)=>{timings.push(run);timing?.(run);}};'''
def drv(t):
    t=one(t,OLD,NEW)
    t=one(t,'import {createReadyPortsV1,runReadyV1,','import {createReadyPortsV1,runReadyStageV1,runReadyV1,')
    t=one(t,'warm?:{source:string;tools:readonly string[]|"all"}','warm?:{source:string;tools:readonly string[]|"all";prime:readonly string[]}')
    return t
def hs(t): return one(t,'        "warmConcurrency": 8,','        "warmConcurrency": 8,\n        "primeTools": ["uv"],')
def lab(t): return one(t,'warm:{source:options.warm,tools}','warm:{source:options.warm,tools,prime:HOME.primeTools}')
PATCH={"drv":drv,"hs":hs,"lab":lab}
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    os.makedirs(backup,exist_ok=True);manifest={};new={}
    for k,p in FILES.items(): new[k]=PATCH[k](read(p))
    for i,(k,p) in enumerate(FILES.items()):
        old=read(p);write(backup+"/%d.bak"%i,old);manifest[k]={"i":i,"before":sha(old),"after":sha(new[k])}
    json.dump(manifest,open(backup+"/manifest.json","w"))
    for k,p in FILES.items(): write(p,new[k])
    print("applied")
elif cmd=="restore":
    manifest=json.load(open(backup+"/manifest.json"));bad=[]
    for k,p in FILES.items():
        m=manifest[k];now=sha(read(p))
        if now==m["after"]: write(p,read(backup+"/%d.bak"%m["i"]))
        elif now!=m["before"]: bad.append(k)
    print("CONFLICT" if bad else "restored",bad)
    sys.exit(4 if bad else 0)
