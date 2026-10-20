#!/usr/bin/env python3
import json, os, sys, glob
G = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated", "bzl-lazy")
ORDER = ["fg-generate","fg-test","layout-src","writer-test","writer-build","ui-styling","hub-test","hub-typecheck","arch-test","arch-typecheck","arch-build","proctor-test","mb-bericht","mb-demo-build","mb-demo-activate","play-activate"]
rows = []
for n in ORDER:
    try:
        g = json.load(open(f"{G}/gen-{n}.json")); m = json.load(open(f"{G}/m-{n}.json"))
    except Exception as e:
        continue
    ws = f"/private/tmp/bzlazy/ws-{n}"
    build = os.path.getsize(ws + "/BUILD.bazel") if os.path.exists(ws + "/BUILD.bazel") else 0
    plan = set(g["allLabels"].keys()) if "allLabels" in g else set()
    closure = set(m.get("closure", []))
    exp = {l for l, v in g.get("allLabels", {}).items() if v["kind"] in ("task", "test", "launch")}
    eq = (closure == exp) if closure else None
    def a(k): return m[k]
    ac, aw = a("analysis_cold"), a("analysis_warm")
    ec, ew, cl, sl = a("exec_cold"), a("exec_warm"), a("after_clean"), a("after_server_loss")
    rows.append([n, g["tasks"], g["cached"], ac.get("configured"), build, g["implementationMs"], g["generateMs"],
                 ac.get("loadMs"), ac.get("analyzeMs"), ac.get("analysisMs"), ac.get("wallMs"), aw.get("wallMs"),
                 ec.get("actions"), ec.get("executed"), ec.get("wallMs"), ew.get("executed"), ew.get("wallMs"),
                 cl.get("executed"), cl.get("hits"), cl.get("wallMs"), sl.get("executed"), sl.get("hits"), sl.get("wallMs"),
                 m["aquery"].get("actions"), eq, ec.get("uploadSkipped", 0) + cl.get("uploadSkipped", 0)])
H = ["target","closure","cached","configured","BUILD B","impl ms","gen ms","load ms","analyze ms","bep analysis ms","cold wall","warm wall","cold actions","cold exec","cold wall(x)","warm exec","warm wall(x)","clean exec","clean hits","clean wall","srvloss exec","srvloss hits","srvloss wall","aquery actions","closure==cquery","uploadSkipped"]
print("\t".join(H))
for r in rows: print("\t".join(str(x) for x in r))
