"""🧾️ Seeds the split norm components' descriptors from the former whole-norm descriptor until each component build regenerates its own."""
import json, re, sys
from pathlib import Path
N = Path(sys.argv[1]) / "🌎️hub/🧩️compositions/📕️norm"
OWNERS = {"norm": (N / "🧩️component", "Norm", ["din4108", "din16798", "din18599", "iso16757", "vdi3805"]),
          "norm-eurocode": (N / "🧩️extensions/🏛️eurocode/🧩️component", "Norm Eurocode", ["en1990", "en1991", "en1997", "en1998"]),
          "norm-eurocode-materials": (N / "🧩️extensions/🧱️eurocode-materials/🧩️component", "Norm Eurocode Materials", ["en1992", "en1993", "en1994", "en1995", "en1996", "en1999"])}
whole = json.loads((N / "🧩️component/🔣️.json").read_text())
def fam(text):
    found = re.search(r"norm\.(\w+)", json.dumps(text, ensure_ascii=False))
    return found.group(1) if found else None
for owner, (path, label, families) in OWNERS.items():
    d = json.loads(json.dumps(whole))
    d["packageId"] = f"semio:{owner}"
    m = d["manifest"]; m["pluginId"] = owner; m["label"] = label
    m["apps"] = [a for a in m["apps"] if fam(a["id"]) in families]
    m["examples"] = [e for e in m["examples"] if fam(e["dialect"]) in families]
    d["activationEvents"] = [e for e in d["activationEvents"] if fam(e) in families]
    d["contributions"] = {k: [x for x in v if fam(x) in families or fam(x) is None] for k, v in d["contributions"].items()}
    (path / "🔣️.json").write_text(json.dumps(d, ensure_ascii=False, indent=2) + "\n")
    print(owner, len(m["apps"]), len(m["examples"]), {k: len(v) for k, v in d["contributions"].items()})
