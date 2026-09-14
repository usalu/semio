#!/usr/bin/env python3
"""🗂️ DOCS: renders the taxonomy §76 namespace chapter of 🔓️viz-api.tex from the schema's
`x-semio-family-options` — one section per namespace, one key table per family, bilingual.

Usage: python 🔧️docs-namespaces.py            → prints the chapter to stdout
       python 🔧️docs-namespaces.py --splice   → splices it into the document before \\end{document}
"""
import io
import json
import os
import sys

PRINT = r"C:\git\semio\🧰️framework\🛍️products\📓️print"
SCHEMA = os.path.join(PRINT, "🧬️schema", "🔣️.json")
DOC = os.path.join(PRINT, "🧾️template", "📊️viz-api", "🔓️viz-api.tex")
MARKER = "%region 🔖️GeneratedNamespaces"

# 🗂️ Taxonomy §76 namespaces, in the order of architecture §2, with the owning agent of each.
NAMESPACES = [
    ("kernel", {"GRAMMAR-CORE", "SHAPES", "GUIDES", "CATALOG"},
     ("Kernel", "Kern"),
     ("The grammar itself: the families that expose a kernel capability — a scale, an axis, a mark, a "
      "shape, a layout algorithm — as a chart kind of its own, so the catalogue covers taxonomy "
      "sections 74 to 79 the same way it covers a bar chart.",
      "Die Grammatik selbst: die Familien, die eine Kernfähigkeit — eine Skala, eine Achse, eine Marke, "
      "eine Form, einen Layoutalgorithmus — als eigene Diagrammart anbieten, damit der Katalog die "
      "Taxonomieabschnitte 74 bis 79 genauso abdeckt wie ein Balkendiagramm.")),
    ("charts", {"CHARTS-A", "CHARTS-B"},
     ("Charts", "Diagramme"),
     ("The cartesian and polar chart namespaces: bar, line, area, scatter, financial, timeline, "
      "dashboard, funnel, distribution, statistical, polar, matrix and table.",
      "Die kartesischen und polaren Diagrammnamensräume: Balken, Linie, Fläche, Streuung, Finanzen, "
      "Zeitstrahl, Dashboard, Trichter, Verteilung, Statistik, Polar, Matrix und Tabelle.")),
    ("hierarchy", {"HIERARCHY"},
     ("Hierarchy", "Hierarchie"),
     ("Trees, dendrograms, treemaps, partitions and circle packings — everything that reads a "
      "parent-linked table.",
      "Bäume, Dendrogramme, Kacheldiagramme, Partitionen und Kreispackungen — alles, was eine "
      "elternverknüpfte Tabelle liest.")),
    ("network", {"NETWORK-FLOW"},
     ("Network and flow", "Netzwerk und Fluss"),
     ("Graphs, adjacency matrices, arc and chord diagrams, edge bundling, and the flow namespaces "
      "sankey, alluvial and parallel sets.",
      "Graphen, Adjazenzmatrizen, Bogen- und Sehnendiagramme, Kantenbündelung sowie die "
      "Flussnamensräume Sankey, Alluvial und Parallelmengen.")),
    ("geo", {"GEO-SPATIAL"},
     ("Geo and spatial", "Geo und Raum"),
     ("Maps, projections, choropleths, symbol maps, contours and routes, plus the planar spatial "
      "kernel: voronoi, delaunay, hull, hexbin, contour and density.",
      "Karten, Projektionen, Choroplethen, Symbolkarten, Höhenlinien und Routen sowie der ebene "
      "Raumkern: Voronoi, Delaunay, Hülle, Hexbin, Höhenlinie und Dichte.")),
    ("diagram", {"DIAGRAMS"},
     ("Diagram", "Diagramm"),
     ("Flowcharts, UML, architecture, process, concept diagrams, infographics and interaction states.",
      "Flussdiagramme, UML, Architektur, Prozess, Begriffsdiagramme, Infografiken und "
      "Interaktionszustände.")),
    ("scientific", {"SCIENTIFIC"},
     ("Scientific", "Wissenschaft"),
     ("Fields, surfaces, signals, physics, chemistry, biology, engineering, mathematics, geometry and "
      "three-dimensional projection.",
      "Felder, Flächen, Signale, Physik, Chemie, Biologie, Ingenieurwesen, Mathematik, Geometrie und "
      "dreidimensionale Projektion.")),
]

SPECIAL = {"\\": r"\textbackslash{}", "&": r"\&", "%": r"\%", "$": r"\$", "#": r"\#",
           "_": r"\_", "{": r"\{", "}": r"\}", "~": r"\textasciitilde{}", "^": r"\textasciicircum{}",
           # 🔤 The document fonts carry no mathematical relations; spell them out.
           "≤": "<=", "≥": ">=", "≠": "!=", "×": "x"}


def tex(value: str) -> str:
    """🔤 Escapes a schema string for LaTeX text; backticks become typewriter quotes."""
    out = []
    for ch in str(value):
        out.append(SPECIAL.get(ch, ch))
    return "".join(out).replace("`", "'")


def localized(entry, field: str) -> tuple[str, str]:
    value = entry.get(field)
    if isinstance(value, dict):
        return tex(value.get("en", "")), tex(value.get("de", ""))
    return tex(value or ""), tex(value or "")


def key_rows(options: dict) -> list[str]:
    rows = []
    for name, spec in options.items():
        en, de = localized(spec, "description")
        kind = tex(spec.get("type", ""))
        default = spec.get("default")
        default = "\\Key{%s}" % tex(default) if default not in (None, "") else "---"
        rows.append(
            "  \\SemioTableRow{\\Key{%s} & \\Key{%s} & %s & \\ApiText{%s}{%s}}"
            % (tex(name), kind, default, en or de, de or en))
    return rows


def render() -> str:
    schema = json.load(io.open(SCHEMA, encoding="utf8"))
    families = schema["x-semio-family-options"]
    lines = [MARKER, ""]
    lines.append("\\chapter{\\ApiText{The namespaces}{Die Namensräume}}")
    lines.append("")
    lines.append("\\ApiText{%")
    lines.append("  Taxonomy section 76 divides the library into package namespaces, and every chart kind of")
    lines.append("  the catalogue belongs to exactly one family of exactly one namespace. A family is a")
    lines.append("  renderer registered with \\Cs{SemioVizFamily}; its options are l3keys in")
    lines.append("  \\Key{semio / viz / family / <name>} and are listed below with their type, their default")
    lines.append("  and their meaning. Two options are universal and therefore not repeated in every table:")
    lines.append("  \\Key{variant} is the catalogue slug the family is rendering, and \\Key{data} names the")
    lines.append("  table it reads.%")
    lines.append("}{%")
    lines.append("  Taxonomieabschnitt 76 teilt die Bibliothek in Paketnamensräume, und jede Diagrammart des")
    lines.append("  Katalogs gehört zu genau einer Familie genau eines Namensraums. Eine Familie ist ein mit")
    lines.append("  \\Cs{SemioVizFamily} registrierter Renderer; ihre Optionen sind l3keys in")
    lines.append("  \\Key{semio / viz / family / <Name>} und stehen unten mit Typ, Vorgabe und Bedeutung. Zwei")
    lines.append("  Optionen sind allgemein und werden deshalb nicht in jeder Tabelle wiederholt:")
    lines.append("  \\Key{variant} ist der Katalog-Slug, den die Familie gerade zeichnet, und \\Key{data}")
    lines.append("  benennt die Tabelle, die sie liest.%")
    lines.append("}")
    lines.append("")

    seen = set()
    for _slug, owners, (title_en, title_de), (intro_en, intro_de) in NAMESPACES:
        names = sorted(name for name, spec in families.items() if spec.get("owner") in owners)
        if not names:
            continue
        seen.update(names)
        lines.append("\\section{\\ApiText{%s}{%s}}" % (tex(title_en), tex(title_de)))
        lines.append("")
        lines.append("\\ApiText{%s}{%s}" % (tex(intro_en), tex(intro_de)))
        lines.append("")
        for name in names:
            options = {k: v for k, v in families[name].get("options", {}).items()
                       if k not in ("variant", "data")}
            lines.append("\\subsection{\\Key{%s}}" % tex(name))
            if not options:
                lines.append("")
                lines.append("\\ApiText{This family takes no options beyond \\Key{variant} and \\Key{data}.}"
                             "{Diese Familie nimmt außer \\Key{variant} und \\Key{data} keine Optionen.}")
                lines.append("")
                continue
            lines.append("")
            lines.append("\\SemioTableLong[text-size=8pt]{\\Key{%s}}{0.20,0.13,0.15,0.52}"
                         "{\\ApiKey & \\ApiType & \\ApiDefault & \\ApiMeaning}{%%" % tex(name))
            lines.extend(key_rows(options))
            lines.append("}")
            lines.append("")
    lines.append("%endregion 🔖️GeneratedNamespaces")
    lines.append("")
    missing = sorted(set(families) - seen)
    if missing:
        sys.stderr.write("[DEBUG] families with an unmapped owner: %s\n" % " ".join(missing))
    return "\n".join(lines)


def main() -> None:
    chapter = render()
    if "--splice" not in sys.argv:
        sys.stdout.write(chapter)
        return
    source = io.open(DOC, encoding="utf8").read()
    if MARKER in source:
        head, rest = source.split(MARKER, 1)
        rest = rest.split("%endregion 🔖️GeneratedNamespaces", 1)[1]
        source = head + chapter.split(MARKER, 1)[1].rsplit("%endregion 🔖️GeneratedNamespaces", 1)[0] + rest
        source = head + chapter + rest.lstrip("\n")
    else:
        source = source.replace("\\end{document}", chapter + "\n\\end{document}")
    # 🪟 Windows rejects an absolute path with emoji segments for writing; a relative name works.
    os.chdir(os.path.dirname(DOC))
    io.open(os.path.basename(DOC), "w", encoding="utf8", newline="\n").write(source)
    sys.stderr.write("[DEBUG] spliced %d lines into %s\n" % (chapter.count("\n"), DOC))


main()
