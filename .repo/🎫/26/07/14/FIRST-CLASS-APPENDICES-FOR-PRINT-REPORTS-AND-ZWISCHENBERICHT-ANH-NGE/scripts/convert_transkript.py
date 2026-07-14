#!/usr/bin/env python3
"""One-off conversion of temp/Anhang_Interviewtranskript_Raoul_Bunschoten_DE.md
into LaTeX for the zwischenbericht Anlage G appendix. Throwaway."""
import re

SRC = "/Users/ueli/Documents/semio/temp/Anhang_Interviewtranskript_Raoul_Bunschoten_DE.md"

with open(SRC, encoding="utf-8") as f:
    text = f.read()

def tex_escape(s):
    s = s.replace("&", r"\&")
    s = s.replace("%", r"\%")
    s = s.replace("#", r"\#")
    return s

def inline(s):
    # *italic* -> \textit{...} (single asterisk, not preceded/followed by another asterisk)
    s = tex_escape(s)
    s = re.sub(r"\*([^*]+)\*", lambda m: r"\textit{" + m.group(1) + "}", s)
    return s

lines = text.split("\n")

# ---- info table ----
info = {}
in_info = False
for ln in lines:
    if ln.startswith("## Interviewinformationen"):
        in_info = True
        continue
    if in_info:
        if ln.startswith("## "):
            break
        m = re.match(r"^\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|$", ln)
        if m and m.group(1) != "Feld" and not m.group(1).startswith("---"):
            info[m.group(1).strip()] = m.group(2).strip()

# ---- redaktioneller hinweis ----
hinweis = ""
in_hinweis = False
for ln in lines:
    if ln.startswith("## Redaktioneller Hinweis"):
        in_hinweis = True
        continue
    if in_hinweis:
        if ln.startswith("## ") or ln.strip() == "---":
            if hinweis:
                break
            continue
        if ln.strip():
            hinweis = ln.strip()
            break

# ---- sections ----
body_start = None
for i, ln in enumerate(lines):
    if re.match(r"^## \d+\.", ln):
        body_start = i
        break

sections = []
current = None
for ln in lines[body_start:]:
    h2 = re.match(r"^## \d+\.\s*(.+)$", ln)
    if h2:
        current = {"title": h2.group(1).strip(), "blocks": []}
        sections.append(current)
        continue
    if current is None:
        continue
    speaker = re.match(r"^\*\*([A-ZÄÖÜ .]+):\*\*\s*$", ln)
    if speaker:
        current["blocks"].append(("speaker", speaker.group(1).strip()))
        continue
    stage = re.match(r"^\*([^*]+)\*\s*$", ln)
    if stage and not ln.strip().startswith("**"):
        current["blocks"].append(("stage", stage.group(1).strip()))
        continue
    if ln.strip() == "" or ln.strip() == "---":
        continue
    current["blocks"].append(("text", ln.strip()))

# ---- emit ----
out = []
out.append(r"\label{anlage:transkript-bunschoten}")
out.append("")
out.append(r"\begin{Table}[title=Interviewinformationen]")
out.append(r"\begin{SemioTable}{@{}T{0.3\linewidth-2\tabcolsep}T{0.7\linewidth-2\tabcolsep}@{}}")
for key in ["Interviewpartner", "Interviewer", "Datum", "Ort / Format", "Sprache", "Transkripttyp"]:
    out.append(f"\\SemioTableRow{{{tex_escape(key)} & {tex_escape(info[key])}}}")
out.append(r"\end{SemioTable}")
out.append(r"\end{Table}")
out.append("")
out.append(inline(hinweis))
out.append("")

speaker_label = {
    "INTERVIEWER": "Interviewer",
    "RAOUL BUNSCHOTEN": "Raoul Bunschoten",
}

for sec in sections:
    out.append(f"\\subsection{{{tex_escape(sec['title'])}}}")
    out.append("")
    pending_speaker = None
    for kind, val in sec["blocks"]:
        if kind == "speaker":
            pending_speaker = val
        elif kind == "stage":
            out.append(f"\\textit{{{inline(val)}}}")
            out.append("")
            pending_speaker = None
        elif kind == "text":
            label = speaker_label.get(pending_speaker, pending_speaker)
            if label:
                out.append(f"\\textbf{{{label}:}} {inline(val)}")
                pending_speaker = None
            else:
                out.append(inline(val))
            out.append("")

print(f"% info={len(info)} sections={len(sections)}")
print("\n".join(out))
