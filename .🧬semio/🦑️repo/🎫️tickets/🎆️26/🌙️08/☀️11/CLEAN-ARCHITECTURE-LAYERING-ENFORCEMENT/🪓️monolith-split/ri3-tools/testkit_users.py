"""🧪 RI3 scan: items gated by a `testkit`/`test` cfg in crate D that are named by another crate C, with C's enclosing cfg and whether C's `testkit` closure reaches D's `testkit`."""
import os, re, sys, collections, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import testkit_forward as TF
from owners import *

GATE = re.compile(r'#\[cfg\((.*)\)\]\s*$')
ITEM = re.compile(r'^\s*(?:pub(?:\([^)]*\))?\s+)(?:unsafe\s+|async\s+|const\s+)*(?:fn|struct|enum|const|static|type|trait)\s+([A-Za-z_]\w*)')


def main():
    tbl, cf, idx = build()
    man = {s: TF.load(f) for s, f in tbl.items()}
    for s, m in man.items():
        if "testkit" in m.get("features", {}):
            m["features"]["testkit"] = re.findall(r'"([^"]+)"', m["features"]["testkit"])
    PFX = TF.PFX

    def tk_closure(s):
        seen, st = set(), [s]
        while st:
            x = st.pop()
            if x in seen or x not in man:
                continue
            seen.add(x)
            for e in man[x].get("features", {}).get("testkit", []):
                e2 = e[4:] if e.startswith("dep:") else e
                if "/" in e2:
                    d, feat = e2.split("/", 1)
                    d = d[len(PFX):] if d.startswith(PFX) else d
                    if feat == "testkit":
                        st.append(d)
        return seen

    gated = collections.defaultdict(set)
    texts = {}
    for suffix, files in cf.items():
        for f in files:
            try:
                raw = read(f)
            except Exception:
                continue
            texts[(suffix, f)] = raw
            lines = raw.split("\n")
            for i, ln in enumerate(lines):
                m = GATE.search(ln.strip()) if ln.strip().startswith("#[cfg(") else None
                if not m or "testkit" not in m.group(1) or "not(" == m.group(1)[:4]:
                    continue
                if re.search(r'not\(any\(test, feature = "testkit"\)\)', m.group(1)):
                    continue
                j = i + 1
                while j < len(lines) and (lines[j].strip().startswith("#[") or not lines[j].strip()):
                    j += 1
                if j < len(lines):
                    im = ITEM.match(lines[j])
                    if im:
                        gated[im.group(1)].add(suffix)
    rows = collections.defaultdict(set)
    for (suffix, f), raw in texts.items():
        clean = blank(raw)
        lines = raw.split("\n")
        for name, owners_ in gated.items():
            if suffix in owners_ or len(name) < 6:
                continue
            if name not in clean:
                continue
            for m in re.finditer(r'(?<![\w$])' + re.escape(name) + r'(?!\w)', clean):
                ln = clean.count("\n", 0, m.start())
                k = ln
                gate = ""
                while k >= max(0, ln - 6):
                    s = lines[k].strip()
                    gm = GATE.search(s) if s.startswith("#[cfg(") else None
                    if gm:
                        gate = gm.group(1)
                        break
                    k -= 1
                for d in owners_:
                    if d == suffix:
                        continue
                    reach = d in tk_closure(suffix) if suffix in man and "testkit" in man[suffix].get("features", {}) else False
                    rows[(suffix, d)].add((name, "gated" if "testkit" in gate or "test" in gate else "ungated:" + gate[:40], reach))
    print("gated names", len(gated), "pairs", len(rows))
    for (c, d), items in sorted(rows.items()):
        bad = [x for x in items if not x[2]]
        if bad:
            print(f"{c} -> {d}: {len(items)} names, unforwarded: {sorted(bad)[:6]}")


main()
