import re
import string
import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
SKIP = {".venv", "site-packages", "__pycache__", "🔊️speech", "🧪️tests"}
STR = re.compile(r"""(?P<p>rf|fr|r|f)?(?P<q>"|')(?P<b>(?:\\.|(?!(?P=q)).)*)(?P=q)""")


def check(prefix, body):
    if "frac" not in body:
        return True
    if prefix and "f" in prefix:
        try:
            body = "".join(lit for lit, *_ in string.Formatter().parse(body))
        except ValueError:
            return True
    return body.count("{") == body.count("}")


def main():
    bad = 0
    for p in ROOT.rglob("🐍️.py"):
        if set(p.parts) & SKIP:
            continue
        for i, line in enumerate(p.read_text(encoding="utf-8").splitlines(), 1):
            if "\\frac" not in line:
                continue
            for m in STR.finditer(line):
                if not check(m.group("p"), m.group("b")):
                    print(f"{p.relative_to(ROOT)}:{i}: {m.group(0)[:200]}")
                    bad += 1
    print("unbalanced", bad)


main()
