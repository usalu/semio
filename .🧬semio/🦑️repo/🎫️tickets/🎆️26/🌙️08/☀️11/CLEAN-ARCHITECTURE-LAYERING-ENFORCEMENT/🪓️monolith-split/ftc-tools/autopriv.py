"""🔓 Apply the privacy diagnostics of the union oracle to the repo: widen the definitions rustc names (E0624 methods, E0603 items, E0616 fields).
usage: autopriv.py <family> <cfg-err-glob> [--apply] [--only-pkg pkg,pkg]
Reads the last iteration's cur-side .err (JSON), maps sandbox spans to repo files (`.../ftc-work/<family>/cur/__m_<pkg>/<repo-rel>`), prints the plan and, with --apply, edits the repo files atomically.
A fix is applied only when the target line still carries the definition (fn/struct/field) so a shifted line never edits the wrong text."""
import glob, json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ftcstub import mask
from hooks import write

ROOT = "/Users/ueli/Documents/semio/"
GEN = ROOT + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/"


def repo_of(sandbox_path, fam):
    m = re.search(r"/ftc-work/" + re.escape(fam) + r"/cur/__m_[^/]+/(.*)$", sandbox_path)
    if not m:
        return None
    return os.path.normpath(m.group(1))


def parse(errfile):
    for l in open(errfile, encoding="utf-8"):
        if l.startswith("{"):
            try:
                d = json.loads(l)
            except Exception:
                continue
            if d.get("level") == "error" and (d.get("code") or {}).get("code") in ("E0616", "E0624", "E0603", "E0451"):
                yield d


def widen_fn_line(lines, i, name):
    l = lines[i]
    m = re.search(r"(pub\s*\(\s*(?:crate|super|self|in\s+[\w:]+)\s*\)\s+)?((?:async\s+|const\s+|unsafe\s+)*)fn\s+" + re.escape(name) + r"\b", l)
    if not m or (m.group(1) is None and re.search(r"\bpub\s+(?:async\s+|const\s+|unsafe\s+)*fn\s+" + re.escape(name), l)):
        return False
    lines[i] = l[: m.start()] + "#[doc(hidden)] pub " + m.group(2) + "fn " + name + l[m.end():]
    return True


def widen_item_line(lines, i, kw, name):
    l = lines[i]
    m = re.search(r"(pub\s*\(\s*(?:crate|super|self|in\s+[\w:]+)\s*\)\s+|(?<![\w])(?=" + kw + r"\s+" + re.escape(name) + r"\b))(" + kw + r")\s+" + re.escape(name) + r"\b", l)
    if not m or re.search(r"\bpub\s+" + kw + r"\s+" + re.escape(name), l):
        return False
    lines[i] = l[: m.start()] + "pub " + l[m.start(2):]
    return True


def widen_field(path_lines, struct, field):
    text = "\n".join(path_lines)
    t = mask(text)
    m = re.search(r"\bstruct\s+" + re.escape(struct) + r"\b", t)
    if not m:
        return None
    j = m.end()
    while j < len(t) and t[j] not in "{(;":
        j += 1
    if t[j] == ";":
        return None
    opener, closer = t[j], {"{": "}", "(": ")"}[t[j]]
    pub_struct = re.search(r"pub(?:\s*\([^)]*\))?\s*$", t[max(0, m.start() - 12): m.start()]) is not None and re.search(r"pub\s*$", t[max(0, m.start() - 8): m.start()]) is not None
    depth = 1
    k = j + 1
    start = k
    segs = []
    d2 = 0
    while k < len(t) and depth:
        ch = t[k]
        if ch in "([{<":
            d2 += 1
            depth += (ch == opener) and 0
        if ch == opener:
            pass
        if ch in ")]}>" and not (ch == ">" and t[k - 1] == "-"):
            d2 -= 1
        if ch == closer and d2 < 0:
            segs.append((start, k))
            depth = 0
            break
        if ch == "," and d2 == 0:
            segs.append((start, k))
            start = k + 1
        k += 1
    idx = None
    if field.isdigit():
        idx = int(field)
        if idx >= len(segs):
            return None
        a, b = segs[idx]
        seg = t[a:b]
        fm = re.match(r"^(\s*(?:#\[[^\]]*\]\s*)*)(pub\s*\(\s*(?:crate|super|self|in\s+[\w:]+)\s*\)\s+|pub\s+)?", seg)
    else:
        for a, b in segs:
            seg = t[a:b]
            fm = re.match(r"^(\s*(?:#\[[^\]]*\]\s*)*)(pub\s*\(\s*(?:crate|super|self|in\s+[\w:]+)\s*\)\s+|pub\s+)?" + re.escape(field) + r"\s*:", seg)
            if fm:
                break
        else:
            return None
    if fm.group(2) and fm.group(2).strip() == "pub":
        return False
    s0 = a + len(fm.group(1))
    s1 = s0 + (len(fm.group(2)) if fm.group(2) else 0)
    repl = ("#[doc(hidden)] " if pub_struct else "") + "pub "
    return text[:s0] + repl + text[s1:]


def main():
    fam = sys.argv[1]
    files = sorted(glob.glob(GEN + sys.argv[2]))
    apply = "--apply" in sys.argv
    only = None
    if "--only-pkg" in sys.argv:
        only = set(sys.argv[sys.argv.index("--only-pkg") + 1].split(","))
    plan = {}
    for ef in files:
        for d in parse(ef):
            code = d["code"]["code"]
            msg = d["message"]
            sec = [s for s in d["spans"] if not s["is_primary"] and "defined here" in (s.get("label") or "")]
            if code == "E0624" and sec:
                nm = re.search(r"method `(\w+)`", msg).group(1)
                rp = repo_of(sec[0]["file_name"], fam)
                if rp:
                    plan.setdefault(rp, set()).add(("fn", sec[0]["line_start"], nm))
            elif code == "E0603" and sec:
                m = re.search(r"(function|struct|enum|constant|type alias|trait|static|union|tuple struct|unit struct) `(\w+)`", msg)
                rp = repo_of(sec[0]["file_name"], fam)
                if m and rp:
                    kw = {"function": "fn", "constant": "const", "type alias": "type", "tuple struct": "struct", "unit struct": "struct"}.get(m.group(1), m.group(1))
                    plan.setdefault(rp, set()).add(("item", sec[0]["line_start"], kw + ":" + m.group(2)))
            elif code == "E0616":
                m = re.search(r"field `(\w+)` of struct `([\w:]+)`", msg)
                if m:
                    struct = m.group(2).split("::")[-1]
                    plan.setdefault("*field*", set()).add(("field", struct, m.group(1)))
            elif code == "E0603":
                plan.setdefault("*unlocated*", set()).add(("msg", msg, ""))
    for k, v in sorted(plan.items()):
        print(k if len(k) < 100 else "…" + k[-90:], sorted(map(str, v))[:12])
    if not apply:
        return
    for rp, items in plan.items():
        if rp.startswith("*"):
            continue
        path = ROOT + rp
        lines = open(path, encoding="utf-8").read().split("\n")
        for kind, line, name in sorted(items, key=lambda x: str(x)):
            ok = widen_fn_line(lines, line - 1, name) if kind == "fn" else widen_item_line(lines, line - 1, *name.split(":")) if kind == "item" else False
            print(("fixed " if ok else "SKIPPED ") + f"{os.path.basename(os.path.dirname(path))}:{line} {name}")
        write(path, "\n".join(lines))
    # fields: locate the struct among the closure files of the family crates by scanning the repo files named in plan or given roots
    roots = [ROOT + p for p in os.environ.get("AUTOPRIV_FILES", "").split(",") if p]
    for _, struct, field in sorted(plan.get("*field*", [])):
        done = False
        for path in roots:
            lines = open(path, encoding="utf-8").read().split("\n")
            r = widen_field(lines, struct, field)
            if r:
                write(path, r)
                print(f"fixed field {struct}.{field} in {os.path.basename(os.path.dirname(path))}")
                done = True
                break
            if r is False:
                done = True
                break
        if not done:
            print(f"UNLOCATED field {struct}.{field}")


if __name__ == "__main__":
    main()
