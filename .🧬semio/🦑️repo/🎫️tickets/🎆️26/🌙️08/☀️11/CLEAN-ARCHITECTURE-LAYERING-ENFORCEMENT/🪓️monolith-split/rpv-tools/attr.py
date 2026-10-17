"""📎 RPV attribution: reads the privacy diagnostics (`*-priv.err`) of one oracle run of the `ri2` family, resolves each to its owner crate, member definition line and user crate, and writes an aggregate JSON/TSV under GEN.
Read-only on the repo.
usage: attr.py <tag> <lib-priv.err> <test-priv.err>        (POS_PREFIX=rpv)"""
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../pos-tools")
import posunion as U
from poslib import *
from posstub import mask

PRIV = {"E0603", "E0616", "E0624", "E0451"}
FN_RX = re.compile(r'^(\s*(?:#\[[^\]]*\]\s*)*)(pub\s*\([^)]*\)\s+|pub\s+)?((?:const\s+|async\s+|unsafe\s+|extern\s+"[^"]*"\s+)*(?:fn|const|static|type|struct|enum|trait|mod|use)\b)')
FIELD_RX = re.compile(r"^(\s*(?:#\[[^\]]*\]\s*)*)(pub\s*\([^)]*\)\s+|pub\s+)?(\w+)\s*:")


def repo_of(fn):
    m = re.match(r"^(__m_[A-Za-z0-9_]+)/(.*)$", fn)
    if not m:
        return None, None
    return m.group(1)[4:], os.path.normpath(m.group(2))


def load(paths):
    out = []
    for cfg, p in paths:
        for l in open(p, encoding="utf-8"):
            l = l.strip()
            if not l.startswith("{"):
                continue
            try:
                d = json.loads(l)
            except Exception:
                continue
            c = (d.get("code") or {}).get("code")
            if d.get("level") == "error" and c in PRIV:
                d["cfg"] = cfg
                out.append(d)
    return out


def main():
    tag, libp, testp = sys.argv[1:4]
    spec = json.load(open(HERE + "/../pos-tools/families/ri2.json", encoding="utf-8"))
    mans = Manifests(Source(None))
    mans.build_index()
    crates = {c.id: c for c in U.prepare(Source(None), mans, spec["crates"], False)}
    byid = dict(crates)
    rows = []
    unresolved = []
    cache = {}

    def lines_of(c, rel):
        k = (c.id, rel)
        if k not in cache:
            cache[k] = c.files[rel].decode("utf-8", "replace").split("\n")
        return cache[k]

    def find_field(c, sname, fname):
        for rel in sorted(c.code):
            tx = c.files[rel].decode("utf-8", "replace")
            mt = mask(tx)
            for sm in re.finditer(r"\b(?:struct)\s+" + re.escape(sname) + r"\b[^;{]*\{", mt):
                i, depth, j = sm.end() - 1, 0, sm.end() - 1
                while j < len(mt):
                    if mt[j] == "{":
                        depth += 1
                    elif mt[j] == "}":
                        depth -= 1
                        if depth == 0:
                            break
                    j += 1
                lines = tx.split("\n")
                for k in range(tx.count("\n", 0, i) + 1, tx.count("\n", 0, j) + 2):
                    if k - 1 < len(lines):
                        fm = FIELD_RX.match(lines[k - 1])
                        if fm and fm.group(3) == fname:
                            return rel, k, lines[k - 1]
            for sm in re.finditer(r"\b(?:struct)\s+" + re.escape(sname) + r"\b\s*(?:<[^>]*>)?\s*\(", mt):
                pass
        return None

    for d in load([("lib", libp), ("test", testp)]):
        code = d["code"]["code"]
        msg = d["message"]
        prim = [s for s in d["spans"] if s["is_primary"]]
        if not prim:
            continue
        ucrate, urel = repo_of(prim[0]["file_name"])
        row = dict(cfg=d["cfg"], code=code, msg=msg[:200], use_crate=ucrate, use_file=urel, use_line=prim[0]["line_start"], member=None, owner=None, def_file=None, def_line=None, def_text=None)
        if code in ("E0616", "E0451"):
            m = re.match(r"field `([^`]+)` of struct `([^`]+)` is private", msg)
            if not m:
                unresolved.append((msg, "pattern"))
                continue
            fname, spath = m.groups()
            segs = spath.split("::")
            sname = re.sub(r"<.*", "", segs[-1])
            oc = segs[0][4:] if segs[0].startswith("__m_") else ucrate
            row["owner"] = oc
            row["member"] = f"{sname}.{fname}"
            row["kind"] = "field"
            c = crates.get(oc)
            f = find_field(c, sname, fname) if c else None
            if f:
                row["def_file"], row["def_line"], row["def_text"] = f
            else:
                unresolved.append((msg, "field def not found"))
        else:
            defs = []
            def walk(x):
                for s in x.get("spans", []):
                    if not s["is_primary"] and "defined here" in (s.get("label") or ""):
                        defs.append(s)
                    if s["is_primary"] and x is not d:
                        defs.append(s)
                for ch in x.get("children", []):
                    walk(ch)
            walk(d)
            nm = re.search(r"`([^`]+)`", msg)
            nm = nm.group(1) if nm else "?"
            row["kind"] = "method" if code == "E0624" else "item"
            row["member"] = nm
            if not defs:
                unresolved.append((msg, "no def span"))
            else:
                s = defs[-1]
                oc, drel = repo_of(s["file_name"])
                row["owner"] = oc
                c = crates.get(oc)
                if c is None or drel not in c.files:
                    unresolved.append((msg, "def crate/file not in family"))
                else:
                    lines = lines_of(c, drel)
                    k = s["line_start"]
                    cand = None
                    for off in [0, -1, 1, -2, 2, -3, 3, -4, 4, -5, 5]:
                        kk = k + off
                        if 1 <= kk <= len(lines) and re.search(r"\b" + re.escape(nm) + r"\b", lines[kk - 1]) and FN_RX.match(lines[kk - 1]):
                            cand = kk
                            break
                    if cand:
                        row["def_file"], row["def_line"], row["def_text"] = drel, cand, lines[cand - 1]
                    else:
                        row["def_file"], row["def_line"], row["def_text"] = drel, k, lines[k - 1] if k - 1 < len(lines) else ""
                        unresolved.append((msg, "line mismatch %s:%d" % (drel[-50:], k)))
        rows.append(row)
    agg = collections.OrderedDict()
    for r in rows:
        k = (r["owner"], r["def_file"], r["def_line"], r["member"], r["kind"])
        a = agg.setdefault(k, dict(owner=r["owner"], def_file=r["def_file"], def_line=r["def_line"], member=r["member"], kind=r["kind"], def_text=r["def_text"], lib=0, test=0, users=collections.Counter(), codes=set()))
        a[r["cfg"]] += 1
        a["users"][r["use_crate"]] += 1
        a["codes"].add(r["code"])
    members = []
    for a in agg.values():
        a["users"] = dict(a["users"])
        a["codes"] = sorted(a["codes"])
        members.append(a)
    json.dump(dict(rows=rows, members=members, unresolved=unresolved), open(f"{GEN}/{PREFIX}-{tag}-attr.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    byowner = collections.Counter()
    for r in rows:
        byowner[(r["cfg"], r["owner"])] += 1
    print(len(rows), "rows;", len(members), "members;", len(unresolved), "unresolved")
    for cfg in ("lib", "test"):
        print(cfg, sum(1 for r in rows if r["cfg"] == cfg))
        for (cf, o), n in sorted(byowner.items(), key=lambda x: -x[1]):
            if cf == cfg:
                print(f"   {n:4} {str(o).replace('semio_framework_os_renderer_wgpu', '~')}")


main()
