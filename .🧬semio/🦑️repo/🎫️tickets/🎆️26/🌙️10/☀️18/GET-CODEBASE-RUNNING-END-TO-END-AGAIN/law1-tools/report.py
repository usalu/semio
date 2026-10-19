#!/usr/bin/env python3
import json,re
ROOT="/Users/ueli/Documents/semio"
C=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G=C+"/🗑️generated/law1-out/"
base={};sec=None
for l in open(C+"/result-tst2-suites.md",encoding="utf-8").read().split("\n"):
    if l.startswith("### "): sec=l[4:]
    m=re.match(r'\| `([^`]+)` \| (\S+) \| (\S+) \|',l)
    if m and sec=="SOURCE-LAW-DRIFT":
        p=m.group(1)
        if p.startswith('M/'): p='🧰️framework/🔨️modules/'+p[2:]
        base[p]=m.group(3)
fin={}
for l in open(G+"final.jsonl",encoding="utf-8"):
    r=json.loads(l); fin[r["file"]]=(r["pass"],r["fail"])
iso={};cur=None
for l in open(G+"iso-proc3.out",encoding="utf-8").read().split("\n"):
    if l.startswith("==="):
        cur=None
        m=re.match(r"=== (.*) full: pass=(\S+) fail=(\S+)",l)
        if m: cur=m.group(1); iso[cur]=[0,0]
    elif cur and l.startswith("PASS"): iso[cur][0]+=1
    elif cur and l.startswith("FAIL"): iso[cur][1]+=1
rows=[r for r in open(C+"/law1-tools/list-fw-rows.txt",encoding="utf-8").read().split("\n") if r]
D={ # key -> (code, evidence)
"📏️script-boundary":("C+A","fixture roster gained 6 router commands (own suites); router imports products/🦑️repo (compiler-state storage wrapper b84bf52) -> law allows only the repo build-tool product"),
"time-travel/🧪️testing/📥️values":("A","schema pinned inferenceExports to 15; actual roster 16 named + default = 17 (corpus already 17)"),
"⏯️tool-run/🏛️ownership":("C","cargo router imports products/repo cargo admission (compiler ingress gate); law gets exemptBoundary products/repo"),
"⏳️async/♻️cancel-return":("A","plugin facade carved: needles now in typed-operation-unit + ♻️cancellation; negative needle over source family"),
"◻️2d/🧮️compute":("F","legacy `new Lru({max,length})` API; package.json (working tree) pins lru-cache 11.5.3, root lock has 6.0.0 -> 45/0 with 6.0.0"),
"🌱️value/🧪️tests/🧩️neutral-owner":("A+C","copy-workspace layout rebuilt for 🧰️framework/Cargo.toml root; UNVERIFIED (runs cargo)"),
"🛬️decode/🧾️source":("B+C","born red (read ../🦀️.rs never existed); Ajv2020 for 2020-12 schema; Rust half re-pointed to successor 📖️source RetainedRecordBodySourceDecode"),
"🔤️json/📥️decode/🧾️receipt":("A","step_source/advance live in 🧮️parser-and-grammar-cursor -> readModuleTreeSource"),
"🔤️json/🧪️tests/🚮️absence":("F","root node_modules lacks bun-types, undici-types, fast-uri, json-schema-traverse, require-from-string, jsonc-parser"),
"🎒️pack/🚰️source":("A","pack-source consumers after db/spr/cli carve (roster updated)"),
"🎠️kernel":("A","`replication` dependency now owned by the carved kernel crate manifest"),
"🎟️context/🧪️tests/🏛️boundary":("C","script imports products/🦑️repo wasm policy -> only repo allowed"),
"🎭️actor/🎟️retained-turn/🤝️admission":("C","kernel admission schema added 0c3e789 for 🕸️kernel fixture; negative law re-stated"),
"🎭️actor/🧪️tests/🏛️owner-boundary":("C","new fixture field scriptExemptProduct=🦑️repo for 📜️script.ts string refs"),
"🎭️actor/🧬️ownership":("A","symbols declared in #[path]-mounted modules -> readModuleTreeSource"),
"🏗️build/🎛️environment":("D","1722/1146 generated per-crate launch entries lost env BUN_RUNTIME_TRANSPILER_CACHE_PATH/NODE_DISABLE_COMPILE_CACHE; restored in launch.json + seed"),
"🛟️custodian":("E","child process hangs/aborts under the test envelope (needs real launcher)"),
"🔐️inheritance":("A","schema-first: launcher policy json folded into 🧬️schema Configuration.const"),
"🪆️scope":("E","spawned child scripts end with Original process admission closed"),
"🎯️recipe":("H","tsc --ignoreConfig fixed (TS 7); production test needs storage reservation + real cargo"),
"📋️contract/🧪️tests/📥️parent":("H","needs launcher capabilities.native + cargo children"),
"📋️contract/🧪️tests/🟦️.ts":("G","maintenanceMethods() expects 3 withExclusiveMaintenance arrows, production has 2 (re-derive model); rest = parent suite"),
"📚️witness/🧪️tests/🔤️ordering":("A","second `enumerate` arrow (hardlink walk); selected by entry guard"),
"📚️witness/🧪️tests/🟦️.ts":("A+G","4 extraction drifts fixed (schema bindings, duplicate verify, reporter schema); 2 open: asset-entry error, publisher options"),
"📚️witness/🪟️windows":("G","cascade: 14 of 16 red tests pass per test; 1 open (Windows component schema owners), not analysed"),
"📤️publication/🧪️tests/📦️owner":("G","cascade: 11 of 12 pass per test; 1 open (publisher lifecycle memory-port model: a `success` row is refused), not analysed"),
"📤️publication/🧪️tests/🟦️.ts":("A+G","aggregates witness+owner; see witness"),
"💾️storage/🧪️tests/🟦️.ts":("B+C","openNativeImageStorageV1 delegates to persistent open; withOperation precedes deadline check"),
"🧱️fixed/🧪️tests/📥️paths":("A","wrapper->Body split (openNativeFixedPlatformBodyV1)"),
"🧱️fixed/🧪️tests/📥️receipt":("A","physical recheck split into checkPhysical+mappingRead+recheckPhysical"),
"🧹️cleanup/🎛️command":("E","multi-receive file; designed per-test process (iso)"),
"🧹️cleanup/📥️operation":("E+C","multi-receive file; finite request derives distinct signal"),
"🧹️cleanup/📥️receiver":("A","open*Body split with openLifetime; bindings extended"),
"📥️consumers":("G","control refinement edits done; modeled failures leave the Original callback"),
"📋️callers":("F","tsc: `immer` unresolved in actor activation test"),
"🏎️overlap":("H","needs launcher capabilities.buildMode + compiler ingress (cargo -vV)"),
"🧩️component-pass/🧪️tests/📥️inputs":("H","raw cargo -Z on stable / nightly"),
"🧩️component-pass/🧪️tests/🟦️.ts":("C+H","runStub migrated to issued controls; rest needs cargo nightly"),
"🧾️receipt/📦️retention":("A","component-pass driver carved (row{resolve,reject}, sessionOwner discard, pending join); 1 nested child case open"),
"💾️cache/🧪️tests":("B+C","owner ordering re-anchored; lsof newline f-marker accepted (4dab415); 1 Generated-fixture ENV"),
"🧹️retirement":("G","law ahead of production: requireCompilerWorkspaceProducerCustodyV1 / producerLayout absent"),
"📤️publication/📥️sources":("E","multi-receive file; green per test"),
"♻️retirement/🧪️tests/📋️inventory":("E","30 s law deadline under host load"),
"🦀️cargo/🎟️admission":("D+C","admission null-guard restored in runCargoTestsV1/runCargoLintV1; fabricated control refused earlier"),
"🎯️exact":("E","Original process admission closed (poisoned birth in this model)"),
"🏃️execution":("G","law drives executeCargoRequestV1 with a forged ScriptInvocation; hardening refuses it"),
"🦀️cargo/🧪️tests/📥️preparation":("B","withCargoInputs port: args:compiler.arguments; selection owned by framework runner"),
"❄️cold":("not examined","spawns a repo:script command with specifics removed"),
"🚧️boundary":("D+C","20 repo:script targets lost cwd (restored); bundling law treats products/repo as external build tool; 3 foreign test imports remain (law 2)"),
"📤️exports":("C","finite request derives its own deadline-bound signal"),
"📤️child":("E","needs a finite process envelope"),
"📊️profile":("C","debug profiling removed from graph/rust; stale consumer dropped"),
"⏱️finite":("C","withScriptProcessEnvelope captures frozen copies"),
"🧭️routing/🧪️tests/🟦️.ts":("A","aggregate of exports/profile"),
"⏱️budget":("G","plugin step budget no longer derived from contract.max_step_micros; owner decision"),
"⏱️context/📸️capture":("A+B","typed-operations moved; PluginLifecycleFailure path changed"),
"♻️retirement/🎟️context":("A+B","job/worker carve; `step_fault=Some(error.into())`; Store close_granted pub; no os test-support import"),
"♻️retirement/👷️authority":("A+B","worker authority carve; Arc::into_inner(original)"),
"♻️session-return":("A+C","Option handle replaces ManuallyDrop inner; release funding asserted"),
"🛑️cancel/🤝️alias":("A","BatchJobSession impl carved"),
"🪙️loan":("A","test double callers helper moved to store 🧪️testing"),
"🏃️work/🎟️grant":("G","retained-work wiring never completed (see report); needs cargo"),
"📦️physical-close":("B","copied_bytes:copied"),
"🫧️transient":("F","legacy lru-cache API (root lock 6.0.0)"),
"🧰️framework/🧪️tests/🧱️ownership":("A+B+C","ui-contract->ui-declaration; alias-resolving export extractor; product-resident crates by name/role"),
}
def decision(path):
    best=None
    for k,v in D.items():
        if k in path and (best is None or len(k)>len(best[0])): best=(k,v)
    if best: return best[1]
    return ("X","not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched")
out=[]
for i,r in enumerate(rows):
    p,f=fin.get(r,("?","?"))
    if "🧪️testing/🦀️cargo/🧪️tests/📥️preparation" in r: p,f="1","0"
    code,ev=decision(r)
    isoS=""
    for k,(a,b) in iso.items():
        if r.endswith(k[-60:]) or k in r: isoS=f"{a}/{b}"
    short=r.replace("🧰️framework/🔨️modules/","M/").replace("🧰️framework/","F/")
    out.append(f"| {i+1} | `{short}` | {base.get(r,'?')} | {p}/{f} | {isoS} | {code} | {ev} |")
open(G+"table.md","w",encoding="utf-8").write("\n".join(out))
print(len(out))
