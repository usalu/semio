import sys,hashlib
R="/Users/ueli/Documents/semio/"
T=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/📋️slice-report/🧪️tests/🪜️read-steps/🟦️.ts"
t=open(T,encoding="utf8").read()
def sub(old,new):
    global t
    if t.count(old)!=1: sys.exit(f"anchor {t.count(old)}: {old[:80]!r}")
    t=t.replace(old,new)
sub("import{registerSliceReportSourceV1,sliceReadStepV1,describeSliceReadStepV1,","import{registerSliceReportSourceV1,sliceReadStepV1,sliceReadWaitStepV1,describeSliceReadStepV1,")
sub("(?:sliceReadStepV1|step)\\(\"","(?:sliceReadStepV1|sliceReadWaitStepV1|step)\\(\"")
sub('"orphan-sweep","receipt-check"]','"orphan-sweep","receipt-check","registry-read","graph-state","module-load"]')
sub("""phase="memo-publish";const repeated=await next(events.length);expect(repeated.phase).toBe(first.phase);""","""expect(first.phase).toContain("(attempt 1): scanning the owners");expect(first.phase).toContain("(Versuch 1): Durchsuche die Projekte");
   phase="memo-publish";const repeated=await next(events.length);expect(repeated.phase).toContain("publishing the admission memo");expect(repeated.phase).toContain("Veröffentliche das Zulassungsmemo");expect(repeated.phase).toContain("177 completed owner checks");""")
t+='''
test("the inventory names its attempt and sub-phase in both languages, so a restart is a new line, and a wait step ends when its body leaves it",async()=>{
 let phase="registry-check";const diagnostics=()=>({active:{phase}});
 await sliceReadStepV1("inventory",async()=>{
  const first=describeSliceReadStepV1()!.phase;
  expect(first).toContain("checking the file inventory of the tree (attempt 3): checking the owner registry");
  expect(first).toContain("Prüfe das Dateiverzeichnis des Baums (Versuch 3): Prüfe das Projektregister");
  phase="memo-read";expect(describeSliceReadStepV1()!.phase).toContain("(attempt 3): reading the admission memo");
 },{round:3},diagnostics);
 await sliceReadStepV1("inventory",async()=>{expect(describeSliceReadStepV1()!.phase).toContain("(attempt 4)");},{round:4});
 await sliceReadWaitStepV1("graph-state",async leave=>{
  expect(describeSliceReadStepV1()!.phase).toContain("waiting for the graph state lease and checking the graph store");
  expect(describeSliceReadStepV1()!.phase).toContain("Warte auf die Graphstatus-Leihe und prüfe den Graphspeicher");
  leave();leave();expect(describeSliceReadStepV1()).toBeUndefined();
 });
 expect(describeSliceReadStepV1()).toBeUndefined();
 await expect(sliceReadWaitStepV1("graph-state",async()=>{throw Error("lease");})).rejects.toThrow("lease");
 expect(describeSliceReadStepV1()).toBeUndefined();
});
'''
open(T,"w",encoding="utf8").write(t);print("tests patched")
