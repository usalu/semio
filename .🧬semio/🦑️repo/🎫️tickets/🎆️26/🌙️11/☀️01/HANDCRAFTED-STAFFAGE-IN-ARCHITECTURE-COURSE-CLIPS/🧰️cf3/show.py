import json, sys
sys.stdout.reconfigure(encoding="utf-8")
d = json.load(open("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🧰️cf3/why.json", encoding="utf-8"))
for p, w in d["why"].items():
    if not w.startswith(("v2 report", "staffage archive", "visuals module")):
        print(w, "|", p.replace("🎓️teaching/🏛️architecture/", ""))
print("---")
for e in d["excluded"]:
    print("EXCL", e)
