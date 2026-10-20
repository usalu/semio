#!/usr/bin/env python3
"""Generates staged/sb-ledger.json (the ratcheted debt ledger) from a TypeScript scan result of the post-apply tree. usage: gen_ledger.py SCAN.json"""
import json,sys,collections,re,os
HERE=os.path.dirname(os.path.abspath(__file__))
scan=json.load(open(sys.argv[1],encoding="utf8"))
groups=collections.OrderedDict()
for x in scan["findings"]:
    if x["severity"]!="error": continue
    groups[(x["file"],x["rule"])]=groups.get((x["file"],x["rule"]),0)+1
R=[
 ("fixed-tmp-dir",r"\.rs$","Rust test names a constant directory below temp_dir(): collision-free across agents only while the runner points TMPDIR at the per-run sandbox; two tests of one binary may still share the name (make it unique per test, then drop this row)"),
 ("fixed-tmp-dir",r"hub-freshness","probes a path that must stay absent; nothing is written"),
 ("fixed-tmp-dir",r"providers/.*tests","builds provider-map strings only; nothing is written"),
 ("fixed-tmp-dir",r"sandbox/.*\.go$","observes a path outside the checkout on purpose; nothing is written"),
 ("fixed-tmp-dir",r"incremental|workspace|mutation-taxonomy","string/path construction under the shared temp name; the SUT reads it (use testScratchRootV1)"),
 ("fixed-tmp-dir",r".*","constant name below the temp directory (use testScratchRootV1)"),
 ("rust-checkout-write",r".*","bless-style generator test rewrites a checked-in fixture next to the crate on every run (gate it behind an explicit bless switch)"),
 ("ticket-scratch-root",r"playpages","vitest lane: scratch lives in the generated folder of a foreign ticket; vite cannot load the sandbox module (JSON import attributes) until vitest gets its own scratch helper"),
 ("ticket-scratch-root",r"contribution","symlink-boundary proof must run below the workspace root; needs a sandbox workspace root"),
 ("ticket-scratch-root",r".*","scratch must lie below the workspace (inputs are workspace-relative); needs a sandbox workspace root, then testScratchRootV1"),
 ("persistent-native-state",r"receiver","refusal proof: copied and expired controls must be refused before any physical intent; its scratch area is the sandbox since this change"),
 ("persistent-native-state",r"local-hub","dev local-hub session callback proof under S_LOCAL_ONLY; no partition birth is requested by the test"),
 ("fixed-port-bind",r".*","asserts the declared `--port` argument of a target command; nothing binds"),
 ("checkout-write",r"reporting","publishes build reports into the real diagnostics partition under unique random names and removes them; needs a sandbox workspace root with its own partition"),
 ("checkout-write",r"observation","passes process.cwd() as workspaceRoot to a read-only project inference inside a child script text"),
 ("checkout-write",r"build-producer","child script text writes next to its own temp directory"),
 ("go-unsandboxed-writer",r"mcp","adapter builds the repo binary into the shared cache `🗃️bin` of the real checkout (concurrent builds race); needs a per-run output directory in the build script"),
 ("go-unsandboxed-writer",r"hostile-source","lists forbidden call names as strings; nothing is written"),
 ("go-unsandboxed-writer",r".*","Go unit writes a helper script below a path derived by the test; use t.TempDir()"),
]
entries=[]
for (f,r),n in groups.items():
    reason=next(why for rule,pat,why in R if rule==r and re.search(pat,f))
    entries.append({"file":f,"rule":r,"count":n,"reason":reason})
entries.sort(key=lambda e:(e["rule"],e["file"]))
json.dump({"version":1,"entries":entries},open(HERE+"/staged/sb-ledger.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
open(HERE+"/staged/sb-ledger.json","a").write("\n")
print(len(entries),"ledger entries")
