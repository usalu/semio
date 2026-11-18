#!/usr/bin/env python3
"""R77b stage builder: the file-count ceilings of the source-closure family are one declared set (`SourceClosureFamily` in the authorizer schema) that a law walks against every schema of build and caching; the bazel scope traces and the revision fixture follow the family bound."""
import os,shutil,sys,re,json
R="/Users/ueli/Documents/semio/"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
AU=L+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/"
H=os.path.dirname(os.path.abspath(__file__))+"/stage/"
shutil.rmtree(H,ignore_errors=True);os.makedirs(H)
rows=[];n=[0]
def stage(rel,edits=None,new=None):
    i="f%d"%n[0];n[0]+=1
    if new is not None:
        open(H+i+".new","w",encoding="utf8").write(new);rows.append("%s %s new"%(i,rel));return
    text=open(R+rel,encoding="utf8").read();base=text
    for old,rep,count in edits:
        got=text.count(old)
        if got!=count:sys.exit("anchor count %d != %d in %s: %r"%(got,count,rel,old[:80]))
        text=text.replace(old,rep)
    open(H+i+".base","w",encoding="utf8").write(base);open(H+i+".new","w",encoding="utf8").write(text);rows.append("%s %s mod"%(i,rel))
names=re.compile(r"^(sources|files|images|modules|reads|physical|sourceCount|closure|paths)$")
FAM={"files":"closure","sources":"closure","images":"closure","sourceCount":"closure"}
hits=[]
def walk(node,ptr,f):
    if isinstance(node,dict):
        for k,v in node.items():
            if isinstance(v,dict) and names.match(k):
                for key in("maxItems","maximum"):
                    if isinstance(v.get(key),int) and v[key]>=64:hits.append((f,"/".join(ptr+[k]),key,v[key]))
            walk(v,ptr+[k],f)
    elif isinstance(node,list):
        for i,v in enumerate(node):walk(v,ptr+[str(i)],f)
for sub in[B,L+"⚡️caching/"]:
    for d,_,fs in os.walk(R+sub):
        if "🤖️generated" in d or "node_modules" in d or "/🧪️tests" in d:continue
        for fn in fs:
            if fn.endswith(".json") and ("🧬️schema" in d or fn=="schema.json"):
                p=os.path.join(d,fn)
                try:walk(json.load(open(p,encoding="utf8")),[],p[len(R)+len(L):])
                except Exception:pass
CL="🏗️build/📥️source/🔏️closure/🧬️schema/🔣️.json";RV="🏗️build/📥️inputs/🔏️revision/🧬️schema/🔣️.json";AUS="⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧬️schema/🔣️.json";SN="⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🧬️schema/🔣️.json";BZ="🏗️build/🔧️tools/🏗️bazel/🎬️scopes/🧬️schema/🔣️.json"
members=[];exempt=[]
reasons={
 "list":("A hand-authored list of at most this many entries, not derived from a source closure.","Eine von Hand geschriebene Liste mit höchstens so vielen Einträgen, nicht aus einem Quellabschluss abgeleitet."),
 "batch":("One request or event batch, not the source closure.","Ein Anfrage- oder Ereignisstapel, nicht der Quellabschluss."),
 "cargo":("Rust crate sources: a separate closure family with its own bound.","Rust-Crate-Quellen: eine eigene Abschlussfamilie mit eigener Grenze."),
 "package":("The file roster of one dependency package or artifact, not the source closure.","Die Dateiliste eines Abhängigkeitspakets oder Artefakts, nicht der Quellabschluss."),
}
def cat(f,p):
    if "cargo" in f:return "cargo"
    if any(x in f for x in("pruning","artifacts","wasm","dependencies","parsers","derivative")):return "package"
    if any(x in f for x in("sealed","resident","watch")):return "batch"
    return "list"
for f,p,key,v in sorted(hits):
    if f in(CL,RV,AUS,SN) and v>=2048:
        members.append({"file":f,"pointer":p,"key":key,"offset":v-16384 if v>=16384 else 0}) if v in(16384,16385,2048,2049) else None
    elif f==BZ and v==2048:members.append({"file":f,"pointer":p,"key":key,"offset":0})
    elif v<16384:exempt.append({"file":f,"pointer":p,"key":key,"reason":{"en":reasons[cat(f,p)][0],"de":reasons[cat(f,p)][1]}})
for m in members:
    if m["file"]!=BZ and m["offset"]<0:m["offset"]=0
for m in members:
    m["offset"]=1 if m["pointer"].endswith("SourceCapacityDiagnostic/properties/files") else 0
fam={"version":1,"bound":"closure.definitions.Policy.maximumFiles","members":members,"exempt":exempt}
text=json.dumps(fam,ensure_ascii=False,indent=2)
text="\n".join("    "+l if i else l for i,l in enumerate(text.split("\n")))
stage(AU+"🧬️schema/🔣️.json",[('    "SourceCapacityTexts": {','    "SourceClosureFamily": { "const": '+text+' },\n    "SourceCapacityTexts": {',1)])
stage(B+"🔧️tools/🏗️bazel/🎬️scopes/🧬️schema/🔣️.json",[('"maxItems": 2048,','"maxItems": 16384,',3)])
stage(B+"📥️inputs/🔏️revision/🧪️tests/🔣️.json",[('"sources": 2048,','"sources": 16384,',1),('"sourceCount": 2048\n','"sourceCount": 16384\n',1)])
GT=AU+"🧪️tests/📈️growth/🟦️.ts"

LAW='''
test("every file-count ceiling of the closure family is declared once, equals the family bound and clears the real closure by a quarter",async()=>{
 const family=schema.$defs.SourceClosureFamily.const,bound=policy.maximumFiles,library=fileURLToPath(new URL("../../../../../../",import.meta.url));
 const read=(file:string)=>JSON.parse(readFileSync(library+file,"utf8")),at=(file:string,pointer:string)=>pointer.split("/").reduce((node:any,key)=>node[key],read(file)),declared=new Set<string>();
 for(const row of family.members){declared.add(row.file+"#"+row.pointer+"#"+row.key);expect(at(row.file,row.pointer)[row.key],row.file+" "+row.pointer).toBe(bound+row.offset);}
 for(const row of family.exempt){declared.add(row.file+"#"+row.pointer+"#"+row.key);const value=at(row.file,row.pointer)[row.key];expect(value,row.file+" "+row.pointer).toBeLessThan(bound);expect(row.reason.en.length&&row.reason.de.length,row.pointer).toBeGreaterThan(0);}
 const names=/^(sources|files|images|modules|reads|physical|sourceCount|closure|paths)$/u,undeclared:string[]=[];
 const walk=(node:any,pointer:string[],file:string)=>{if(Array.isArray(node)){node.forEach((value,index)=>walk(value,[...pointer,String(index)],file));return;}if(!node||typeof node!=="object")return;for(const[key,value]of Object.entries<any>(node)){if(value&&typeof value==="object"&&!Array.isArray(value)&&names.test(key))for(const limit of["maxItems","maximum"])if(Number.isInteger(value[limit])&&value[limit]>=64&&value[limit]<bound&&!declared.has(file+"#"+[...pointer,key].join("/")+"#"+limit))undeclared.push(file+" "+[...pointer,key].join("/")+" "+limit+"="+value[limit]);walk(value,[...pointer,key],file);}};
 for(const sub of["🏗️build","⚡️caching"])for(const rel of readdirSync(library+sub,{recursive:true})as string[]){if(!rel.endsWith(".json")||!(rel.includes("🧬️schema")||rel.endsWith("schema.json"))||rel.includes("🧪️tests")||rel.includes("🤖️generated")||rel.includes("node_modules"))continue;const file=sub+"/"+rel;let value:unknown;try{value=read(file);}catch{continue;}walk(value,[],file);}
 expect(undeclared,"ceilings below the closure bound that are neither family members nor exempt with a reason; list them in SourceClosureFamily").toEqual([]);
 let at_=dirname(fileURLToPath(import.meta.url));while(!existsSync(join(at_,"go.work"))){const up=dirname(at_);if(up===at_)throw Error("Workspace root required");at_=up;}
 const census=await censusNxAuthorizerSourcesV1(at_,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts",AbortSignal.timeout(240000),{installed:true});
 expect(bound,"files "+census.files).toBeGreaterThanOrEqual(Math.ceil(census.files*1.25));expect(policy.maximumSourceBytes,"bytes "+census.sourceBytes).toBeGreaterThanOrEqual(Math.ceil(census.sourceBytes*1.25));
},300000);
'''
stage(GT,[
 ('import{join,relative}from"node:path";','import{dirname,join,relative}from"node:path";\nimport{existsSync,readdirSync,readFileSync}from"node:fs";\nimport{fileURLToPath}from"node:url";',1),
 ('import{captureNxAuthorizerSourcesV1,nxAuthorizerCapacityTextV1}from"../../🟦️.ts";','import{captureNxAuthorizerSourcesV1,censusNxAuthorizerSourcesV1,nxAuthorizerCapacityTextV1}from"../../🟦️.ts";',1),
 ('\ntest("the capacity text fills every field in both languages"',LAW+'\ntest("the capacity text fills every field in both languages"',1),
])
DP=L+"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/"
stage(DP+"🟦️.ts",[
 ('await control.publish({resource:"dependencies:store-input-replay",mode:"shared",elapsedMs:0});','await control.publish({resource:"dependencies:store-input-replay:"+work.completed,mode:"shared",elapsedMs:0});',1),
 ('if(++published%installSchema.$defs.InstalledPathProtocol.const.fullFenceInterval===0)await fence();else{','if(++published===1||published%8===0)await control.publish({resource:"dependencies:publish-binding:"+published,mode:"shared",elapsedMs:0});if(published%installSchema.$defs.InstalledPathProtocol.const.fullFenceInterval===0)await fence();else{',1),
])
PROGRESS='''/** 📶️ Contract: every long phase of the dependency producer names a counter in its progress lines, so the stall rule of the ready stage (a line is new when it differs after the elapsed-seconds counters are replaced) cuts a hung phase and never a slow phase that moves: the input replay and the Source endpoint publication of a Windows store take longer than the stall limit. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{readyStallTrackerV1}from"../../../../../../🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🟦️.ts";

const source=readFileSync(fileURLToPath(new URL("../../🟦️.ts",import.meta.url)),"utf8");
const line=(resource:string)=>'[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"'+resource+'","mode":"shared","elapsedMs":0}}';

test("the input replay and the publication loop publish a counter",()=>{
 expect(source).toContain('resource:"dependencies:store-input-replay:"+work.completed');
 const from=source.indexOf("await publishJavascriptDependencyPreservingSourceDescriptorV1(binding"),to=source.indexOf("await verifyReady();const finals"),loop=source.slice(from,to);
 expect(from).toBeGreaterThan(0);expect(to).toBeGreaterThan(from);expect(loop).toContain('resource:"dependencies:publish-binding:"+published');
});

test("a moving counter is progress for the stall rule and a repeated line is not",()=>{
 for(const phase of["dependencies:store-input-replay:","dependencies:publish-binding:"]){
  const tracker=readyStallTrackerV1();expect(tracker.observe([line(phase+"64")]),phase+" first").toBe(true);expect(tracker.observe([line(phase+"64")]),phase+" repeated").toBe(false);expect(tracker.observe([line(phase+"128")]),phase+" moved").toBe(true);
 }
 const old=readyStallTrackerV1();expect(old.observe([line("dependencies:store-input-replay")])).toBe(true);expect(old.observe([line("dependencies:store-input-replay")])).toBe(false);
});
'''
stage(DP+"🧪️tests/📶️progress/🟦️.ts",new=PROGRESS)
print(len(members),"members",len(exempt),"exempt")
for m in members:print(m["file"][-50:],m["pointer"][-60:],m["key"],m["offset"])
open(H+"index.txt","w",encoding="utf8").write("\n".join(rows)+"\n")
