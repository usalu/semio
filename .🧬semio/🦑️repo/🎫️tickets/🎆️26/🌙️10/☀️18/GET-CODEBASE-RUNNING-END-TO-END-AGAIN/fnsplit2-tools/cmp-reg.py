"""⚖️ Compares the old and new registration traces of run-reg.sh. usage: cmp-reg.py <tag>"""
import json, sys
C = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/"
a = json.load(open(C + "fnsplit2-reg-%s-old.json" % sys.argv[1])); b = json.load(open(C + "fnsplit2-reg-%s-new.json" % sys.argv[1]))
same = a["trace"] == b["trace"] and a.get("error") == b.get("error")
print("events old=%d new=%d error old=%s new=%s %s" % (a["count"], b["count"], (a.get("error") or "none")[:80], (b.get("error") or "none")[:80], "TRACE IDENTICAL" if same else "TRACE DIFFERENT"))
if not same:
    for i, (x, y) in enumerate(zip(a["trace"], b["trace"])):
        if x != y:
            print(i, x, "|", y)
            break
