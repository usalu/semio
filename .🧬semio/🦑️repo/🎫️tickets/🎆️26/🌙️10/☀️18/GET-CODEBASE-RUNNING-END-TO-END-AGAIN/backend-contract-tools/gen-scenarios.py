#!/usr/bin/env python3
"""Generates src/conformance.scenarios.json: the stub workspace and every scenario, as data. Run again after editing."""
import json,os
here=os.path.dirname(os.path.abspath(__file__))
def T(cache=True,reads=None,outputs=None,mode="emit",ms=None,normalize=False,report=False,deps=None,kind="build"):
    action={"mode":mode}
    if ms:action["milliseconds"]=ms
    if normalize:action["normalize"]=True
    if report:action["report"]=True
    row={"cache":cache,"kind":kind,"reads":reads or [],"outputs":outputs or [],"action":action}
    if deps:row["dependsOn"]=deps
    return row
def P(name,targets,deps=None): return {"name":name,"dependsOn":deps or [],"targets":targets}
src=lambda p:[f"{p}/src/**"]
projects=[
 P("core",{"build":T(reads=src("core"),outputs=["core/out/core.txt"])}),
 P("util",{"build":T(reads=src("util"),outputs=["util/out/util.txt"])}),
 P("app",{"build":T(reads=src("app"),outputs=["app/out/app.txt"],deps=["core:build","util:build"]),
          "test":T(reads=["app/test/**"],outputs=["app/out/test-report.txt"],deps=["app:build"],report=True,kind="test"),
          "lint":T(cache=False,reads=src("app"),kind="check")},["core","util"]),
 P("sibling",{"build":T(reads=src("sibling"),outputs=["sibling/out/sibling.txt"])}),
 P("edge",{"build":T(reads=src("edge"),outputs=["edge/out/edge.txt"],normalize=True)}),
 P("user",{"build":T(reads=src("user"),outputs=["user/out/user.txt"],deps=["edge:build"])},["edge"]),
 P("multi",{"build":T(reads=src("multi"),outputs=["multi/out/a.txt","multi/out/nested/b.txt"])}),
 P("broken",{"build":T(reads=src("broken"),mode="fail")}),
 P("after",{"build":T(reads=src("after"),outputs=["after/out/after.txt"],deps=["broken:build"])},["broken"]),
 P("badtest",{"test":T(reads=src("badtest"),mode="fail",kind="test")}),
 P("slow",{"build":T(reads=src("slow"),outputs=["slow/out/slow.txt"],mode="sleep",ms=3000)}),
 P("shared",{"build":T(reads=src("shared"),outputs=["shared/out/shared.txt"],mode="sleep",ms=1500)}),
]
for n in ("p1","p2","p3"): projects.append(P(n,{"build":T(reads=src(n),outputs=[f"{n}/out/{n}.txt"],mode="sleep",ms=500)}))
files={}
for p in projects:
    n=p["name"]
    if any(t["reads"] for t in p["targets"].values()):
        files[f"{n}/src/a.txt"]=f"{n} source a\n"
        files[f"{n}/src/b.txt"]=f"{n} source b\n"
files["edge/src/a.txt"]="edge source a\n# note one\n"
files["app/test/t.txt"]="app test one\n"
def ev(*items): return [{"suite":s,"id":i} for s,i in items]
def run(targets,expect,**options):
    row={"step":"run","targets":targets,"expect":expect}
    if options:row["options"]=options
    return row
CORE_APP=["core:build","util:build","app:build"]
def out(task,path,state="expected"): return {"task":task,"path":path,"state":state}
scen=[]
def S(i,cap,en,de,steps,evidence=None):
    row={"id":i,"capability":cap,"title":{"en":en,"de":de},"steps":steps}
    if evidence:row["evidence"]=ev(*evidence)
    scen.append(row)
S("plan-closure","plan","A plan lists exactly the dependency closure with its edges.","Ein Plan nennt genau den Abhängigkeitsabschluss mit seinen Kanten.",
  [{"step":"plan","target":"app:test","tasks":["core:build","util:build","app:build","app:test"],"edges":{"app:test":["app:build"],"app:build":["core:build","util:build"],"core:build":[],"util:build":[]}}],[("parity","fw-graph-generate")])
S("run-cold","run","A cold run executes exactly the closure, in dependency order, with correct outputs, and nothing else.","Ein kalter Lauf führt genau den Abschluss in Abhängigkeitsreihenfolge mit korrekten Ausgaben aus, sonst nichts.",
  [run(["app:build"],{"exit":"zero","executed":CORE_APP,"ordered":True,"outputs":[out("core:build","core/out/core.txt"),out("util:build","util/out/util.txt"),out("app:build","app/out/app.txt"),out("sibling:build","sibling/out/sibling.txt","absent")]})],[("parity","fw-graph-test")])
S("cache-restore","cache-restore","A second identical run executes nothing and restores every output.","Ein zweiter identischer Lauf führt nichts aus und stellt alle Ausgaben wieder her.",
  [run(["app:build"],{"exit":"zero","executed":CORE_APP}),run(["app:build"],{"exit":"zero","executed":[],"outputs":[out("app:build","app/out/app.txt")]})],[("incremental","fw-ts-comment"),("bazel-contract","cache")])
S("restore-deleted-output","cache-restore","Deleted outputs come back from the cache without executing.","Gelöschte Ausgaben kommen aus dem Cache zurück, ohne auszuführen.",
  [run(["app:build"],{"exit":"zero","executed":CORE_APP}),{"step":"delete","paths":["core/out/core.txt","app/out/app.txt"]},run(["app:build"],{"exit":"zero","executed":[],"outputs":[out("core:build","core/out/core.txt"),out("app:build","app/out/app.txt")]})])
S("invalidation","invalidation","Editing one input re-executes exactly the task that reads it and what depends on its output.","Das Ändern einer Eingabe führt genau die lesende Aufgabe und ihre Abhängigen neu aus.",
  [run(["app:build"],{"exit":"zero","executed":CORE_APP}),{"step":"edit","file":"core/src/a.txt","op":"append-line"},run(["app:build"],{"exit":"zero","executed":["core:build","app:build"],"outputs":[out("core:build","core/out/core.txt","changed"),out("util:build","util/out/util.txt","unchanged"),out("app:build","app/out/app.txt","expected")]})],[("incremental","fw-ts-body")])
S("early-cutoff","early-cutoff","A change that leaves a task's output identical does not re-execute its dependents.","Eine Änderung, die die Ausgabe einer Aufgabe unverändert lässt, führt deren Abhängige nicht erneut aus.",
  [run(["user:build"],{"exit":"zero","executed":["edge:build","user:build"]}),{"step":"edit","file":"edge/src/a.txt","op":"append-comment"},run(["user:build"],{"exit":"zero","executed":["edge:build"],"outputs":[out("edge:build","edge/out/edge.txt","unchanged"),out("user:build","user/out/user.txt","unchanged")]})],[("incremental","fw-ts-comment")])
S("uncached-always-runs","uncached","A task that declares no cache executes on every run.","Eine Aufgabe ohne Cache wird bei jedem Lauf ausgeführt.",
  [run(["app:lint"],{"exit":"zero","executed":["app:lint"]}),run(["app:lint"],{"exit":"zero","executed":["app:lint"]})],[("bazel-contract","uncached")])
S("outputs-nested","outputs","Several declared outputs, including nested ones, are captured and restored together.","Mehrere deklarierte Ausgaben, auch verschachtelte, werden gemeinsam erfasst und wiederhergestellt.",
  [run(["multi:build"],{"exit":"zero","executed":["multi:build"],"outputs":[out("multi:build","multi/out/a.txt"),out("multi:build","multi/out/nested/b.txt")]}),{"step":"delete","paths":["multi/out/a.txt","multi/out/nested/b.txt"]},run(["multi:build"],{"exit":"zero","executed":[],"outputs":[out("multi:build","multi/out/a.txt"),out("multi:build","multi/out/nested/b.txt")]})],[("parity","fw-graph-generate")])
S("failure-isolation","failure-isolation","A failing task fails the run, its dependents do not execute and independent tasks still finish.","Eine fehlgeschlagene Aufgabe lässt den Lauf scheitern, ihre Abhängigen laufen nicht, unabhängige Aufgaben werden beendet.",
  [run(["after:build","sibling:build"],{"exit":"nonzero","executed":["broken:build","sibling:build"],"outputs":[out("sibling:build","sibling/out/sibling.txt"),out("after:build","after/out/after.txt","absent")]})])
S("failure-not-cached","failure-isolation","A failed task is never cached: it executes again on the next run.","Eine fehlgeschlagene Aufgabe wird nie zwischengespeichert: sie läuft beim nächsten Lauf erneut.",
  [run(["broken:build"],{"exit":"nonzero","executed":["broken:build"]}),run(["broken:build"],{"exit":"nonzero","executed":["broken:build"]})])
S("test-results","test-results","A passing test is cached, a failing test fails the run and is not cached.","Ein bestandener Test wird zwischengespeichert, ein fehlgeschlagener lässt den Lauf scheitern und wird nicht zwischengespeichert.",
  [run(["app:test"],{"exit":"zero","executed":CORE_APP+["app:test"],"outputs":[out("app:test","app/out/test-report.txt")]}),run(["app:test"],{"exit":"zero","executed":[]}),run(["badtest:test"],{"exit":"nonzero","executed":["badtest:test"]}),run(["badtest:test"],{"exit":"nonzero","executed":["badtest:test"]})],[("incremental","test-body")])
S("affected-closure","affected","Changed files select their projects and everything that depends on them, no more.","Geänderte Dateien wählen ihre Projekte und alle Abhängigen, nicht mehr.",
  [{"step":"affected","files":["core/src/a.txt"],"projects":["core","app"]},{"step":"affected","files":["sibling/src/a.txt"],"projects":["sibling"]}],[("incremental","fw-ts-unrelated")])
S("cancellation","cancellation","Cancelling stops the running task promptly and publishes nothing; the next run completes.","Abbrechen stoppt die laufende Aufgabe zügig und veröffentlicht nichts; der nächste Lauf wird abgeschlossen.",
  [{"step":"cancel","targets":["slow:build"],"afterStart":"slow:build","settled":"slow:build"},run(["slow:build"],{"exit":"zero","executed":["slow:build"],"outputs":[out("slow:build","slow/out/slow.txt")]})],[("bazel-contract","cancellation")])
S("progress","progress","The run reports a start and an end for every task it executes.","Der Lauf meldet für jede ausgeführte Aufgabe einen Start und ein Ende.",
  [run(["app:build"],{"exit":"zero","executed":CORE_APP,"events":"covers-executed"})])
S("parallelism-bounded","parallelism","Independent tasks overlap when allowed and never exceed the bound.","Unabhängige Aufgaben überlappen, wenn erlaubt, und überschreiten die Grenze nie.",
  [run(["p1:build","p2:build","p3:build"],{"exit":"zero","executed":["p1:build","p2:build","p3:build"],"minConcurrent":2},parallel=3)])
S("serial-bound","parallelism","With a bound of one, tasks never overlap.","Mit der Grenze eins überlappen Aufgaben nie.",
  [run(["p1:build","p2:build","p3:build"],{"exit":"zero","executed":["p1:build","p2:build","p3:build"],"maxConcurrent":1},parallel=1)])
S("concurrent-disjoint","concurrent-invocations","Two simultaneous invocations on disjoint targets both succeed.","Zwei gleichzeitige Aufrufe mit verschiedenen Zielen gelingen beide.",
  [{"step":"concurrent","runs":[["p1:build"],["p2:build"]],"expect":{"exit":"zero","executed":["p1:build","p2:build"]}}],[("concurrency","disjoint-projects")])
S("single-flight","single-flight","Two simultaneous invocations of one target execute its body once.","Zwei gleichzeitige Aufrufe desselben Ziels führen dessen Rumpf einmal aus.",
  [{"step":"concurrent","runs":[["shared:build"],["shared:build"]],"expect":{"exit":"zero","executed":["shared:build"],"startsOnce":["shared:build"]}}],[("concurrency","same-target-cold")])
S("remote-cache","remote-cache","A fresh checkout with an empty local cache restores everything from the shared cache.","Ein frischer Checkout mit leerem lokalem Cache stellt alles aus dem gemeinsamen Cache wieder her.",
  [run(["app:build"],{"exit":"zero","executed":CORE_APP},remote="read-write"),{"step":"fresh-checkout"},run(["app:build"],{"exit":"zero","executed":[],"outputs":[out("app:build","app/out/app.txt")]},remote="read-write")])
S("run-record","run-record","The backend's own record names the restored tasks of a cached run.","Die eigene Aufzeichnung des Backends nennt die wiederhergestellten Aufgaben eines zwischengespeicherten Laufs.",
  [run(["app:build"],{"exit":"zero","executed":CORE_APP}),run(["app:build"],{"exit":"zero","executed":[],"restored":CORE_APP})])
doc={"version":1,"workspace":{"files":dict(sorted(files.items())),"projects":projects},"scenarios":scen}
json.dump(doc,open(os.path.join(here,"src","conformance.scenarios.json"),"w",encoding="utf8"),ensure_ascii=False,indent=1)
print(len(scen),"scenarios",len(projects),"projects",len(files),"files")
