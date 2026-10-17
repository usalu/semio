"""🔒 Compact privacy diagnostics (E0603/E0616/E0624/E0451) from <fam>-<label>-cur-<cfg>-priv.err
usage: priv.py <family> <label> <cfg> [max]"""
import json, re, sys, collections
fam, label, cfg = sys.argv[1:4]
mx = int(sys.argv[4]) if len(sys.argv) > 4 else 40
g = collections.OrderedDict()
for line in open(f"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/ohp-{fam}-{label}-cur-{cfg}-priv.err", encoding="utf-8", errors="replace"):
    if not line.startswith("{"):
        continue
    try:
        d = json.loads(line)
    except Exception:
        continue
    pr = [s for s in d["spans"] if s["is_primary"]]
    if not pr or "__pos_root" in pr[0]["file_name"]:
        continue
    msg = re.sub(r"__m_\w+", "M", d["message"])[:130]
    f = re.sub(r"^__m_\w+/", "", pr[0]["file_name"]).split("artifacts/")[-1]
    f = "/".join(f.split("/")[-4:])
    txt = (pr[0]["text"][0]["text"].strip() if pr[0]["text"] else "")[:100]
    g.setdefault((d["code"]["code"], msg), []).append((f, pr[0]["line_start"], txt))
for (c, m), v in list(g.items())[:mx]:
    print(f"{c} x{len(v)} {m} | {v[0][0][-45:]}:{v[0][1]} | {v[0][2]}")
