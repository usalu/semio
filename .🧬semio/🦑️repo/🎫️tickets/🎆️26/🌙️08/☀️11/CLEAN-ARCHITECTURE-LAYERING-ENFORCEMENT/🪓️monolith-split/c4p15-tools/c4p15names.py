"""🏷️ c4p15 folder-name checks: `<emoji>U+FE0F<kebab>`, the emoji is RGI (node `\\p{RGI_Emoji}`) and already a folder prefix in the repo, unique among the siblings, equal to the docstring emoji, path <= 240 bytes. Writes the directory list for the taxonomy check."""
import json, os, re, subprocess, sys, unicodedata
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4p15lib import *
from plans15 import PLANS, ORDER

repo_emojis = json.load(open(WORK + "/folder-emojis.json", encoding="utf-8"))
names = [c["folder"] for k in ORDER for c in PLANS[k]["children"]]
emo = sorted({n.split("\ufe0f")[0] for n in names})
js = "const e=" + json.dumps(emo, ensure_ascii=False) + ";for(const x of e){console.log(x+'\\t'+(/^\\p{RGI_Emoji}$/v.test(x+'\\ufe0f')||/^\\p{RGI_Emoji}$/v.test(x)))}"
rgi = dict(l.split("\t") for l in subprocess.run(["node", "-e", js], capture_output=True, text=True).stdout.strip().split("\n"))
ok = True
dirs = []
for k in ORDER:
    spec = PLANS[k]
    base_dir = os.path.dirname(spec["rel"])
    seen = {}
    for c in spec["children"]:
        f = c["folder"]
        m = re.match(r"^(\S+?)\ufe0f([a-z0-9][a-z0-9-]*)$", f)
        bad = []
        if not m:
            bad.append("format")
        else:
            e = m.group(1)
            if rgi.get(e) != "true":
                bad.append("not RGI")
            if e + "\ufe0f" not in repo_emojis:
                bad.append("not a repo folder prefix")
            if not c["doc"].startswith(e + "\ufe0f "):
                bad.append("doc emoji differs")
            if e in seen:
                bad.append("repeats " + seen[e])
            seen[e] = f
        live = ROOT + "/" + base_dir
        if os.path.isdir(live):
            for sib in os.listdir(live):
                if sib.split("\ufe0f")[0] == f.split("\ufe0f")[0] and sib != f:
                    bad.append("sibling emoji " + sib)
        path = f"{base_dir}/{f}/🦀️.rs"
        if len(path.encode("utf-8")) > 240:
            bad.append("path>240")
        dirs.append(f"{base_dir}/{f}")
        if bad:
            ok = False
            print(k, f, bad)
print("NAMES", "OK" if ok else "FAILED", len(dirs), "dirs; longest path", max(len((d + "/🦀️.rs").encode()) for d in dirs))
open(WORK + "/newdirs.txt", "w", encoding="utf-8").write("\n".join(dirs) + "\n")
