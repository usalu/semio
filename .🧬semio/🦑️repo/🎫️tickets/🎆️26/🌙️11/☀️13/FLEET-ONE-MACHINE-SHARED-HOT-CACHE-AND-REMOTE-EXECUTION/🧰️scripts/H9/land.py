import json,os,sys
R="/Users/ueli/Documents/semio/"
G=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/"
def rd(p): return open(p,encoding="utf8").read()
def wr(p,s):
    t=p+".h9tmp"; open(t,"w",encoding="utf8").write(s); os.replace(t,p)
def sub(s,old,new):
    assert s.count(old)==1,(old[:60],s.count(old)); return s.replace(old,new)
files={}
p=G+"🤝️prefetch/🟦️.ts"; s=rd(p)
s=sub(s,"&&!input.roots.has(stub.root)&&(moved||forced.has(name))),projects=kept","&&(forced.has(name)||moved&&!input.roots.has(stub.root))),projects=kept")
s=sub(s,"always refreshed, never carried, also while the environment stays)","always refreshed, never carried, also while the environment stays, also under a scanned root: an owner the scan did not infer again is never left behind)")
files[p]=s
p=G+"🕰️generations/🪡️edge-heal/🟨️.mjs"; s=rd(p)
s=sub(s,"export function edgeHealPlanV1({owners,environment,cold,healed,limit=2}){\n if(cold||healed>=limit)return{edges:[],infer:[]};\n const{edges,tagged}=staleEdgeOwnersV1(owners,environment);\n",
"export function edgeHealPlanV1({owners,environment,cold,healed,previous,limit=2}){\n if(cold)return{edges:[],infer:[]};\n const{edges,tagged}=staleEdgeOwnersV1(owners,environment);\n if(healed>=limit&&!(edges.length+tagged.length<previous))return{edges:[],infer:[]};\n")
s=sub(s,"after `limit` rounds, or on a cold build, nothing is left to try and the caller refuses the publication.","after `limit` rounds nothing is left to try and the caller refuses the publication unless the last round shrank the stale set (`previous` stale owners before it): `limit` bounds the rounds that repair nothing, never the owners, so a read whose rounds keep repairing owners goes on; on a cold build nothing is tried.")
files[p]=s
p=G+"🕰️generations/🪡️edge-heal/🟨️.d.mts"; s=rd(p)
s=sub(s,"cold:boolean;healed:number;limit?:number}","cold:boolean;healed:number;previous?:number;limit?:number}")
files[p]=s
p=G+"🕰️generations/🟨️.mjs"; s=rd(p)
s=sub(s,"cold,healed:carry.healed}),healing=","cold,healed:carry.healed,previous:carry.healStale}),healing=")
s=sub(s,"if(healing){carry.healed++;","if(healing){carry.healed++;carry.healStale=healing;")
files[p]=s
p=G+"🧪️tests/🔗️edge-refresh/🐍️oracle/🐍️.py"; s=rd(p)
s=sub(s,'    if heal["cold"] or heal["healed"] >= 2:\n        return {"edges": [], "infer": []}\n    stale = sorted(name for name, row in heal["owners"].items() if row["edgesEnv"] != heal["environment"])\n',
'    if heal["cold"]:\n        return {"edges": [], "infer": []}\n    stale = sorted(name for name, row in heal["owners"].items() if row["edgesEnv"] != heal["environment"])\n    if heal["healed"] >= 2 and not (len(stale) < heal.get("previous", 0)):\n        return {"edges": [], "infer": []}\n')
s=sub(s,"or when they are named stale (their stored edges are of another environment: always refreshed, never carried, also while the environment stays)","or when they are named stale (their stored edges are of another environment: always refreshed, never carried, also while the environment stays, also under a scanned root)")
s=sub(s,'if stub.get("tag") or name in world["inferred"] or stub["root"] in world["roots"]:\n            continue\n        if not (moved or name in forced):\n            continue\n','if stub.get("tag") or name in world["inferred"]:\n            continue\n        if not (name in forced or (moved and stub["root"] not in world["roots"])):\n            continue\n')
s=sub(s,"nothing is left after two rounds or on a cold build","nothing is left after two rounds that repaired nothing (a round that shrank the stale set, `previous` before it, goes on) or on a cold build")
files[p]=s
p=G+"🧪️tests/🔗️edge-refresh/🔣️.json"; d=json.loads(rd(p))
w=d["worlds"]
w[13]["id"]="a stale owner that is inferred again and a tagged one are never refreshed alone, one under a scanned root is"
heal=lambda i,h,pv:{"id":i,"world":{"heal":{"owners":{"a":{"edgesEnv":"e2"},"b":{"edgesEnv":"e1"},"c":{"edgesEnv":"e0"},"t":{"edgesEnv":"e1","tag":"tests-v2"}},"environment":"e2","cold":False,"healed":h,**({"previous":pv} if pv is not None else {})}}}
w.append(heal("heal after two rounds that shrank the stale set: the owners still stale are inferred",2,5))
w.append(heal("heal after two rounds that repaired nothing: nothing is left to try",2,3))
w.append(heal("heal after five rounds that shrank the stale set: goes on",5,4))
files[p]=json.dumps(d,ensure_ascii=False,indent=1)+"\n"
p=G+"🧪️tests/🔗️edge-refresh/🟦️.ts"; s=rd(p)
s=sub(s,"expect(call(fixture.worlds[13]!.world)).toEqual({projects:[],withheld:[],carried:[]});","const scanned=call(fixture.worlds[13]!.world);expect(scanned.projects).toEqual([\"dirty-one\"]);expect(scanned.withheld).toEqual([\"dirty-withheld\"]);expect(scanned.carried).toEqual([]);")
s=sub(s,"cold:boolean;healed:number}},own=","cold:boolean;healed:number;previous?:number}},own=")
s=s.rstrip("\n")+'''

test("the heal limit bounds the rounds that repair nothing, never the owners",()=>{
 const heal=(i:number)=>(fixture.worlds[i]!.world as any).heal;
 expect(edgeHealPlanV1(heal(17))).toEqual({edges:[],infer:["b","c","t"]});
 expect(edgeHealPlanV1(heal(18))).toEqual({edges:[],infer:[]});
 expect(edgeHealPlanV1(heal(19))).toEqual({edges:[],infer:["b","c","t"]});
});
'''
files[p]=s
if "--check" in sys.argv: print("anchors ok",len(files)); sys.exit(0)
for p,s in files.items(): wr(p,s)
print("landed",len(files))
