#!/usr/bin/env python3
"""🔎 Inventory of consistency findings across the POLISH-owned LaTeX viz packages."""
import os
import re
import sys
import json

LATEX = "C:/git/semio/\U0001f9f0\ufe0fframework/\U0001f6cd\ufe0fproducts/\U0001f4d3\ufe0fprint/\U0001f58b\ufe0flatex"

OWNED = """data transform scale format theme mark shape coordinate guide annotation label facet
composition plot family probe layout hierarchy network flow geo spatial
charts-area charts-bar charts-dashboard charts-distribution charts-financial charts-funnel
charts-line charts-polar charts-scatter charts-statistical charts-timeline
matrix-adjacency matrix-correlation matrix-heatmap table
hierarchy-tree hierarchy-dendrogram hierarchy-treemap hierarchy-partition hierarchy-pack
flow-sankey flow-alluvial flow-parallelsets
geo-map geo-projection geo-choropleth geo-symbols geo-contours geo-routes""".split()

FILES = ["semio-viz.sty"] + ["semio-viz-%s.sty" % n for n in OWNED]

DEF = re.compile(
    r"^\s*\\(cs_new[a-z_]*:(?:Npn|Nn|Nx|cpn|cn)|cs_set[a-z_]*:(?:Npn|Nn|Nx|cpn|cn)"
    r"|NewDocumentCommand|NewDocumentEnvironment|DeclareDocumentCommand"
    r"|keys_define:nn|SemioVizFamily|semio_viz_family_define:nn"
    r"|prg_new_[a-z_]*:[A-Za-z]+|cs_generate_variant:Nn|tl_new:N|seq_new:N|prop_new:N"
    r"|fp_new:N|int_new:N|bool_new:N|clist_new:N|box_new:N|dim_new:N"
    r"|msg_new:nnn+|semio_viz_layout_define:nn)\b"
)
DEFHEAD = re.compile(
    r"^\\(cs_new[a-z_]*:(?:Npn|Nn|Nx|cpn|cn)|cs_set[a-z_]*:(?:Npn|Nn|Nx|cpn|cn)"
    r"|NewDocumentCommand|NewDocumentEnvironment|DeclareDocumentCommand"
    r"|keys_define:nn|SemioVizFamily|semio_viz_family_define:nn"
    r"|prg_new_[a-z_]*:[A-Za-z]+|semio_viz_layout_define:nn)\b"
)
EMOJI = re.compile("[\U0001F000-\U0001FAFF\u2190-\u21FF\u2300-\u27BF\u2B00-\u2BFF\uFE0F\u3030\u203C\u2049]")

SKIPPABLE = re.compile(
    r"^\\(tl|seq|prop|fp|int|bool|clist|box|dim|str|skip|muskip|coffin)_(new|const):[Nc]\b"
    r"|^\\cs_generate_variant:Nn\b|^\\prg_generate_conditional_variant:Nnn\b"
)


def documented(lines, index):
    """📝 True when the definition on line `index` is covered by an emoji docstring above it.

    A docstring may run over several lines and only its first line carries the emoji, so the
    comment block is walked upwards. The walk also steps over what does not interrupt a docstring's
    reach: blank lines, variable declarations and `\\cs_generate_variant:Nn` lines. A `%region`
    marker is not a docstring and ends the walk, and so does another definition — every definition
    carries its own note.
    """
    cursor = index - 1
    while cursor >= 0:
        text = lines[cursor].strip()
        if text.startswith("%region") or text.startswith("%endregion"):
            return False
        if text.startswith("%"):
            if EMOJI.search(text):
                return True
            cursor -= 1
            continue
        if text == "" or SKIPPABLE.match(text):
            cursor -= 1
            continue
        return False
    return False


def main():
    report = {}
    for fn in FILES:
        p = os.path.join(LATEX, fn)
        if not os.path.exists(p):
            report[fn] = {"missing": True}
            continue
        lines = open(p, encoding="utf-8").read().split("\n")
        missing_doc = []
        for i, ln in enumerate(lines):
            if DEFHEAD.match(ln):
                if not documented(lines, i):
                    missing_doc.append((i + 1, ln.strip()[:70]))
        # regions
        opens = [(i + 1, ln.strip()) for i, ln in enumerate(lines) if ln.strip().startswith("%region")]
        closes = [(i + 1, ln.strip()) for i, ln in enumerate(lines) if ln.strip().startswith("%endregion")]
        bad_region = [r for _, r in opens + closes if "\U0001f516" not in r]
        legacy = [(i + 1, ln.strip()[:80]) for i, ln in enumerate(lines) if re.search(r"\\semio@(?!stroke@)", ln)]
        colors = [(i + 1, ln.strip()[:80]) for i, ln in enumerate(lines)
                  if re.search(r"semio-(primary|secondary|tertiary|accent|success|warning|danger|info|neutral)", ln)]
        report[fn] = {
            "lines": len(lines),
            "missing_doc": len(missing_doc),
            "missing_doc_first": missing_doc[:5],
            "regions": [len(opens), len(closes)],
            "bad_region_names": len(bad_region),
            "legacy_semio_at": len(legacy),
            "legacy_first": legacy[:6],
            "direct_colors": len(colors),
            "color_first": colors[:4],
        }
    print(json.dumps(report, indent=1, ensure_ascii=False))

main()
