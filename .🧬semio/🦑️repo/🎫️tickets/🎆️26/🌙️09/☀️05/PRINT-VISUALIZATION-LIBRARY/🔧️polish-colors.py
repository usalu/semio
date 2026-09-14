#!/usr/bin/env python3
"""🎨 Routes the direct brand-colour uses of the POLISH-owned renderers through semio-viz-theme.

Categorical brand names become the theme's own categorical slots, semantic names become the
theme's semantic roles, so the grayscale and pattern themes reach every renderer. Writes with a
retry loop because a concurrent tectonic run holds the packages open.
"""
import sys
import time
from pathlib import Path

MAP = [
    ("semio-primary", "\\semio_viz_theme_color:n { 0 }"),
    ("semio-secondary", "\\semio_viz_theme_color:n { 1 }"),
    ("semio-tertiary", "\\semio_viz_theme_color:n { 2 }"),
    ("semio-danger", "\\semio_viz_theme_semantic:n { negative }"),
    ("semio-success", "\\semio_viz_theme_semantic:n { positive }"),
    ("semio-warning", "\\semio_viz_theme_semantic:n { caution }"),
    ("semio-info", "\\semio_viz_theme_semantic:n { informational }"),
]

FILES = """semio-viz-annotation.sty semio-viz-charts-area.sty semio-viz-charts-bar.sty
semio-viz-charts-dashboard.sty semio-viz-charts-financial.sty semio-viz-charts-line.sty
semio-viz-charts-scatter.sty semio-viz-charts-timeline.sty semio-viz-flow-sankey.sty
semio-viz-geo-map.sty semio-viz-geo-projection.sty semio-viz-geo-choropleth.sty
semio-viz-geo-symbols.sty semio-viz-geo-routes.sty""".split()


def write_retry(path: Path, text: str) -> None:
    for _ in range(240):
        try:
            path.write_text(text, encoding="utf-8", newline="\n")
            return
        except OSError:
            time.sleep(0.5)
    raise SystemExit(f"{path.name}: still locked")


def main() -> None:
    root = Path(sys.argv[1])
    for name in FILES:
        path = root / name
        text = path.read_text(encoding="utf-8")
        hits = 0
        for old, new in MAP:
            hits += text.count(old)
            text = text.replace(old, new)
        write_retry(path, text)
        print(f"{name}: {hits}")


main()
