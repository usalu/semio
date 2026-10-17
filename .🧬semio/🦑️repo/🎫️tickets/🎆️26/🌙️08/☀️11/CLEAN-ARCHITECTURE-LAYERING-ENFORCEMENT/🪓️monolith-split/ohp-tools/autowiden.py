"""🔓 Widens private/pub(crate) definitions that another crate (or module) of the union uses: E0603 (item/const/fn), E0624 (method), E0616 (struct field).
Cross-crate use => `pub`; same-crate use => `pub(crate)`. Definition lines come from rustc's own "defined here" spans; fields are located through the struct name (unique definition only).
usage: autowiden.py <family> <label> <cfg> [--apply] [--skip SUBSTR]..."""
import json, os, re, sys, collections
ROOT = "/Users/ueli/Documents/semio"
GEN = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
a = sys.argv[1:]
fam, label, cfg = a[:3]
apply = "--apply" in a
skips = [a[i + 1] for i, x in enumerate(a) if x == "--skip"]
DEFS = json.load(open("/Users/ueli/.cache/ohp/defs.json", encoding="utf-8"))
ITEM = r"(?:const|static|fn|struct|enum|type|trait|mod|union|async fn|unsafe fn|const fn|extern)"
def repo_path(fn):
    m = re.match(r"^__m_([A-Za-z0-9_]+)/(.*)$", fn)
    if not m: return None, None
    rel = os.path.normpath(m.group(2))
    return m.group(1), (rel if os.path.exists(os.path.join(ROOT, rel)) else None)
todo = collections.OrderedDict()
skipped = collections.Counter()
for line in open(f"{GEN}/ohp-{fam}-{label}-cur-{cfg}-priv.err", encoding="utf-8", errors="replace"):
    if not line.startswith("{"): continue
    try: d = json.loads(line)
    except Exception: continue
    code = d["code"]["code"]
    pr = [s for s in d["spans"] if s["is_primary"]]
    if not pr or code not in ("E0603", "E0624", "E0616"): continue
    upkg, _ = repo_path(pr[0]["file_name"])
    if code in ("E0603", "E0624"):
        cands = []
        for s in d["spans"]:
            if not s["is_primary"] and "defined here" in (s.get("label") or ""): cands.append(s)
        for ch in d["children"]:
            if "defined here" in ch["message"] or "refers to" in ch["message"]:
                for s in ch["spans"]: cands.append(s)
        cands = [s for s in cands if s["text"]]
        if not cands: skipped["no-def-span"] += 1; continue
        s = cands[-1]
        dpkg, rel = repo_path(s["file_name"])
        if not rel or any(x in rel for x in skips): skipped["no-file/skip"] += 1; continue
        txt = s["text"][0]["text"]
        if re.match(r"^\s*(pub\s+)?use\b", txt): skipped["import"] += 1; continue
        vis = "pub" if dpkg != upkg else "pub(crate)"
        todo.setdefault(rel, []).append(("item", s["line_start"], vis, d["message"][:70]))
    else:
        m = re.match(r"field `([^`]+)` of struct `([^`]+)` is private", d["message"])
        if not m: skipped["e0616-shape"] += 1; continue
        field, sp = m.group(1), m.group(2).split("::")[-1]
        sp = re.sub(r"<.*", "", sp)
        locs = [(f, l) for f, l in DEFS.get(sp, []) if f.endswith(".rs") and "/🧪️tests/" not in f and not f.startswith(".🧬")]
        if len(locs) != 1: skipped["struct-ambiguous"] += 1; continue
        rel = locs[0][0]
        if any(x in rel for x in skips): skipped["skip"] += 1; continue
        mm = re.match(r"^__m_([A-Za-z0-9_]+)::", m.group(2))
        vis = "pub" if (mm and mm.group(1) != upkg) or not mm else "pub(crate)"
        todo.setdefault(rel, []).append(("field", (sp, field), vis, d["message"][:70]))
n = 0
for rel, ops in todo.items():
    p = os.path.join(ROOT, rel)
    lines = open(p, encoding="utf-8").read().split("\n")
    done = set()
    changed = []
    for kind, where, vis, msg in ops:
        if kind == "item":
            i = where - 1
            if i in done: continue
            l = lines[i]
            if any(re.match(r"^\s*#\[cfg\(test\)\]", lines[k]) or "#[cfg(test)]" in lines[k] for k in range(max(0, i - 3), i + 1)):
                skipped["cfg-test (needs testkit)"] += 1; continue
            m = re.match(r"^(\s*(?:#\[[^\]]*\]\s*)*)(?:pub\((?:crate|super|self|in [^)]+)\)\s+)?(" + ITEM + r"\b.*)$", l)
            if not m or re.match(r"^\s*pub\s", l) and not re.match(r"^\s*pub\(", l): skipped["shape/already-pub"] += 1; continue
            lines[i] = m.group(1) + vis + " " + m.group(2)
            done.add(i); changed.append((i + 1, msg))
        else:
            sp, field = where
            start = None
            for i, l in enumerate(lines):
                if re.match(r"^\s*(?:pub(?:\([^)]*\))?\s+)?struct " + re.escape(sp) + r"\b[^;]*\{\s*$", l) or re.match(r"^\s*(?:pub(?:\([^)]*\))?\s+)?struct " + re.escape(sp) + r"\b.*\{", l):
                    start = i; break
            if start is None: skipped["struct-not-braced"] += 1; continue
            depth = 0
            for i in range(start, len(lines)):
                depth += lines[i].count("{") - lines[i].count("}")
                m = re.match(r"^(\s*)(?:pub\((?:crate|super|self|in [^)]+)\)\s+)?" + re.escape(field) + r"\s*:", lines[i])
                if m and i > start and (i, field) not in done:
                    if re.match(r"^\s*pub\s+" + re.escape(field), lines[i]): break
                    lines[i] = re.sub(r"^(\s*)(?:pub\([^)]*\)\s+)?" + re.escape(field), lambda mm: mm.group(1) + vis + " " + field, lines[i], count=1)
                    done.add((i, field)); changed.append((i + 1, msg)); break
                if depth <= 0 and i > start: break
    if changed:
        print(rel.split("artifacts/")[-1][-70:], len(changed), "|", changed[0][1])
        n += len(changed)
        if apply: open(p, "w", encoding="utf-8").write("\n".join(lines))
print("widened", n, dict(skipped), "APPLIED" if apply else "dry")
