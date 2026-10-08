"""🧱 Expands the probe scenarios into literal paragraphs, because the print paragraph hook scans source text up to a blank line."""
import sys

def lines(count, label):
    return "\n\n".join(f"Zeile {index} {label}." for index in range(1, count + 1)) + "\n\n"

def window(title, body, options=""):
    return f"\\begin{{Figure}}[title={title}{options}]\n{body}\\end{{Figure}}\n\nText danach.\n\n\\clearpage\n"

def table(title, rows):
    body = "".join(f"  \\SemioTableRow{{Risiko {index} mit einer laengeren Beschreibung ueber mehrere Zeilen & aktiv & beide Partner & Massnahme {index} mit Zustaendigkeiten und Terminen & Restrisiko {index}}}\n" for index in range(1, rows + 1))
    return f"\\begin{{Table}}[title={title}]\n\\begin{{SemioTable}}{{0.20,0.10,0.18,0.27,0.25}}\n  \\SemioTableHeaderRow{{Risiko & Status & Verantwortung & Massnahme & Restrisiko}}\n{body}\\end{{SemioTable}}\n\\end{{Table}}\n\nText danach.\n\n\\clearpage\n"

image = "\\rule{\\linewidth}{90mm}\n"
scenarios = [
    ("Opt Out", 38, window("Opt Out Figure", image, ", break=false")),
    ("Default", 38, window("Default Figure", image)),
    ("Text", 36, window("Text Figure", lines(12, "im Fenster"))),
    ("Tall", 20, window("Tall Figure", lines(70, "im Fenster"))),
    ("Tall Image", 38, window("Tall Image Figure", image + lines(60, "im Fenster"))),
    ("Table", 41, table("Risiken und Massnahmen", 3)),
    ("Tall Table", 30, table("Lange Tabelle", 30)),
]
body = "\\begin{document}\n\\makecoverpages\n\\maketableofcontents\n\\makemainmatter\n\\part{Ergebnisse}\n\\label{teil:ergebnisse}\n\n"
for heading, before, block in scenarios:
    body += f"\\section{{{heading}}}\n" + lines(before, "davor") + block
sys.stdout.write(body + "\\end{document}\n")
