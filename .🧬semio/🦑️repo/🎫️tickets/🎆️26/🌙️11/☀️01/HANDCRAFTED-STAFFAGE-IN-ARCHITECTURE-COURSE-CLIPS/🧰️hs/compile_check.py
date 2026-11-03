import io
import os
import py_compile
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
LIST = os.path.join(HERE, "files.txt")


def main():
    failures = 0
    with io.open(LIST, encoding="utf-8") as handle:
        paths = [line.strip() for line in handle if line.strip().endswith(".py")]
    for rel in paths:
        if not os.path.exists(rel):
            continue
        try:
            py_compile.compile(rel, cfile=os.path.join(HERE, "compiled.pyc"), doraise=True)
        except py_compile.PyCompileError as error:
            failures += 1
            print("FAIL", rel, str(error)[:300])
    print("checked", len(paths), "failures", failures)
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
