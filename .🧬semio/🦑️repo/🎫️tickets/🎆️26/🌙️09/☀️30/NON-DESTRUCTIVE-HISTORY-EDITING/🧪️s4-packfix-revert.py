"""⏪️ S4-PACKFIX revert — undoes `🧪️s4-packfix-sweep.py` rewrites in files it must not own, keeping every other edit.

Usage: `python3 🧪️s4-packfix-revert.py [--write] <file>...` (repo-root relative). For each file the committed HEAD text is
migrated with the sweep's own `migrate`; when the working copy equals that migration it is restored to HEAD, otherwise every
line the migration introduced is mapped back to its HEAD original and everything else is kept.
"""
import difflib
import pathlib
import subprocess
import sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[6]
source = (HERE / "🧪️s4-packfix-sweep.py").read_text(encoding="utf-8").replace("\nmain()\n", "\n")
scope = {"__name__": "sweep", "__file__": str(HERE / "🧪️s4-packfix-sweep.py")}
exec(compile(source, "sweep", "exec"), scope)
migrate = scope["migrate"]


def revert(path):
    relative = path.relative_to(ROOT).as_posix()
    head = subprocess.run(["git", "-C", str(ROOT), "show", f"HEAD:{relative}"], capture_output=True).stdout.decode("utf-8")
    current = path.read_text(encoding="utf-8")
    migrated = migrate(head)
    if migrated == head:
        return current, "untouched-by-sweep"
    if current == migrated:
        return head, "restored"
    mapping = {}
    old, new = head.split("\n"), migrated.split("\n")
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, old, new, autojunk=False).get_opcodes():
        if tag == "replace" and i2 - i1 == j2 - j1:
            for offset in range(i2 - i1):
                mapping.setdefault(new[j1 + offset], old[i1 + offset])
        elif tag != "equal":
            return current, f"MANUAL non-1:1 hunk {tag} {i1}:{i2}->{j1}:{j2}"
    lines = [mapping.get(line, line) for line in current.split("\n")]
    return "\n".join(lines), "line-mapped"


def main():
    write = "--write" in sys.argv[1:]
    for argument in [arg for arg in sys.argv[1:] if arg != "--write"]:
        path = ROOT / argument
        after, status = revert(path)
        changed = after != path.read_text(encoding="utf-8")
        print(f"{status:24} {'changed' if changed else 'same'} {argument}")
        if write and changed and not status.startswith("MANUAL"):
            path.write_text(after, encoding="utf-8")


main()
