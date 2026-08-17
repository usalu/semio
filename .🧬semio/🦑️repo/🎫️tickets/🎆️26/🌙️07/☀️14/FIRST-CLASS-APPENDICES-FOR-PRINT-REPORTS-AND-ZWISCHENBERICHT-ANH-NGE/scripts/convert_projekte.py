#!/usr/bin/env python3
"""One-off conversion of temp/anhang_projekte.md into LaTeX for the zwischenbericht
Anlage A appendix. Throwaway — lives only in the ticket folder, per repo rules."""
import re
import html

SRC = "/Users/ueli/Documents/semio/temp/anhang_projekte.md"
IMG_DIR = "asset/projekt"

with open(SRC, encoding="utf-8") as f:
    lines = f.readlines()

def split_row(line):
    line = line.rstrip("\n")
    assert line.startswith("|"), line
    parts = line.split("|")
    # drop the empty leading and trailing pieces from the outer pipes
    parts = parts[1:]
    if parts and parts[-1].strip() == "":
        parts = parts[:-1]
    return [p.strip() for p in parts]

def tex_escape(s):
    s = s.replace("\\", "")  # no backslashes expected in source text
    s = s.replace("&", r"\&")
    s = s.replace("%", r"\%")
    s = s.replace("#", r"\#")
    s = s.replace("_", r"\_")
    return s

# ---- Projects table (lines with "P" IDs) ----
project_rows = []
for ln in lines:
    if re.match(r"^\|\s*P\d\d-E\d\d\s*\|", ln):
        project_rows.append(split_row(ln))

# Manual fix for a shifted row in the source markdown (P46-E02): the source
# accidentally has 12 data cells instead of 11 (Material cell empty, values
# shifted one column right). Correct alignment restored by hand.
for row in project_rows:
    if row[0] == "P46-E02":
        # as-parsed: [ID, Projekt, Bild, Ort, Jahr, BauteilMenge, Material(empty), Spender, ReUseOrt, Schicht, Prozess, extra]
        assert len(row) == 12, row
        fixed = [row[0], row[1], row[2], row[3], row[4], row[5], "Beton", row[8], row[9], row[10], row[11]]
        row[:] = fixed

projects = []  # list of dicts: id, title, image, ort, jahr, elements: list of 6-tuples
current = None
for row in project_rows:
    pid, projekt, bild, ort, jahr, bauteil, material, spender, reuse, schicht, prozess = row[:11]
    proj_id = pid.split("-E")[0]
    if projekt:
        m = re.search(r'src="images/([^"]+\.jpg)"', bild)
        img = m.group(1) if m else None
        title = projekt.strip("*")
        current = {"id": proj_id, "title": title, "image": img, "ort": ort, "jahr": jahr, "elements": []}
        projects.append(current)
    assert current is not None and current["id"] == proj_id
    current["elements"].append((bauteil, material, spender, reuse, schicht, prozess))

assert len(projects) == 67, len(projects)
total_elements = sum(len(p["elements"]) for p in projects)

# ---- Quellen table ----
quellen_rows = []
in_quellen = False
for ln in lines:
    if ln.startswith("## Quellen"):
        in_quellen = True
        continue
    if in_quellen and re.match(r"^\|\s*S\d+\s*\|", ln):
        quellen_rows.append(split_row(ln))

quellen = []
for row in quellen_rows:
    qid, herausgeber, titel, jahr, link, ereignisse = row[:6]
    quellen.append((qid, herausgeber, titel, link, ereignisse))

# ---- Emit LaTeX ----
out = []
out.append(r"\label{anlage:projekte}")
out.append("")
out.append(
    "Katalog realisierter und geplanter ReUse-Projekte mit wiederverwendeten Bauteilen, "
    "erhoben als Sekundärrecherche zur Einordnung der im Projekt entwickelten Plattform. "
    "Jede Projektzeile fasst Ort und Stand zusammen; die nachfolgenden Zeilen listen die "
    "einzelnen dokumentierten Bauteile.%"
)
out.append("")
out.append(r"\subsection{Projekte}")
out.append("")

header = "Bauteil / Menge & Material & Spender & ReUse-Ort & Schicht & Prozess"
colspec = (
    r"|@{\hspace{\semio@chrome@padding}}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.18\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.10\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.29\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.09\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.17\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.17\semio@table@long@inner@w\relax}"
    r"@{}@{\hspace{\semio@chrome@padding}}|"
)

out.append(r"\SemioTableLong{Projekte}{%")
out.append(f"  {colspec}%")
out.append(f"}}{{{header}}}{{%")
for p in projects:
    meta = f"{tex_escape(p['ort'])} \\textperiodcentered\\ {tex_escape(p['jahr'])}"
    title = f"{p['id']} \\textperiodcentered\\ {tex_escape(p['title'])}"
    image = f"{IMG_DIR}/{p['image']}" if p["image"] else ""
    if image:
        out.append(f"  \\SemioTableBandRow[{image}]{{6}}{{{title}}}{{{meta}}}")
    else:
        out.append(f"  \\SemioTableBandRow{{6}}{{{title}}}{{{meta}}}")
    for (bauteil, material, spender, reuse, schicht, prozess) in p["elements"]:
        cells = " & ".join(tex_escape(c) if c else "--" for c in (bauteil, material, spender, reuse, schicht, prozess))
        out.append(f"  \\SemioTableRow{{{cells}}}")
out.append("}")
out.append("")

out.append(r"\subsection{Quellen}")
out.append("")
q_header = "Quelle-ID & Herausgeber \\textbullet\\ Titel & Zugeordnete Ereignisse"
q_colspec = (
    r"|@{\hspace{\semio@chrome@padding}}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.08\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.62\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.30\semio@table@long@inner@w\relax}"
    r"@{}@{\hspace{\semio@chrome@padding}}|"
)
out.append(r"\SemioTableLong{Quellen}{%")
out.append(f"  {q_colspec}%")
out.append(f"}}{{{q_header}}}{{%")
for (qid, herausgeber, titel, link, ereignisse) in quellen:
    cell2 = f"{tex_escape(herausgeber)} \\textperiodcentered\\ {tex_escape(titel)}" + r"\\{\footnotesize\url{" + link + "}}"
    cells = f"{tex_escape(qid)} & {cell2} & {tex_escape(ereignisse)}"
    out.append(f"  \\SemioTableRow{{{cells}}}")
out.append("}")
out.append("")

print(f"% projects={len(projects)} elements={total_elements} quellen={len(quellen)}")
print("\n".join(out))
