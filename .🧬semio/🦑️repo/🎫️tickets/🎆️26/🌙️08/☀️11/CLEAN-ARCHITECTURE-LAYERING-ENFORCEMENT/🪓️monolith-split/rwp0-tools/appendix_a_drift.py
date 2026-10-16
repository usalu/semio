# 📐️ Compares Appendix A of PLAN-RENDERER-WGPU-CRATES.md (file, lines, bytes) with a live closure listing (closure.ts LIST=...).
import json, os, re, sys, unicodedata
R = "/Users/ueli/Documents/semio"
E = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
PLAN = os.path.dirname(os.path.abspath(__file__)) + "/../PLAN-RENDERER-WGPU-CRATES.md"
def norm(s): return s.replace("️", "")
def candidates(cell):
    c = cell.strip().strip("`")
    T = E + "/🎯️targets/🧊️wgpu"
    out = []
    if c.startswith("T/"): out.append(T + "/" + c[2:])
    elif c.startswith("E/"): out.append(E + "/" + c[2:])
    elif c.startswith("🧰️framework"): out.append(c)
    else:
        parts = c.split("/")
        el = parts[0]
        rest = parts[1:]
        if rest and rest[0] == "T": rest = rest[1:]; out.append(E + "/🧱️elements/" + el + "/🎯️targets/🧊️wgpu/" + "/".join(rest))
        out.append(E + "/🧱️elements/" + el + "/🎯️targets/🧊️wgpu/" + "/".join(rest))
        out.append(E + "/🧱️elements/" + el + "/" + "/".join(rest))
    return out
def main():
    rows = []
    active = False
    for line in open(PLAN, encoding="utf-8"):
        if line.startswith("## Appendix A"): active = True
        elif line.startswith("## Appendix B"): active = False
        if not active: continue
        m = re.match(r"^\| (\d+) \| `([^`]+)` \| (\d+) \| (\d+) \| (.*?) \| (\d+) \| (.*?) \|$", line)
        if m:
            rows.append((int(m.group(1)), m.group(2), int(m.group(3)), int(m.group(4)), m.group(5), int(m.group(6)), m.group(7)))
    print("appendix rows", len(rows))
    live = json.load(open(sys.argv[1]))
    files = {}
    for label, items in live.items():
        if label.startswith("lib"):
            for i in items:
                if i["kind"] == "source": files.setdefault(i["path"], {})[label] = (i["lines"], i["bytes"])
    byn = {norm(unicodedata.normalize("NFC", p)): p for p in files}
    seen = set(); out = []
    for n, cell, lines, nbytes, gate, tests, target in rows:
        resolved = None
        for cand in candidates(cell):
            for suffix in ("/🦀️.rs", ""):
                p = os.path.normpath(cand + suffix)
                if os.path.isfile(R + "/" + p) and p.endswith(".rs"): resolved = p; break
            if resolved: break
        if not resolved: out.append((n, cell, "UNRESOLVED")); continue
        key = byn.get(norm(unicodedata.normalize("NFC", resolved)))
        seen.add(key)
        cur = files.get(key, {})
        nat = cur.get("lib native", (0, 0)); wa = cur.get("lib wasm32-unknown-unknown", (0, 0))
        best = nat if nat != (0, 0) else wa
        if best != (lines, nbytes): out.append((n, cell, "plan", (lines, nbytes), "live", best))
    for k in files:
        if k not in seen: out.append((None, k, "NEW IN CLOSURE", files[k]))
    for o in out: print(o)

if __name__ == '__main__': main()
