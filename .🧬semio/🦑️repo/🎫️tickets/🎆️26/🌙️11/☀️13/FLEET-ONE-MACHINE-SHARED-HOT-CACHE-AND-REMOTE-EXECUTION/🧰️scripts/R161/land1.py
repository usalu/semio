import sys,os,json,hashlib
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab"
MIRROR=os.environ.get("R161_MIRROR")
def loc(p): return p if MIRROR is None else MIRROR+"/"+os.path.relpath(p,L).replace("/","~")
FILES={"hs":L+"/🏠️home/🧬️schema/🔣️.json","hm":L+"/🏠️home/🟦️.ts","ht":L+"/🏠️home/🧪️tests/🟦️.ts","ls":L+"/🧬️schema/🔣️.json","lab":L+"/🟦️.ts","script":L+"/📜️script.ts","drv":L+"/👷️machine/🟦️.ts"}
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
def one(text,old,new):
    assert text.count(old)==1,("anchor count",text.count(old),old[:90])
    return text.replace(old,new)
HS_WARM='''        "registry": {"url": "http://127.0.0.1:9/", "files": {".npmrc": "registry={url}\\n", ".bunfig.toml": "[install]\\nregistry = \\"{url}\\"\\n"}},
        "warmConcurrency": 8,
        "warm": [
          {"stage": "dependencies", "partition": "d", "items": [{"id": "bun-cache", "path": "bun-cache", "tools": []}]},
          {"stage": "tools", "partition": "t", "items": [
            {"id": "downloads", "path": "downloads", "tools": []},
            {"id": "bun", "path": "bun", "tools": ["bun"]},
            {"id": "go", "path": "native/go", "tools": ["go"]},
            {"id": "uv", "path": "native/uv", "tools": ["uv"]},
            {"id": "cmake", "path": "native/cmake", "tools": ["cmake"]},
            {"id": "ninja", "path": "native/ninja", "tools": ["ninja"]},
            {"id": "cargo-nextest", "path": "native/cargo-nextest", "tools": ["cargo-nextest"]},
            {"id": "cargo-llvm-cov", "path": "native/cargo-llvm-cov", "tools": ["cargo-llvm-cov"]},
            {"id": "dotnet", "path": "native/dotnet", "tools": ["dotnet"]},
            {"id": "python", "path": "python", "tools": ["python-main", "python-oracle"]},
            {"id": "binaryen", "path": "binaryen", "tools": ["wasm-opt"]}
          ]}
        ],
'''
def hs(t):
    t=one(t,'        "stageNames": [','''%s        "stageNames": ['''%HS_WARM)
    t=one(t,'"GOFLAGS": "-mod=mod",','"GOFLAGS": "-mod=mod", "NPM_CONFIG_REGISTRY": "http://127.0.0.1:9/", "BUN_CONFIG_REGISTRY": "http://127.0.0.1:9/",')
    return t
HM_CODE='''
/** 📵️ Writes the two files every package manager of the account reads (`.npmrc`, `.bunfig.toml`) so that the registry is a loopback address nobody listens on: a dependency the seeded cache does not hold fails at once with a refused connection instead of fetching (the dependency producer copies both files from the account into its own home). */
export function sliceLabRegistryRefusalV1(home:string):readonly string[]{
 const rule=H.registry,made:string[]=[];
 for(const[name,template]of Object.entries(rule.files)){writeFileSync(join(home,name),(template as string).replaceAll("{url}",rule.url));made.push(name);}
 return made;
}

/** 🍲️ One partition item of the warm table: what a stage finds already in its partition on a worker that has run before. */
export type SliceLabWarmItemV1=Readonly<{stage:string;partition:string;id:string;path:string}>;
/** 🍲️ The items of a stage for the tools prepared: every item without a tool, and every item one of whose tools is prepared (`"all"` takes every item). */
export function sliceLabWarmItemsV1(stage:string,tools:readonly string[]|"all"):readonly SliceLabWarmItemV1[]{
 return H.warm.filter(row=>row.stage===stage).flatMap(row=>row.items.filter(item=>item.tools.length===0||tools==="all"||item.tools.some(id=>tools.includes(id))).map(item=>({stage:row.stage,partition:row.partition,id:item.id,path:item.path})));
}
/** 🍲️ The result of warming one item: `skipped` names the reason (`no-source`, `not-mounted`, `copy-failed`) when nothing was copied. */
export type SliceLabWarmRowV1=Readonly<{stage:string;id:string;ok:boolean;method:string;seconds:number;megabytes:number}>;
const mountedPartitionV1=(state:string,partition:string):boolean=>{try{return statSync(join(state,"b",partition,"m")).dev!==statSync(join(state,"b",partition)).dev;}catch{return false;}};
const megabytesOfV1=(path:string):number=>{const run=spawnSync("du",["-sk",path],{encoding:"utf8"});return Math.round(Number((run.stdout??"").split(/\\s/u)[0]??0)/102.4)/10;};
const copyEntriesV1=(source:string,target:string):boolean=>{
 mkdirSync(target,{recursive:true});
 return spawnSync("sh",["-c",'find "$1" -mindepth 1 -maxdepth 1 -print0 | xargs -0 -P "$3" -I{} cp -R {} "$2"/',"sh",source,target,String(H.warmConcurrency)],{encoding:"utf8"}).status===0;
};
/** 🍲️ Fills the mounted partitions of a lab machine with what the live checkout holds in the same places (a read of the source, a copy into the target: the partitions are images of their own, so no clone exists across them), for one stage and the tools it prepares: a target that already holds entries is left as it is. Never writes below the source; every row says how long it took and how much it moved. `mounted` is the partition test (the real one compares device numbers of `<state>/b/<partition>/m` and its parent). */
export function sliceLabWarmV1(input:Readonly<{state:string;source:string;stage:string;tools:readonly string[]|"all";mounted?:(state:string,partition:string)=>boolean}>):readonly SliceLabWarmRowV1[]{
 const mounted=input.mounted??mountedPartitionV1;
 return sliceLabWarmItemsV1(input.stage,input.tools).map(item=>{
  const source=join(input.source,item.partition,...item.path.split("/")),target=join(input.state,"b",item.partition,"m",...item.path.split("/")),started=performance.now(),row=(ok:boolean,method:string,megabytes=0)=>({stage:item.stage,id:item.id,ok,method,seconds:round((performance.now()-started)/1000),megabytes});
  if(!existsSync(source))return row(true,"skipped: no-source");
  if(!mounted(input.state,item.partition))return row(true,"skipped: not-mounted");
  try{if(readdirSync(target).length>0)return row(true,"present");}catch{}
  if(!copyEntriesV1(source,target))return row(false,"skipped: copy-failed");
  return row(true,"copy",megabytesOfV1(target));
 });
}
/** 🥘️ Builds the warm directory a lab reads (`<out>/<partition>/<path>` for every item of the table) from the live checkout's partitions, read only: the one slow copy (the dependency cache is several gigabytes in another disk image), done once and then used by every run. An item that is already in `out` is kept. */
export function sliceLabWarmBuildV1(live:string,out:string):readonly SliceLabWarmRowV1[]{
 return H.warm.flatMap(row=>row.items.map(item=>{
  const source=join(live,".cache","b",row.partition,"m",...item.path.split("/")),target=join(out,row.partition,...item.path.split("/")),started=performance.now(),made=(ok:boolean,method:string,megabytes=0)=>({stage:row.stage,id:item.id,ok,method,seconds:round((performance.now()-started)/1000),megabytes});
  if(!existsSync(source))return made(true,"skipped: no-source");
  if(existsSync(target))return made(true,"present");
  return copyEntriesV1(source,target)?made(true,"copy",megabytesOfV1(target)):made(false,"skipped: copy-failed");
 }));
}
'''
def hm(t):
    t=one(t,'import {chmodSync,existsSync,lstatSync,mkdirSync,readdirSync,symlinkSync,writeFileSync} from "node:fs";','import {chmodSync,existsSync,lstatSync,mkdirSync,readdirSync,statSync,symlinkSync,writeFileSync} from "node:fs";')
    t=one(t,'return Object.freeze(real.map(stage=>stage.name==="workspace"?workspace:stage.name==="tools"?{...stage,arguments:["tools","setup",...tools==="all"?["all"]:tools]}:stage));','return Object.freeze(real.map(stage=>stage.name==="workspace"?workspace:stage.name==="tools"?{...stage,arguments:["tools","setup",...tools==="all"?["all"]:["toolchains",...tools]]}:stage));')
    t=one(t,'/** 🪜️ The plan of a worker, stage by stage','%s\n/** 🪜️ The plan of a worker, stage by stage'%HM_CODE.rstrip("\n"))
    return t
HT_TESTS='''
describe("slice lab home: registry, warm partitions, tool selection",()=>{
 test("the tools stage of a slice names the toolchains command (the bare ids are read as a build backend and refused)",async()=>{
  const{requireBuildBackendIdV1}=await import("../../../../🏗️build/🔌️backends/🟦️.ts");
  expect(()=>requireBuildBackendIdV1("bun")).toThrow();
  const workspace={name:"workspace",kind:"repo" as const,arguments:["run-many"],timeoutMilliseconds:1000,inputs:[],bootstrapInput:false};
  expect(sliceLabAllStagesV1(READY_STAGES_V1,workspace,["bun","go"]).find(stage=>stage.name==="tools")!.arguments).toEqual(["tools","setup","toolchains","bun","go"]);
  const{parseToolSetupSelectionV1}=await import("../../../../🏗️build/🧰️tool-setup/⌨️command/🟦️.ts");
  expect(parseToolSetupSelectionV1(["tools","setup","toolchains","bun","go"])).toEqual(["bun","go"]);
 });
 test("the refused registry is honoured by the pinned bun: its request goes to the loopback address of the files, nowhere else",async()=>{
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slw-t161-")));
  try{
   const seen:string[]=[],server=Bun.serve({port:0,hostname:"127.0.0.1",fetch:request=>{seen.push(new URL(request.url).pathname);return new Response("{}",{status:404});}});
   mkdirSync(join(directory,"h"));mkdirSync(join(directory,"p"));writeFileSync(join(directory,"p","package.json"),JSON.stringify({name:"g",dependencies:{"semio-r161-nonexistent-zzz":"1.0.0"}}));
   const made=sliceLabRegistryRefusalV1(join(directory,"h"));expect(made).toEqual([".npmrc",".bunfig.toml"]);
   expect(readFileSync(join(directory,"h",".npmrc"),"utf8")).toBe("registry=http://127.0.0.1:9/\\n");
   for(const name of made)writeFileSync(join(directory,"h",name),readFileSync(join(directory,"h",name),"utf8").replaceAll("http://127.0.0.1:9/",`http://127.0.0.1:${server.port}/`));
   const run=Bun.spawn([process.execPath,"install","--cache-dir",join(directory,"c")],{cwd:join(directory,"p"),env:{PATH:process.env.PATH!,HOME:join(directory,"h"),HTTPS_PROXY:"http://127.0.0.1:9",HTTP_PROXY:"http://127.0.0.1:9",NO_PROXY:"127.0.0.1"},stdout:"pipe",stderr:"pipe"});
   await run.exited;server.stop();
   expect(seen).toEqual(["/semio-r161-nonexistent-zzz"]);
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
 test("the warm table names known tools and stages, and a stage gets the items of its tools only",()=>{
  const known=new Set(TOOL_SETUP_V1.groups.all);
  for(const row of H.warm){expect(H.stageNames).toContain(row.stage);for(const item of row.items)for(const id of item.tools)expect(known.has(id)).toBe(true);}
  expect(sliceLabWarmItemsV1("dependencies",["bun","go"]).map(item=>item.id)).toEqual(["bun-cache"]);
  expect(sliceLabWarmItemsV1("tools",["bun","go"]).map(item=>item.id)).toEqual(["downloads","bun","go"]);
  expect(sliceLabWarmItemsV1("tools","all").length).toBe(H.warm.find(row=>row.stage==="tools")!.items.length);
  expect(sliceLabWarmItemsV1("workspace","all")).toEqual([]);
 });
 test("warming copies exactly the source tree into a mounted partition, leaves a filled target, and names why it skips",()=>{
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slw-t161-"))),source=join(directory,"warm"),state=join(directory,"s");
  try{
   mkdirSync(join(source,"d","bun-cache","@scope","pkg"),{recursive:true});writeFileSync(join(source,"d","bun-cache","@scope","pkg","a.txt"),"a");writeFileSync(join(source,"d","bun-cache","b b.txt"),"b");
   mkdirSync(join(state,"b","d","m","bun-cache"),{recursive:true});
   const mounted=()=>true;
   const first=sliceLabWarmV1({state,source,stage:"dependencies",tools:["bun","go"],mounted});
   expect(first.map(row=>[row.id,row.ok,row.method])).toEqual([["bun-cache",true,"copy"]]);
   const diff=spawnSync("diff",["-r",join(source,"d","bun-cache"),join(state,"b","d","m","bun-cache")],{encoding:"utf8"});expect(diff.status).toBe(0);
   expect(sliceLabWarmV1({state,source,stage:"dependencies",tools:["bun"],mounted}).map(row=>row.method)).toEqual(["present"]);
   rmSync(join(state,"b","d","m","bun-cache"),{recursive:true});mkdirSync(join(state,"b","d","m","bun-cache"));
   expect(sliceLabWarmV1({state,source,stage:"dependencies",tools:["bun"],mounted:()=>false}).map(row=>row.method)).toEqual(["skipped: not-mounted"]);
   expect(sliceLabWarmV1({state,source,stage:"tools",tools:["bun"],mounted}).map(row=>[row.id,row.method])).toEqual([["downloads","skipped: no-source"],["bun","skipped: no-source"]]);
   expect(sliceLabWarmV1({state,source:join(directory,"nowhere"),stage:"dependencies",tools:"all",mounted}).map(row=>row.method)).toEqual(["skipped: no-source"]);
   expect(()=>statSync(join(source,"d","bun-cache","@scope","pkg","a.txt"))).not.toThrow();
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
 test("the real partition test tells a plain directory from a mounted one by the device of its mount directory",()=>{
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slw-t161-"))),source=join(directory,"warm");
  try{
   mkdirSync(join(source,"d","bun-cache"),{recursive:true});writeFileSync(join(source,"d","bun-cache","x"),"x");mkdirSync(join(directory,"s","b","d","m","bun-cache"),{recursive:true});
   expect(sliceLabWarmV1({state:join(directory,"s"),source,stage:"dependencies",tools:["bun"]}).map(row=>row.method)).toEqual(["skipped: not-mounted"]);
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
 test("the warm directory is built read only from the live partitions and an item already there is kept",()=>{
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slw-t161-"))),live=join(directory,"live"),out=join(directory,"out");
  try{
   mkdirSync(join(live,".cache","b","d","m","bun-cache"),{recursive:true});writeFileSync(join(live,".cache","b","d","m","bun-cache","y"),"y");
   const rows=sliceLabWarmBuildV1(live,out);
   expect(rows.find(row=>row.id==="bun-cache")).toMatchObject({ok:true,method:"copy"});
   expect(readFileSync(join(out,"d","bun-cache","y"),"utf8")).toBe("y");
   expect(rows.filter(row=>row.id!=="bun-cache").every(row=>row.method==="skipped: no-source")).toBe(true);
   expect(sliceLabWarmBuildV1(live,out).find(row=>row.id==="bun-cache")!.method).toBe("present");
   expect(readFileSync(join(live,".cache","b","d","m","bun-cache","y"),"utf8")).toBe("y");
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
});
'''
def ht(t):
    t=one(t,'sliceLabAllStagesV1,','sliceLabAllStagesV1,sliceLabRegistryRefusalV1,sliceLabWarmBuildV1,sliceLabWarmItemsV1,sliceLabWarmV1,')
    t=one(t,'expect(plan.find(stage=>stage.name==="tools")!.arguments).toEqual(["tools","setup","bun","go"]);','expect(plan.find(stage=>stage.name==="tools")!.arguments).toEqual(["tools","setup","toolchains","bun","go"]);')
    t=t.rstrip("\n")+"\n"+HT_TESTS
    return t
def ls(t):
    return one(t,'        "skip": {"type": "array", "items": {"type": "string", "minLength": 1}},\n','        "skip": {"type": "array", "items": {"type": "string", "minLength": 1}},\n        "warm": {"type": "string", "minLength": 1},\n')
def lab(t):
    t=one(t,'sliceLabShimsV1,','sliceLabRegistryRefusalV1,sliceLabShimsV1,')
    t=one(t,'skip?:readonly string[];','skip?:readonly string[];warm?:string;')
    t=one(t,'export type SliceLabMachineResultV1=Readonly<{ready:boolean;','export type SliceLabMachineResultV1=Readonly<{warm?:readonly Readonly<{stage:string;id:string;ok:boolean;method:string;seconds:number;megabytes:number}>[];ready:boolean;')
    t=one(t,'for(const plan of plans){mkdirSync(plan.home,{recursive:true});sliceLabShimsV1(plan.home,','for(const plan of plans){mkdirSync(plan.home,{recursive:true});sliceLabRegistryRefusalV1(plan.home);sliceLabShimsV1(plan.home,')
    t=one(t,'const config={origin:plan.origin,root:plan.root,group:P.group,stages,state:plan.state,','const config={origin:plan.origin,root:plan.root,group:P.group,stages,state:plan.state,...options.warm===undefined?{}:{warm:{source:options.warm,tools}},')
    t=one(t,'seeds:seedRows,','seeds:[...seedRows,...plans.flatMap(plan=>(results[plan.origin]?.warm??[]).map(row=>({machine:plan.origin,id:`warm:${row.stage}:${row.id}`,ok:row.ok,method:row.method+(row.megabytes>0?` ${row.megabytes} MB`:""),seconds:row.seconds})))],')
    t=one(t,' const live=resolve(root),driftMarker',' if(options.warm!==undefined&&!existsSync(options.warm))throw Error(`The warm directory ${options.warm} is absent; build it with slice-lab warm --out=<dir> / Das Warm-Verzeichnis ${options.warm} fehlt; mit slice-lab warm --out=<dir> bauen`);\n const live=resolve(root),driftMarker')
    return t
def script(t):
    t=one(t,'...option("kinds")===undefined?{}:','...option("warm")===undefined?{}:{warm:option("warm")},...option("kinds")===undefined?{}:')
    t=one(t,' if(argv[0]!=="run"){',' if(argv[0]==="warm"){const out=option("out");if(out===undefined){process.stderr.write("slice-lab warm --out=<dir>\\n");return 2;}const rows=sliceLabWarmBuildV1(process.env.SEMIO_WORKSPACE_ROOT??process.cwd(),out);process.stdout.write(rows.map(row=>`${row.stage} ${row.id} ${row.method} ${row.seconds}s ${row.megabytes} MB`).join("\\n")+"\\n");return rows.every(row=>row.ok)?0:1;}\n if(argv[0]!=="run"){')
    t=one(t,'import {sliceLabHomeReportV1} from','import {sliceLabHomeReportV1,sliceLabWarmBuildV1} from')
    return t
def drv(t):
    t=one(t,'import {dirname} from "node:path";','import {dirname} from "node:path";\nimport {sliceLabWarmV1,type SliceLabWarmRowV1} from "../🏠️home/🟦️.ts";')
    t=one(t,'type Config={origin:string;root:string;group:string;stages:readonly ReadyStageV1[];state:string;result:string;timeoutSeconds:number};','type Config={origin:string;root:string;group:string;stages:readonly ReadyStageV1[];state:string;result:string;timeoutSeconds:number;warm?:{source:string;tools:readonly string[]|"all"}};')
    t=one(t,'const wrapped={...ports,timing:(run:ReadyTimingV1)=>{timings.push(run);timing?.(run);}};','const warmed:SliceLabWarmRowV1[]=[],warmSeconds:Record<string,number>={};\nconst wrapped={...ports,...config.warm===undefined?{}:{run:async(stage:ReadyStageV1,options:Parameters<typeof ports.run>[1])=>{const started=performance.now(),rows=sliceLabWarmV1({state:config.state,source:config.warm!.source,stage:stage.name,tools:config.warm!.tools});warmed.push(...rows);warmSeconds[stage.name]=(performance.now()-started)/1000;return ports.run(stage,options);}},timing:(run:ReadyTimingV1)=>{const spent=warmSeconds[run.stage]??0;const row=spent>0?{...run,seconds:Math.max(0,run.seconds-spent)}:run;timings.push(row);timing?.(run);}};')
    t=one(t,'file({ready,failure:','file({warm:warmed,ready,failure:')
    return t
PATCH={"hs":hs,"hm":hm,"ht":ht,"ls":ls,"lab":lab,"script":script,"drv":drv}
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    os.makedirs(backup,exist_ok=True);manifest={};new={}
    for k,p in FILES.items(): new[k]=PATCH[k](read(loc(p)))
    for i,(k,p) in enumerate(FILES.items()):
        old=read(loc(p));write(backup+"/%d.bak"%i,old);manifest[k]={"i":i,"before":sha(old),"after":sha(new[k])}
    json.dump(manifest,open(backup+"/manifest.json","w"))
    for k,p in FILES.items(): write(loc(p),new[k])
    print("applied")
elif cmd=="restore":
    manifest=json.load(open(backup+"/manifest.json"));bad=[]
    for k,p in FILES.items():
        m=manifest[k];now=sha(read(loc(p)))
        if now==m["after"]: write(loc(p),read(backup+"/%d.bak"%m["i"]))
        elif now!=m["before"]: bad.append(k)
    print("CONFLICT" if bad else "restored",bad)
    sys.exit(4 if bad else 0)
