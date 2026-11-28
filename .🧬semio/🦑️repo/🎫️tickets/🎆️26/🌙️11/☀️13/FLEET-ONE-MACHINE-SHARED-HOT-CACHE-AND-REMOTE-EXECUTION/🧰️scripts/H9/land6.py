import os,sys,json,re
R="/Users/ueli/Documents/semio/"
G=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/"
def rd(p): return open(p,encoding="utf8").read()
def wr(p,s):
    t=p+".h9tmp"; open(t,"w",encoding="utf8").write(s); os.replace(t,p)
def sub(s,old,new):
    assert s.count(old)==1,(old[:70],s.count(old)); return s.replace(old,new)
files={}
p=G+"🕰️generations/🪡️edge-heal/🟨️.mjs"; s=rd(p)
a=s.index("/** 🩹️ What a build does about owners")
b=s.index("/** 🚧️ Refuses")
new='''/** 🩹️ What a build does about owners that are still stale after its compute (owners `pending` for inference in a later wave of the read are never stale: their wave infers them under the then published environment). Every stale untagged owner is first given its edges again without inference (`edges`); an owner that stayed stale under the same environment although it was given its edges, and every stale tagged owner, is inferred (`infer`) once per environment; whatever is left after both has nothing left to try and the caller refuses the publication with the diagnosis. `tried` and `inferred` are the names (arrays or sets) or the maps name to environment of earlier rounds, so a moving environment starts every owner afresh and a round that repairs nothing for an owner is never repeated for it; `limit` only bounds the number of rounds of a read against a runaway. A cold build tries nothing. */
export function edgeHealPlanV1({owners,environment,cold,healed,tried,inferred,pending=[],limit=64}){
 if(cold||healed>=limit)return{edges:[],infer:[]};
 const{edges,tagged}=staleEdgeOwnersV1(owners,environment,pending),seen=(source,name)=>source instanceof Map?source.get(name)===environment:Array.isArray(source)?source.includes(name):source instanceof Set&&source.has(name);
 return{edges:edges.filter(name=>!seen(tried,name)),infer:[...tagged,...edges.filter(name=>seen(tried,name))].filter(name=>!seen(inferred,name)).sort(compareCodePointsV1)};
}

'''
s=s[:a]+new+s[b:]
files[p]=s
p=G+"🕰️generations/🪡️edge-heal/🟨️.d.mts"; s=rd(p)
s=sub(s,"previous?:number;tried?:Iterable<string>;pending?:Iterable<string>;limit?:number}","tried?:ReadonlyMap<string,string>|readonly string[]|ReadonlySet<string>;inferred?:ReadonlyMap<string,string>|readonly string[]|ReadonlySet<string>;pending?:Iterable<string>;limit?:number}")
files[p]=s
p=G+"🕰️generations/🟨️.mjs"; s=rd(p)
s=sub(s,"healed:carry.healed,previous:carry.healStale,tried:carry.edgeTried,pending:rest})","healed:carry.healed,tried:carry.edgeTried,inferred:carry.healInferred,pending:rest})")
s=sub(s,"carry.healStale=healing;for(const name of healEdges)(carry.edgeTried??=new Set).add(name);","for(const name of healEdges)(carry.edgeTried??=new Map).set(name,envDigest);for(const name of healInfer)(carry.healInferred??=new Map).set(name,envDigest);")
files[p]=s
p=G+"🧪️tests/🔗️edge-refresh/🐍️oracle/🐍️.py"; s=rd(p)
a=s.index("def heal(world):"); b=s.index("def refresh(world):")
s=s[:a]+'''def heal(world):
    heal = world["heal"]
    if heal["cold"] or heal["healed"] >= heal.get("limit", 64):
        return {"edges": [], "infer": []}
    pending = set(heal.get("pending", []))
    stale = sorted(name for name, row in heal["owners"].items() if row["edgesEnv"] != heal["environment"] and name not in pending)
    edges = [name for name in stale if not heal["owners"][name].get("tag")]
    tagged = [name for name in stale if heal["owners"][name].get("tag")]
    tried, inferred = set(heal.get("tried", [])), set(heal.get("inferred", []))
    return {"edges": [name for name in edges if name not in tried], "infer": sorted(name for name in tagged + [name for name in edges if name in tried] if name not in inferred)}


'''+s[b:]
s=sub(s,"A world with a `heal` key is a stored owner table after a compute: the stale untagged owners get their edges again in the first round, tagged ones and whatever the first round could not repair are inferred, nothing is left after two rounds that repaired nothing (a round that shrank the stale set, `previous` before it, goes on) or on a cold build.","A world with a `heal` key is a stored owner table after a compute: the stale untagged owners get their edges again first, tagged ones and the untagged ones that stayed stale although they were given their edges (`tried`) are inferred once (`inferred`), owners `pending` for a later wave are never stale, nothing is left on a cold build or after `limit` rounds.") if "A world with a `heal` key" in s and "(a round that shrank" in s else s
files[p]=s
p=G+"🧪️tests/🔗️edge-refresh/🔣️.json"; d=json.loads(rd(p)); w=d["worlds"]
w[16]["id"]="heal after the round limit: nothing is left to try"; w[16]["world"]["heal"]["healed"]=64
base=lambda **k:{"owners":{"a":{"edgesEnv":"e2"},"b":{"edgesEnv":"e1"},"c":{"edgesEnv":"e0"},"t":{"edgesEnv":"e1","tag":"tests-v2"}},"environment":"e2","cold":False,"healed":5,**k}
w[19]={"id":"heal: an owner that was given its edges and inferred once is not inferred again","world":{"heal":base(tried=["b","c"],inferred=["b"])}}
w[20]={"id":"heal: every stale owner was given its edges and inferred: nothing is left to try","world":{"heal":base(tried=["b","c"],inferred=["b","c","t"])}}
w[21]={"id":"heal: five rounds in, a new stale owner is still given its edges first","world":{"heal":base(tried=["b"],inferred=["t"])}}
w[23]["world"]["heal"].pop("previous",None); w[23]["world"]["heal"].update({"healed":2,"tried":["b"],"inferred":["b"]})
w[23]["world"]["heal"]["owners"].pop("c"); w[23]["world"]["heal"]["owners"]["c"]={"edgesEnv":"e0"}
w[24]["world"]["heal"].pop("previous",None)
files[p]=json.dumps(d,ensure_ascii=False,indent=1)+"\n"
p=G+"🧪️tests/🔗️edge-refresh/🟦️.ts"; s=rd(p)
s=sub(s,"tried?:string[];pending?:string[]}}","tried?:string[];inferred?:string[];pending?:string[]}}")
s=sub(s,'''test("the heal limit bounds the rounds that repair nothing, never the owners",()=>{
 const heal=(i:number)=>(fixture.worlds[i]!.world as any).heal;
 expect(edgeHealPlanV1(heal(19))).toEqual({edges:[],infer:["b","c","t"]});
 expect(edgeHealPlanV1(heal(20))).toEqual({edges:[],infer:[]});
 expect(edgeHealPlanV1(heal(21))).toEqual({edges:[],infer:["b","c","t"]});
});''','''test("an owner is given its edges, then inferred once, then nothing is left; the limit only bounds a runaway",()=>{
 const heal=(i:number)=>(fixture.worlds[i]!.world as any).heal;
 expect(edgeHealPlanV1(heal(19))).toEqual({edges:[],infer:["c","t"]});
 expect(edgeHealPlanV1(heal(20))).toEqual({edges:[],infer:[]});
 expect(edgeHealPlanV1(heal(21))).toEqual({edges:["c"],infer:["b"]});
 expect(edgeHealPlanV1(heal(16))).toEqual({edges:[],infer:[]});
 const env=(name:string)=>new Map([[name,"e2"]]),old=new Map([["b","e1"]]);
 expect(edgeHealPlanV1({...heal(19),tried:env("b"),inferred:new Map()})).toEqual({edges:["c"],infer:["b","t"]});
 expect(edgeHealPlanV1({...heal(19),tried:old,inferred:old})).toEqual({edges:["b","c"],infer:["t"]});
});''')
files[p]=s
if "--check" in sys.argv: print("anchors ok",len(files)); sys.exit(0)
for p,s in files.items(): wr(p,s)
print("landed",len(files))
