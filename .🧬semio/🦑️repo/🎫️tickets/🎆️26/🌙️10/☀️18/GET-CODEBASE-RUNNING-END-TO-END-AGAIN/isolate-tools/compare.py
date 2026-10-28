"""⚖️ [isolate] compares the failure sets of a batch: base ledger vs new ledger (sweep-vb-<batch>-base/new.jsonl); a file REGRESSES when the new run fails a test the base did not fail (or fails more with unnamed failures), and is LEAKY when the new run still hits the live store fence. usage: compare.py <batch> [ledger prefix, default vb]"""
import json, os, sys
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__))); G = os.path.join(C, "🗑️generated", "isolate"); b = sys.argv[1]
P = sys.argv[2] if len(sys.argv) > 2 else "vb"
load = lambda t: {json.loads(l)["file"]: json.loads(l) for l in open(os.path.join(G, "sweep-%s-%s-%s.jsonl" % (P, b, t)), encoding="utf-8")}
base, new = load("base"), load("new"); bad = 0
for f in [l.strip() for l in open(os.path.join(C, "isolate-tools", "batches", b + ".txt"), encoding="utf-8") if l.strip()]:
    x, y = base.get(f), new.get(f)
    if not x or not y: print("MISSING", f[-90:]); bad += 1; continue
    extra = sorted(set(y["fails"]) - set(x["fails"]))
    verdict = "REGRESSION" if extra or y["pass"] < x["pass"] or y["fail"] > x["fail"] and not y["fails"] else "LEAKY" if y.get("refusals") else "BETTER" if y["fail"] < x["fail"] or y["pass"] > x["pass"] else "SAME"
    bad += verdict in ("REGRESSION", "LEAKY")
    print("%-10s base %3d/%-3d new %3d/%-3d %s%s" % (verdict, x["pass"], x["fail"], y["pass"], y["fail"], f.split("🔨️modules/")[-1][-80:], (" new fails: " + "; ".join(extra)[:300]) if extra else ""))
print("batch", b, "bad", bad); sys.exit(1 if bad else 0)
