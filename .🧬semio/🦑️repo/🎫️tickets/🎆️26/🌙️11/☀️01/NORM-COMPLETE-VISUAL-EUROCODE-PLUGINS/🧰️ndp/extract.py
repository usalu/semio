import json, re, sys, pathlib
T = pathlib.Path(__file__).resolve().parent.parent
SPECS = [("en1990-en1991-target.md", "en1990"), ("en1992-target.md", "en1992"), ("en1993-target.md", "en1993"),
         ("en1994-en1999-target.md", "en1994"), ("en1995-en1996-target.md", "en1995"), ("en1997-en1998-target.md", "en1997")]
TAG = re.compile(r"`?\[(?:V|T|verify)\]`?|NA-DE \?")
CODE = re.compile(r"EN (199\d)(?:-(\d(?:-\d)?))?")
CLAUSE = re.compile(r"Table [A-Z]?\d[\w.]*|(?<![\d.])(?!0\.)[A-Z]?[1-9]\d?(?:\.\d{1,2}){1,4}(?:\([\w]+\))?(?![\d%])")
HEAD = re.compile(r"^(?:#+\s*)?\*{0,2}([A-Z]{1,3}[0-9]{0,3}[.\-][0-9A-Za-z.\-]+|[A-Z]{1,3}-\d+[a-z]?)\b")
NUM = re.compile(r"EC\s*`?([-\d.,/ ]+?)`?\s*/\s*DE\s*`?(-?\d+(?:\.\d+)?)`?")
def slug(s): return re.sub(r"[^a-z0-9]+", "-", s.lower()).strip("-")
def parse():
    rows = []
    for fn, fam in SPECS:
        code, item, head = fam, None, ""
        seen = {}
        for ln, line in enumerate((T / "📐️specs" / fn).read_text().splitlines(), 1):
            if line.startswith("#"):
                head = line.lstrip("# ").strip()
                m = CODE.search(head)
                if m: code = "en" + m.group(1)
                item = None
            m = HEAD.match(line.strip())
            if m: item = m.group(1).rstrip(".-")
            m2 = re.match(r"\s*\*\*([^*]{1,60}?)\*\*", line)
            if m2 and not item: item = slug(m2.group(1))[:24]
            if not TAG.search(line): continue
            if line.startswith(("- Status", "- `[V]`", "> ", "- Verification", "- NDP columns")) or (line.startswith("- ") and "must be" in line.lower() and ln < 15): continue
            segs = [s.strip() for s in re.split(r";\s+|(?<=[.)\]`])\s+(?=[A-Z`])|\s\|\s", line) if TAG.search(s) and len(re.sub(r"[\W_]|\[V\]|\[T\]", "", s)) >= 10]
            if not segs: segs = [line]
            ctx = (head + " " + line)
            cm = CODE.search(line) or CODE.search(head)
            ccode = ("en" + cm.group(1)) if cm else code
            part = cm.group(2) if cm and cm.group(2) else ""
            clause_ctx = CLAUSE.search(line[:160]) or CLAUSE.search(head)
            base = slug(item or f"l{ln}")
            for s in segs:
                k = seen.get(base, 0) + 1; seen[base] = k
                sym = re.search(r"`([^`]{1,24})`", s) or re.search(r"`([^`]{1,24})`", line)
                cl = (re.search(r"Table [A-Z]?\d[\w.]*", s)) or clause_ctx
                nm = NUM.search(s)
                rec = de = None
                if nm:
                    try: rec = float(nm.group(1).strip()); de = float(nm.group(2))
                    except ValueError: rec = de = None
                rows.append(dict(id=f"ndp.{ccode}.{base}.{k}", code=ccode.upper().replace("EN", "EN "), part=part,
                    clause=cl.group(0) if cl else "?", symbol=sym.group(1) if sym else "?",
                    recommended=rec, naDe=de, unit="", source=dict(document=f"DIN EN {ccode[2:]}" + (f"-{part}" if part else "") + "/NA",
                    edition="to be recorded at verification", locator=(cl.group(0) if cl else head)[:80]),
                    status="unverified", specRef=f"{fn}:{ln}", asWritten=re.sub(r"\s+", " ", TAG.sub("", s))[:240].strip()))
    return rows
def conflicts():
    out = []
    for l in (T / "🔍️audits/ndp-crosscheck-en1992-en1995-en1996.md").read_text().splitlines():
        c = [x.strip() for x in l.strip("|").split("|")]
        if l.startswith("| E9") and c[-1] == "conflict" and len(c) >= 6:
            out.append(c)
    return out
if __name__ == "__main__":
    rows = parse(); confl = conflicts()
    byline = {}
    for r in rows: byline.setdefault(r["specRef"], []).append(r["id"])
    cf = {"E92": "en1992-target.md", "E95": "en1995-en1996-target.md", "E96": "en1995-en1996-target.md"}
    ids = {r["id"] for r in rows}
    for c in confl:
        ref = f"{cf[c[0][:3]]}:{re.match(r"\d+", c[1]).group(0)}"
        if ref not in byline:
            i = f"ndp.{c[0].lower()}.{slug(c[2])[:30]}.1"
            rows.append(dict(id=i, code="EN " + {"E92":"1992","E95":"1995","E96":"1996"}[c[0][:3]], part="", clause="?", symbol="?", recommended=None, naDe=None, unit="", source=dict(document="see spec", edition="to be recorded at verification", locator=c[2][:80]), status="unverified", specRef=ref, asWritten=c[2]))
            byline[ref] = [i]
    rows.sort(key=lambda r: r["id"])
    assert len({r["id"] for r in rows}) == len(rows)
    json.dump(dict(entries=rows), open(sys.argv[1], "w"), ensure_ascii=False, indent=1)
    json.dump(dict(conflicts=confl, byline=byline), open(sys.argv[2], "w"), ensure_ascii=False)
    print(len(rows), len(confl))
