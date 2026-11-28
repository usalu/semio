import os,sys,json
R="/Users/ueli/Documents/semio/"
G=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/"
def rd(p): return open(p,encoding="utf8").read()
def wr(p,s):
    t=p+".h9tmp"; open(t,"w",encoding="utf8").write(s); os.replace(t,p)
def sub(s,old,new):
    assert s.count(old)==1,(old[:60],s.count(old)); return s.replace(old,new)
files={}
p=G+"🕰️generations/🪡️edge-heal/🟨️.mjs"; s=rd(p)
s=sub(s,"export function staleEdgeOwnersV1(owners,environment){\n const edges=[],tagged=[];\n for(const name of Object.keys(owners).sort(compareCodePointsV1))if(owners[name].edgesEnv!==environment)","export function staleEdgeOwnersV1(owners,environment,pending=[]){\n const edges=[],tagged=[],later=new Set(pending);\n for(const name of Object.keys(owners).sort(compareCodePointsV1))if(owners[name].edgesEnv!==environment&&!later.has(name))")
s=sub(s,"export function edgeHealPlanV1({owners,environment,cold,healed,previous,tried=[],limit=2}){\n if(cold)return{edges:[],infer:[]};\n const{edges,tagged}=staleEdgeOwnersV1(owners,environment);","export function edgeHealPlanV1({owners,environment,cold,healed,previous,tried=[],pending=[],limit=2}){\n if(cold)return{edges:[],infer:[]};\n const{edges,tagged}=staleEdgeOwnersV1(owners,environment,pending);")
s=sub(s,"export function requireEdgeEnvironmentV1({owners,environment,fail,wave,returned,dropped,carried,cold,repair,healed,resumed,before,previous,spec}){\n for(const[name,row]of Object.entries(owners))if(row.edgesEnv!==environment)throw","export function requireEdgeEnvironmentV1({owners,environment,fail,wave,returned,dropped,carried,cold,repair,healed,resumed,before,previous,spec,pending=[]}){\n const later=new Set(pending);\n for(const[name,row]of Object.entries(owners))if(row.edgesEnv!==environment&&!later.has(name))throw")
s=sub(s,"the first round gives","the first round gives") if False else s
s=sub(s,"every round gives the untagged owners","every round gives (owners still `pending` for inference in a later wave of the read are never stale: the wave infers them under the then published environment, so neither the heal nor the publication check looks at them) the untagged owners")
files[p]=s
p=G+"🕰️generations/🪡️edge-heal/🟨️.d.mts"; s=rd(p)
s=sub(s,"previous?:number;tried?:Iterable<string>;limit?:number}","previous?:number;tried?:Iterable<string>;pending?:Iterable<string>;limit?:number}")
s=sub(s,"previous:string|null|undefined;spec?:","previous:string|null|undefined;pending?:Iterable<string>;spec?:")
files[p]=s
p=G+"🕰️generations/🟨️.mjs"; s=rd(p)
s=sub(s,"previous:carry.healStale,tried:carry.edgeTried})","previous:carry.healStale,tried:carry.edgeTried,pending:rest})")
s=sub(s,"before:owners0,previous:spec.previousEnv,spec});","before:owners0,previous:spec.previousEnv,spec,pending:rest});")
files[p]=s
p=G+"🧪️tests/🔗️edge-refresh/🐍️oracle/🐍️.py"; s=rd(p)
s=sub(s,'    stale = sorted(name for name, row in heal["owners"].items() if row["edgesEnv"] != heal["environment"])\n','    pending = set(heal.get("pending", []))\n    stale = sorted(name for name, row in heal["owners"].items() if row["edgesEnv"] != heal["environment"] and name not in pending)\n')
files[p]=s
p=G+"🧪️tests/🔗️edge-refresh/🔣️.json"; d=json.loads(rd(p)); w=d["worlds"]
w.append({"id":"heal: an owner still pending for inference in a later wave is not stale","world":{"heal":{"owners":{"a":{"edgesEnv":"e2"},"b":{"edgesEnv":"e1"},"c":{"edgesEnv":"e0"},"t":{"edgesEnv":"e1","tag":"tests-v2"}},"environment":"e2","cold":False,"healed":2,"previous":1,"pending":["c","t"]}}})
w.append({"id":"heal: every stale owner is pending, nothing to do","world":{"heal":{"owners":{"b":{"edgesEnv":"e1"},"c":{"edgesEnv":"e0"}},"environment":"e2","cold":False,"healed":3,"pending":["b","c"]}}})
files[p]=json.dumps(d,ensure_ascii=False,indent=1)+"\n"
p=G+"🧪️tests/🔗️edge-refresh/🟦️.ts"; s=rd(p)
s=sub(s,"tried?:string[]}}","tried?:string[];pending?:string[]}}")
s=s.rstrip("\n")+'''

test("an owner that waits for inference in a later wave is neither healed nor refused (read 3 and 5, @leutwiler/realparts-of-powers-z-n)",()=>{
 const heal=(i:number)=>(fixture.worlds[i]!.world as any).heal;
 expect(edgeHealPlanV1(heal(23))).toEqual({edges:[],infer:[]});
 expect(edgeHealPlanV1(heal(24))).toEqual({edges:[],infer:[]});
 const refuse=(pending:string[])=>requireEdgeEnvironmentV1({owners:heal(24).owners,environment:"e2",fail:(message:string)=>new Error(message),wave:[],returned:new Set<string>(),dropped:new Set<string>(),cold:false,repair:true,healed:4,resumed:false,before:{},previous:"e2",pending});
 expect(()=>refuse(["b"])).toThrow("Every owner needs edges of the published environment: c");
 expect(()=>refuse(["b","c"])).not.toThrow();
});
'''
s=sub(s,'import{edgeHealPlanV1}from"../../🕰️generations/🪡️edge-heal/🟨️.mjs";','import{edgeHealPlanV1,requireEdgeEnvironmentV1}from"../../🕰️generations/🪡️edge-heal/🟨️.mjs";')
files[p]=s
if "--check" in sys.argv: print("anchors ok",len(files)); sys.exit(0)
for p,s in files.items(): wr(p,s)
print("landed",len(files))
