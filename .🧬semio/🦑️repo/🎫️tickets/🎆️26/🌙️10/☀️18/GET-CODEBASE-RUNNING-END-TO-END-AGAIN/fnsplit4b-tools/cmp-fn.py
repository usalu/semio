"""⚖️ Compares the old and new outcomes of run-fn.sh. usage: cmp-fn.py <tag>"""
import json, sys
C = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/"
a = json.load(open(C + "fnsplit4b-fn-%s-old.json" % sys.argv[1])); b = json.load(open(C + "fnsplit4b-fn-%s-new.json" % sys.argv[1]))
same = a["outcome"] == b["outcome"] and a["logs"] == b["logs"]
print("old=%s | new=%s logs old=%d new=%d %s" % (a["outcome"][:80], b["outcome"][:80], len(a["logs"]), len(b["logs"]), "IDENTICAL" if same else "DIFFERENT"))
