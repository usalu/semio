#!/usr/bin/env python3
import json,sys,os,shutil,hashlib
L=os.environ.get("R160_LIBRARY","/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/")
B=L+"🏗️build/"
SCALE=152768
BOUND=[
 (B+"🧭️selection/🎯️scope/🧬️schema/🔣️.json","definitions/TargetScope/properties/targets","maxItems",4096,'"targets": { "type": "array"',0),
 (B+"🏡️resident/🧬️schema/🔣️.json","definitions/PlanRequest/properties/targets","maxItems",4096,'"targets": {',-3),
 (B+"🏡️resident/🧬️schema/🔣️.json","definitions/Plan/properties/tasks","maxItems",65536,'"tasks": {',-2),
 (B+"🧭️selection/📥️closure/🧬️schema/🔣️.json","properties/tasks","maxItems",65536,'"tasks": {',-3),
 (B+"📥️inputs/🎯️selected/🦀️rust/🧬️schema/🔣️.json","properties/tasks","maxItems",65536,'"tasks": {',-3),
 (B+"🔧️tools/🏗️bazel/🧬️schema/🔣️.json","definitions/Planner/properties/request/properties/tasks","maxItems",65536,'"tasks": {',-3),
 (L+"⚡️caching/🕸️graph/🔏️seal/🎯️selection/🧬️schema/🔣️.json","$defs/ScopeV1/properties/tasks","maxItems",2000,'"tasks": {',-8),
]
LINES={ # file -> (line number 1-based of the maxItems line)
}
FILES={
 "scope":B+"🧭️selection/🎯️scope/🧬️schema/🔣️.json","resident":B+"🏡️resident/🧬️schema/🔣️.json","closure":B+"🧭️selection/📥️closure/🧬️schema/🔣️.json",
 "rust":B+"📥️inputs/🎯️selected/🦀️rust/🧬️schema/🔣️.json","bazel":B+"🔧️tools/🏗️bazel/🧬️schema/🔣️.json","seal":L+"⚡️caching/🕸️graph/🔏️seal/🎯️selection/🧬️schema/🔣️.json",
 "texts":B+"📖️texts/🔣️.json","plan":B+"🧭️invocation/🛰️plan/🟦️.ts","scopets":B+"🧭️selection/🎯️scope/🟦️.ts","family":B+"📏️workspace-scale/🧬️schema/🔣️.json",
 "growth":L+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📈️growth/🟦️.ts","plantest":B+"🧭️invocation/🧪️tests/🛰️plan/🟦️.ts",
}
GROUPS={"tests":["family","growth","plantest"],"runtime":["scope","resident","closure","rust","bazel","seal","texts","plan","scopets"]}
def rd(p): return open(p,encoding="utf8").read()
def wr(p,t): open(p,"w",encoding="utf8").write(t)
def sub(t,old,new,count=1):
    assert t.count(old)==count,(old[:60],t.count(old))
    return t.replace(old,new)
def edit_line(t,line,old,new,anchor):
    ls=t.split("\n"); assert ls[line-1].strip().startswith('"maxItems": '+old),(line,ls[line-1])
    assert any(anchor in x for x in ls[max(0,line-10):line]),(line,anchor)
    ls[line-1]=ls[line-1].replace('"maxItems": '+old,'"maxItems": '+str(SCALE)); return "\n".join(ls)
MEMBERS=[("🏗️build/🧭️selection/🎯️scope/🧬️schema/🔣️.json","definitions/TargetScope/properties/targets"),("🏗️build/🏡️resident/🧬️schema/🔣️.json","definitions/PlanRequest/properties/targets"),("🏗️build/🏡️resident/🧬️schema/🔣️.json","definitions/Plan/properties/tasks"),("🏗️build/🧭️selection/📥️closure/🧬️schema/🔣️.json","properties/tasks"),("🏗️build/📥️inputs/🎯️selected/🦀️rust/🧬️schema/🔣️.json","properties/tasks"),("🏗️build/🔧️tools/🏗️bazel/🧬️schema/🔣️.json","definitions/Planner/properties/request/properties/tasks"),("⚡️caching/🕸️graph/🔏️seal/🎯️selection/🧬️schema/🔣️.json","$defs/ScopeV1/properties/tasks")]
def reason(en,de): return {"en":en,"de":de}
CMD=reason("Tasks or targets a person names on the command line (an explicit selection), not the workspace; a whole-workspace selection uses the 'all' form without a list.","Aufgaben oder Ziele, die eine Person auf der Befehlszeile nennt (eine ausdrückliche Auswahl), nicht der Arbeitsbereich; eine Auswahl des ganzen Arbeitsbereichs nutzt die Form 'all' ohne Liste.")
EXEMPT=[("🏗️build/🧬️schema/🔣️.json","properties/admission/properties/selection/oneOf/1/properties/tasks",CMD),("🏗️build/🧬️schema/🔣️.json","properties/admission/properties/selection/oneOf/2/properties/targets",CMD),("🏗️build/🧬️schema/🚪️selection.json","oneOf/1/properties/tasks",CMD),("🏗️build/🧬️schema/🚪️selection.json","oneOf/2/properties/targets",CMD),
 ("🏗️build/📥️caller/📤️orchestration/🧬️schema/🔣️.json","definitions/Context/properties/targets",reason("The delegation chain of one orchestration context, not workspace targets.","Die Delegationskette eines Orchestrierungskontexts, nicht Ziele des Arbeitsbereichs.")),
 ("⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧬️schema/🔣️.json","definitions/CurrentMaintenancePlan/properties/targets",reason("The fixed set of maintenance images of the Nx graph store, not workspace targets.","Die feste Menge der Wartungsabbilder des Nx-Graphspeichers, nicht Ziele des Arbeitsbereichs."))]
def family(t):
    d=json.loads(t); c=d["definitions"]["Family"]["const"]; first=len(c["members"])
    for f,p in MEMBERS: c["members"].append({"file":f,"pointer":p,"key":"maxItems","scale":"owners*tasksPerOwner"})
    c["equal"].append(list(range(first,first+len(MEMBERS))))
    for f,p,r in EXEMPT: c["exempt"].append({"file":f,"pointer":p,"key":"maxItems","reason":r})
    return json.dumps(d,indent=2,ensure_ascii=False)+"\n"
TEXT_ANCHOR='    "graph.plan-refused": {\n      "en": "{text}",\n      "de": "{text}"\n    },\n'
TEXT_NEW=TEXT_ANCHOR+'''    "graph.plan-not-closed": {
      "en": "The plan of this command breaks the closed plan schema and was not run: {detail} ({tasks} tasks, {edges} edges, {projects} projects, {targets} scope targets). This is a defect of the build tool, not of the command; report it with this text",
      "de": "Der Plan dieses Befehls verletzt das geschlossene Plan-Schema und wurde nicht ausgeführt: {detail} ({tasks} Aufgaben, {edges} Kanten, {projects} Projekte, {targets} Bereichsziele). Das ist ein Fehler des Build-Werkzeugs, nicht des Befehls; melde ihn mit diesem Text"
    },
'''
PLAN_OLD='throw Error("Closed scoped plan required: "+detail+"\\nGeschlossener Plan erforderlich: "+detail);}'
PLAN_NEW='throw refuseBuildV1("refused","graph.plan-not-closed",{detail:violations.slice(0,4).map(rule=>rule.slice(0,300)).join("; "),tasks:String(ids.length),edges:String(edges.length),projects:String(projects.size),targets:String(plan.scope.targets.length)});}'
PLAN_DETAIL='const detail=violations.slice(0,4).map(rule=>rule.slice(0,300)).join("; ")+" ("+ids.length+" tasks, "+edges.length+" edges, "+projects.size+" projects, "+plan.scope.targets.length+" scope targets)";'
SC_OLD='if(validateJsonSchemaSubset({definitions:schema.definitions,$ref:"#/definitions/ScopedPlan"},plan).length)throw Error("Closed scoped plan required");'
SC_NEW='const violations=validateJsonSchemaSubset({definitions:schema.definitions,$ref:"#/definitions/ScopedPlan"},plan);\n if(violations.length)throw refuseBuildV1("refused","graph.plan-not-closed",{detail:violations.slice(0,4).map(rule=>rule.slice(0,300)).join("; "),tasks:String(plan.tasks.length),edges:String(plan.edges.length),projects:String(plan.projects.length),targets:String(scope.targets.length)});'
def apply_group(g,backup):
    os.makedirs(backup,exist_ok=True)
    for k in GROUPS[g]:
        p=FILES[k]; shutil.copy2(p,os.path.join(backup,k)); open(os.path.join(backup,k+".sha"),"w").write(hashlib.sha256(open(p,"rb").read()).hexdigest())
    for k in GROUPS[g]:
        p=FILES[k]; t=rd(p)
        if k in("scope","resident","closure","rust","bazel","seal"):
            for f,ptr,key,old,anchor,_ in BOUND:
                if f!=p: continue
                ls=t.split("\n"); found=None
                for i,x in enumerate(ls):
                    if '"maxItems": '+str(old) not in x: continue
                    trial=list(ls); trial[i]=x.replace('"maxItems": '+str(old),'"maxItems": '+str(SCALE))
                    n=json.loads("\n".join(trial))
                    try:
                        for part in ptr.split("/"): n=n[int(part)] if isinstance(n,list) else n[part]
                    except Exception: continue
                    if n.get("maxItems")==SCALE: assert found is None,ptr; found=i
                assert found is not None,ptr
                ls[found]=ls[found].replace('"maxItems": '+str(old),'"maxItems": '+str(SCALE)); t="\n".join(ls)
        elif k=="texts": t=sub(t,TEXT_ANCHOR,TEXT_NEW)
        elif k=="plan":
            t=sub(t,PLAN_DETAIL,""); t=sub(t,PLAN_OLD,PLAN_NEW)
        elif k=="scopets":
            t=sub(t,SC_OLD,SC_NEW)
            if 'refuseBuildV1' not in t.split("export type")[0]: t=sub(t,'import schema from "./🧬️schema/🔣️.json" with {type:"json"};','import schema from "./🧬️schema/🔣️.json" with {type:"json"};\nimport {refuseBuildV1} from "../../📖️texts/🟦️.ts";')
        elif k=="family": t=family(t)
        elif k=="growth": t=sub(t,"arrays=/^(projects|owners|documents|closures)$/u","arrays=/^(projects|owners|documents|closures|targets|tasks)$/u")
        elif k=="plantest":
            old=t[t.index('test("a plan that breaks the closed schema names'):t.index('test("the commands that plan a task graph ask the service')]
            new='''test("a plan that breaks the closed schema is a typed refusal that names the violated rule, the path and the counts in both languages",()=>{
 const tasks=[{id:"p".repeat(300)+":build",dependsOn:[] as string[],stale:true}],request={arguments:["run"],tool:"nx",targets:[tasks[0]!.id]};
 let caught:unknown;try{scopedPlanOfReplyV1(request,tasks);}catch(error){caught=error;}
 expect(isRepoErrorV1(caught),String(caught)).toBe(true);
 const english=renderRepoErrorV1(caught as never,"en"),german=renderRepoErrorV1(caught as never,"de");
 expect(english).toContain("at most 256 character(s)");expect(english).toContain("/scope/targets/0/project");expect(english).toContain("1 tasks");expect(german).toContain("1 Aufgaben");
});

'''
            t=t.replace(old,new)
            t=sub(t,'import {isRepoErrorV1,repoErrorDataV1} from','import {isRepoErrorV1,renderRepoErrorV1,repoErrorDataV1} from')
        wr(p,t)
def restore(g,backup):
    for k in GROUPS[g]:
        p=FILES[k]; shutil.copy2(os.path.join(backup,k),p)
        assert hashlib.sha256(open(p,"rb").read()).hexdigest()==open(os.path.join(backup,k+".sha")).read(),k
if __name__=="__main__":
    cmd,g,b=sys.argv[1:4]
    (apply_group if cmd=="apply" else restore)(g,b); print(cmd,g,"ok")
