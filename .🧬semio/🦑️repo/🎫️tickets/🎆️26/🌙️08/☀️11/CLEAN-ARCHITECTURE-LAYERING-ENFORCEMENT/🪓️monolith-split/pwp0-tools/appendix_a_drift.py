# 📐️ Compares Appendix A of PLAN-PLUGIN-CRATES.md (file, lines, KB) with a live closure listing (closure.ts LIST=...). usage: appendix_a_drift.py closure.json
import json, os, re, sys, unicodedata
HERE = os.path.dirname(os.path.abspath(__file__))
PLAN = HERE + "/../PLAN-PLUGIN-CRATES.md"
PLUG = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/"
def norm(s): return unicodedata.normalize("NFC", s).replace("️", "")
def rows():
    out = []; active = False
    for line in open(PLAN, encoding="utf-8"):
        if line.startswith("## Appendix A"): active = True
        elif line.startswith("## Appendix B") or line.startswith("## Appendix C") or line.startswith("## Appendix D"): active = False
        if not active: continue
        m = re.match(r"^\| `([^`]+)` \| (\d+) \| (\d+) \| (.*?) \|$", line)
        if m: out.append((m.group(1), int(m.group(2)), int(m.group(3)), m.group(4)))
    return out
def main():
    live = json.load(open(sys.argv[1]))
    union = {}
    for label in ("lib native", "lib wasm32-wasip2", "lib wasm32-unknown-unknown", "lib native canvas"):
        for i in live.get(label, []):
            if i["kind"] == "source": union.setdefault(norm(i["path"]), (i["lines"], i["bytes"], label))
    seen = set(); drift = []; missing = []
    R = rows()
    print("appendix rows", len(R), "live union files", len(union))
    for cell, lines, kb, crate in R:
        key = norm(PLUG + cell)
        if key not in union: missing.append((cell, crate)); continue
        seen.add(key)
        l, b, _ = union[key]
        if l != lines or b // 1024 != kb: drift.append((cell, "plan", lines, kb, "live", l, b // 1024, b - kb * 1024, crate))
    new = [(k, union[k][:2]) for k in union if k not in seen]
    print("DRIFT", len(drift))
    for d in drift: print(" ", d)
    print("MISSING (plan row, not in closure)", len(missing))
    for m in missing: print(" ", m)
    print("NEW IN CLOSURE", len(new))
    for n in new: print(" ", n)
if __name__ == "__main__": main()
