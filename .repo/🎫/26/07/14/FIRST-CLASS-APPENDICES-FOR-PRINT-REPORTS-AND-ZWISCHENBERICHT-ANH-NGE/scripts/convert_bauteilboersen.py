#!/usr/bin/env python3
"""One-off conversion of temp/anhang_Bauteilboersen.md into LaTeX for the
zwischenbericht Anlage C appendix. Throwaway — lives only in the ticket folder."""
import re

SRC = "/Users/ueli/Documents/semio/temp/anhang_Bauteilboersen.md"

with open(SRC, encoding="utf-8") as f:
    lines = f.readlines()

def split_row(line):
    line = line.rstrip("\n")
    parts = line.split("|")[1:]
    if parts and parts[-1].strip() == "":
        parts = parts[:-1]
    return [p.strip() for p in parts]

def tex_escape(s):
    s = s.replace("&", r"\&")
    s = s.replace("%", r"\%")
    s = s.replace("#", r"\#")
    s = s.replace("_", r"\_")
    return s

def convert_cell(s):
    parts = re.split(r"\*\*(.+?)\*\*", s)
    return "".join(
        (r"\textbf{" + tex_escape(part) + "}") if i % 2 else tex_escape(part)
        for i, part in enumerate(parts)
    )

GROUPS = [
    ("Marktplätze", "marktplaetze"),
    ("Depots / Shops", "depots"),
    ("Vermittlung / Kontakt", "vermittlung"),
    ("Ressourcenkataloge", "ressourcenkataloge"),
]

sections = {}
current_group = None
for ln in lines:
    h2 = re.match(r"^## (.+)$", ln)
    if h2:
        title = h2.group(1).strip()
        current_group = title if any(title == g[0] for g in GROUPS) else None
        if current_group:
            sections[current_group] = []
        continue
    if current_group and re.match(r"^\|\s*\[P\d+\]", ln):
        sections[current_group].append(split_row(ln))

colspec_a = (
    r"|@{\hspace{\semio@chrome@padding}}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.07\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.28\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.24\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.13\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.15\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.13\semio@table@long@inner@w\relax}"
    r"@{}@{\hspace{\semio@chrome@padding}}|"
)
header_a = r"ID & Plattform \textperiodcentered\ Land & Angebotsform & Zugang & Digitaler Kanal & Physischer Zugang"

colspec_b = (
    r"|@{\hspace{\semio@chrome@padding}}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.07\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.45\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.22\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.14\semio@table@long@inner@w\relax}"
    r"@{}>{\raggedright\arraybackslash\hspace{0pt}}p{\dimexpr0.12\semio@table@long@inner@w\relax}"
    r"@{}@{\hspace{\semio@chrome@padding}}|"
)
header_b = "ID & Angebotsbezogene Datenfelder & Such- und Auswahlfunktionen & Beschaffungsschritte & Übergabe"

out = []
for title, slug in GROUPS:
    rows = sections.get(title, [])
    if not rows:
        continue
    out.append(f"\\subsection{{{title}}}")
    out.append("")
    out.append(f"{len(rows)} Fälle.")
    out.append("")

    out.append(f"\\SemioTableLong{{{title} \\textperiodcentered\\ Zugang und Kanäle}}{{%")
    out.append(f"  {colspec_a}%")
    out.append(f"}}{{{header_a}}}{{%")
    for row in rows:
        pid, platt, form, felder, zugang, kanal, physisch, such, beschaffung, uebergabe = row[:10]
        cells = " & ".join(convert_cell(c) for c in (pid, platt, form, zugang, kanal, physisch))
        out.append(f"  \\SemioTableRow{{{cells}}}")
    out.append("}")
    out.append("")

    out.append(f"\\SemioTableLong{{{title} \\textperiodcentered\\ Datenfelder und Beschaffung}}{{%")
    out.append(f"  {colspec_b}%")
    out.append(f"}}{{{header_b}}}{{%")
    for row in rows:
        pid, platt, form, felder, zugang, kanal, physisch, such, beschaffung, uebergabe = row[:10]
        cells = " & ".join(convert_cell(c) for c in (pid, felder, such, beschaffung, uebergabe))
        out.append(f"  \\SemioTableRow{{{cells}}}")
    out.append("}")
    out.append("")

print("\n".join(out))
