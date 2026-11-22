import json, re, os, collections
G = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122/🗑️generated/E9"
def load(n):
    r = {}
    if os.path.exists(G + "/" + n):
        for l in open(G + "/" + n): x = json.loads(l); r[x["file"]] = x
    return r
r1, r2, r3 = load("results.jsonl"), load("results2.jsonl"), load("results3.jsonl")
nontests = set(json.load(open(G + "/nontests.json")))
FIX = json.load(open(G + "/fixes.json"))
errs = {}
for n in ("logs3.txt",):
    for b in open(G + "/" + n, encoding="utf8", errors="replace").read().split("\n=== "):
        h = b.split("\n")[0].replace("=== ", ""); f = h.split(" rc=")[0]
        m = re.findall(r"^(?:error|\w*Error)[^\n]{0,140}", b, re.M)
        errs[f] = (m[0] if m else "see log")
def short(f): return f.split("library/")[-1]
def klass(e):
    if "Tool registry invalid" in e: return "BLOCKED in-flight: tool registry schema change (rootVariables), tools lane"
    if "hostUnderPressureV1" in e: return "BLOCKED in-flight: host-capacity decision export removed, host-capacity lane"
    if re.search(r"Original .*(capabilit|SDK|process birth|artifact|repository|snapshot|Script|installed tooling|bounded|inventory|collection|incoming)|capabilities-differ|SEMIO_TEST_PYTHON|Exact historical|ticket|live repository store", e): return "needs suite launcher / declared grant (run through repo:test-<suite>)"
    return "assertion or runtime failure, not yet analysed (see logs3)"
rows = []; c = collections.Counter(); k2 = collections.Counter()
for f, x in sorted(r1.items()):
    b = "pass" if x["rc"] == 0 else "fail"
    if f in nontests: c["helper"] += 1; rows.append((short(f), "helper", "helper", "not a test file", "should leave the test manifest")); continue
    fix = next((v for k, v in FIX.items() if k in f), "")
    a2 = r2.get(f); a3 = r3.get(f)
    s2 = "-" if a2 is None else ("pass" if a2["rc"] == 0 else "fail")
    s3 = "-" if a3 is None else ("pass" if a3["rc"] == 0 else "fail")
    final = "pass" if b == "pass" else ("pass" if s3 == "pass" else ("pass-earlier" if s2 == "pass" and fix == "" else "FAIL"))
    c[(b, final)] += 1
    if b == "pass": continue
    e = errs.get(f, "") if s3 == "fail" else ""
    cause = "needs the Original process envelope (raw bun test)" if final.startswith("pass") and not fix else (klass(e) + ": " + e[:90] if e else "")
    if final == "FAIL": k2[klass(e)] += 1
    rows.append((short(f), b, "env2=%s env3=%s" % (s2, s3), cause, fix))
out = ["| file | before (raw) | after (envelope runs 02:00 / 03:00) | cause | fix |", "|---|---|---|---|---|"]
for r in rows: out.append("| " + " | ".join(str(x).replace("|", "/") for x in r) + " |")
open(G + "/table.md", "w").write("\n".join(out) + "\n")
json.dump({"counts": {str(k): v for k, v in c.items()}, "fail_classes": k2}, open(G + "/summary.json", "w"), ensure_ascii=False, indent=1)
print(dict(c)); print(dict(k2))
