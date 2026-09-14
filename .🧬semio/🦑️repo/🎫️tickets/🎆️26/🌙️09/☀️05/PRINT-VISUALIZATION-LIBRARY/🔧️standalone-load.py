#!/usr/bin/env python3
"""🧪 Compiles a minimal `\\documentclass{article}\\usepackage{semio-viz-<pkg>}` for every
`semio-viz-*.sty` under the print product and reports the packages that do not load standalone."""

import glob
import os
import re
import subprocess
import sys

LATEX = glob.glob("C:/git/semio/*framework/*products/*print/*latex")[0]
OUT = os.path.join(
    glob.glob("C:/git/semio/.*semio/*repo/*tickets/*26/*09/*05/PRINT-VISUALIZATION-LIBRARY")[0],
    "\U0001f5d1\ufe0fgenerated",
    "FINALIZE",
    "load",
)


def main() -> int:
    os.makedirs(OUT, exist_ok=True)
    fonts = glob.glob("C:/git/semio/.*semio/*repo/*cache/print-fonts")
    fonts += glob.glob("C:/git/semio/*framework/*products/*print/*assets/*font")
    env = dict(
        os.environ,
        TEXINPUTS=";".join([LATEX] + fonts) + ";",
        OSFONTDIR=";".join(fonts),
        TTFONTS=";".join(fonts) + ";",
        OPENTYPEFONTS=";".join(fonts) + ";",
    )
    only = sys.argv[1:] if len(sys.argv) > 1 else None
    packages = sorted(
        os.path.basename(p)[:-4]
        for p in glob.glob(os.path.join(LATEX, "semio-viz-*.sty"))
    )
    if only:
        packages = [p for p in packages if p in only]
    failed = []
    for pkg in packages:
        job = pkg
        tex = os.path.join(OUT, job + ".tex")
        with open(tex, "w", encoding="utf-8") as handle:
            handle.write(
                "\\documentclass{article}\n"
                "\\usepackage{" + pkg + "}\n"
                "\\begin{document}\nload\\end{document}\n"
            )
        run = subprocess.run(
            ["xelatex", "-interaction=nonstopmode", "-halt-on-error", job + ".tex"],
            cwd=OUT,
            env=env,
            capture_output=True,
            text=True,
            errors="replace",
        )
        errors = [
            line
            for line in run.stdout.splitlines()
            if line.startswith("!") and "fontspec" not in line
        ]
        state = "ok" if run.returncode == 0 else "FAIL"
        if run.returncode != 0:
            failed.append((pkg, errors[:3]))
        print(f"{state}\t{pkg}", flush=True)
    print(f"\npackages={len(packages)} failed={len(failed)}")
    for pkg, errors in failed:
        print(f"--- {pkg}")
        for line in errors:
            print("   ", line)
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
