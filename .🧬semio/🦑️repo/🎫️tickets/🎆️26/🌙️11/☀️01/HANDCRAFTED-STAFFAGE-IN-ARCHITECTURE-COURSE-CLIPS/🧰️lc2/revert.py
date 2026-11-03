import ast
import glob
import os
import re
from pathlib import Path

ROOT = "🎓️teaching/🏛️architecture/⚡️energy"
BACK = [
    (r"\text{occ}", r"\text{Pers}"), (r"\text{{occ}}", r"\text{{Pers}}"),
    (r"\text{equip}", r"\text{Geräte}"), (r"\text{light}", r"\text{Licht}"),
    (r"\theta_{room}", r"\theta_{Raum}"),
    (r"\mathrm{loss}", r"\mathrm{Verlust}"), (r"\mathrm{{loss}}", r"\mathrm{{Verlust}}"),
    (r"\mathrm{storage}", r"\mathrm{Speicher}"),
    (r"\mathrm{fresh}", r"\mathrm{frisch}"), (r"\mathrm{{fresh}}", r"\mathrm{{frisch}}"),
]


def main():
    for path in sorted(glob.glob(f"{ROOT}/*/🎬️clip/*/🐍️.py")):
        if "summer-strategy" in path or "🏗️" in path or "🧪️tests" in path:
            continue
        src = Path(path).read_bytes().decode("utf-8")
        tree = ast.parse(src)
        lines = src.split("\n")
        edits = []
        for n in ast.walk(tree):
            if not (isinstance(n, ast.Call) and isinstance(n.func, ast.Name) and n.func.id == "tr" and len(n.args) == 2):
                continue
            a, b = n.args
            if n.lineno != n.end_lineno or a.lineno != a.end_lineno or b.lineno != b.end_lineno or a.lineno != b.lineno:
                continue
            ln = lines[n.lineno - 1].encode("utf-8")
            sa = ln[a.col_offset:a.end_col_offset].decode("utf-8")
            sb = ln[b.col_offset:b.end_col_offset].decode("utf-8")
            back = sa
            for x, y in BACK:
                back = back.replace(x, y)
            if back == sb:
                edits.append((n.lineno - 1, n.col_offset, n.end_col_offset, sb))
        if not edits:
            continue
        for l, c0, c1, new in sorted(edits, reverse=True):
            b = lines[l].encode("utf-8")
            lines[l] = (b[:c0] + new.encode("utf-8") + b[c1:]).decode("utf-8")
        out = "\n".join(lines)
        ast.parse(out)
        tmp = path + ".lc2tmp"
        Path(tmp).write_bytes(out.encode("utf-8"))
        os.replace(tmp, path)
        print(len(edits), path.split("clip")[-1])


main()
