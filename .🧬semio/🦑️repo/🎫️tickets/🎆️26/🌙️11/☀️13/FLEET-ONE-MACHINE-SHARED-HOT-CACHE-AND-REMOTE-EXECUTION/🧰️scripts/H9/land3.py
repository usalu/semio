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
s=sub(s,"export function edgeHealPlanV1({owners,environment,cold,healed,previous,limit=2}){","export function edgeHealPlanV1({owners,environment,cold,healed,previous,tried=[],limit=2}){")
s=sub(s,"return healed===0?{edges,infer:tagged}:{edges:[],infer:[...edges,...tagged].sort(compareCodePointsV1)};","const seen=new Set(tried);\n return{edges:edges.filter(name=>!seen.has(name)),infer:[...tagged,...edges.filter(name=>seen.has(name))].sort(compareCodePointsV1)};")
s=sub(s,"the first round gives the untagged ones their edges again without inference (`edges`) and infers the tagged ones (`infer`); the second round infers whatever the first could not repair;","every round gives the untagged owners that were not given their edges again before (`tried`) their edges without inference (`edges`) and infers the tagged ones and the untagged ones that stayed stale although they were given their edges (`infer`), so a moving environment never turns a cheap refresh into the inference of thousands of owners;")
files[p]=s
p=G+"🕰️generations/🪡️edge-heal/🟨️.d.mts"; s=rd(p)
s=sub(s,"previous?:number;limit?:number}","previous?:number;tried?:Iterable<string>;limit?:number}")
files[p]=s
p=G+"🕰️generations/🟨️.mjs"; s=rd(p)
s=sub(s,"previous:carry.healStale}),healing=","previous:carry.healStale,tried:carry.edgeTried}),healing=")
s=sub(s,"carry.healStale=healing;","carry.healStale=healing;for(const name of healEdges)(carry.edgeTried??=new Set).add(name);")
files[p]=s
p=G+"🧪️tests/🔗️edge-refresh/🐍️oracle/🐍️.py"; s=rd(p)
s=sub(s,'    if heal["healed"] == 0:\n        return {"edges": edges, "infer": tagged}\n    return {"edges": [], "infer": sorted(edges + tagged)}\n','    tried = set(heal.get("tried", []))\n    return {"edges": [name for name in edges if name not in tried], "infer": sorted(tagged + [name for name in edges if name in tried])}\n')
files[p]=s
p=G+"🧪️tests/🔗️edge-refresh/🔣️.json"; d=json.loads(rd(p)); w=d["worlds"]
for i in (15,19,21): w[i]["world"]["heal"]["tried"]=["b","c"]
w.append({"id":"heal: only the untagged owners that were given their edges and stayed stale are inferred","world":{"heal":{"owners":{"a":{"edgesEnv":"e2"},"b":{"edgesEnv":"e1"},"c":{"edgesEnv":"e0"},"t":{"edgesEnv":"e1","tag":"tests-v2"}},"environment":"e2","cold":False,"healed":1,"tried":["b"]}}})
files[p]=json.dumps(d,ensure_ascii=False,indent=1)+"\n"
p=G+"🧪️tests/🔗️edge-refresh/🟦️.ts"; s=rd(p)
s=sub(s,"healed:number;previous?:number}}","healed:number;previous?:number;tried?:string[]}}")
s=s.rstrip("\n")+'''

test("a stale owner is given its edges before it is ever inferred",()=>{
 expect(edgeHealPlanV1((fixture.worlds[22]!.world as any).heal)).toEqual({edges:["c"],infer:["b","t"]});
});
'''
files[p]=s
if "--check" in sys.argv: print("anchors ok",len(files)); sys.exit(0)
for p,s in files.items(): wr(p,s)
print("landed",len(files))
