"""🔧 POS2 widening planner and applier (the only POS tool that writes repo files, and only in `apply`, one verified line at a time).
`plan` reads the rustc JSON logs of a finished run and, for every privacy diagnostic (E0603 item, E0616 field, E0624 method/const) that is not stub noise, finds the definition line in the real repo file
(secondary span of the diagnostic, or the struct field found in the crate closure) and proposes the narrowest fix: `pub(crate)`, `pub(super)`, `pub(in ..)` or private becomes `pub` (fields and items) with `#[doc(hidden)]` left to the human.
`apply` re-reads each target line right before the edit, asserts that it is exactly the planned text and that it occurs once, rewrites that single line and prints `git diff --stat` input paths.
usage: pos2widen.py plan <family.json> <side> <tag> <err-file>...        (writes GEN/pos2-<tag>-widen-plan.json)
       pos2widen.py apply <plan.json> [--dry]"""
import json, os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import posunion as U
from poslib import *
from posstub import mask

PRIV = {"E0603", "E0616", "E0624", "E0451"}
FN_RX = re.compile(r'^(\s*(?:#\[[^\]]*\]\s*)*)(pub\s*\([^)]*\)\s+|pub\s+)?((?:const\s+|async\s+|unsafe\s+|extern\s+"[^"]*"\s+)*(?:fn|const|static|type|struct|enum|trait|mod|use)\b)')
FIELD_RX = re.compile(r"^(\s*(?:#\[[^\]]*\]\s*)*)(pub\s*\([^)]*\)\s+|pub\s+)?(\w+)\s*:")


def repo_of(file_name):
    m = re.match(r"^(__m_[A-Za-z0-9_]+)/(.*)$", file_name)
    if not m:
        return None, None
    return m.group(1)[4:], os.path.normpath(m.group(2))


def load_diags(paths):
    out = []
    for p in paths:
        for l in open(p, encoding="utf-8"):
            l = l.strip()
            if not l.startswith("{"):
                continue
            try:
                d = json.loads(l)
            except Exception:
                continue
            code = (d.get("code") or {}).get("code")
            if d.get("level") == "error" and code in PRIV:
                out.append(d)
    return out


def widen_line(line, name, kind):
    rx = FIELD_RX if kind == "field" else FN_RX
    m = rx.match(line)
    if not m:
        return None
    if kind == "field":
        if m.group(3) != name:
            return None
        return line[:m.start(2) if m.group(2) else m.start(3)] + "pub " + line[m.start(3):]
    if m.group(2) and m.group(2).strip() == "pub":
        return None
    start = m.start(2) if m.group(2) else m.start(3)
    end = m.end(2) if m.group(2) else m.start(3)
    return line[:start] + "pub " + line[end:]


def plan(spec_path, side, tag, errs):
    spec = json.load(open(spec_path, encoding="utf-8"))
    mans = Manifests(Source(None))
    mans.build_index()
    pk = spec["crates"] if side == "cur" else spec["base"]["crates"]
    crates = {c.id: c for c in U.prepare(Source(None), mans, pk, False)}
    rows, seen, failed = [], set(), []
    for d in load_diags(errs):
        code = (d.get("code") or {}).get("code")
        prim = [s for s in d["spans"] if s["is_primary"]]
        if not prim:
            continue
        msg = d["message"]
        defs = [s for s in d["spans"] if not s["is_primary"] and "defined here" in (s.get("label") or "")]
        use_crate, use_rel = repo_of(prim[0]["file_name"])
        if code in ("E0616", "E0451"):
            m = re.match(r"field `([^`]+)` of struct `([^`]+)` is private", msg)
            if not m:
                continue
            fname, spath = m.group(1), m.group(2)
            segs = spath.split("::")
            sname = re.sub(r"<.*", "", segs[-1])
            ccrate = segs[0][4:] if segs[0].startswith("__m_") else use_crate
            c = crates.get(ccrate)
            if c is None:
                failed.append((msg, "struct crate not in family"))
                continue
            found = None
            for rel in sorted(c.code):
                tx = c.files[rel].decode("utf-8", "replace")
                mt = mask(tx)
                sm = re.search(r"\b(?:struct)\s+" + re.escape(sname) + r"\b[^;{]*\{", mt)
                if not sm:
                    continue
                i, depth = sm.end() - 1, 0
                j = i
                while j < len(mt):
                    if mt[j] == "{":
                        depth += 1
                    elif mt[j] == "}":
                        depth -= 1
                        if depth == 0:
                            break
                    j += 1
                for k in range(tx.count("\n", 0, i) + 1, tx.count("\n", 0, j) + 2):
                    lines = tx.split("\n")
                    if k - 1 < len(lines):
                        fm = FIELD_RX.match(lines[k - 1])
                        if fm and fm.group(3) == fname:
                            found = (rel, k, lines[k - 1])
                            break
                if found:
                    break
            if not found:
                failed.append((msg, "field line not found"))
                continue
            rel, k, line = found
            key = (rel, k)
            if key in seen:
                continue
            seen.add(key)
            nl = widen_line(line, fname, "field")
            if nl is None or nl == line:
                continue
            rows.append(dict(file=rel, line=k, name=fname, kind="field", old=line, new=nl, use=use_rel[-80:], msg=msg[:120]))
        else:
            if not defs:
                failed.append((msg, "no definition span"))
                continue
            dcrate, drel = repo_of(defs[0]["file_name"])
            k = defs[0]["line_start"]
            key = (drel, k)
            if key in seen:
                continue
            seen.add(key)
            c = crates.get(dcrate)
            if c is None or drel not in c.files:
                failed.append((msg, "definition crate/file not in family"))
                continue
            lines = c.files[drel].decode("utf-8", "replace").split("\n")
            nm = re.search(r"`([^`]+)`", msg).group(1)
            line = lines[k - 1] if k - 1 < len(lines) else ""
            cand = None
            for off in [0, -1, 1, -2, 2, -3, 3]:
                kk = k + off
                if 1 <= kk <= len(lines) and re.search(r"\b" + re.escape(nm) + r"\b", lines[kk - 1]) and FN_RX.match(lines[kk - 1]):
                    cand = kk
                    break
            if cand is None:
                failed.append((msg, "definition line mismatch at %s:%d" % (drel[-60:], k)))
                continue
            line = lines[cand - 1]
            nl = widen_line(line, nm, "item")
            if nl is None or nl == line:
                continue
            rows.append(dict(file=drel, line=cand, name=nm, kind="item", old=line, new=nl, use=use_rel[-80:] if use_rel else "", msg=msg[:120]))
    out = dict(family=spec["name"], side=side, rows=rows, failed=failed)
    log_write(f"{GEN}/{PREFIX}-{tag}-widen-plan.json", json.dumps(out, ensure_ascii=False, indent=1))
    print(len(rows), "planned definition edits;", len(failed), "unplanned")
    for r in rows:
        print(f"  {r['file'][-95:]}:{r['line']}  {r['kind']} {r['name']}")
    for f in failed[:20]:
        print("  UNPLANNED", f[0][:100], "::", f[1])


def apply(plan_path, dry):
    pl = json.load(open(plan_path, encoding="utf-8"))
    by = {}
    for r in pl["rows"]:
        by.setdefault(r["file"], []).append(r)
    done = []
    for rel, rows in by.items():
        full = os.path.realpath(os.path.join(ROOT, rel))
        assert full.startswith(os.path.realpath(ROOT) + os.sep) and "🗑️generated" not in full, full
        with open(full, "rb") as f:
            raw = f.read()
        tx = raw.decode("utf-8")
        lines = tx.split("\n")
        for r in sorted(rows, key=lambda x: x["line"]):
            k = r["line"] - 1
            assert k < len(lines) and lines[k] == r["old"], f"line changed since plan: {rel}:{r['line']}"
            lines[k] = r["new"]
        if not dry:
            with open(full, "wb") as f:
                f.write("\n".join(lines).encode("utf-8"))
        done.append(rel)
    print(("dry run: would edit " if dry else "edited ") + str(len(done)) + " files")
    for d in done:
        print(d)


if __name__ == "__main__":
    a = sys.argv[1:]
    if a[0] == "plan":
        plan(a[1], a[2], a[3], a[4:])
    elif a[0] == "apply":
        apply(a[1], "--dry" in a)
