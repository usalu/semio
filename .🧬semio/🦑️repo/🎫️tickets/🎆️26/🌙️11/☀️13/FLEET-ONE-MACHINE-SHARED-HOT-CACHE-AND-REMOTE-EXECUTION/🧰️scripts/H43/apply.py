import sys,os,re
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def edit(rel,pairs):
    p=B+rel;s=open(p,encoding="utf-8").read()
    for old,new in pairs:
        if s.count(old)!=1:sys.exit("anchor count %d in %s: %s"%(s.count(old),rel,old[:70]))
        s=s.replace(old,new)
    t=p+".h43.tmp";open(t,"w",encoding="utf-8").write(s);os.replace(t,p)
st="📥️inputs/💾️storage/"
edit(st+"📒️ledger/🟨️.mjs",[
 ("const clean=bytes=>database.row(\"update semio_graph_optional set used_bytes=?,dirty=0 where id=1 returning used_bytes\",bytes);",
  "const clean=bytes=>database.row(\"update semio_graph_optional set used_bytes=?,dirty=0 where id=1 returning used_bytes\",bytes),counted=()=>{const bytes=measure();database.row(\"insert into semio_graph_optional_measure values(1,?) on conflict(id) do update set measured_ms=excluded.measured_ms returning id\",Date.now());return clean(bytes);},measuredAgo=()=>{const at=database.row(\"select measured_ms measuredMs from semio_graph_optional_measure where id=1\")?.measuredMs,age=Date.now()-at;return Number.isSafeInteger(at)&&age>=0?age:Infinity;};"),
 ("if(state.dirty){clean(measure());state=observe();}","if(state.dirty){counted();state=observe();}"),
 ("observe();clean(measure());});}catch(error){openingFailed=true;","database.exec(\"create table if not exists semio_graph_optional_measure(id integer primary key check(id=1),measured_ms integer not null)\");if(observe().dirty||measuredAgo()>=protocol.remeasureMilliseconds)counted();});}catch(error){openingFailed=true;"),
])
edit(st+"🧬️schema/🔣️.json",[('"inventoryEntryLimit": 1048576,','"inventoryEntryLimit": 1048576,\n            "remeasureMilliseconds": 86400000,')])
edit("🧠️memory/📋️slice-report/🧬️schema/🔣️.json",[
 ('"en": "waiting for the graph state lease and checking the graph store", "de": "Warte auf die Graphstatus-Leihe und prüfe den Graphspeicher"',
  '"en": "opening the graph store and its quota ledger and taking the graph state lease", "de": "Öffne den Graphspeicher samt Kontingentbuch und nehme die Graphstatus-Leihe"')])
edit("🧠️memory/📋️slice-report/🧪️tests/🪜️read-steps/🟦️.ts",[("waiting for the graph state lease and checking the graph store","opening the graph store and its quota ledger and taking the graph state lease")])
edit(st+"🧪️tests/📒️ledger/🟦️.ts",[
 ("second=await api.openBuildGraphOptionalLedgerV1(options);\n","let reopened=0;const reopenRead=fs.readdirSync,reopenSpy=spyOn(fs,\"readdirSync\").mockImplementation(((...args:any[])=>{if(String(args[0])===join(root,\"inference\"))reopened++;return(reopenRead as any)(...args);})as any);try{second=await api.openBuildGraphOptionalLedgerV1(options);}finally{reopenSpy.mockRestore();}expect(reopened).toBe(0);\n"),
])
print("applied")
