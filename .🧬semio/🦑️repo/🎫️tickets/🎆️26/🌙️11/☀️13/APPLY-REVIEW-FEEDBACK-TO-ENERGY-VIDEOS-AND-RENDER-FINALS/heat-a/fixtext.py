import re
import sys

for path in sys.argv[1:]:
    s = open(path, encoding="utf-8").read()
    o = s
    for lb, rb in (("{", "}"), ("{{", "}}")):
        l, r = re.escape(lb), re.escape(rb)
        s = re.sub(r"\\text" + l + r" ([^{}]*?)" + r, lambda m: r"\;\text" + lb + m.group(1) + rb, s)
        s = re.sub(r"\\text" + l + r"([^{}]*?) " + r, lambda m: r"\text" + lb + m.group(1) + rb + r"\;", s)
    if s != o:
        open(path, "w", encoding="utf-8", newline="").write(s)
        print("fixed", path)
