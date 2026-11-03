import ast
import glob
import os
import re
import sys
from pathlib import Path

ROOT = "🎓️teaching/🏛️architecture/⚡️energy"
SKIP = ("summer-strategy", "🏗️authoring")


def main():
    for path in sorted(glob.glob(f"{ROOT}/*/🎬️clip/*/🐍️.py")):
        if any(s in path for s in SKIP) or "🧪️tests" in path:
            continue
        src = Path(path).read_bytes().decode("utf-8")
        if "de_num" not in src:
            continue
        out = src.replace("de_num(", "label_number(")
        has_ln = re.search(r"^label_number = ", out, re.M)
        if has_ln:
            out = re.sub(r"^de_num = _manim_module\.de_num\r?\n", "", out, flags=re.M)
        else:
            out = out.replace("de_num = _manim_module.de_num", "label_number = _manim_module.label_number")
        if "de_num" in out:
            print("LEFT", path, [m.start() for m in re.finditer("de_num", out)][:3])
        ast.parse(out)
        tmp = path + ".lc2tmp"
        Path(tmp).write_bytes(out.encode("utf-8"))
        os.replace(tmp, path)
        print("ok", path)


main()
