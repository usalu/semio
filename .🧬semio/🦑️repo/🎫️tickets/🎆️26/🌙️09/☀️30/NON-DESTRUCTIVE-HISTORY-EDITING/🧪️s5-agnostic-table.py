#!/usr/bin/env python3
"""📊️ S5-AGNOSTIC: rewrites the per-crate table of `📓️s2-agnostic-report.md` § Session 5 (between the `s5-table` markers) from the
wiring census (`🗑️generated/s5-agnostic/census.json`), the runner's rows (`acceptance-results.tsv`, last row per crate) and my
classification of every non-pass (`🧪️s5-agnostic-classification.tsv`: crate, class, owner, note). A crate without a row is NOT RUN.
Usage: no arguments; prints the tally."""
import collections, json, os

T = os.path.dirname(os.path.abspath(__file__))
G = os.path.join(T, "🗑️generated", "s5-agnostic")
REPORT = os.path.join(T, "📓️s2-agnostic-report.md")
BEGIN, END = "<!-- s5-table:begin -->", "<!-- s5-table:end -->"
OWNERS = {
    "🧩️puzzle": "S5-PUZZLE",
    "🕸️dag": "S5-GRAPHS-WIRES", "🎬️sequence": "S5-GRAPHS-WIRES", "📜️imperative": "S5-GRAPHS-WIRES", "🪐️space": "S5-GRAPHS-WIRES", "💡️reasoning": "S5-GRAPHS-WIRES", "➗️mathematical": "S5-GRAPHS-WIRES",
    "🌊️flow": "S5-FLOWCAD", "📐️cad": "S5-FLOWCAD",
    "🖍️draw": "S5-TOOLS", "🗒️note": "S5-TOOLS", "📏️layout": "S5-TOOLS", "🏗️fem": "S5-TOOLS", "💠️lowpoly": "S5-TOOLS", "🎥️shooting": "S5-TOOLS", "🔋️energy": "S5-TOOLS", "📋️forms": "S5-TOOLS", "🌍️gis": "S5-TOOLS", "🌀️procedural": "S5-TOOLS", "📖️playbook": "S5-TOOLS",
    "✒️writer": "S5-TEXT-STDIO", "🔱️trinity": "S5-TEXT-STDIO", "🌿️vcs": "S5-TEXT-STDIO", "🗄️stdio": "S5-TEXT-STDIO",
    "🖨️raster": "S5-STROKES-NORM", "📸️remodel": "S5-STROKES-NORM", "🀄️wfc": "S5-STROKES-NORM", "🏭️process": "S5-STROKES-NORM", "📕️norm": "S5-STROKES-NORM",
}


def tsv(path, key):
    rows = {}
    if os.path.isfile(path):
        for line in open(path, encoding="utf-8"):
            cells = line.rstrip("\n").split("\t")
            if len(cells) > key and not line.startswith("#"):
                rows[cells[key]] = cells
    return rows


def main():
    census = json.load(open(os.path.join(G, "census.json"), encoding="utf-8"))
    results = tsv(os.path.join(G, "acceptance-results.tsv"), 1)
    classes = tsv(os.path.join(T, "🧪️s5-agnostic-classification.tsv"), 0)
    tally = collections.Counter()
    lines = ["| Plugin | Crate | Laws wired | Status | G12 / inputs / reload / child / payload ok·fail | Ran | Detail |", "|---|---|---|---|---|---|---|"]
    for row in census:
        crate = row["crate"]
        owner = OWNERS.get(row["plugin"], "main (no S5 owner)")
        result, cls = results.get(crate), classes.get(crate)
        if result is None:
            status, laws, ran, detail = "NOT RUN", "", "", (cls[3] if cls else "")
        else:
            when, _, code, kind, g12, inputs, reload, child, payload, summary = (result + [""] * 10)[:10]
            laws, ran = f"{g12} / {inputs} / {reload} / {child} / {payload.replace('/', '·')}", when[11:16]
            if kind == "PASS":
                status, detail = "PASS", summary
            elif cls:
                status, detail = f"{cls[1]} → {cls[2] or owner}", cls[3]
            elif kind == "COMPILE-RED":
                status, detail = f"COMPILE-RED → {owner}", summary
            else:
                status, detail = f"{kind} (unclassified)", summary
        tally[status.split(" → ")[0]] += 1
        detail = detail.replace("|", "¦").replace("\n", " ")
        lines.append(f"| {row['plugin']} | `{crate.replace('semio-s-artifact-', '')}` | {'+'.join(row['laws'])} | {status} | {laws} | {ran} | {detail[:420]} |")
    lines.append("")
    lines.append("Tally (" + str(len(census)) + " crates): " + ", ".join(f"{name} {count}" for name, count in sorted(tally.items())) + ".")
    text = open(REPORT, encoding="utf-8").read()
    start, end = text.index(BEGIN) + len(BEGIN), text.index(END)
    with open(REPORT, "w", encoding="utf-8") as out:
        out.write(text[:start] + "\n" + "\n".join(lines) + "\n" + text[end:])
    print(", ".join(f"{name} {count}" for name, count in sorted(tally.items())))


if __name__ == "__main__":
    main()
