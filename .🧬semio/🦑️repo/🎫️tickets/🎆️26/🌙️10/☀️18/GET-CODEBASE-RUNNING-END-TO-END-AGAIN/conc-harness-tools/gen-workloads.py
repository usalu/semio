#!/usr/bin/env python3
"""Writes src/workloads.json: the authored workloads, lease classes, criteria and bilingual labels of the concurrent-session harness."""
import json,os,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
n=names.n;F=names.FRAMEWORK;V=names.V
def L(en,de): return {"en":en,"de":de}
SCRIPT=chr(0x1F4DC)+V+"script.ts";RS=chr(0x1F980)+V+".rs";GO=chr(0x1F439)+V+".go"
RUST=n(0x1F980,"rust");PKG=n(0x1F4E6,"packages");MOD=n(0x1F528,"modules")
FILES={
 "ts":f"{F}/{MOD}/{n(0x1F578,'graph')}/{PKG}/{RUST}/{SCRIPT}",
 "rust":f"{n(0x1F393,'teaching')}/{n(0x1F6C2,'proctor')}/{PKG}/{RUST}/{RS}",
 "go":f"{F}/{names.PRODUCTS}/{names.REPO}/{MOD}/{n(0x1F69A,'move')}/{PKG}/{n(0x1F439,'go')}/{GO}",
 "schema":f"{F}/{MOD}/{n(0x1F9EC,'schema')}/{PKG}/{RUST}/{RS}",
}
for k,v in FILES.items():
    if not os.path.exists(os.path.join(names.R,v)):sys.exit(f"missing {k}: {v}")
    if len(os.path.join(names.R,v))>255:sys.exit("too long "+v)
TARGETS={"ts":"@semio-tech/framework-graph:test","rust":"@teaching/proctor:build","go":"@semio-tech/repo-move-go:test","schema":"@semio-tech/framework-schema:test-subset-contract"}
LANG={"ts":"ts","rust":"rs","go":"go","schema":"rs"}
ALL=["conc-graph","conc-bazel","conc-cargo","conc-deps","conc-leases"]
def disjoint(area):
    return {"id":area,"area":"disjoint-"+area,"group":"g-"+area,"steps":[
     {"kind":"edit","file":FILES[area],"op":"comment","language":LANG[area]},
     {"kind":"barrier","id":"go"},
     {"kind":"run","target":TARGETS[area]},
     {"kind":"revert"},
     {"kind":"run","target":TARGETS[area]}]}
cold=lambda sid,delay:{"id":sid,"area":"same-target","group":"g-same","delayMilliseconds":delay,"steps":[{"kind":"barrier","id":"go"},{"kind":"run","target":TARGETS["ts"]}]}
def reader(sid):
    return {"id":sid,"area":"reader","group":"g-"+sid,"steps":[{"kind":"barrier","id":"go"},{"kind":"repo","args":["show","projects"],"stdout":True}]}
workloads=[
 {"id":"disjoint-projects","title":L("Agents edit and build disjoint projects","Agenten ändern und bauen getrennte Projekte"),
  "summary":L("A TypeScript package, a Rust crate, a Go module and a schema crate are edited, built and tested at the same time; nobody may wait for an unrelated project and every result equals a serial run.","Ein TypeScript-Paket, eine Rust-Crate, ein Go-Modul und eine Schema-Crate werden gleichzeitig geändert, gebaut und getestet; niemand darf auf ein fremdes Projekt warten und jedes Ergebnis entspricht einem seriellen Lauf."),
  "criteria":"default","setup":[],"sessions":[disjoint("ts"),disjoint("rust"),disjoint("go"),disjoint("schema")]},
 {"id":"same-target-cold","title":L("Agents build the same changed target at once","Agenten bauen gleichzeitig dasselbe geänderte Ziel"),
  "summary":L("One edit makes every cache stale; three agents then build the same target together. Each task may execute once, the other agents must find its result.","Eine Änderung macht jeden Cache ungültig; drei Agenten bauen dann zusammen dasselbe Ziel. Jeder Task darf einmal laufen, die anderen Agenten müssen sein Ergebnis finden."),
  "criteria":"default","setup":[{"kind":"edit","file":FILES["ts"],"op":"private-body","language":"ts"}],
  "sessions":[cold("a",0),cold("b",250),cold("c",500)]},
 {"id":"warm-fanout","title":L("Agents build a warm target at once","Agenten bauen gleichzeitig ein warmes Ziel"),
  "summary":L("The target is built once before; three agents then build it together and every one must find everything in the cache.","Das Ziel wird vorab einmal gebaut; drei Agenten bauen es dann zusammen und jeder muss alles im Cache finden."),
  "criteria":"warm","setup":[{"kind":"run","target":TARGETS["ts"]}],
  "sessions":[cold("a",0),cold("b",0),cold("c",0)]},
 {"id":"readers-while-edit","title":L("Agents read the graph while another agent edits","Agenten lesen den Graphen, während ein anderer Agent ändert"),
  "summary":L("One agent edits a source and lists the projects while two others list the projects too: a source edit must not make unrelated readers wait for a rebuild of the graph facts.","Ein Agent ändert eine Quelle und listet die Projekte, zwei weitere listen die Projekte ebenfalls: eine Quelländerung darf unbeteiligte Leser nicht auf den Neuaufbau der Graph-Fakten warten lassen."),
  "criteria":"default","setup":[],
  "sessions":[{"id":"editor","area":"editor","group":"g-editor","steps":[{"kind":"edit","file":FILES["ts"],"op":"comment","language":"ts"},{"kind":"barrier","id":"go"},{"kind":"repo","args":["show","projects"],"stdout":True},{"kind":"revert"},{"kind":"repo","args":["show","projects"],"stdout":True}]},reader("reader-a"),reader("reader-b")]},
]
def waitrule(rid,classes,owners,title_en,title_de,limit=5000):
    return {"id":rid,"kind":"max-wait","title":L(title_en,title_de),"owners":owners,"classes":classes,"relation":"unrelated","maximumMilliseconds":limit}
core=[
 {"id":"all-steps-succeed","kind":"no-failure","title":L("Every step exits successfully","Jeder Schritt endet erfolgreich"),"owners":ALL},
 {"id":"outputs-equal-serial","kind":"serial-equal","compare":["exit","outputs","stdout"],"title":L("Every session's results equal a serial run","Die Ergebnisse jeder Sitzung entsprechen einem seriellen Lauf"),"owners":ALL},
 waitrule("graph-facts-unrelated-wait",["graph-facts"],["conc-graph"],"No wait above 5 s on the graph facts for an unrelated project","Keine Wartezeit über 5 s auf die Graph-Fakten für ein fremdes Projekt"),
 waitrule("dependency-image-unrelated-wait",["dependency-image"],["conc-deps"],"No wait above 5 s on the dependency image for an unrelated project","Keine Wartezeit über 5 s auf das Abhängigkeits-Image für ein fremdes Projekt"),
 waitrule("bazel-server-unrelated-wait",["bazel-server"],["conc-bazel"],"No wait above 5 s on the Bazel server for an unrelated project","Keine Wartezeit über 5 s auf den Bazel-Server für ein fremdes Projekt"),
 waitrule("cargo-units-unrelated-wait",["cargo-units"],["conc-cargo"],"No wait above 5 s on Cargo units for an unrelated project","Keine Wartezeit über 5 s auf Cargo-Einheiten für ein fremdes Projekt"),
 waitrule("capacity-admission-unrelated-wait",["capacity-admission"],["conc-leases"],"No wait above 5 s for capacity admission behind an unrelated project","Keine Wartezeit über 5 s auf die Kapazitätszulassung hinter einem fremden Projekt"),
 waitrule("other-unrelated-wait",["toolchain-acquisition","other"],["conc-leases"],"No wait above 5 s on any other lock for an unrelated project","Keine Wartezeit über 5 s auf eine andere Sperre für ein fremdes Projekt"),
]
once={"id":"second-session-zero-actions","kind":"executes-once","sessions":["*"],"excludeUncached":True,"title":L("A task executes in one session only; the others execute no action for it","Ein Task läuft nur in einer Sitzung; die anderen führen dafür keine Aktion aus"),"owners":["conc-bazel","conc-graph"]}
warm={"id":"warm-sessions-zero-actions","kind":"max-executed","sessions":["*"],"maximum":0,"excludeUncached":True,"title":L("A warm target executes no cached action in any session","Ein warmes Ziel führt in keiner Sitzung eine zwischenspeicherbare Aktion aus"),"owners":["conc-bazel","conc-graph"]}
criteria=[
 {"id":"default","title":L("Isolation, sharing and correctness","Isolation, gemeinsame Nutzung und Korrektheit"),"rules":core+[once]},
 {"id":"warm","title":L("Isolation and a fully warm cache","Isolation und ein vollständig warmer Cache"),"rules":core+[warm]},
 {"id":"smoke","title":L("Successful steps and serial equality only","Nur erfolgreiche Schritte und serielle Gleichheit"),"rules":core[:2]},
]
pat=lambda *p:list(p)
classes=[
 {"id":"graph-facts","label":L("Graph facts","Graph-Fakten"),"scope":"workspace","patterns":pat("^repository-graph-facts:")},
 {"id":"dependency-image","label":L("Dependency image","Abhängigkeits-Image"),"scope":"workspace","patterns":pat("^native-fixed-image:")},
 {"id":"bazel-server","label":L("Bazel server","Bazel-Server"),"scope":"workspace","patterns":pat("^bazel:server$","^bazel-server")},
 {"id":"cargo-units","label":L("Cargo units","Cargo-Einheiten"),"scope":"workspace","patterns":pat("^cargo:","^cargo-unit")},
 {"id":"capacity-admission","label":L("Capacity admission","Kapazitätszulassung"),"scope":"host","patterns":pat("^semio\\.host\\.capacity/","^compiler:capacity")},
 {"id":"toolchain-acquisition","label":L("Toolchain acquisition","Werkzeugbeschaffung"),"scope":"host","patterns":pat("^/.*/(?:bun|bazel|cargo|node)(?:\\.exe)?$","^[A-Za-z]:[\\\\/].*(?:bun|bazel|cargo|node)(?:\\.exe)?$")},
 {"id":"other","label":L("Other lock","Andere Sperre"),"scope":"workspace","patterns":pat("^")},
]
labels={
 "title":L("Concurrent session conformance","Konformität gleichzeitiger Sitzungen"),"workload":L("workload","Arbeitslast"),"tool":L("tool","Werkzeug"),"session":L("session","Sitzung"),"group":L("group","Gruppe"),
 "step":L("step","Schritt"),"wall":L("wall ms","Dauer ms"),"wait":L("wait ms","Wartezeit ms"),"executed":L("executed","ausgeführt"),"restored":L("restored","wiederhergestellt"),"cached":L("cached","im Cache"),
 "steps":L("steps","Schritte"),"concurrent":L("concurrent","gleichzeitig"),"serial":L("serial","seriell"),"leases":L("lock waits","Sperrwartezeiten"),"lease":L("lock","Sperre"),"count":L("waits","Wartevorgänge"),
 "total":L("total ms","gesamt ms"),"maximum":L("longest ms","längste ms"),"relation":L("relation","Beziehung"),"related":L("related","verwandt"),"unrelated":L("unrelated","fremd"),"external":L("external","extern"),
 "holders":L("holders","Halter"),"rules":L("pass criteria","Abnahmekriterien"),"rule":L("criterion","Kriterium"),"status":L("status","Status"),"pass":L("pass","bestanden"),"fail":L("fail","fehlgeschlagen"),"skip":L("skipped","übersprungen"),
 "value":L("value","Wert"),"limit":L("limit","Grenze"),"owner":L("executor","Executor"),"acceptance":L("acceptance by executor","Abnahme je Executor"),"summary":L("summary","Zusammenfassung"),"criteria":L("criteria","Kriterien"),
 "sessions":L("sessions","Sitzungen"),"milliseconds":L("milliseconds","Millisekunden"),"area":L("area","Bereich"),"setup":L("setup","Vorbereitung"),"progress":L("done","fertig"),"cancelled":L("cancelled","abgebrochen"),
 "unproven":L("unproven","unbewiesen"),"exact":L("exact","exakt"),"window":L("by time window","nach Zeitfenster"),"ambiguous":L("ambiguous","mehrdeutig"),"none":L("none","keine"),"start":L("start ms","Beginn ms"),"end":L("end ms","Ende ms"),
 "exit":L("exit","Ende"),"target":L("target","Ziel"),"phase":L("phase","Phase"),"list":L("workloads","Arbeitslasten"),"uncached":L("uncached tasks","nicht zwischengespeicherte Tasks"),"independent":L("independent","unabhängig"),"refused":L("refused","abgelehnt"),
}
data={"version":1,"labels":labels,"classes":classes,"criteria":criteria,"workloads":workloads}
json.dump(data,open(os.path.join(here,"src","workloads.json"),"w",encoding="utf8"),ensure_ascii=False,indent=2);open(os.path.join(here,"src","workloads.json"),"a").write("\n")
print("workloads",len(workloads),"criteria",len(criteria),"classes",len(classes))
