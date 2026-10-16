"""Apply the visibility widenings that the union oracle reports (E0603/E0616/E0624/E0451 and 'exists but is inaccessible').
Reads the current-side diagnostics json of union.py, locates each definition in the REPO file (never the sandbox) and widens it to `pub`
(plus #[doc(hidden)] on members and fields). Re-reads every file before the edit. usage: autowiden.py <crate-id> <diag json> [--apply]"""
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import edit as E
from defs import scan, struct_fields

ROOT = E.ROOT
W = E.F + "/🗑️generated/otcp-work"


def rel_of(file_name, cid):
    p = os.path.normpath(file_name if os.path.isabs(file_name) else os.path.join(f"{W}/{cid}/B", file_name))
    base = f"{W}/{cid}/B/"
    if p.startswith(base):
        return p[len(base):]
    return None


def line_off(text):
    offs = [0]
    for m in re.finditer("\n", text):
        offs.append(m.end())
    return offs


def targets(diags, cid, base_unresolved=None):
    """yield (kind, name, repo_rel_file, line, extra) for every definition to widen."""
    out = []
    for d in diags:
        msg = d["message"]
        code = (d.get("code") or {}).get("code")
        if base_unresolved is not None:
            mname = re.match(r"cannot find [\w, ]+ `(\w+)` in ", msg)
            if mname and mname.group(1) in base_unresolved:
                continue
        m = re.match(r"field `(\w+)` of (?:struct|union) `(?:[\w$]+::)*(\w+)(?:<[^`]*>)?` is private", msg)
        if m and code in ("E0616", "E0451"):
            sp0 = [x for x in d.get("spans", []) if x["is_primary"]]
            out.append(("field", m.group(1), m.group(2), None, rel_of(sp0[0]["file_name"], cid) if sp0 else None))
            continue
        mn = re.match(r"cannot find (?:function|value|type|struct|trait|macro|constant|static|unit struct|tuple struct|struct, variant or union type|function, tuple struct or tuple variant|attribute|derive macro) `(\w+)` in this scope", msg)
        if mn and d.get("__new"):
            sp0 = [x for x in d.get("spans", []) if x["is_primary"]]
            out.append(("item-by-name", mn.group(1), None, None, rel_of(sp0[0]["file_name"], cid) if sp0 else None))
            continue
        mm = re.match(r"no method named `(\w+)` found for (?:struct|enum|union) `(?:[\w$]+::)*(\w+)", msg)
        if mm and code == "E0599" and d.get("__new"):
            out.append(("method-by-name", mm.group(1), mm.group(2), None, None))
            continue
        privacy = code in ("E0603", "E0624", "E0451", "E0616") or any(("inaccessible" in c["message"] or "is private" in c["message"]) for c in d.get("children", []))
        if not privacy:
            continue
        cands = [("defined here", sp) for sp in d.get("spans", []) if not sp["is_primary"] and "defined here" in (sp.get("label") or "")]
        for c in d.get("children", []):
            cm = c["message"]
            if "defined here" in cm or "exists but is inaccessible" in cm or "which is defined here" in cm or "is defined here" in cm:
                for s in c.get("spans", []):
                    cands.append((cm, s))
        if not cands:
            continue
        # prefer the span of the real definition: "refers to ... defined here" > plain defined here
        cm, s = cands[-1]
        f = rel_of(s["file_name"], cid)
        if f is None:
            continue
        out.append(("def", None, f, s["line_start"], dict(msg=msg, code=code, note=cm, col=s["column_start"], label=s.get("label"))))
    return out


def find_struct(cid, sname, fname, prod_files):
    hits = []
    for f in prod_files:
        t = E.read(f)
        sc = scan(t)
        for it in sc["items"]:
            if it["kind"] in ("struct", "union") and it.get("name") == sname:
                for (nm, vis, vpos, npos) in struct_fields(sc["m"], it, t):
                    if nm == fname:
                        hits.append((f, it, nm, vis, vpos, npos))
    return hits


def widen_text(t, vpos, vis, headpos, indent_attr=None, doc_hidden=False, field=False):
    """return new text for a declaration whose visibility range is vpos (empty if none) and whose first token starts at headpos."""
    if vis == "pub":
        return None
    if vis:
        a, b = vpos
        new = t[:a] + "pub" + t[b:]
        shift = 3 - (b - a)
        pos_after = a
    else:
        new = t[:headpos] + "pub " + t[headpos:]
        pos_after = headpos
    if doc_hidden:
        ins = "#[doc(hidden)] "
        at = headpos
        if field:
            new = new[:at] + ins + new[at:]
        else:
            new = new[:at] + ins + new[at:]
    return new


def apply(cid, diags, prod_files, do_apply, base_unresolved=None):
    changes = collections.OrderedDict()
    log = []
    for kind, name, a, line, extra in targets(diags, cid, base_unresolved):
        if kind == "field":
            hits = find_struct(cid, a, name, prod_files)
            if len(hits) > 1 and extra:
                anc = [h for h in hits if extra.startswith(os.path.dirname(h[0]) + "/")]
                if anc:
                    best = max(len(os.path.dirname(h[0])) for h in anc)
                    hits = [h for h in anc if len(os.path.dirname(h[0])) == best]
            if len(hits) != 1:
                log.append(f"AMBIGUOUS/ABSENT field {a}.{name}: {len(hits)} hits")
                continue
            f, it, nm, vis, vpos, npos = hits[0]
            key = ("field", f, a, name)
            changes[key] = (f, vis, vpos, npos, True)
            continue
        if kind == "item-by-name":
            hit = []
            for f in sorted(prod_files):
                t = E.read(f)
                sc = scan(t)
                for r in sc["items"]:
                    if r.get("name") == name and r["vis"] != "pub" and r["kind"] in ("fn", "struct", "enum", "const", "static", "type", "trait", "union") and r["ctx"] == "":
                        hit.append((f, r))
            if len(hit) > 1 and extra:
                anc = [(f, r) for f, r in hit if extra.startswith(os.path.dirname(f) + "/")]
                if anc:
                    best = max(len(os.path.dirname(f)) for f, r in anc)
                    hit = [(f, r) for f, r in anc if len(os.path.dirname(f)) == best]
            if len(hit) == 1:
                f, r = hit[0]
                changes[("def", f, r["kwpos"])] = (f, r["vis"], r["vpos"], r["hpos"], False)
                log.append(f"widen {r['kind']} {name} @ {os.path.basename(os.path.dirname(f))} ({r['vis'] or 'priv'} -> pub) by name")
            else:
                log.append(f"AMBIGUOUS/ABSENT private item {name}: {len(hit)}")
            continue
        if kind == "method-by-name":
            hit = 0
            for f in sorted(prod_files):
                t = E.read(f)
                sc = scan(t)
                for r in sc["members"]:
                    if r["kind"] == "fn" and r.get("name") == name and r["vis"] != "pub" and r["ctx"].startswith("impl:" + a) and not r["ctx"].endswith(":trait"):
                        changes[("def", f, r["kwpos"])] = (f, r["vis"], r["vpos"], r["hpos"], True)
                        log.append(f"widen method {a}::{name} @ {os.path.basename(os.path.dirname(f))} ({r['vis'] or 'priv'} -> pub)")
                        hit += 1
            if not hit:
                log.append(f"NO private method {a}::{name} found")
            continue
        f = a
        if f not in prod_files:
            log.append(f"SKIP not a production file: {f} ({extra['msg'][:70]})")
            continue
        t = E.read(f)
        sc = scan(t)
        offs = line_off(t)
        lo, hi = offs[line - 1], offs[line] if line < len(offs) else len(t)
        rec = None
        for r in sc["items"] + sc["members"]:
            if lo <= r["kwpos"] < hi and r["kind"] not in ("impl", "mod", "use"):
                rec = r
                break
        if rec is None:
            # a field of a struct on that line?
            log.append(f"NO-ITEM at {f}:{line} ({extra['msg'][:80]})")
            continue
        member = rec in sc["members"]
        key = ("def", f, rec["kwpos"])
        changes[key] = (f, rec["vis"], rec["vpos"], rec["hpos"], member)
        log.append(f"widen {rec['kind']} {rec.get('name')} @ {os.path.basename(os.path.dirname(f))}:{line} ({rec['vis'] or 'priv'} -> pub){' member' if member else ''}")
    # group by file, apply from the end
    byfile = collections.defaultdict(list)
    for key, (f, vis, vpos, headpos, doc) in changes.items():
        byfile[f].append((key, vis, vpos, headpos, doc))
    for f, lst in byfile.items():
        t = E.read(f)
        ops = []
        for key, vis, vpos, headpos, doc in lst:
            if key[0] == "field":
                ops.append((vpos[0] if vis else headpos, vpos, vis, doc))
            else:
                ops.append((headpos, vpos, vis, doc))
        # dedupe by position
        seen = set()
        uniq = []
        for o in sorted(ops, key=lambda o: -o[0]):
            if o[0] in seen:
                continue
            seen.add(o[0])
            uniq.append(o)
        new = t
        for pos, vpos, vis, doc in uniq:
            if vis == "pub":
                continue
            ins = ("#[doc(hidden)] " if doc else "")
            if vis:
                new = new[:vpos[0]] + ins + "pub" + new[vpos[1]:]
            else:
                new = new[:pos] + ins + "pub " + new[pos:]
        if new != t:
            log.append(f"EDIT {f} ({len(uniq)} decl)")
            if do_apply:
                E.pre_copy(f)
                E.write(f, new)
                E.note("edit", f)
    return log


if __name__ == "__main__":
    cid = sys.argv[1]
    diags = json.load(open(sys.argv[2], encoding="utf-8"))
    cfg = json.load(open(f"{os.path.dirname(__file__)}/cfg/{cid}.json", encoding="utf-8"))
    pf = set(cfg.get("prod_files", []))
    # production files: all .rs files in p_dirs that are not companion files
    prod_files = set()
    for d in cfg["p_dirs"]:
        base = ROOT + "/" + d
        for dp, dn, fn in os.walk(base):
            if "node_modules" in dp:
                continue
            for x in fn:
                if x.endswith(".rs"):
                    prod_files.add(os.path.relpath(os.path.join(dp, x), ROOT))
    for c in cfg["comps"]:
        cdir = os.path.dirname(c["lib"])
        prod_files = {p for p in prod_files if not p.startswith(cdir + "/")}
    for l in apply(cid, diags, prod_files, "--apply" in sys.argv):
        print(l)
