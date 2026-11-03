import json, re, sys, pathlib

CATALOGUE = re.compile(r"(?<![A-Za-z0-9])[A-Z]{1,4}-\d{1,3}(?![\d])")
PARA = re.compile(r"\((\d{3,})\)")
KEY = re.compile(r"^ndp\.en(\d{4})\.")
DOC = re.compile(r"^DIN EN (199\d)(?:/NA|-((?:\d-)?\d)/NA)$")
CLAUSE_PART = re.compile(r"EN (199\d)(?:-(\d+(?:-\d+)?))?")


def clause_parts(text):
    return sorted({f"{fam}-{part}" if part else fam for fam, part in CLAUSE_PART.findall(text)})


def flags(x):
    src = x.get("source") or {}
    clause, locator, doc = x.get("clause", "") or "", src.get("locator", "") or "", src.get("document", "") or ""
    key = KEY.match(x["id"])
    out = []
    if CATALOGUE.search(clause) or CATALOGUE.search(locator):
        out.append(("certain", "catalogue-id"))
    if (PARA.search(clause) or PARA.search(locator)) and not any(re.fullmatch(r"199\d-[23]", p) for p in clause_parts(clause + " " + locator)):
        out.append(("certain", "paragraph>=100 outside parts 2/3"))
    m = DOC.match(doc)
    parts = clause_parts(clause + " " + locator)
    own = sorted({q for q in parts if q.startswith(key.group(1)) and "-" in q}) if key else []
    doc_part = (f"{m.group(1)}-{m.group(2)}" if m and m.group(2) else (m.group(1) if m else None))
    if m and len(own) == 1 and own[0] != doc_part and own[0].startswith(m.group(1)):
        out.append(("certain", f"document {doc!r} does not match clause part {own[0]}"))
    doc_family = re.search(r"EN (199\d)", doc)
    if key and doc_family and key.group(1) != doc_family.group(1):
        out.append(("uncertain", f"document family {doc_family.group(1)} != key {key.group(1)}"))
    if m and m.group(2) is None and not own and key and m.group(1) == key.group(1):
        out.append(("info", f"document {doc!r} has no part number and the clause names none"))
    return out


def main():
    show_info = "--all" in sys.argv
    args = [a for a in sys.argv[1:] if a != "--all"]
    doc = json.loads(pathlib.Path(args[0]).read_text(encoding="utf-8"))
    n = 0
    for x in doc["entries"]:
        for level, kind in flags(x):
            if level == "info" and not show_info:
                continue
            n += 1
            src = x.get("source") or {}
            print(f"{level}\t{x['id']}\t{kind}\tclause={x.get('clause')!r}\tlocator={src.get('locator')!r}\tdocument={src.get('document')!r}")
    print(f"{n} flags", file=sys.stderr)


if __name__ == "__main__":
    main()
