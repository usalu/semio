import sys,os,json,hashlib
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
MIRROR=os.environ.get("R158_MIRROR")
FILES={"planner":B+"/🏡️resident/🔌️ports/🧭️plan/🟨️.mjs","resident":B+"/🏡️resident/🧬️schema/🔣️.json"}
def loc(k): return FILES[k] if MIRROR is None else MIRROR+"/"+k
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
def one(text,old,new):
    assert text.count(old)==1,("anchor count",text.count(old),old[:90])
    return text.replace(old,new)
def planner(t):
    t=one(t,'export const ALL_TARGETS_V1="*:*";','''export const ALL_TARGETS_V1="*:*";

/** 🎯 The tasks a roster request (`*:*`) names in a slice: for the owner selection of an invocation with literal projects and targets (`run-many`) exactly those projects times those targets that exist (what they need follows from the plan walk), for every other selection every task of the slice. A selected project without the target contributes nothing, as the tool skips it too.
 * @see ../../🧪️tests/🎯️roster-scope/🟦️.ts */
export function rosterTaskIdsV1(selection,tasks){
 if(selection?.kind!=="owners"||!Array.isArray(selection.targets)||selection.targets.length===0)return[...tasks.keys()];
 const excluded=new Set(selection.exclude??[]);
 return selection.projects.filter(name=>!excluded.has(name)).flatMap(name=>selection.targets.map(target=>name+":"+target)).filter(id=>tasks.has(id));
}''')
    return one(t,'if(slice&&targets.includes(ALL_TARGETS_V1))targets=[...tasks.keys()];','if(slice&&targets.includes(ALL_TARGETS_V1))targets=rosterTaskIdsV1(slice.graph.admission?.selection,tasks);')
def resident(t):
    t=one(t,'"host-plan-roster": "Planning every target of the workspace.",\n      "host-plan-roster": "Planning every target of the workspace.",','"host-plan-roster": "Planning the selected targets and what they need.",\n      "host-plan-roster": "Planning the selected targets and what they need.",')
    return one(t,'"host-plan-roster": "Plane jedes Ziel des Arbeitsbereichs.",\n      "host-plan-roster": "Plane jedes Ziel des Arbeitsbereichs.",','"host-plan-roster": "Plane die gewählten Ziele und was sie brauchen.",\n      "host-plan-roster": "Plane die gewählten Ziele und was sie brauchen.",')
PATCH={"planner":planner,"resident":resident}
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    os.makedirs(backup,exist_ok=True);manifest={};new={}
    for k in FILES: new[k]=PATCH[k](read(loc(k)))
    for i,k in enumerate(FILES):
        old=read(loc(k));write(backup+"/%d.bak"%i,old);manifest[k]={"i":i,"before":sha(old),"after":sha(new[k])}
    json.dump(manifest,open(backup+"/manifest.json","w"))
    for k in FILES: write(loc(k),new[k])
    print("applied")
elif cmd=="restore":
    manifest=json.load(open(backup+"/manifest.json"));bad=[]
    for k in FILES:
        m=manifest[k];now=sha(read(loc(k)))
        if now==m["after"]: write(loc(k),read(backup+"/%d.bak"%m["i"]))
        elif now!=m["before"]: bad.append(k)
    print("CONFLICT" if bad else "restored",bad)
    sys.exit(4 if bad else 0)
