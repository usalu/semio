#!/usr/bin/env python3
"""Generates src/backends.registry.json: one row per backend. Capability claims are the registry's declared support; the conformance suite measures them."""
import json,os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import names
NATIVE_CALLER="../../"+"/".join([names.n(0x26A1,"caching"),names.n(0x1F680,"bootstrap"),names.n(0x1F50C,"nx-plugin"),names.n(0x1F4E4,"arguments"),names.n(0x1F9EC,"schema"),names.f(0x1F4E5,"native-caller.json")])
here=os.path.dirname(os.path.abspath(__file__))
def tx(en,de): return {"en":en,"de":de}
CAPS=["plan","run","cache-restore","invalidation","early-cutoff","uncached","outputs","failure-isolation","test-results","affected","cancellation","progress","parallelism","concurrent-invocations","single-flight","remote-cache","run-record"]
def caps(default,**over):
    out={}
    for c in CAPS:
        v=over.get(c.replace("-","_"))
        out[c]=v if isinstance(v,dict) else {"support":v or default}
    return out
def note(support,en,de): return {"support":support,"note":tx(en,de)}
nx_caps=caps("native",progress=note("adapter","Task start and end are published by the repository adapter, not by Nx.","Start und Ende einer Aufgabe veröffentlicht der Repository-Adapter, nicht Nx."),
   single_flight=note("missing","Two Nx processes on one workspace do not coordinate: a cold task can run in both.","Zwei Nx-Prozesse auf einem Workspace stimmen sich nicht ab: eine kalte Aufgabe kann in beiden laufen."),
   run_record=note("native","Nx stores a run record with the cache status of every task.","Nx speichert einen Laufbericht mit dem Cache-Status jeder Aufgabe."))
bz_caps=caps("adapter",run_record=note("adapter","Read from Bazel's compact execution log.","Aus Bazels kompaktem Ausführungsprotokoll gelesen."),
   single_flight=note("native","One Bazel server per output base serialises invocations, so the second one finds the first one's result.","Ein Bazel-Server pro Output-Base serialisiert Aufrufe, der zweite findet das Ergebnis des ersten."),
   early_cutoff=note("native","Action keys contain the digests of the inputs, so equal outputs stop propagation.","Aktionsschlüssel enthalten die Digests der Eingaben, gleiche Ausgaben stoppen die Weitergabe."))
sm_caps=caps("native")
ops_nx={"run":"native","run-many":"native","affected":"native","exec":"native","inputs":"adapter","reset":"adapter","tools":"adapter","cache":"adapter"}
ops_bz={"run":"adapter","run-many":"adapter","affected":"adapter","exec":"adapter","inputs":"missing","reset":"adapter","tools":"adapter","cache":"adapter"}
ops_sm={"run":"native","run-many":"native","affected":"native","exec":"native","inputs":"native","reset":"native","tools":"native","cache":"native"}
doc={"version":1,"backends":[
 {"id":"nx","title":tx("Nx with Bun","Nx mit Bun"),"kind":"external","default":True,"module":"../@@TOOLS@@/@@NX@@/@@BACKEND@@/@@TS@@","driver":"../@@TOOLS@@/@@NX@@/@@CONF@@/@@TS@@","dependencyModel":"consumers",
  "processEnvironment":{"NX_DAEMON":"false"},"nativeSnapshot":{"variable":"SEMIO_NX_NATIVE_SOURCE_SNAPSHOT","environment":{"NX_DAEMON":"false"}},"nativeCaller":{"namespace":"nx","schema":NATIVE_CALLER},"cacheProof":{"logMarker":"read the output from the cache|\\[local cache\\]|\\[existing outputs match the cache\\]"},"operations":ops_nx,"capabilities":nx_caps},
 {"id":"bazel","title":tx("Bazel","Bazel"),"kind":"external","module":"../@@TOOLS@@/@@BAZEL@@/@@BACKEND@@/@@TS@@","driver":"../@@TOOLS@@/@@BAZEL@@/@@CONF@@/@@TS@@","dependencyModel":"all","cacheProof":{"logMarker":"(?:disk|remote) cache hit","activity":True,"spawnMarker":{"executed":"[0-9]+ (?:linux-sandbox|darwin-sandbox|processwrapper-sandbox|windows-sandbox|local|worker)","completed":"Build completed successfully"}},"operations":ops_bz,"capabilities":bz_caps},
 {"id":"semio","title":tx("Semio build engine","Semio-Build-Engine"),"kind":"in-house","module":"../@@TOOLS@@/@@SEMIO@@/@@BACKEND@@/@@TS@@","driver":"../@@TOOLS@@/@@SEMIO@@/@@CONF@@/@@TS@@","dependencyModel":"all","cacheProof":{"activity":True},"operations":ops_sm,"capabilities":sm_caps},
]}
json.dump(doc,open(os.path.join(here,"src","backends.registry.json"),"w",encoding="utf8"),ensure_ascii=False,indent=1)
print("ok")
