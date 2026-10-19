"""🔬 RMOD error-multiset differential: compares the rustc JSON diagnostics of a pre and a post oracle run, keyed by (code, normalised message, normalised label, crate module). The renamed module identifiers and crate names are folded to one placeholder on both sides, so only a new, lost or changed error shows. usage: rdiff.py <pre.err> <post.err>"""
import collections, json, re, sys, os

HERE = os.path.dirname(os.path.abspath(__file__))
CM = json.load(open(HERE + "/crate-map.json"))
MODS = ["base", "core", "shared", "helpers", "common", "operator_registration", "mesh_operator_io", "part_libraries", "check_evidence", "editor_vocabulary", "sweep_machinery", "intersection_numerics", "issuance_clone", "immutable_alias", "arc_custody"]
CRATES = set()
for o, n in CM["crates"].items():
    CRATES.add(o.replace("-", "_")); CRATES.add(n.replace("-", "_"))
for o, n in CM["libNames"].items():
    CRATES.add(o); CRATES.add(n)
RX_MOD = re.compile(r"(?<![A-Za-z0-9_])(" + "|".join(MODS) + r")(?![A-Za-z0-9_])")
RX_CRATE = re.compile(r"(?<![A-Za-z0-9_])(" + "|".join(sorted(CRATES, key=len, reverse=True)) + r")(?![A-Za-z0-9_])")
PATHRX = re.compile(r"\b(?:_*[a-z][a-z0-9_]*::)+(?=[A-Za-z_{(<])")


def nrm(s):
    s = PATHRX.sub("", s)
    s = RX_CRATE.sub("@C", s)
    s = RX_MOD.sub("@M", s)
    s = re.sub(r"\bTr\d+\b", "Tr", s)
    s = PATHRX.sub("", s)
    s = re.sub(r"\{(closure|async block|async fn body)@[^}]*\}", r"{\1}", s)
    return s


def load(path):
    keys = collections.Counter()
    where = {}
    for line in open(path, encoding="utf-8", errors="replace"):
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("level") not in ("error", "error: internal compiler error"):
            continue
        msg = d.get("message", "")
        if msg.startswith("aborting due to") or msg.startswith("could not compile") or "previous error" in msg:
            continue
        code = (d.get("code") or {}).get("code") or ""
        prim = next((s for s in d.get("spans", []) if s.get("is_primary")), (d.get("spans") or [None])[0])
        fn, label, ln = "", "", 0
        if prim:
            fn = prim.get("file_name", "")
            label = prim.get("label") or ""
            ln = prim.get("line_start", 0)
        if fn.endswith("__pos_root.rs"):
            continue
        m = re.match(r"^(__m_[A-Za-z0-9_]+)/(.*)$", fn)
        crate = nrm(m.group(1)) if m else ""
        rel = m.group(2) if m else fn
        key = (code, nrm(msg), nrm(label), crate, rel)
        keys[key] += 1
        where.setdefault(key, []).append(ln)
    return keys, where


def main():
    a, wa = load(sys.argv[1])
    b, wb = load(sys.argv[2])
    lost = a - b
    new = b - a
    print(f"pre {sum(a.values())} errors, post {sum(b.values())} errors; lost {sum(lost.values())}, new {sum(new.values())}")
    for k, n in sorted(new.items(), key=lambda x: str(x[0])):
        print(f"  NEW  x{n} {k[0]} [{k[3]}] {k[4][-70:]} lines {wb[k][:4]}: {k[1][:150]}")
    for k, n in sorted(lost.items(), key=lambda x: str(x[0])):
        print(f"  LOST x{n} {k[0]} [{k[3]}] {k[4][-70:]} lines {wa[k][:4]}: {k[1][:150]}")
    return 0 if not new and not lost else 1


sys.exit(main())
