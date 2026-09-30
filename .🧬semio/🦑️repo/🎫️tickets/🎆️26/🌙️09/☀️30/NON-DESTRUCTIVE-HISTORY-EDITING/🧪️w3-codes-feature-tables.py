"""📐️ W3-CODES: keeps Gherkin table columns aligned after a code rename. For every table row that differs from the
`HEAD` revision only in ASCII cell text, the renamed cells are re-padded so each `|` sits where it sat before."""
import pathlib
import subprocess
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")


def repad(old, new):
    if not (new.strip().startswith("|") and old.strip().startswith("|")) or old.count("|") != new.count("|"):
        return new
    old_cells, new_cells = old.rstrip("\n").split("|"), new.rstrip("\n").split("|")
    out = []
    for before, after in zip(old_cells, new_cells):
        if before == after or not after.strip() or len(after.strip()) + 2 > len(before):
            out.append(after)
        else:
            out.append(" " + after.strip() + " " * (len(before) - len(after.strip()) - 1))
    return "|".join(out) + ("\n" if new.endswith("\n") else "")


for name in sys.argv[1:]:
    path = ROOT / name
    head = subprocess.run(["git", "show", f"HEAD:{name}"], cwd=ROOT, capture_output=True, text=True).stdout.splitlines(keepends=True)
    lines = path.read_text(encoding="utf-8").splitlines(keepends=True)
    if len(head) != len(lines):
        sys.exit(f"{name}: line count changed, repad by hand")
    path.write_text("".join(repad(old, new) for old, new in zip(head, lines)), encoding="utf-8")
    print("[w3-codes] re-padded", name)
