#!/usr/bin/env python3
"""Writes src/fixture.json: language-agnostic cases (inputs) and their expected outputs, computed by the independent Python oracle; the TypeScript implementation is held to the same answers."""
import copy,json,os,subprocess,sys
here=os.path.dirname(os.path.abspath(__file__))
SCHEMA=os.path.join(here,"src","schema.json");ORACLE=os.path.join(here,"src","oracle.py")
W=json.load(open(os.path.join(here,"src","workloads.json"),encoding="utf8"))
CLASSES=W["classes"]
def ask(op,cases):
    run=subprocess.run(["python3","-I",ORACLE],input=json.dumps({"op":op,"schemaPath":SCHEMA,"cases":cases}),capture_output=True,text=True)
    if run.returncode: sys.exit(run.stderr)
    return json.loads(run.stdout)
def row(at,resource,mode,elapsed,command="repository:command"):
    return {"atMs":at,"line":"[script] "+json.dumps({"owner":"repository:build","command":command,"stage":"running","activity":{"resource":resource,"mode":mode,"elapsedMs":elapsed}},separators=(",",":"))}
def queue(at,resource,mode,position,queued,holder,seconds):
    info={"version":1,"position":position,"queued":queued,"holderMode":"exclusive","holderPid":holder,"holderSeconds":seconds,"resource":resource}
    return row(at,"lease-queue:"+json.dumps(info,separators=(",",":")),mode,0)
def plain(at,text): return {"atMs":at,"line":text}
IMAGE="native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d";FACTS="repository-graph-facts:/Users/ueli/Documents/semio"
BAZEL="Another command (pid=4242) is running. Waiting for it to complete on the server (server_pid=77)..."
wait_cases=[
 {"name":"real-queue-log","classes":CLASSES,"endMs":60000,"lines":[
   plain(0,'[script] {"owner":"repository:build","command":"repo","stage":"loading"}'),
   row(40,"boot-recovery:observe:0/0","exclusive",24),row(60,"boot-recovery:observe:0/0","exclusive",17),
   row(100,IMAGE,"shared",14),queue(100,IMAGE,"shared",3,3,44358,3949),
   plain(100,"[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d (shared): position 3 of 3, behind the request of process 44358 (exclusive, queued 3949 s ago)"),
   row(15243,IMAGE,"shared",15143),plain(15243,"[script] repository:command remains active on native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d (shared, 15 s)"),
   queue(15243,IMAGE,"shared",3,3,44358,3964),row(30320,IMAGE,"shared",30220),queue(30320,IMAGE,"shared",3,3,44358,3979),row(45406,IMAGE,"shared",45306)]},
 {"name":"graph-facts-exclusive","classes":CLASSES,"endMs":40000,"lines":[row(10,FACTS,"exclusive",18),row(15161,FACTS,"exclusive",15151),row(30388,FACTS,"exclusive",30378)]},
 {"name":"short-wait-first-row-only","classes":CLASSES,"endMs":900,"lines":[row(5,IMAGE,"shared",12)]},
 {"name":"restart-splits-episodes","classes":CLASSES,"endMs":9000,"lines":[row(100,IMAGE,"shared",50),row(1100,IMAGE,"shared",1050),row(2100,IMAGE,"shared",20),row(3100,IMAGE,"shared",1020)]},
 {"name":"gap-splits-episodes","classes":CLASSES,"endMs":40000,"lines":[row(100,FACTS,"exclusive",10),row(1100,FACTS,"exclusive",1010),row(30000,FACTS,"exclusive",1500)]},
 {"name":"shared-and-exclusive-are-separate","classes":CLASSES,"endMs":5000,"lines":[row(0,IMAGE,"shared",5),row(10,IMAGE,"exclusive",5),row(1000,IMAGE,"shared",1005),row(1010,IMAGE,"exclusive",1005)]},
 {"name":"typed-progress-rows-are-ignored","classes":CLASSES,"endMs":5000,"lines":[row(0,"javascript-source-provider:7712","shared",1200),row(10,"build:graph-phase:inference:4/9","shared",0),row(20,"dependencies:store-input-replay","shared",900),row(30,"cargo:source-admission:{}","shared",0),row(40,"cache-scan:/x","shared",800),row(50,"compiler:capacity-subdivision","shared",0)]},
 {"name":"queue-row-without-generic-row","classes":CLASSES,"endMs":5000,"lines":[queue(200,IMAGE,"shared",2,2,555,7)]},
 {"name":"queue-row-extends-open-episode","classes":CLASSES,"endMs":5000,"lines":[row(100,FACTS,"exclusive",12),queue(110,FACTS,"exclusive",2,3,777,9),row(1110,FACTS,"exclusive",1012)]},
 {"name":"malformed-rows-are-ignored","classes":CLASSES,"endMs":5000,"lines":[plain(0,"[script] {not json}"),plain(1,'[script] {"activity":{"resource":"","mode":"shared","elapsedMs":1}}'),plain(2,'[script] {"activity":{"resource":"x","mode":"weird","elapsedMs":1}}'),plain(3,'[script] {"activity":{"resource":"x","mode":"shared","elapsedMs":-5}}'),plain(4,'[script] {"activity":{"resource":"x","mode":"shared","elapsedMs":true}}'),plain(5,"[script] {\"activity\":5}"),plain(6,"no prefix"),plain(7,"[script] {\"owner\":\"o\",\"stage\":\"running\"}")]},
 {"name":"queue-row-with-bad-info-is-ignored","classes":CLASSES,"endMs":5000,"lines":[row(10,"lease-queue:{broken","shared",0),row(11,'lease-queue:{"resource":5,"holderPid":1,"position":2}',"shared",0),row(12,'lease-queue:{"resource":"r","holderPid":"x","position":2}',"shared",0)]},
 {"name":"bazel-wait-until-next-line","classes":CLASSES,"endMs":30000,"lines":[plain(1000,"INFO: Streaming build results"),plain(2000,BAZEL),plain(8000,"INFO: Invocation ID: abc"),plain(9000,"INFO: Analyzed target")]},
 {"name":"bazel-wait-with-continuations","classes":CLASSES,"endMs":60000,"lines":[plain(2000,BAZEL),plain(32000,"WARNING: Another command (pid=4242) is running. Waiting for it to complete on the server (server_pid=77)..."),plain(40000,"Waiting for it to complete on the server (server_pid=77)..."),plain(45000,"INFO: Invocation ID: abc")]},
 {"name":"bazel-wait-open-at-end","classes":CLASSES,"endMs":12000,"lines":[plain(2000,BAZEL)]},
 {"name":"bazel-8-4-2-real-concurrent-build","classes":CLASSES,"endMs":17100,"lines":[plain(130,"Another command (pid=36644) is running. Waiting for it to complete on the server (server_pid=36684)..."),plain(17030,"INFO: Analyzed target //:quick (0 packages loaded, 1 target configured)."),plain(17070,"INFO: Found 1 target..."),plain(17080,"  bazel-bin/quick.out"),plain(17080,"INFO: 2 processes: 1 internal, 1 darwin-sandbox."),plain(17080,"INFO: Build completed successfully, 2 total actions"),plain(17080,"INFO: Elapsed time: 17.044s, Critical Path: 0.04s")]},
 {"name":"cargo-wait-build-directory","classes":CLASSES,"endMs":20000,"lines":[plain(500,"   Compiling a v0.1.0"),plain(1000,"    Blocking waiting for file lock on build directory"),plain(6500,"   Compiling b v0.1.0")]},
 {"name":"cargo-two-locks","classes":CLASSES,"endMs":20000,"lines":[plain(100,"    Blocking waiting for file lock on package cache"),plain(900,"    Blocking waiting for file lock on build directory"),plain(2400,"   Compiling c v0.1.0")]},
 {"name":"mixed-sources-sorted","classes":CLASSES,"endMs":30000,"lines":[plain(20000,BAZEL),plain(21000,"done"),row(5,FACTS,"exclusive",15),row(15010,FACTS,"exclusive",15005),plain(25000,"    Blocking waiting for file lock on build directory"),plain(26000,"x")]},
 {"name":"capacity-and-toolchain-classes","classes":CLASSES,"endMs":9000,"lines":[row(0,"semio.host.capacity/v1","shared",300),row(1,"/Users/ueli/.bun/bin/bun","exclusive",200),row(2,"C:\\Users\\a\\bun.exe","exclusive",250),row(3,"something-else","shared",100),row(4,"cargo-unit:abc","exclusive",90)]},
 {"name":"unsorted-input","classes":CLASSES,"endMs":9000,"lines":[row(3000,IMAGE,"shared",2010),row(1000,IMAGE,"shared",10),row(2000,IMAGE,"shared",1010)]},
 {"name":"empty","classes":CLASSES,"endMs":0,"lines":[]},
]
for c,answer in zip(wait_cases,ask("waits",wait_cases)): c["waits"]=answer
def act(i,g,iv): return {"id":i,"group":g,"intervals":iv}
rel_cases=[
 {"name":"unrelated-holder","wait":{"startMs":1000,"endMs":2000},"self":{"id":"a","group":"ga"},"others":[act("b","gb",[[0,1500]])]},
 {"name":"related-holder","wait":{"startMs":1000,"endMs":2000},"self":{"id":"a","group":"g"},"others":[act("b","g",[[900,1100]])]},
 {"name":"mixed-holders-are-related","wait":{"startMs":1000,"endMs":2000},"self":{"id":"a","group":"g"},"others":[act("b","h",[[0,5000]]),act("c","g",[[1500,1600]])]},
 {"name":"external-when-nobody-runs","wait":{"startMs":1000,"endMs":2000},"self":{"id":"a","group":"g"},"others":[act("b","h",[[0,900]]),act("c","h",[[2100,3000]])]},
 {"name":"touching-intervals-overlap","wait":{"startMs":1000,"endMs":2000},"self":{"id":"a","group":"g"},"others":[act("b","h",[[0,1000]]),act("c","i",[[2000,3000]])]},
 {"name":"self-is-never-a-holder","wait":{"startMs":0,"endMs":10},"self":{"id":"a","group":"g"},"others":[act("a","g",[[0,100]])]},
 {"name":"zero-length-wait","wait":{"startMs":500,"endMs":500},"self":{"id":"a","group":"g"},"others":[act("b","h",[[400,600]])]},
 {"name":"holders-sorted","wait":{"startMs":0,"endMs":10},"self":{"id":"a","group":"g"},"others":[act("z","h",[[0,10]]),act("b","i",[[0,10]]),act("m","j",[[0,10]])]},
]
for c,answer in zip(rel_cases,ask("relation",rel_cases)): c["relation"]=answer["relation"];c["holders"]=answer["holders"]
def wait(cls,resource,start,end,relation=None,precise=True,source="trace",holders=None,mode="exclusive",waitms=None):
    w={"class":cls,"resource":resource,"mode":mode,"source":source,"precise":precise,"startMs":start,"endMs":end,"waitMs":end-start if waitms is None else waitms,"holderPid":None,"position":None}
    if relation: w["relation"]=relation
    if holders is not None: w["holders"]=holders
    return w
def step(index,kind="run",start=0,end=100,exit=0,waits=None,executed=None,restored=None,observed="exact",outputs=None,stdout=None,command=None):
    obs=None if executed is None else {"executed":executed,"restored":restored or [],"cached":[]}
    return {"index":index,"kind":kind,"command":command or (["run","t:test"] if kind=="run" else ["show","projects"]),"startMs":start,"endMs":end,"exit":exit,"waits":waits or [],"observation":obs,"observed":observed if obs else "none","outputs":outputs,"stdout":stdout}
def session(i,g,steps,start=0,end=None): return {"id":i,"group":g,"startMs":start,"endMs":end if end is not None else max([s["endMs"] for s in steps],default=0),"steps":steps}
def measure(sessions,serial=None,uncached=None,tool="bazel"): return {"workload":"w","tool":tool,"uncached":uncached or [],"sessions":sessions,"serial":serial}
crit={c["id"]:c for c in W["criteria"]}
def only(*ids,base="default"):
    c=copy.deepcopy(crit[base]);c["rules"]=[r for r in c["rules"] if r["id"] in ids];return c
good=lambda i,g,**kw:session(i,g,[step(0,start=0,end=100,executed=["a:b"] if i=="a" else [],restored=[] if i=="a" else ["a:b"],outputs="d1",**kw)])
serial_ok=[session("a","ga",[step(0,outputs="d1")]),session("b","gb",[step(0,outputs="d1")])]
eval_cases=[
 {"name":"default-all-pass","criteria":crit["default"],"measurement":measure([good("a","ga"),good("b","gb")],serial_ok)},
 {"name":"no-failure-fails-on-exit-and-signal","criteria":only("all-steps-succeed"),"measurement":measure([session("a","g",[step(0,exit=1),step(1,exit=None)])],None)},
 {"name":"serial-equal-skip-without-serial","criteria":only("outputs-equal-serial"),"measurement":measure([session("a","g",[step(0)])],None)},
 {"name":"serial-equal-mismatches","criteria":only("outputs-equal-serial"),"measurement":measure([session("a","g",[step(0,exit=0,outputs="x",stdout="s1"),step(1,kind="repo"),step(2)]),session("b","g",[step(0)])],[session("a","g",[step(0,exit=2,outputs="y",stdout="s2"),step(1,kind="repo")])])},
 {"name":"serial-equal-all-equal","criteria":only("outputs-equal-serial"),"measurement":measure([session("a","g",[step(0,outputs="x",stdout="s")])],[session("a","g",[step(0,outputs="x",stdout="s")])])},
 {"name":"graph-facts-wait-fails-for-unrelated","criteria":only("graph-facts-unrelated-wait"),"measurement":measure([session("a","ga",[step(0,waits=[wait("graph-facts",FACTS,100,7000,"unrelated",holders=["b"])])]),session("b","gb",[step(0)])],None)},
 {"name":"related-waits-are-not-limited","criteria":only("graph-facts-unrelated-wait"),"measurement":measure([session("a","g",[step(0,waits=[wait("graph-facts",FACTS,100,70000,"related",holders=["b"])])])],None)},
 {"name":"external-waits-count-as-unrelated","criteria":only("graph-facts-unrelated-wait"),"measurement":measure([session("a","g",[step(0,waits=[wait("graph-facts",FACTS,100,9000,"external",holders=[])])])],None)},
 {"name":"exact-limit-passes","criteria":only("graph-facts-unrelated-wait"),"measurement":measure([session("a","g",[step(0,waits=[wait("graph-facts",FACTS,0,5000,"unrelated",holders=["b"])])])],None)},
 {"name":"other-classes-do-not-count","criteria":only("graph-facts-unrelated-wait"),"measurement":measure([session("a","g",[step(0,waits=[wait("bazel-server","bazel:server",0,50000,"unrelated",holders=["b"])])])],None)},
 {"name":"imprecise-wait-below-limit-is-unproven","criteria":only("graph-facts-unrelated-wait"),"measurement":measure([session("a","g",[step(0,waits=[wait("graph-facts",FACTS,0,15,"unrelated",precise=False,source="lease",holders=["b"])])])],None)},
 {"name":"imprecise-wait-above-limit-fails","criteria":only("graph-facts-unrelated-wait"),"measurement":measure([session("a","g",[step(0,waits=[wait("graph-facts",FACTS,0,15151,"unrelated",precise=False,source="lease",holders=["b"])])])],None)},
 {"name":"any-relation-wildcard-class","criteria":{"id":"x","title":{"en":"x","de":"x"},"rules":[{"id":"any","kind":"max-wait","title":{"en":"x","de":"x"},"owners":["conc-leases"],"classes":["*"],"relation":"any","maximumMilliseconds":100}]},"measurement":measure([session("a","g",[step(0,waits=[wait("other","r",0,50,"related"),wait("cargo-units","cargo:build directory",10,400,"unrelated",holders=["b"],source="cargo")])])],None)},
 {"name":"related-only-rule","criteria":{"id":"x","title":{"en":"x","de":"x"},"rules":[{"id":"rel","kind":"max-wait","title":{"en":"x","de":"x"},"owners":["conc-leases"],"classes":["other"],"relation":"related","maximumMilliseconds":100}]},"measurement":measure([session("a","g",[step(0,waits=[wait("other","r",0,500,"related"),wait("other","r",0,900,"unrelated",holders=["b"])])])],None)},
 {"name":"executes-once-duplicate-fails","criteria":only("second-session-zero-actions"),"measurement":measure([session("a","ga",[step(0,executed=["x:build","y:build"])]),session("b","ga",[step(0,executed=["y:build"],restored=["x:build"])])],None)},
 {"name":"executes-once-uncached-excluded","criteria":only("second-session-zero-actions"),"measurement":measure([session("a","ga",[step(0,executed=["x:build","live:x"])]),session("b","ga",[step(0,executed=["live:x"],restored=["x:build"])])],None,uncached=["live:x"])},
 {"name":"executes-once-same-session-twice-is-fine","criteria":only("second-session-zero-actions"),"measurement":measure([session("a","ga",[step(0,executed=["x:build"]),step(1,executed=["x:build"])])],None)},
 {"name":"executes-once-ambiguous-is-skipped","criteria":only("second-session-zero-actions"),"measurement":measure([session("a","ga",[step(0,executed=["x:build"])]),session("b","ga",[step(0,observed="ambiguous")])],None)},
 {"name":"executes-once-window-observation-counts","criteria":only("second-session-zero-actions"),"measurement":measure([session("a","ga",[step(0,executed=["x:build"],observed="window")]),session("b","ga",[step(0,executed=["x:build"],observed="window")])],None)},
 {"name":"executes-once-without-runs-is-skipped","criteria":only("second-session-zero-actions"),"measurement":measure([session("a","ga",[step(0,kind="repo")])],None)},
 {"name":"executes-once-pass","criteria":only("second-session-zero-actions"),"measurement":measure([session("a","ga",[step(0,executed=["x:build"])]),session("b","ga",[step(0,restored=["x:build"],executed=[])])],None)},
 {"name":"warm-sessions-execute-nothing-pass","criteria":only("warm-sessions-zero-actions",base="warm"),"measurement":measure([session("a","ga",[step(0,executed=[],restored=["x:build"])]),session("b","ga",[step(0,executed=[])])],None)},
 {"name":"warm-sessions-execute-something-fail","criteria":only("warm-sessions-zero-actions",base="warm"),"measurement":measure([session("a","ga",[step(0,executed=["x:build","live:x"])]),session("b","ga",[step(0,executed=[])])],None,uncached=["live:x"])},
 {"name":"max-executed-named-sessions","criteria":{"id":"x","title":{"en":"x","de":"x"},"rules":[{"id":"two","kind":"max-executed","title":{"en":"x","de":"x"},"owners":["conc-bazel"],"sessions":["b"],"maximum":1,"excludeUncached":False}]},"measurement":measure([session("a","g",[step(0,executed=["1:a","2:b","3:c"])]),session("b","g",[step(0,executed=["1:a","2:b"])])],None)},
]
for c,answer in zip(eval_cases,ask("evaluate",eval_cases)): c["results"]=answer
def mutation(name,text,op,language,nonce): return {"name":name,"text":text,"op":op,"language":language,"nonce":nonce}
mut_cases=[mutation(f"{lang}-{op}",text,op,lang,"abc123x9") for lang in ("ts","rs","go","py") for op in ("comment","private-body","public-api") for text in ("line\n",)]
mut_cases+=[mutation("text-comment","a","comment","text","abc123x9"),mutation("go-comment-no-newline","package a","comment","go","zzzzzz"),mutation("go-empty","","public-api","go","a1b2c3d4e5"),mutation("whitespace-go","x","whitespace","go","abc123"),mutation("whitespace-ts","x\n","whitespace","ts","abc123"),mutation("go-private","package a\n","private-body","go","0a0a0a0a")]
for c,answer in zip(mut_cases,ask("mutate",mut_cases)): c["result"]=answer
ind_cases=[
 {"name":"independent","rows":[{"id":"a","group":"ga","edits":["x"],"touches":["x"]},{"id":"b","group":"gb","edits":["y"],"touches":["y"]}]},
 {"name":"unrelated-edit-reaches-reader","rows":[{"id":"a","group":"ga","edits":["x"],"touches":["x"]},{"id":"b","group":"gb","edits":["y"],"touches":["x","y"]}]},
 {"name":"same-group-may-share","rows":[{"id":"a","group":"g","edits":["x"],"touches":["x"]},{"id":"b","group":"g","edits":["y"],"touches":["x","y"]}]},
 {"name":"issues-sorted","rows":[{"id":"b","group":"gb","edits":["y"],"touches":["x","y"]},{"id":"a","group":"ga","edits":["x"],"touches":["x","y"]}]},
]
for c,answer in zip(ind_cases,ask("independence",ind_cases)): c["issues"]=answer
def variant(change):
    s=copy.deepcopy(W);change(s);return s
def dup_session(s): s["workloads"][0]["sessions"][1]["id"]=s["workloads"][0]["sessions"][0]["id"]
def unknown_criteria(s): s["workloads"][1]["criteria"]="nope"
def unknown_class(s): s["criteria"][0]["rules"][2]["classes"]=["graph-facts","made-up"]
def dup_barrier(s): s["workloads"][0]["sessions"][0]["steps"].insert(2,{"kind":"barrier","id":"go"})
def same_file(s): s["workloads"][0]["sessions"][1]["steps"][0]["file"]=s["workloads"][0]["sessions"][0]["steps"][0]["file"]
def unknown_session(s):
    for r in s["criteria"][0]["rules"]:
        if r["kind"]=="executes-once": r["sessions"]=["ghost"]
def bad_op(s): s["workloads"][0]["sessions"][0]["steps"][0]["op"]="private-body";s["workloads"][0]["sessions"][0]["steps"][0]["language"]="text"
def bad_pattern(s): s["classes"][0]["patterns"]=["(unclosed"]
def dup_class(s): s["classes"].append(copy.deepcopy(s["classes"][0]))
def dup_rule(s): s["criteria"][0]["rules"].append(copy.deepcopy(s["criteria"][0]["rules"][0]))
def setup_vs_session(s): s["workloads"][1]["sessions"][0]["steps"].insert(0,copy.deepcopy(s["workloads"][1]["setup"][0]))
wl_cases=[{"name":"authored","set":W}]+[{"name":n,"set":variant(f)} for n,f in [("duplicate-session",dup_session),("unknown-criteria",unknown_criteria),("unknown-class",unknown_class),("duplicate-barrier",dup_barrier),("one-file-two-owners",same_file),("unknown-rule-session",unknown_session),("unsupported-op",bad_op),("invalid-pattern",bad_pattern),("duplicate-class",dup_class),("duplicate-rule",dup_rule),("setup-and-session-edit-same-file",setup_vs_session)]]
for c,answer in zip(wl_cases,ask("workloads",wl_cases)): c["issues"]=answer
def rec(tasks,start=None,end=None):
    r={"tasks":[{"taskId":i,"cacheStatus":s,"status":0} for i,s in tasks]}
    if start is not None: r["run"]={"command":"nx run","startTime":start,"endTime":end}
    return r
nx_cases=[
 {"name":"exact-without-times","record":rec([("a:b","cache-miss"),("c:d","local-cache-hit")]),"ids":["a:b","c:d","e:f"],"windowStart":1000,"windowEnd":2000,"concurrent":False},
 {"name":"ambiguous-without-times-when-concurrent","record":rec([("a:b","cache-miss")]),"ids":["a:b"],"windowStart":1000,"windowEnd":2000,"concurrent":True},
 {"name":"window-with-numeric-times","record":rec([("a:b","cache-miss"),("c:d","remote-cache-hit")],1200,1800),"ids":["a:b","c:d"],"windowStart":1000,"windowEnd":2000,"concurrent":True},
 {"name":"window-with-iso-times","record":rec([("a:b","cache-miss")],"1970-01-01T00:00:01.200Z","1970-01-01T00:00:01.800Z"),"ids":["a:b"],"windowStart":1000,"windowEnd":2000,"concurrent":True},
 {"name":"exact-with-times-inside-skew","record":rec([("a:b","cache-miss")],900,2500),"ids":["a:b"],"windowStart":1000,"windowEnd":2000,"concurrent":False},
 {"name":"ambiguous-when-record-is-older","record":rec([("a:b","cache-miss")],100,500),"ids":["a:b"],"windowStart":5000,"windowEnd":6000,"concurrent":False},
 {"name":"ambiguous-without-known-tasks","record":rec([("z:z","cache-miss")]),"ids":["a:b"],"windowStart":1000,"windowEnd":2000,"concurrent":False},
 {"name":"ambiguous-without-tasks-array","record":{"run":{}},"ids":["a:b"],"windowStart":1000,"windowEnd":2000,"concurrent":False},
]
for c,answer in zip(nx_cases,ask("nx",nx_cases)): c["result"]=answer
dig_cases=[
 {"name":"two-files","parts":[{"path":"b.txt","text":"two"},{"path":"a.txt","text":"one"}]},
 {"name":"order-does-not-matter","parts":[{"path":"a.txt","text":"one"},{"path":"b.txt","text":"two"}]},
 {"name":"length-is-part-of-the-digest","parts":[{"path":"a","text":"bc"},{"path":"ab","text":"c"}]},
 {"name":"unicode","parts":[{"path":"🧰️x/📜️script.ts","text":"é\u00e9ß"},{"path":"a","text":""}]},
 {"name":"empty","parts":[]},
]
for c,answer in zip(dig_cases,ask("digest",dig_cases)): c["digest"]=answer
def trace_row(resource,mode,start,acq,pid=7,holder=None):
    r={"version":1,"resource":resource,"mode":mode,"startedMs":start,"acquiredMs":acq,"pid":pid}
    if holder is not None: r["holderPid"]=holder
    return json.dumps(r,separators=(",",":"))
trace_cases=[
 {"name":"precise-waits","classes":CLASSES,"origin":1000,"text":"\n".join([trace_row(FACTS,"exclusive",1500,9500,7,4242),trace_row(IMAGE,"shared",1200,1400),""])},
 {"name":"malformed-rows-are-ignored","classes":CLASSES,"origin":0,"text":"\n".join(["not json",'{"version":2}',trace_row("r","shared",10,5),trace_row("r","shared",5,10),'{"version":1,"resource":"x","mode":"shared","startedMs":1,"acquiredMs":2,"pid":0}','{"version":1,"resource":"x","mode":"shared","startedMs":1,"acquiredMs":2,"pid":3,"extra":1}',"",trace_row("ok","exclusive",3,9)])},
 {"name":"empty","classes":CLASSES,"origin":0,"text":""},
 {"name":"outer-and-inner-rows-of-one-process-coalesce","classes":CLASSES,"origin":0,"text":"\n".join([trace_row(FACTS,"exclusive",1000,9000,7),trace_row(FACTS,"exclusive",8900,9000,7,4242),trace_row(FACTS,"exclusive",8900,9000,8),trace_row(FACTS,"shared",100,500,7)])},
]
for c,answer in zip(trace_cases,ask("traces",trace_cases)): c["waits"]=answer
lease_wait=wait("graph-facts",FACTS,1000,1015,source="lease",precise=False,waitms=15)
merge_cases=[
 {"name":"trace-replaces-overlapping-lease-wait","derived":[lease_wait,wait("bazel-server","bazel:server",3000,4000,source="bazel")],"traced":[wait("graph-facts",FACTS,990,9000)]},
 {"name":"other-resource-is-kept","derived":[lease_wait],"traced":[wait("dependency-image",IMAGE,990,9000)]},
 {"name":"other-mode-is-kept","derived":[lease_wait],"traced":[wait("graph-facts",FACTS,990,9000,mode="shared")]},
 {"name":"non-overlapping-lease-wait-is-kept","derived":[wait("graph-facts",FACTS,20000,20015,source="lease",precise=False,waitms=15)],"traced":[wait("graph-facts",FACTS,990,9000)]},
 {"name":"no-trace","derived":[lease_wait],"traced":[]},
]
for c,answer in zip(merge_cases,ask("merges",merge_cases)): c["waits"]=answer
runs=[{"workload":"w","tool":"bazel","criteria":"default","measurement":e["measurement"],"rules":e["results"]} for e in eval_cases[:6]]
tally_cases=[{"name":"mixed","runs":runs},{"name":"empty","runs":[]},{"name":"single","runs":runs[:1]}]
for c,answer in zip(tally_cases,ask("tally",tally_cases)): c["result"]=answer
fixture={"version":1,"lines":wait_cases,"relations":rel_cases,"evaluations":eval_cases,"mutations":mut_cases,"independence":ind_cases,"workloads":wl_cases,"nx":nx_cases,"digests":dig_cases,"traces":trace_cases,"merges":merge_cases,"tallies":tally_cases}
out=os.path.join(here,"src","fixture.json")
json.dump(fixture,open(out,"w",encoding="utf8"),ensure_ascii=False,indent=1);open(out,"a").write("\n")
print({k:len(v) for k,v in fixture.items() if isinstance(v,list)})
