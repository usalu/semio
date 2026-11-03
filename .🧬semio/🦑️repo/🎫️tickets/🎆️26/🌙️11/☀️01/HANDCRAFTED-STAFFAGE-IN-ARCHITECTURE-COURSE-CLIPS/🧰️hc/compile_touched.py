"""🧪 Byte-compile every ticket-touched Python file under teaching/architecture and report failures."""
import os

import sys

HERE = os.path.dirname(os.path.abspath(__file__))
LIST = os.path.join(HERE, "touched-py.txt")


def main():
    ok = bad = skipped = 0
    for line in open(LIST, encoding="utf-8"):
        path = line.strip()
        if not path:
            continue
        if not os.path.isfile(path):
            skipped += 1
            print("SKIP-missing", path)
            continue
        try:
            compile(open(path, "rb").read(), path, "exec")
            ok += 1
        except SyntaxError as exc:
            bad += 1
            print("FAIL", path, exc)
    print("ok", ok, "fail", bad, "skipped", skipped)
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
