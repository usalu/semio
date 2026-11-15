#!/usr/bin/env python3
"""Adds the `bucketing` and `agreements` sections to the attest fixture.
usage: fixture-agree.py <attest/🧪️tests directory>
Inputs are written by hand below (bytes, hashes, events, cuts). The expected report of each case is produced by the Python oracle (🐍️.py, independent of the TypeScript module) and has to agree with the verdicts asserted here by hand."""
import copy,hashlib,json,os,sys
DIR=sys.argv[1]
PATH=os.path.join(DIR,"🧫️fixtures","🔣️.json")
fixture=json.load(open(PATH,encoding="utf8"))
for key in ("agreements","bucketing"): fixture.pop(key,None)
fixture["bucketing"]={"paths":[],"gitPaths":[],"manifests":[],"views":[]}
fixture["agreements"]=[]
open(PATH,"w",encoding="utf8").write(json.dumps(fixture,ensure_ascii=False,indent=1)+"\n")
source=open(os.path.join(DIR,"🐍️.py"),encoding="utf8").read()
scope={"__file__":os.path.join(DIR,"🐍️.py"),"__name__":"oracle"}
try: exec(compile(source,"🐍️.py","exec"),scope)
except SystemExit: pass
judge=scope["judge"];agreement=scope["agreement"]
S=lambda c:hashlib.sha256(c.encode()).hexdigest()

paths=["a.txt","src/one.txt","src/two.txt","docs/three.md","lib/four.ts","readme.md","Makefile","zeta/z.txt","ü/日本.txt","ü/日本.txt",".gitignore","x"]
fixture["bucketing"]["paths"]=[{"path":p,"bucket":agreement.bucket_of(p)} for p in paths]
GIT=[(".git/HEAD",True),(".git/refs/heads/main",True),(".git/refs/heads/feature/x",True),(".git/packed-refs",True),(".git/shallow",True),(".cache/f/index-base",True),(".cache/f/index/src/a.ts",True),(".gitignore",True),("sub/dir/.gitignore",True),("sub/.gitattributes",True),(".gitmodules",True),(".git/objects/ab/cdef",False),(".git/refs/remotes/origin/main",False),(".git/refs/tags/v1",False),(".git/index",False),(".git/config",False),("src/gitignore",False),("x.gitignore",False),("docs/.github/x.md",False)]
fixture["bucketing"]["gitPaths"]=[{"path":p,"git":g} for p,g in GIT]
assert all(agreement.git_path(p)==g for p,g in GIT)
assert agreement.bucket_of("ü/日本.txt")==agreement.bucket_of("ü/日本.txt"),"NFC"

BASE=[{"path":"readme.md","operation":"put","value":S("r")},{"path":"Makefile","operation":"put","value":S("m")},{"path":"zeta/z.txt","operation":"put","value":S("z")},{"path":"link","operation":"link","value":"readme.md"}]
EVENTS=[("src/one.txt",1),("src/two.txt",2),("docs/three.md",3),("lib/four.ts",4)]
def entries(count,drop=None,extra=()):
    found=[e for e in BASE if e["path"]!=drop]
    found+=[{"path":p,"operation":"put","value":S(p)} for p,i in EVENTS if i<=count]
    return found+list(extra)
fixture["bucketing"]["manifests"]=[
 {"name":"base","entries":BASE,"buckets":agreement.manifest_buckets(BASE)},
 {"name":"four events","entries":entries(4),"buckets":agreement.manifest_buckets(entries(4))},
 {"name":"a delete and the realm contribute nothing","entries":entries(2)+[{"path":"gone","operation":"delete","value":""},{"path":".cache/f/index-base","operation":"link","value":"tree 1"}],"buckets":agreement.manifest_buckets(entries(2))},
]
vl=["?? local.txt"," M src/one.txt","?? ü/日本.txt"]
fixture["bucketing"]["views"]=[
 {"name":"no lines","lines":[],"buckets":"0"*1024,"pending":[],"mask":"0"*64},
 {"name":"three lines","lines":vl,"buckets":agreement.view_buckets(vl),"pending":[0,5,255],"mask":agreement.mask_of([0,5,255])},
]
assert fixture["bucketing"]["views"][1]["mask"]=="8"+"0"*61+"21"

NOW=10_000_000
base=fixture["verdicts"][0]["attestations"]["a"]
HEAD={"a":{"segment":0,"bytes":400}}
ENDS={1:100,2:200,3:300,4:400}
def cut(count): return {"a":{"segment":0,"bytes":ENDS[count]}} if count>0 else {}
def window(paths_and_ms,frm=0,to=4):
    return {"a":{"from":{"segment":0,"bytes":ENDS[frm] if frm else 0},"to":{"segment":0,"bytes":ENDS[to]},"events":[{"end":{"segment":0,"bytes":ENDS[i]},"path":p,"ms":paths_and_ms.get(i,NOW-5000)} for p,i in EVENTS if frm<i<=to]}}
def fold(es):
    d=bytearray(32)
    for e in es:
        t=agreement.manifest_term(e)
        if t is None: continue
        for i in range(32): d[i]^=t[i]
    return d.hex()
def manifest_cp(count,age=2,drop=None,extra=()):
    e=entries(count,drop,extra)
    return {"cursors":cut(count),"wallMs":NOW-age*1000,"files":len(e),"manifest":fold(e),"buckets":agreement.manifest_buckets(e)}
C1,C2="a"*40,"b"*40
T1="c"*40
def git_cp(count,until=None,age=2,head=C1,tree=T1):
    r={"cursors":cut(count),"wallMs":NOW-age*1000,"head":head,"indexTree":tree}
    if until is not None: r["until"]=cut(until)
    return r
def view_cp(count,lines,pending=(),until=None,age=2):
    r={"cursors":cut(count),"wallMs":NOW-age*1000,"digest":hashlib.sha256("\n".join(lines).encode()).hexdigest(),"buckets":agreement.view_buckets(lines),"pending":agreement.mask_of(pending)}
    if until is not None: r["until"]=cut(until)
    return r
def attest(origin,count,checkpoints,platform="linux",age=2,commit=C1,view_lines=None):
    a=copy.deepcopy(base)
    a["originId"]=origin;a["platform"]=platform;a["wallMs"]=NOW-age*1000;a["hlc"]="%013d-000000-%s"%(NOW-age*1000,origin)
    a["cursors"]=cut(count);a["head"]={"ref":"refs/heads/main","commit":commit}
    cp=(checkpoints or {}).get("manifest")
    if cp: a["files"]=cp["files"];a["manifest"]=cp["manifest"]
    if view_lines is not None:
        a["view"]={"staged":0,"modified":0,"deleted":0,"untracked":len(view_lines),"digest":hashlib.sha256("\n".join(view_lines).encode()).hexdigest()}
    if checkpoints is not None: a["checkpoints"]=checkpoints
    return a
def case(name,atts,windows,verdicts,ok,differing=None,rets=(),heads=HEAD):
    c={"name":name,"now":NOW,"heads":heads,"attestations":dict((a["originId"],a) for a in atts),"retirements":dict((r["originId"],r) for r in rets),"windows":windows}
    got=judge(c)
    summary=dict((r["originId"],r["verdict"]) for r in got["rows"])
    assert summary==verdicts,(name,summary,verdicts)
    assert got["ok"]==ok,(name,got["ok"])
    if differing is not None: assert [(d["originId"],d["field"]) for d in got["differing"]]==differing,(name,got["differing"])
    c["expected"]=got
    return c
def full(count,lines=None,view=True,pending=(),vcount=None,**kw):
    cps={"manifest":manifest_cp(count,**{k:v for k,v in kw.items() if k in ("age","drop","extra")}),"git":git_cp(count)}
    if view: cps["view"]=view_cp(count if vcount is None else vcount,lines or [],pending)
    return cps

W=window({})
buckets=dict((p,agreement.bucket_of(p)) for p in paths)
assert len({buckets[p] for p,_ in EVENTS}|{buckets["readme.md"],buckets["Makefile"],buckets["zeta/z.txt"]})==7,buckets
LN=["?? local.txt"]
cases=[]
cases.append(case("three cuts of one history agree although they never meet",[attest("a",4,full(4,LN)),attest("b",2,full(2,LN)),attest("c",3,full(3,LN))],W,{"a":"ok","b":"ok","c":"ok"},True,[]))
cases.append(case("the same cut with the same digest agrees",[attest("a",3,full(3,LN)),attest("b",3,full(3,LN))],window({},0,3),{"a":"ok","b":"ok"},True,[],heads={"a":{"segment":0,"bytes":300}}))
cases.append(case("one dropped event is diverged at a common cut and the shared files are named",[attest("a",3,full(3,LN)),attest("b",3,full(3,LN,drop="Makefile")),attest("c",3,full(3,LN))],window({},0,3),{"a":"ok","b":"diverged","c":"ok"},False,[("b","manifest")],heads={"a":{"segment":0,"bytes":300}}))
cases.append(case("a difference in a bucket that the events between two cuts do not touch is diverged",[attest("a",4,full(4,LN)),attest("b",2,full(2,LN,drop="Makefile")),attest("c",3,full(3,LN))],W,{"a":"ok","b":"diverged","c":"ok"},False,[("b","manifest")]))
cases.append(case("a difference in a bucket that an event between the cuts touches is not a divergence",[attest("a",4,full(4,LN)),attest("b",2,full(2,LN,drop="docs/three.md"))],W,{"a":"ok","b":"ok"},True,[]))
cases.append(case("a view that differs outside the pending buckets is diverged and a pending bucket is exempt",[attest("a",4,full(4,LN)),attest("b",4,full(4,LN+["?? tampered.bin"])),attest("c",4,full(4,LN+["?? busy.tmp"],pending=[agreement.bucket_of("busy.tmp")]))],window({},0,4),{"a":"ok","b":"diverged","c":"ok"},False,[("b","view.digest")]))
saved=EVENTS[2]
EVENTS[2]=(".git/refs/heads/main",3)
cases.append(case("an event on the git state between the cuts leaves the git state and the view undecided",[attest("a",4,full(4,LN)),attest("b",2,full(2,LN))],window({}),{"a":"lagging","b":"lagging"},False,[]))
EVENTS[2]=saved
cases.append(case("a different HEAD commit at cuts without a git event between them is diverged",[attest("a",4,full(4,LN)),attest("b",3,{**full(3,LN),"git":git_cp(3,head=C2)}),attest("c",4,full(4,LN))],W,{"a":"ok","b":"diverged","c":"ok"},False,[("b","head.commit")]))
cases.append(case("a reading that spans events is compared with the events inside its span left out",[attest("a",4,full(4,LN)),attest("b",4,{**full(4,LN),"view":view_cp(2,LN+["?? docs/three.md"],until=4)})],W,{"a":"ok","b":"ok"},True,[]))
cases.append(case("an origin that lacks an event older than 60 s is lagging",[attest("a",4,full(4,LN)),attest("b",2,full(2,LN))],window({3:NOW-70000}),{"a":"ok","b":"lagging"},False,[]))
cases.append(case("an origin whose missing events cannot be read is lagging",[attest("a",4,full(4,LN)),attest("b",2,full(2,LN))],{},{"a":"ok","b":"lagging"},False,[]))
cases.append(case("a checkpoint older than 60 s is lagging although the attestation is fresh",[attest("a",4,full(4,LN)),attest("b",4,full(4,LN,age=70))],W,{"a":"ok","b":"lagging"},False,[]))
legacy=attest("c",4,None)
legacy.pop("checkpoints",None);legacy["manifest"]=fold(entries(4));legacy["cursors"]=cut(4)
cases.append(case("an origin without checkpoints is compared the old way and flagged",[attest("a",4,full(4,LN)),attest("b",4,full(4,LN)),legacy],W,{"a":"ok","b":"ok","c":"ok"},True,[]))
fixture["agreements"]=cases
open(PATH,"w",encoding="utf8").write(json.dumps(fixture,ensure_ascii=False,indent=1)+"\n")
print(len(cases),"agreement cases")
