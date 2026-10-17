"""🔍 Show compact oracle diagnostics matching a message regex.
usage: ex.py <family> <cfg> <label> <message-regex> [max] [code]"""
import json, re, sys
fam, cfg, label, rx = sys.argv[1:5]
mx = int(sys.argv[5]) if len(sys.argv) > 5 else 2
code = sys.argv[6] if len(sys.argv) > 6 else None
c = 0
for line in open(f"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/ohp-{fam}-{label}-cur-{cfg}-final.err", encoding="utf-8", errors="replace"):
    if not line.startswith("{"):
        continue
    try:
        d = json.loads(line)
    except Exception:
        continue
    if d.get("level") != "error" or not re.search(rx, d["message"]):
        continue
    if code and (d.get("code") or {}).get("code") != code:
        continue
    p = [s for s in d["spans"] if s["is_primary"]]
    if not p:
        continue
    p = p[0]
    print(d["message"][:150])
    print("  ", re.sub(r"^__m_[A-Za-z0-9_]+/", "", p["file_name"]).split("artifacts/")[-1][-110:], p["line_start"])
    print("  >", (p["text"][0]["text"].strip() if p["text"] else "")[:220])
    for ch in d.get("children", [])[:3]:
        print("   ~", ch["message"][:150])
        for s in ch.get("spans", [])[:1]:
            print("     @", s["file_name"].split("artifacts/")[-1][-90:], s["line_start"], (s["text"][0]["text"].strip()[:110] if s["text"] else ""))
    c += 1
    if c >= mx:
        break
