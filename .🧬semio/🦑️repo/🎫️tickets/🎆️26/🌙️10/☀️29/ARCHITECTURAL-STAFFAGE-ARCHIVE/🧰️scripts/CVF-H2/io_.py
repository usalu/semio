import json, os, math, re
S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"
T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
JOBS = f"{T}/🗑️generated/CVF-H2/jobs"
HERE = os.path.dirname(os.path.abspath(__file__))
IDS = ["worker-carrying-box", "parent-with-stroller", "jogger", "person-with-umbrella", "leaning-person", "tour-guide"]
def path(a): return f"{S}/🗂️catalog/👤️humans/{a}/🔣️.json"
def load(a): return json.load(open(path(a)))
def save(a, doc):
    with open(path(a), "w") as f:
        f.write(json.dumps(doc, indent=2, ensure_ascii=False) + "\n")
def T2(en, de): return {"en": en, "de": de}
