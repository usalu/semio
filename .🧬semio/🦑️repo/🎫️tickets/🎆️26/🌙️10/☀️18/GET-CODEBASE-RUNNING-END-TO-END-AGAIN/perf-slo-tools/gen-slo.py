#!/usr/bin/env python3
"""Writes src/slo.json: the SLO table (labels EN/DE, edit units, probes, objectives). Every number names its source; a row without a goal and a gate only records the value (the regression guard against a stored baseline still applies)."""
import json,os,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
V=names.V
def T(en,de): return {"en":en,"de":de}
L={
 "title":T("Performance objectives","Leistungsziele"),"name":T("Name","Name"),"list":T("Objectives","Ziele"),"probes":T("Probes","Messungen"),"slos":T("Objectives","Ziele"),
 "host":T("Host","Rechner"),"platform":T("Platform","Plattform"),"cpus":T("Cores","Kerne"),"memory":T("Memory","Arbeitsspeicher"),"load":T("Load per core","Last je Kern"),"hostClass":T("Host class","Rechnerklasse"),
 "probe":T("Probe","Messung"),"slo":T("Objective","Ziel"),"tool":T("Implementation","Implementierung"),"lane":T("Lane","Pfad"),"oneshot":T("one-shot","Einzelaufruf"),"resident":T("resident service","residenter Dienst"),"neutral":T("neutral","neutral"),
 "metric":T("Metric","Messgröße"),"statistic":T("Statistic","Statistik"),"value":T("Value","Wert"),"goal":T("Goal","Ziel"),"gate":T("Gate","Schwelle"),"samples":T("Samples","Stichproben"),"status":T("Status","Status"),"reason":T("Reason","Grund"),
 "tolerance":T("Tolerance","Toleranz"),"enforcement":T("Enforcement","Durchsetzung"),"advisory":T("advisory","beratend"),"blocking":T("blocking","verbindlich"),
 "pass":T("pass","erfüllt"),"warn":T("goal missed","Ziel verfehlt"),"fail":T("fail","verletzt"),"unproven":T("unproven","unbewiesen"),"recorded":T("recorded","erfasst"),
 "improved":T("improved","verbessert"),"same":T("same","gleich"),"regressed":T("regressed","verschlechtert"),"inconclusive":T("inconclusive","nicht aussagekräftig"),"incomparable":T("incomparable","nicht vergleichbar"),"new":T("new","neu"),"missing":T("missing","fehlt"),
 "comparison":T("Comparison with the baseline","Vergleich mit der Referenz"),"baseline":T("Baseline","Referenz"),"current":T("Current","Aktuell"),"delta":T("Change","Änderung"),"relative":T("Relative","Relativ"),"summary":T("Summary","Zusammenfassung"),"series":T("Series","Reihen"),
 "quiet":T("quiet","ruhig"),"busy":T("busy","ausgelastet"),"saturated":T("saturated","überlastet"),"unknown":T("unknown","unbekannt"),
 "runs":T("Runs","Läufe"),"warmup":T("Warm-up","Aufwärmläufe"),"kind":T("Kind","Art"),"unit":T("Unit","Einheit"),"source":T("Source","Quelle"),"progress":T("Measured","Gemessen"),"started":T("Started","Gestartet"),"finished":T("Finished","Beendet"),
 "startKinds":T("Start observed by","Start beobachtet durch"),"marker":T("tool marker","Werkzeugmarke"),"first-output":T("first output","erste Ausgabe"),"exit":T("process exit","Prozessende"),
 "inflated":T("inflated by host load","durch Rechnerlast überhöht"),"blockingFailed":T("blocking objectives failed","verbindliche Ziele verletzt"),"blockingRegressed":T("blocking objectives regressed","verbindliche Ziele verschlechtert"),
 "notes":T("Notes","Hinweise"),
 "note-load":T("Times on a saturated host are upper bounds: a missed gate there stays unproven, a met goal stays met.","Zeiten auf einem überlasteten Rechner sind obere Schranken: eine verfehlte Schwelle bleibt dort unbewiesen, ein erreichtes Ziel bleibt erreicht."),
 "note-percentile":T("Percentiles interpolate linearly between ranks (inclusive method); p95 needs 20 samples, p99 needs 100.","Perzentile interpolieren linear zwischen den Rängen (inklusive Methode); p95 braucht 20 Stichproben, p99 braucht 100."),
 "note-start":T("Edit to rebuild start ends at the first tool marker line, else at the first output line, else at the process exit.","Von der Änderung bis zum Start des Neubaus zählt bis zur ersten Werkzeugmarke, sonst bis zur ersten Ausgabezeile, sonst bis zum Prozessende."),
 "note-recorded":T("Objectives without goal and gate only record the value; the baseline comparison guards them.","Ziele ohne Zielwert und Schwelle erfassen nur den Wert; der Vergleich mit der Referenz schützt sie."),
 "cold":T("cold","kalt"),"target":T("Target","Ziel"),"workload":T("Workload","Arbeitslast"),"sessions":T("Sessions","Sitzungen"),"file":T("File","Datei"),"language":T("Language","Sprache"),
 "edit":T("edit to rebuild","Änderung bis Neubau"),"cache":T("cache hit rate","Cache-Trefferquote"),"graph":T("graph","Graph"),"noop":T("warm no-op","warmer Leerlauf"),"cli-help":T("command line help","Kommandozeilenhilfe"),"concurrency":T("concurrent agents","gleichzeitige Agenten"),
 "nx":T("Nx","Nx"),"bazel":T("Bazel","Bazel"),"ms":T("ms","ms"),"bytes":T("bytes","Bytes"),"ratio":T("ratio","Verhältnis"),
 "wallMs":T("wall time","Laufzeit"),"startMs":T("edit to rebuild start","Änderung bis Start des Neubaus"),"greenMs":T("edit to green","Änderung bis grün"),"peakRssBytes":T("peak resident memory","Speicherspitze"),"hitRatio":T("cache hit ratio","Cache-Trefferquote"),"makespanMs":T("makespan","Gesamtdauer"),"maxUnrelatedWaitMs":T("longest wait for an unrelated project","längste Wartezeit auf ein fremdes Projekt"),"speedup":T("speed-up over serial","Beschleunigung gegenüber seriell"),
 "no-samples":T("no samples","keine Stichproben"),"too-few-samples":T("too few samples for the statistic","zu wenige Stichproben für die Statistik"),"probe-failed":T("the measured command failed","der gemessene Befehl ist fehlgeschlagen"),"unavailable":T("not available","nicht verfügbar"),"not-cold":T("the copy was not freshly prepared (use --prepare)","die Kopie ist nicht frisch vorbereitet (--prepare verwenden)"),"host-saturated":T("the host is saturated","der Rechner ist überlastet"),"service-unavailable":T("the resident service is not part of this checkout","der residente Dienst ist nicht Teil dieses Checkouts"),"unsupported":T("not supported here","hier nicht unterstützt"),"host-class":T("another host class","andere Rechnerklasse"),"schema":T("another report version","andere Berichtsversion")
}
def lab(k): return L[k]
UNITS=[
 {"id":"ts","language":"ts","target":"@semio-tech/framework-graph:test","file":"🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📜️script.ts","op":"private-body","title":T("TypeScript package","TypeScript-Paket")},
 {"id":"rust","language":"rs","target":"@teaching/proctor:build","file":"🎓️teaching/🛂️proctor/📦️packages/🦀️rust/🦀️.rs","op":"private-body","title":T("Rust crate","Rust-Crate")},
 {"id":"go","language":"go","target":"@semio-tech/repo-move-go:test","file":"🧰️framework/🛍️products/🦑️repo/🔨️modules/🚚️move/📦️packages/🐹️go/🐹️.go","op":"private-body","title":T("Go module","Go-Modul")},
 {"id":"python","language":"py","target":"@semio-tech/ui-styling-py:test","file":"🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/🎨️styling/__init__.py","op":"private-body","title":T("Python package","Python-Paket")},
]
BUILD="@teaching/proctor:build";TEST="@semio-tech/framework-graph:test"
probes=[];slos=[]
def probe(**kw): probes.append(kw)
def slo(id,title,probe,metric,statistic,direction,goal,gate,tools,enforcement,source,rel=0.15,abs_=25):
    slos.append({"id":id,"title":title,"probe":probe,"metric":metric,"statistic":statistic,"direction":direction,"goal":goal,"gate":gate,"tools":tools,"enforcement":enforcement,"tolerance":{"relative":rel,"absolute":abs_},"source":source})
BOTH=["nx","bazel"]
probe(id="cli-help",kind="cli-help",title=T("repo --help","repo --help"),runs=100,warmup=3)
slo("cli-help-p95",T("Command line help, 95th percentile","Kommandozeilenhilfe, 95. Perzentil"),"cli-help","wallMs","p95","lower",200,400,["neutral"],"advisory","cli-latency")
slo("cli-help-p99",T("Command line help, 99th percentile","Kommandozeilenhilfe, 99. Perzentil"),"cli-help","wallMs","p99","lower",None,1000,["neutral"],"advisory","provisional",0.25,50)
for lane,laneEn,laneDe in (("oneshot","one-shot","Einzelaufruf"),("resident","resident service","residenter Dienst")):
    for verb,target in (("run",BUILD),("test",TEST)):
        pid=f"noop-{verb}-{lane}"
        probe(id=pid,kind="noop",title=T(f"Warm no-op {verb}, {laneEn}",f"Warmer Leerlauf {verb}, {laneDe}"),lane=lane,target=target,runs=30,warmup=2)
        if lane=="oneshot":
            slo(f"{pid}-p50",T(f"Warm no-op {verb}, {laneEn}, median",f"Warmer Leerlauf {verb}, {laneDe}, Median"),pid,"wallMs","p50","lower",300,1000,BOTH,"advisory","user-goal",0.15,25)
        else:
            slo(f"{pid}-p95",T(f"Warm no-op {verb}, {laneEn}, 95th percentile",f"Warmer Leerlauf {verb}, {laneDe}, 95. Perzentil"),pid,"wallMs","p95","lower",300,300,BOTH,"blocking","user-goal",0.15,25)
for unit in UNITS:
    for lane,laneEn,laneDe in (("oneshot","one-shot","Einzelaufruf"),("resident","resident service","residenter Dienst")):
        pid=f"edit-{unit['id']}-{lane}"
        probe(id=pid,kind="edit",title=T(f"Edit and rebuild, {unit['title']['en']}, {laneEn}",f"Ändern und neu bauen, {unit['title']['de']}, {laneDe}"),lane=lane,unit=unit["id"],runs=10 if lane=="oneshot" else 20,warmup=1)
        if lane=="oneshot":
            slo(f"{pid}-start-p50",T(f"Edit to rebuild start, {unit['title']['en']}, {laneEn}, median",f"Änderung bis Start des Neubaus, {unit['title']['de']}, {laneDe}, Median"),pid,"startMs","p50","lower",500,2000,BOTH,"advisory","user-goal",0.2,50)
        else:
            slo(f"{pid}-start-p95",T(f"Edit to rebuild start, {unit['title']['en']}, {laneEn}, 95th percentile",f"Änderung bis Start des Neubaus, {unit['title']['de']}, {laneDe}, 95. Perzentil"),pid,"startMs","p95","lower",500,500,BOTH,"blocking","user-goal",0.2,50)
        slo(f"{pid}-green-p50",T(f"Edit to green, {unit['title']['en']}, {laneEn}, median",f"Änderung bis grün, {unit['title']['de']}, {laneDe}, Median"),pid,"greenMs","p50","lower",None,None,BOTH,"advisory","provisional",0.2,250)
probe(id="graph-scope-warm",kind="graph",title=T("Graph of the selected scope, warm","Graph des gewählten Umfangs, warm"),lane="oneshot",target=TEST,runs=10,warmup=1)
slo("graph-scope-warm-p50",T("Graph of the selected scope, warm, median","Graph des gewählten Umfangs, warm, Median"),"graph-scope-warm","wallMs","p50","lower",300,2000,BOTH,"advisory","provisional",0.2,50)
probe(id="graph-scope-cold",kind="graph",title=T("Graph of the selected scope, cold","Graph des gewählten Umfangs, kalt"),lane="oneshot",target=TEST,runs=1,warmup=0,cold=True)
slo("graph-scope-cold-max",T("Graph of the selected scope, cold","Graph des gewählten Umfangs, kalt"),"graph-scope-cold","wallMs","max","lower",None,300000,BOTH,"advisory","graph-speed",0.25,1000)
for verb,target in (("build",BUILD),("test",TEST)):
    pid=f"cache-second-run-{verb}"
    probe(id=pid,kind="cache",title=T(f"Second run of {verb}, cache hits",f"Zweiter Lauf von {verb}, Cache-Treffer"),target=target,runs=3,warmup=0)
    slo(f"{pid}-min",T(f"Second run of {verb}: every cacheable task restored",f"Zweiter Lauf von {verb}: jeder cachebare Task wiederhergestellt"),pid,"hitRatio","min","higher",1,1,BOTH,"blocking","user-goal",0,0)
probe(id="mem-noop-run",kind="noop",title=T("Peak memory of a warm no-op run","Speicherspitze eines warmen Leerlaufs"),lane="oneshot",target=BUILD,runs=5,warmup=1,memory=True)
slo("mem-noop-run-max",T("Peak memory of a warm no-op run","Speicherspitze eines warmen Leerlaufs"),"mem-noop-run","peakRssBytes","max","lower",536870912,1073741824,BOTH,"advisory","hot-cache",0.2,16777216)
probe(id="mem-edit-ts",kind="edit",title=T("Peak memory of a TypeScript edit and rebuild","Speicherspitze von Ändern und Neubau in TypeScript"),lane="oneshot",unit="ts",runs=3,warmup=1,memory=True)
slo("mem-edit-ts-max",T("Peak memory of a TypeScript edit and rebuild","Speicherspitze von Ändern und Neubau in TypeScript"),"mem-edit-ts","peakRssBytes","max","lower",None,None,BOTH,"advisory","provisional",0.2,16777216)
probe(id="concurrency-disjoint",kind="concurrency",title=T("Three agents on disjoint projects","Drei Agenten auf getrennten Projekten"),workload="disjoint-projects",sessions=3,runs=1,warmup=0)
slo("concurrency-disjoint-wait-max",T("Longest wait of an agent for an unrelated project","Längste Wartezeit eines Agenten auf ein fremdes Projekt"),"concurrency-disjoint","maxUnrelatedWaitMs","max","lower",1000,5000,BOTH,"advisory","conc-harness",0.25,250)
slo("concurrency-disjoint-makespan-max",T("Makespan of three agents on disjoint projects","Gesamtdauer dreier Agenten auf getrennten Projekten"),"concurrency-disjoint","makespanMs","max","lower",None,None,BOTH,"advisory","provisional",0.25,1000)
probe(id="concurrency-warm-fanout",kind="concurrency",title=T("Three agents build the same warm target","Drei Agenten bauen dasselbe warme Ziel"),workload="warm-fanout",sessions=3,runs=1,warmup=0)
slo("concurrency-warm-fanout-makespan-max",T("Makespan of three agents on one warm target","Gesamtdauer dreier Agenten auf einem warmen Ziel"),"concurrency-warm-fanout","makespanMs","max","lower",None,None,BOTH,"advisory","provisional",0.25,1000)
slo("concurrency-warm-fanout-wait-max",T("Longest wait of an agent on one warm target","Längste Wartezeit eines Agenten auf einem warmen Ziel"),"concurrency-warm-fanout","maxUnrelatedWaitMs","max","lower",1000,5000,BOTH,"advisory","conc-harness",0.25,250)
doc={"version":1,"labels":L,"units":UNITS,"probes":probes,"slos":slos}
json.dump(doc,open(os.path.join(here,"src","slo.json"),"w",encoding="utf8"),ensure_ascii=False,indent=1);open(os.path.join(here,"src","slo.json"),"a").write("\n")
print("units",len(UNITS),"probes",len(probes),"slos",len(slos))
