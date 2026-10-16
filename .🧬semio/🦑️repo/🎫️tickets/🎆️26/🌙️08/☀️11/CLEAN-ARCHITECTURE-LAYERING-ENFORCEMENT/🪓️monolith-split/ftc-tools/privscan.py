"""Definition scan: names of non-`pub` structs, enums, traits, consts, fields and fns defined in the lib closure files that a test file mentions.
usage: privscan.py <test-file>... -- <lib-file>...   (lib files = every non-test file of the library closure)
Output: one line per (name, kind, definition file:line) with the test lines that mention it as `.name`, `::name`, `name(` or a type/const word."""
import re, sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ftcstub import mask

VIS = r"(?P<vis>pub(?:\s*\([^)]*\))?\s+)?"
ITEM = re.compile(r"^\s*" + VIS + r"(?:async\s+|unsafe\s+|const\s+|extern\s+\"[^\"]*\"\s+)*(?P<kw>fn|struct|enum|trait|const|static|type|union)\s+(?P<name>[A-Za-z_]\w*)")
FIELD = re.compile(r"^\s*" + VIS + r"(?P<name>[A-Za-z_]\w*)\s*:\s*[^:]")
TUPLE = re.compile(r"\bstruct\s+(?P<name>\w+)\s*(?:<[^>]*>)?\s*\(")


def restricted(vis):
    return vis is None or vis.strip() != "pub"


def scan_defs(path):
    text = mask(open(path, encoding="utf-8").read())
    out = []
    stack = []
    for i, line in enumerate(text.split("\n"), 1):
        top = stack[-1] if stack else ("mod", None)
        m = ITEM.match(line)
        if m and top[0] in ("mod", "impl", "trait"):
            vis = m.group("vis")
            if top[0] == "trait":
                pass
            elif restricted(vis):
                out.append((m.group("name"), m.group("kw"), f"{os.path.basename(os.path.dirname(path))}/{os.path.basename(path)}:{i}", vis.strip() if vis else "private"))
        elif top[0] == "struct":
            fm = FIELD.match(line)
            if fm and restricted(fm.group("vis")):
                out.append((fm.group("name"), "field", f"{os.path.basename(os.path.dirname(path))}/{os.path.basename(path)}:{i}", fm.group("vis").strip() if fm.group("vis") else "private"))
        tm = TUPLE.search(line)
        if tm:
            body = line[tm.end():]
            for k, part in enumerate(re.split(r",", body.split(")")[0])):
                if part.strip() and not part.strip().startswith("pub") and not part.strip().startswith(")"):
                    out.append((f"{tm.group('name')}.{k}", "tuple-field", f"{os.path.basename(os.path.dirname(path))}/{os.path.basename(path)}:{i}", "private"))
        for ch in line:
            pass
        opens = line.count("{") - line.count("}")
        kind = None
        if re.search(r"\bstruct\s+\w+[^;]*\{\s*$", line) or re.search(r"\bstruct\s+\w+[^;{]*$", line) and "{" in line:
            kind = "struct"
        elif re.search(r"\bimpl\b", line) and "{" in line:
            kind = "traitimpl" if re.search(r"\bfor\b", line.split("{")[0]) else "impl"
        elif re.search(r"\btrait\s+\w+", line) and "{" in line:
            kind = "trait"
        elif re.search(r"\benum\s+\w+", line) and "{" in line:
            kind = "enum"
        if opens > 0:
            for _ in range(opens):
                stack.append((kind or "block", None))
                kind = "block"
        elif opens < 0:
            for _ in range(-opens):
                if stack:
                    stack.pop()
    return out


def main():
    a = sys.argv[1:]
    k = a.index("--")
    tests, libs = a[:k], a[k + 1:]
    defs = []
    for f in libs:
        defs += scan_defs(f)
    by = collections.defaultdict(list)
    for d in defs:
        by[d[0]].append(d)
    tmasked = {t: mask(open(t, encoding="utf-8").read()).split("\n") for t in tests}
    own = set()
    for lines in tmasked.values():
        for l in lines:
            m = re.match(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+|unsafe\s+|const\s+)*(?:fn|struct|enum|trait|const|static|type)\s+([A-Za-z_]\w*)", l)
            if m:
                own.add(m.group(1))
    rows = []
    for name, ds in by.items():
        if "." in name or name in own:
            continue
        rx = [re.compile(r"\." + re.escape(name) + r"\b"), re.compile(r"::" + re.escape(name) + r"\b"), re.compile(r"(?<![\w.:])" + re.escape(name) + r"\s*(?:\(|\{|::|\b)")]
        hits = []
        for t, lines in tmasked.items():
            for i, l in enumerate(lines, 1):
                if any(r.search(l) for r in rx[:2]) or (name[0].isupper() or name.isupper()) and rx[2].search(l) or rx[2].search(l) and any(d[1] == "fn" for d in ds):
                    hits.append(f"{os.path.basename(os.path.dirname(t))}:{i}")
        if hits:
            rows.append((name, ds, hits))
    for name, ds, hits in sorted(rows):
        print(f"{name}\t{';'.join(sorted({d[1] + ' ' + d[3] + ' @' + d[2] for d in ds}))}\t{len(hits)}x {','.join(hits[:4])}")


if __name__ == "__main__":
    main()
