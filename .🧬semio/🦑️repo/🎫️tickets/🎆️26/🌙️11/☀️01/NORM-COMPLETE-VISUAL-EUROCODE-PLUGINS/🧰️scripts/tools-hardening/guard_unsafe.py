"""🛡️ Inserts the unsafe-run guard into migration scripts that already ran (rename, retire, split, bridge wiring ...): without TH_FORCE=1 they exit 2 before touching anything.

usage: python3 guard_unsafe.py [--check] SCRIPT...   (paths relative to the scripts folder; idempotent, additive: one line is inserted after the module docstring, nothing else changes)"""
import ast, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
SCRIPTS = os.path.dirname(HERE)
MARK = "# unsafe-guard"
PY = 'import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\\n"), _s.exit(2))  ' + MARK
SH = '[ -n "$TH_FORCE" ] || { echo "unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)" >&2; exit 2; }  ' + MARK


def guarded(path):
    src = open(path, encoding="utf-8").read()
    if MARK in src:
        return src
    lines = src.split("\n")
    if path.endswith(".sh"):
        at = 1 if lines[0].startswith("#!") else 0
        return "\n".join(lines[:at] + [SH] + lines[at:])
    tree = ast.parse(src)
    at = 0
    for node in tree.body:
        if isinstance(node, ast.Expr) and isinstance(node.value, ast.Constant) and isinstance(node.value.value, str) and node is tree.body[0]:
            at = node.end_lineno
        elif isinstance(node, ast.ImportFrom) and node.module == "__future__":
            at = node.end_lineno
        else:
            break
    if at == 0 and lines[0].startswith("#!"):
        at = 1
    return "\n".join(lines[:at] + [PY] + lines[at:])


def main():
    check = "--check" in sys.argv
    changed = 0
    for rel in [a for a in sys.argv[1:] if not a.startswith("--")]:
        path = os.path.join(SCRIPTS, rel)
        new = guarded(path)
        if new != open(path, encoding="utf-8").read():
            changed += 1
            print("guard" if not check else "would guard", rel)
            if not check:
                open(path, "w", encoding="utf-8").write(new)
    sys.exit(1 if check and changed else 0)


if __name__ == "__main__":
    main()
