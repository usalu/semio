import sys,os,json,hashlib
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab"
FILES={"hs":L+"/🏠️home/🧬️schema/🔣️.json","ht":L+"/🏠️home/🧪️tests/🟦️.ts","drv":L+"/👷️machine/🟦️.ts","hm":L+"/🏠️home/🟦️.ts"}
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
def one(text,old,new):
    assert text.count(old)==1,("anchor count",text.count(old),old[:90])
    return text.replace(old,new)
def hs(t): return one(t,'        "warmConcurrency": 8,','''        "warmConcurrency": 8,
        "stageEnvironment": {
          "dependencies": {"unset": ["CARGO_NET_OFFLINE", "RUSTUP_OFFLINE"], "set": {"HTTPS_PROXY": "http://127.0.0.1:9", "HTTP_PROXY": "http://127.0.0.1:9", "ALL_PROXY": "http://127.0.0.1:9", "https_proxy": "http://127.0.0.1:9", "http_proxy": "http://127.0.0.1:9", "NO_PROXY": "127.0.0.1,localhost,[::1]", "no_proxy": "127.0.0.1,localhost,[::1]"}}
        },''')
HM='''
/** 🧯️ The environment of one stage over the machine's: the declared variables set and unset for that stage only. The `dependencies` stage cannot run under the offline authority of the other stages (its producer then refuses to install anything that is not already a ready generation), so for it the offline switches are lifted and every network client is pointed at a refusing proxy instead (loopback stays reachable), next to the refused registry of the account. */
export function sliceLabStageEnvironmentV1(stage:string,base:Readonly<Record<string,string|undefined>>):Record<string,string|undefined>{
 const rule=(H.stageEnvironment as Readonly<Record<string,Readonly<{unset:readonly string[];set:Readonly<Record<string,string>>}>>>)[stage],env:Record<string,string|undefined>={...base};
 if(rule===undefined)return env;
 for(const name of rule.unset)delete env[name];
 Object.assign(env,rule.set);
 return env;
}
'''
def hm(t): return one(t,'/** 🪜️ The plan of a worker, stage by stage',HM.lstrip("\n")+'\n/** 🪜️ The plan of a worker, stage by stage')
def ht(t):
    t=one(t,'sliceLabAllStagesV1,','sliceLabAllStagesV1,sliceLabStageEnvironmentV1,')
    return one(t,'describe("slice lab home: registry, warm partitions, tool selection",()=>{','''describe("slice lab home: registry, warm partitions, tool selection",()=>{
 test("the dependencies stage runs without the offline switches and behind a refusing proxy, every other stage keeps the machine's environment",()=>{
  const base={CARGO_NET_OFFLINE:"true",RUSTUP_OFFLINE:"1",GOPROXY:"off",PATH:"/usr/bin"};
  const dependencies=sliceLabStageEnvironmentV1("dependencies",base);
  expect(dependencies.CARGO_NET_OFFLINE).toBeUndefined();expect(dependencies.RUSTUP_OFFLINE).toBeUndefined();expect(dependencies.GOPROXY).toBe("off");
  for(const name of["HTTPS_PROXY","HTTP_PROXY","ALL_PROXY"])expect(dependencies[name]).toBe("http://127.0.0.1:9");
  expect(dependencies.NO_PROXY).toContain("127.0.0.1");expect(base.CARGO_NET_OFFLINE).toBe("true");
  for(const name of["native","repo-mcp","tools","workspace"])expect(sliceLabStageEnvironmentV1(name,base)).toEqual(base);
 });''')
def drv(t):
    t=one(t,'import {sliceLabWarmV1,type SliceLabWarmRowV1}','import {sliceLabStageEnvironmentV1,sliceLabWarmV1,type SliceLabWarmRowV1}')
    t=one(t,'const wrapped={...ports,timing:','''const machineEnvironment={...process.env};
const wrapped={...ports,run:(async(stage:ReadyStageV1,options:Parameters<typeof ports.run>[1])=>{
 for(const name of Object.keys(process.env))if(!(name in machineEnvironment))delete process.env[name];
 const staged=sliceLabStageEnvironmentV1(stage.name,machineEnvironment);
 for(const[name,value]of Object.entries(staged)){if(value===undefined)delete process.env[name];else process.env[name]=value;}
 for(const name of Object.keys(machineEnvironment))if(!(name in staged))delete process.env[name];
 try{return await ports.run(stage,options);}finally{for(const[name,value]of Object.entries(machineEnvironment)){if(value===undefined)delete process.env[name];else process.env[name]=value;}for(const name of Object.keys(process.env))if(!(name in machineEnvironment))delete process.env[name];}
}) as typeof ports.run,timing:''')
    return t
PATCH={"hs":hs,"ht":ht,"drv":drv,"hm":hm}
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
