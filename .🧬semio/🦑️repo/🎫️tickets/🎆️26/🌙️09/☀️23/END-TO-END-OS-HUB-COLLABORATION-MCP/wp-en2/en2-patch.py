"""🩹️ EN2 prepared-patch tool: snapshot → edit in the overlay → hunks → idempotent apply on any root.

Usage:
  en2-patch.py begin <set> <repo-relative path>...     copy the LIVE file into payload/<set>/base/ and into the overlay
  en2-patch.py make <set>                              diff every begun file (base vs overlay) into payload/<set>/hunks.json
  en2-patch.py apply <set> [--write] [--root <dir>]    dry run by default; every hunk's old block must occur exactly once
                                                       (or its new block already once = applied); new files are created

A hunk is an exact old block (with up to 3 unchanged context lines each side) and its new block. Applying never
overwrites a file whole, so a peer's unrelated newer edits in the same file survive; a hunk whose old block is gone
and whose new block is absent is reported as a conflict and nothing is written.
"""
import difflib
import json
import shutil
import sys
from pathlib import Path

LIVE = Path("/Users/ueli/Documents/semio")
OVERLAY = LIVE / ".🧬semio/🌐hub/s14-en2-overlay"
PAYLOAD = Path(__file__).resolve().parent / "payload"
CONTEXT = 3


def begin(name, paths):
    base = PAYLOAD / name / "base"
    for rel in paths:
        source = LIVE / rel
        target = base / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        if source.exists():
            shutil.copy2(source, target)
            (OVERLAY / rel).parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, OVERLAY / rel)
            print("begun", rel)
        else:
            (target.parent / (target.name + ".absent")).write_text("", encoding="utf-8")
            print("begun (new file)", rel)


def begun_files(name):
    base = PAYLOAD / name / "base"
    for path in sorted(base.rglob("*")):
        if path.is_file():
            rel = path.relative_to(base).as_posix()
            yield (rel[: -len(".absent")], True) if rel.endswith(".absent") else (rel, False)


def make(name):
    hunks = []
    for rel, created in begun_files(name):
        edited = (OVERLAY / rel).read_text(encoding="utf-8")
        if created:
            hunks.append({"path": rel, "create": edited})
            continue
        before = (PAYLOAD / name / "base" / rel).read_text(encoding="utf-8").splitlines(keepends=True)
        after = edited.splitlines(keepends=True)
        for group in difflib.SequenceMatcher(None, before, after, autojunk=False).get_grouped_opcodes(CONTEXT):
            first, last = group[0], group[-1]
            old = "".join(before[first[1] : last[2]])
            new = "".join(after[first[3] : last[4]])
            hunks.append({"path": rel, "old": old, "new": new})
    (PAYLOAD / name / "hunks.json").write_text(json.dumps(hunks, indent=1, ensure_ascii=False) + "\n", encoding="utf-8")
    print("%s: %d hunks over %d files" % (name, len(hunks), len({hunk["path"] for hunk in hunks})))


def apply(name, write, root):
    hunks = json.loads((PAYLOAD / name / "hunks.json").read_text(encoding="utf-8"))
    texts, problems, applied, pending = {}, [], 0, 0
    for hunk in hunks:
        path = root / hunk["path"]
        if "create" in hunk:
            if path.exists():
                if path.read_text(encoding="utf-8") == hunk["create"]:
                    applied += 1
                else:
                    problems.append("%s: exists with different content" % hunk["path"])
            else:
                texts[hunk["path"]] = hunk["create"]
                pending += 1
            continue
        text = texts.get(hunk["path"])
        if text is None:
            if not path.exists():
                problems.append("%s: missing" % hunk["path"])
                continue
            text = path.read_text(encoding="utf-8")
        old_count, new_count = text.count(hunk["old"]), text.count(hunk["new"])
        if old_count == 1:
            texts[hunk["path"]] = text.replace(hunk["old"], hunk["new"], 1)
            pending += 1
        elif new_count >= 1 and old_count == 0:
            applied += 1
        else:
            problems.append("%s: old block found %d times, new block %d times\n--- old\n%s" % (hunk["path"], old_count, new_count, hunk["old"][:600]))
    print("%s on %s: %d hunks, %d pending, %d already applied, %d problems" % (name, root, len(hunks), pending, applied, len(problems)))
    for problem in problems:
        print("PROBLEM", problem)
    if problems or not write:
        print("nothing written" if not write else "refused: problems present, nothing written")
        return 1 if problems else 0
    for rel, text in texts.items():
        target = root / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text, encoding="utf-8")
        print("wrote", rel)
    return 0


def main(argv):
    command, name = argv[0], argv[1]
    if command == "begin":
        begin(name, argv[2:])
        return 0
    if command == "make":
        make(name)
        return 0
    if command == "apply":
        root = Path(argv[argv.index("--root") + 1]) if "--root" in argv else LIVE
        return apply(name, "--write" in argv, root)
    raise SystemExit("unknown command %s" % command)


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
