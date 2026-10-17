"""🧮 OHF family writer: groups the framework, hub and teaching crates by directory and writes pos family specs (no baseline) into families/. Read only except for those files.
usage: mkfam.py list | mkfam.py write"""
import json, os, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.dirname(HERE) + "/🗑️generated"
idx = json.load(open(GEN + "/ohf-manifest-index.json", encoding="utf-8"))
FW = "🧰️framework/"
GROUPS = collections.OrderedDict()


def add(name, pred):
    GROUPS[name] = sorted(n for n, p in idx.items() if pred(p))


add("fw-modules", lambda p: p.startswith(FW + "🔨️modules/") or p.startswith(FW + "📦️packages/"))
add("fw-os", lambda p: p.startswith(FW + "🛍️products/💻️os/"))
add("fw-products", lambda p: p.startswith(FW + "🛍️products/") and not p.startswith(FW + "🛍️products/💻️os/"))
add("hub-core", lambda p: p.startswith("🌎️hub/") and "🧩️compositions" not in p)
add("hub-comp", lambda p: p.startswith("🌎️hub/🧩️compositions"))
add("teaching", lambda p: p.startswith("🎓️teaching/"))
if __name__ == "__main__":
    if sys.argv[1] == "list":
        for k, v in GROUPS.items():
            print(k, len(v))
    else:
        os.makedirs(HERE + "/families", exist_ok=True)
        for k, v in GROUPS.items():
            json.dump(dict(name=k, crates=v, note="OHF group, no baseline"), open(f"{HERE}/families/{k}.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
