#!/usr/bin/env python3
"""🔧️ Rewires the probe fixtures onto the kernel names INTEGRATION-1 moved them to.

The sequence-level statistics, the matrix reshape and the model-evaluation sweeps left
`semio-viz-charts-distribution` for `semio-viz-transform` under the `semio_viz_tr_` prefix, and
`\\semio_viz_stat_pie:Nnnnn` was deleted in favour of the mark kernel's `\\semio_viz_pie_from_seq:Nnnnn`
with its own result sequences. Twelve committed `.tex` fixtures still called the vanished names and
every one of their scenarios errored with `Undefined control sequence`. Idempotent.
"""
import io
import os
import sys

ROOT = "🧰️framework/🛍️products/📓️print/🧪️tests"
RENAMES = [
    ("\\semio_viz_stat_pie:Nnnnn", "\\semio_viz_pie_from_seq:Nnnnn"),
    ("g_semio_viz_stat_arc_start_seq", "g_semio_viz_pie_start_seq"),
    ("g_semio_viz_stat_arc_end_seq", "g_semio_viz_pie_end_seq"),
    ("semio_viz_stat_", "semio_viz_tr_stat_"),
    ("semio_viz_eval_", "semio_viz_tr_eval_"),
    ("semio_viz_mx_", "semio_viz_tr_matrix_"),
    ("semio_viz_corr_pearson", "semio_viz_tr_corr_pearson"),
    ("semio_viz_tr_tr_", "semio_viz_tr_"),
]
REQUIRED = {"semio_viz_tr_": "semio-viz-transform", "semio_viz_pie_": "semio-viz-mark"}


def main() -> int:
    os.chdir(sys.argv[1] if len(sys.argv) > 1 else "C:/git/semio")
    changed = 0
    for case in sorted(os.listdir(ROOT)):
        fixtures = os.path.join(ROOT, case, "🧫️fixtures")
        if not os.path.isdir(fixtures):
            continue
        for name in sorted(os.listdir(fixtures)):
            if not name.endswith(".tex"):
                continue
            path = os.path.join(fixtures, name)
            text = io.open(path, encoding="utf8").read()
            rewritten = text
            for old, new in RENAMES:
                rewritten = rewritten.replace(old, new)
            for prefix, package in REQUIRED.items():
                if prefix in rewritten and f"\\usepackage{{{package}}}" not in rewritten:
                    rewritten = rewritten.replace("\\begin{document}", f"\\usepackage{{{package}}}\n\\begin{{document}}", 1)
            if rewritten != text:
                io.open(path, "w", encoding="utf8", newline="\n").write(rewritten)
                print(f"{case}/{name}")
                changed += 1
    print(f"total {changed}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
