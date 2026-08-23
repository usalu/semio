#!/usr/bin/env python3
# One-shot derivation of the CSV mutation-test fixture from real research data already committed
# in the repo (♻️mit-bestand/📋️bericht/📋️zwischenbericht/anhang/bauteilboersen.tex — a systematic
# survey of 50 real European building-component reuse marketplaces/platforms). Scratch-only script,
# not committed; only its CSV output is committed as the fixture.
import re, sys, csv, io

SRC = "/Users/ueli/Documents/semio/♻️mit-bestand/📋️bericht/📋️zwischenbericht/anhang/bauteilboersen.tex"
OUT = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/csv-rfc4180-any-scratch/reuse-marketplaces.csv"

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

def clean_latex(s):
    s = s.strip()
    s = s.replace("\\&", "&")
    s = re.sub(r"\\textbf\{([^}]*)\}", r"\1", s)
    s = s.replace("\\textperiodcentered\\ ", "·")
    s = s.replace("n.\\ p.", "n. p.")
    s = s.strip()
    return s

def extract_rows(body):
    """Brace-aware extraction of every \\SemioTableRow{...} in body, returns list of field-lists."""
    out = []
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
        raw = body[start:i - 1]
        out.append([clean_latex(c) for c in top_level_split(raw, "&")])
        idx = i
    return out

# Each real appendix section groups platforms by kind (Marktplätze, Depots/Shops,
# Vermittlung/Kontakt, Ressourcenkataloge) — the document's own real taxonomy, not invented.
sections = re.split(r"\\appendixsection\[(\w)\]\{([^}]*)\}", text)
# sections[0] = preamble; then repeating (letter, title, body) triples.
combined = {}
order = []
for i in range(1, len(sections), 3):
    letter, title, body = sections[i], sections[i + 1], sections[i + 2]
    if letter not in ("M", "D", "V", "R"):
        continue
    long_blocks = re.split(r"\\SemioTableLong(?:\[[^\]]*\])?\{[^{}]*\}\{[^{}]*\}\{[^{}]*\}\{", body)[1:]
    if len(long_blocks) < 2:
        continue
    access_rows = extract_rows(long_blocks[0])
    fields_rows = extract_rows(long_blocks[1])
    fields_by_id = {r[0]: r for r in fields_rows}
    for row in access_rows:
        pid = row[0]
        platform_country = row[1]
        if "·" in platform_country:
            platform, country = [p.strip() for p in platform_country.split("·", 1)]
        else:
            platform, country = platform_country, ""
        angebotsform, zugang, kanal, physisch = row[2], row[3], row[4], row[5]
        fr = fields_by_id.get(pid, [pid, "", "", "", ""])
        datenfelder, suche, beschaffung, uebergabe = fr[1], fr[2], fr[3], fr[4]
        combined[pid] = [pid, title, platform, country, angebotsform, zugang, kanal, physisch, datenfelder, suche, beschaffung, uebergabe]
        order.append(pid)

header = ["ID", "Kategorie", "Plattform", "Land", "Angebotsform", "Zugang", "DigitalerKanal", "PhysischerZugang", "Datenfelder", "SuchUndAuswahlfunktionen", "Beschaffungsschritte", "Uebergabe"]

buf = io.StringIO()
# RFC 4180 itself specifies CRLF as the record delimiter (its own §2 rule 1) — using it here (rather
# than the LF this repository's own encoder always writes) means a genuine decode-then-re-encode of
# this real fixture can never coincidentally reproduce the committed bytes, so the no-byte-pass-
# -through tripwire in the identity-round-trip scenario stays meaningful instead of false-firing on
# an accidentally-already-canonical input.
w = csv.writer(buf, lineterminator="\r\n")
w.writerow(header)
for pid in order:
    w.writerow(combined[pid])

open(OUT, "w", encoding="utf-8", newline="").write(buf.getvalue())
print(f"wrote {len(order)} data rows to {OUT}", file=sys.stderr)
