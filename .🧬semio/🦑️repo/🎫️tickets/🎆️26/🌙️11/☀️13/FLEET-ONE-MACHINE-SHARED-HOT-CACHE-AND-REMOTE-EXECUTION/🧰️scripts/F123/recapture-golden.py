#!/usr/bin/env python3
"""F123: fold a SEMIO_CAPTURE_TRANSACTION_BOUNDARIES capture dir into the transaction ledger boundaries golden for the keys whose digests differ, atomically; drops ledgers no boundary references any more. Usage: recapture-golden.py <golden json> <capture dir>"""
import glob, json, os, sys
P = sys.argv[1]
g = json.load(open(P, encoding="utf8"))
sortedL = list(g["transactionLedgers"]) == sorted(g["transactionLedgers"])
changed = []
for f in glob.glob(os.path.join(sys.argv[2], "*.json")):
    c = json.load(open(f, encoding="utf8")); k = c["key"]
    new = {"transaction": c["transaction"]["digest"], "workspace": c["workspace"]["digest"]}
    if g["boundaries"].get(k) == new: continue
    g["boundaries"][k] = new; changed.append(k)
    g["transactionLedgers"][new["transaction"]] = c["transaction"]["nodes"]
    g["workspaceLedgers"][new["workspace"]] = c["workspace"]["nodes"]
for side, ledgers in (("transaction", "transactionLedgers"), ("workspace", "workspaceLedgers")):
    used = {b[side] for b in g["boundaries"].values()}
    g[ledgers] = {d: n for d, n in g[ledgers].items() if d in used}
    if sortedL: g[ledgers] = dict(sorted(g[ledgers].items()))
t = P + ".tmp123"
open(t, "w", encoding="utf8").write(json.dumps(g, indent=2, ensure_ascii=False) + "\n")
os.replace(t, P)
print(sorted(changed))
