#!/usr/bin/env python3
# One-shot derivation of the CSV mutation-test fixture from real research data already committed
# in the repo (♻️mit-bestand/📋️bericht/📋️zwischenbericht/anhang/projekte.tex — a secondary-research
# catalog of real built/planned building-component reuse projects). Scratch-only; not committed.
import re, sys, csv, io

SRC = "/Users/ueli/Documents/semio/♻️mit-bestand/📋️bericht/📋️zwischenbericht/anhang/projekte.tex"
OUT = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/csv-rfc4180-any-scratch/reuse-projects.csv"

text = open(SRC, encoding="utf-8").read()

def top_level_split(s, sep="&"):
    parts, depth, cur = [], 0, []
    for ch in s:
        if ch == "{":
            depth += 1
            cur.append(ch)
        elif ch == "}":
            depth -= 1
            cur.append(ch)
        elif ch == sep and depth == 0:
            parts.append("".join(cur))
            cur = []
        else:
            cur.append(ch)
    parts.append("".join(cur))
    return parts

def extract_rows(braced_row):
    """braced_row: content between the outer { and } of \\SemioTableRow{...}"""
    return [c.strip() for c in top_level_split(braced_row, "&")]

def clean_latex(s):
    s = s.strip()
    s = s.replace("\\&", "&")
    s = re.sub(r"\\cite\{[^}]*\}", "", s)
    s = re.sub(r"\\SemioTableRow", "", s)
    s = s.replace("\\%", "%")
    s = s.replace("--", "-")
    s = s.replace("---", "-")
    s = s.strip()
    return s

# Split into subsections (each is one real project).
sections = re.split(r"\n\\subsection\{", text)[1:]

rows = []
for sec in sections:
    title_end = sec.index("}")
    title = clean_latex(sec[:title_end])
    body = sec[title_end:]

    # Find all \SemioTableRow{...} with brace-aware matching.
    table_rows = []
    idx = 0
    while True:
        m = re.search(r"\\SemioTableRow\{", body[idx:])
        if not m:
            break
        start = idx + m.end()
        depth = 1
        i = start
        while depth > 0:
            if body[i] == "{":
                depth += 1
            elif body[i] == "}":
                depth -= 1
            i += 1
        table_rows.append(body[start:i-1])
        idx = i

    if not table_rows:
        continue
    general = extract_rows(table_rows[0])
    if len(general) != 5:
        print(f"!! unexpected general row shape in {title!r}: {general}", file=sys.stderr)
        continue
    ort, jahr, typ, status, _quelle = [clean_latex(c) for c in general]

    for raw in table_rows[1:]:
        comp = extract_rows(raw)
        if len(comp) != 6:
            print(f"!! unexpected component row shape in {title!r}: {comp}", file=sys.stderr)
            continue
        bauteil, material, spender, reuse_ort, schicht, prozess = [clean_latex(c) for c in comp]
        rows.append([title, ort, jahr, typ, status, bauteil, material, spender, reuse_ort, schicht, prozess])

header = ["Projekt", "Ort", "Jahr", "Typ", "Status", "Bauteil", "Material", "Spender", "ReUse-Ort", "Schicht", "Prozess"]

buf = io.StringIO()
w = csv.writer(buf, lineterminator="\n")
w.writerow(header)
for r in rows:
    w.writerow(r)

open(OUT, "w", encoding="utf-8", newline="").write(buf.getvalue())
print(f"wrote {len(rows)} data rows to {OUT}", file=sys.stderr)
