#!/usr/bin/env python3
"""Generates src/fixture.json: the language-agnostic cases of the incremental conformance model. Expected sets come from src/oracle.py and are checked against hand-derived answers; Bazel logs come from real Bazel runs in a scratch workspace and are cross-checked against Bazel's own summary line; Nx records come from Nx's own run-record class."""
import base64,json,os,re,shutil,subprocess,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,os.path.join(here,"src"))
import oracle
BAZEL=os.environ.get("SEMIO_TEST_BAZEL_BINARY","/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel")
LAB="/private/tmp/bzlincr/fx"
sens=[
 {"family":"bun-script","kind":"comment-only","effect":"identical"},{"family":"bun-script","kind":"body-change","effect":"changed"},{"family":"bun-script","kind":"config","effect":"identical"},
 {"family":"tsc","kind":"comment-only","effect":"identical"},{"family":"tsc","kind":"body-change","effect":"identical"},{"family":"tsc","kind":"config","effect":"identical"},{"family":"tsc","kind":"signature-change","effect":"changed"},
 {"family":"vite","kind":"comment-only","effect":"identical"},{"family":"vite","kind":"body-change","effect":"changed"},{"family":"vite","kind":"config","effect":"identical"},
 {"family":"vitest","kind":"body-change","effect":"identical"},{"family":"cargo","kind":"signature-change","effect":"changed"},{"family":"cargo","kind":"unrelated-sibling","effect":"identical"},
]
def task(family,cache,outputs,deps,consumes,inputs):
    return {"family":family,"cache":cache,"outputs":outputs,"dependsOn":deps,"consumes":consumes,"inputs":inputs}
S1={"version":1,"selected":["shell:activate","other:build"],"tasks":{
 "lib:generate":task("bun-script",True,True,[],[],["lib/gen.ts","lib/src.ts"]),
 "lib:typecheck":task("tsc",True,True,["lib:generate"],["lib:generate"],["lib/src.ts"]),
 "app:build":task("vite",True,True,["lib:generate"],["lib:generate"],["app/main.ts","app/vite.config.ts"]),
 "app:test":task("vitest",True,False,["app:build"],[],["app/main.test.ts","app/main.ts"]),
 "shell:activate":task("bun-script",False,True,["app:build","app:test"],["app:build"],[]),
 "other:build":task("cargo",True,True,[],[],["other/lib.rs"]),
}}
S2={"version":1,"selected":["d:test"],"tasks":{
 "a:gen":task("bun-script",True,True,[],[],["a/a.ts"]),
 "b:build":task("tsc",True,True,["a:gen"],["a:gen"],["b/b.ts"]),
 "c:build":task("vite",True,True,["a:gen","b:build"],["a:gen","b:build"],["c/c.ts"]),
 "d:test":task("vitest",True,False,["c:build"],[],["d/d.ts"]),
}}
S3={"version":1,"selected":["x:tail"],"tasks":{"x:head":task("bun-script",True,True,[],[],["x/head.ts"]),"x:tail":task("vite",True,True,["x:head"],["x:head"],["x/tail.ts"])}}
cases=[
 ("comment in a generator is cut off before its dependents (bazel)","bazel",S1,"comment-only",["lib/gen.ts"],{"must":["lib:generate","shell:activate"],"cutoff":["app:build","lib:typecheck"]}),
 ("comment in a generator is cut off before its dependents (nx)","nx",S1,"comment-only",["lib/gen.ts"],{"must":["lib:generate","shell:activate"],"cutoff":["app:build","lib:typecheck"]}),
 ("body change in a generator reaches every dependent whose output follows (bazel)","bazel",S1,"body-change",["lib/gen.ts"],{"must":["app:build","app:test","lib:generate","lib:typecheck","shell:activate"],"cutoff":[]}),
 ("body change in a generator reaches only the consumers of outputs (nx)","nx",S1,"body-change",["lib/gen.ts"],{"must":["app:build","lib:generate","lib:typecheck","shell:activate"],"cutoff":[]}),
 ("body change in a file only the builder reads reaches the test through its dependency snapshot (bazel)","bazel",S1,"body-change",["app/vite.config.ts"],{"must":["app:build","app:test","shell:activate"],"cutoff":[]}),
 ("body change in a file only the builder reads leaves the test alone (nx)","nx",S1,"body-change",["app/vite.config.ts"],{"must":["app:build","shell:activate"],"cutoff":[]}),
 ("an unrelated sibling re-runs only the uncached task (bazel)","bazel",S1,"unrelated-sibling",["unrelated/x.ts"],{"must":["shell:activate"],"cutoff":[]}),
 ("an unrelated sibling re-runs only the uncached task (nx)","nx",S1,"unrelated-sibling",["unrelated/x.ts"],{"must":["shell:activate"],"cutoff":[]}),
 ("an additive signature change in a crate re-runs the crate and the uncached task","bazel",S1,"signature-change",["other/lib.rs"],{"must":["other:build","shell:activate"],"cutoff":[]}),
 ("a config edit read by two tasks is cut off before the builder (bazel)","bazel",S1,"config",["lib/src.ts"],{"must":["lib:generate","lib:typecheck","shell:activate"],"cutoff":["app:build"]}),
 ("an unknown output effect widens may but not must","bazel",S1,"signature-change",["lib/src.ts"],{"must":["lib:generate","lib:typecheck","shell:activate"],"may":["app:build","app:test","lib:generate","lib:typecheck","shell:activate"],"cutoff":[]}),
 ("a diamond follows changed outputs once (bazel)","bazel",S2,"body-change",["a/a.ts"],{"must":["a:gen","b:build","c:build","d:test"],"cutoff":[]}),
 ("a diamond is cut where the middle output is identical (bazel)","bazel",S2,"comment-only",["a/a.ts"],{"must":["a:gen"],"cutoff":["b:build","c:build"]}),
 ("an edit nothing declares re-runs nothing","bazel",S3,"unrelated-sibling",["y/other.ts"],{"must":[],"cutoff":[]}),
 ("an edit read by the tail re-runs only the tail","nx",S3,"body-change",["x/tail.ts"],{"must":["x:tail"],"cutoff":[]}),
]
evaluation=[]
for name,tool,slice_,kind,files,hand in cases:
    edit={"kind":kind,"files":files};exp=oracle.expected(slice_,tool,edit,sens)
    assert exp["must"]==hand["must"],(name,"must",exp["must"],hand["must"])
    assert exp["cutoff"]==hand["cutoff"],(name,"cutoff",exp["cutoff"],hand["cutoff"])
    if "may" in hand: assert exp["may"]==hand["may"],(name,"may",exp["may"])
    evaluation.append({"name":name,"tool":tool,"slice":slice_,"edit":edit,"expected":exp})
E=lambda must,may,cut=[]:{"direct":[],"must":must,"may":may,"cutoff":cut}
verdict_cases=[
 ("exact run passes",E(["a:x"],["a:x"]),["a:x"]),
 ("a run inside may passes but is not exact",E(["a:x"],["a:x","b:x"]),["a:x"]),
 ("a run covering may passes",E(["a:x"],["a:x","b:x"]),["a:x","b:x"]),
 ("a missing must task is stale",E(["a:x","b:x"],["a:x","b:x"]),["a:x"]),
 ("a task beyond may is excess",E(["a:x"],["a:x"]),["a:x","c:x"]),
 ("stale and excess together",E(["a:x"],["a:x"]),["c:x"]),
 ("nothing expected and nothing run is exact",E([],[]),[]),
]
verdicts=[{"name":n,"expected":e,"executed":x,"verdict":oracle.verdict(e,x)} for n,e,x in verdict_cases]
mut=[("ts comment","comment","ts","k3j9x2a","const a = 1;\n"),("ts comment appends the missing newline","comment","ts","k3j9x2a","const a = 1;"),("ts private body","private-body","ts","zz00aa11","export {};\n"),
 ("ts public api","public-api","ts","zz00aa11","export {};\n"),("rs comment","comment","rs","abc123xyz","fn main() {}\n"),("rs private body","private-body","rs","abc123xyz","fn main() {}\n"),
 ("rs public api","public-api","rs","abc123xyz","fn main() {}\n"),("py comment","comment","py","m1n2o3","x = 1\n"),("py private body","private-body","py","m1n2o3","x = 1\n"),
 ("py public api","public-api","py","m1n2o3","x = 1\n"),("text comment","comment","text","q9w8e7","a = 1\n"),("whitespace on a lock file","whitespace","text","q9w8e7","{\"lockfileVersion\": 1}\n"),
 ("comment on an empty file","comment","ts","k3j9x2a","")]
mutations=[{"name":n,"op":o,"language":l,"nonce":nc,"before":b,"after":oracle.mutate(b,o,l,nc)} for n,o,l,nc,b in mut]
ids=["@semio-tech/framework-graph:test","@teaching/proctor:build","os-hub-ts:test","@semio-tech/mit-bestand-demonstrator:activate-prototype","a:b:c"]
labels=[{"task":t,"label":oracle.label("t",t)} for t in ids]
environments=[
 {"name":"invocation and Nx state are dropped, the language and tools stay","parent":{"PATH":"/usr/bin","HOME":"/home/a","SEMIO_SCRIPT_PROCESS_INVOCATION":"{}","SEMIO_BUILD_TOOL":"bazel","NX_DAEMON":"false","SEMIO_LANGUAGE":"de","LANG":"en_US.UTF-8"},"extra":{},"child":None},
 {"name":"explicit additions win","parent":{"PATH":"/usr/bin","SEMIO_BUILD_TASK_REPORT":"/old"},"extra":{"SEMIO_BUILD_TASK_REPORT":"/new/log.zst"},"child":None},
]
for e in environments: e["child"]=oracle.child(e["parent"],e["extra"])
refusals=[
 {"name":"a dependency cycle","value":{"version":1,"selected":[],"tasks":{"a:x":task("tsc",True,True,["b:x"],[],[]),"b:x":task("tsc",True,True,["a:x"],[],[])}}},
 {"name":"an unknown dependency","value":{"version":1,"selected":[],"tasks":{"a:x":task("tsc",True,True,["zzz:x"],[],[])}}},
 {"name":"a consumed task that is not a dependency","value":{"version":1,"selected":[],"tasks":{"a:x":task("tsc",True,True,[],[],[]),"b:x":task("tsc",True,True,[],["a:x"],[])}}},
 {"name":"an unknown selected task","value":{"version":1,"selected":["nope:x"],"tasks":{"a:x":task("tsc",True,True,[],[],[])}}},
 {"name":"a task id without a colon","value":{"version":1,"selected":[],"tasks":{"plain":task("tsc",True,True,[],[],[])}}},
 {"name":"an unknown family","value":{"version":1,"selected":[],"tasks":{"a:x":task("make",True,True,[],[],[])}}},
]
def w(path,text):
    os.makedirs(os.path.dirname(path),exist_ok=True);open(path,"w").write(text)
def prepare():
    shutil.rmtree(LAB,ignore_errors=True)
    ids2=["lib:generate","app:build","app:test"];L={t:oracle.label("t",t) for t in ids2}
    w(f"{LAB}/MODULE.bazel",'module(name = "incr_fx", version = "0.0.0")\n')
    w(f"{LAB}/lib/gen.py","# header\nvalue = 1\n");w(f"{LAB}/app/main.py","print(value)\n");w(f"{LAB}/app/test.py","assert True\n")
    w(f"{LAB}/BUILD.bazel",f'''exports_files(["lib/gen.py","app/main.py","app/test.py"])
genrule(name = "{L['lib:generate']}", srcs = ["lib/gen.py"], outs = ["gen.out"], cmd = "grep -v -e '^#' -e '^_incremental_' $(location lib/gen.py) > $@ || true")
genrule(name = "{L['app:build']}", srcs = ["app/main.py", ":gen.out"], outs = ["build.out"], cmd = "cat $(SRCS) > $@")
genrule(name = "{L['app:test']}", srcs = ["app/test.py", ":build.out"], outs = ["test.out"], cmd = "cat $(SRCS) > $@")
''')
    return ids2
def bazel(phase,out):
    args=[BAZEL,f"--output_base={LAB}-ob","--nosystem_rc","--nohome_rc","--max_idle_secs=30","--host_jvm_args=-Xmx512m","build","--lockfile_mode=off","--incompatible_autoload_externally=",f"--disk_cache={LAB}-disk","--noshow_progress",f"--execution_log_compact_file={out}","--","//:all"]
    r=subprocess.run(args,cwd=LAB,capture_output=True,text=True)
    assert r.returncode==0,r.stderr
    m=re.search(r"INFO: (\d+) processes?: ([^\n]*)",r.stderr);summary=(m.group(2) if m else "").rstrip(".")
    parts={k.strip().split(" ",1)[1]:int(k.strip().split(" ",1)[0]) for k in re.split(r",\s*",summary) if k.strip()}
    return parts
def logs():
    ids2=prepare();rows=[];
    def record(name,mutate=None,clean=False):
        if mutate: mutate()
        if clean: subprocess.run([BAZEL,f"--output_base={LAB}-ob","--nosystem_rc","--nohome_rc","clean"],cwd=LAB,capture_output=True)
        out=f"{LAB}-{name.replace(' ','_')}.zst";parts=bazel(name,out)
        b64=base64.b64encode(open(out,"rb").read()).decode();sp=oracle.spawns(b64);ob=oracle.observation(sp,ids2)
        hits=parts.get("disk cache hit",0)+parts.get("remote cache hit",0);ran=sum(v for k,v in parts.items() if k not in("internal","action cache hit","disk cache hit","remote cache hit"))
        assert len(ob["executed"])==ran,(name,ob,parts);assert len(ob["restored"])==hits,(name,ob,parts)
        rows.append({"name":name+" ("+", ".join(f"{v} {k}" for k,v in sorted(parts.items()))+")","tasks":ids2,"compact":b64,"spawns":sp,"observation":ob})
    shutil.rmtree(f"{LAB}-ob",ignore_errors=True);shutil.rmtree(f"{LAB}-disk",ignore_errors=True)
    record("cold build")
    record("warm rebuild")
    record("comment edit",lambda:open(f"{LAB}/lib/gen.py","a").write("# [incremental] k3j9x2a\n"))
    record("body edit",lambda:open(f"{LAB}/lib/gen.py","a").write("value = 2\n"))
    record("rebuild after clean",None,True)
    subprocess.run([BAZEL,f"--output_base={LAB}-ob","--nosystem_rc","--nohome_rc","shutdown"],cwd=LAB,capture_output=True)
    return rows
def nxrecords():
    js=r'''
const {StoreRunInformationLifeCycle}=require("/Users/ueli/Documents/semio/node_modules/nx/dist/src/tasks-runner/life-cycles/store-run-information-life-cycle");
const out=[];const lc=new StoreRunInformationLifeCycle("nx run-many -t build",d=>out.push(d),()=>"2026-10-20T00:00:00.000Z");
const t=(id,hash)=>({id,hash,target:{project:id.split(":")[0],target:id.split(":")[1]},startTime:1,endTime:2});
lc.startCommand();lc.startTasks([t("a:build","h1"),t("b:build","h2"),t("c:build","h3"),t("z:other","h4")]);
lc.endTasks([{task:t("a:build","h1"),status:"local-cache",code:0},{task:t("b:build","h2"),status:"success",code:0},{task:t("c:build","h3"),status:"remote-cache",code:0},{task:t("z:other","h4"),status:"local-cache-kept-existing",code:0}]);
lc.endCommand();console.log(JSON.stringify(out[0]));'''
    os.makedirs("/private/tmp/bzlincr/nxrun",exist_ok=True)
    r=subprocess.run(["node","-e",js],capture_output=True,text=True,env={**os.environ,"NX_WORKSPACE_ROOT_PATH":"/private/tmp/bzlincr/nxrun","NX_DAEMON":"false"},cwd="/private/tmp/bzlincr/nxrun")
    assert r.returncode==0,r.stderr
    rec=json.loads(r.stdout);ids3=["a:build","b:build","c:build"]
    return [{"name":"hits, a miss and a task outside the selection","tasks":ids3,"record":rec,"observation":oracle.nx(rec,ids3)}]
fixture={"version":1,"sensitivity":sens,"evaluation":evaluation,"verdicts":verdicts,"mutations":mutations,"labels":labels,"environments":environments,"refusals":refusals,"bazelLogs":logs(),"nxRecords":nxrecords()}
json.dump(fixture,open(sys.argv[1],"w",encoding="utf8"),ensure_ascii=False,indent=1)
print("evaluation",len(evaluation),"verdicts",len(verdicts),"mutations",len(mutations),"bazelLogs",len(fixture["bazelLogs"]))
