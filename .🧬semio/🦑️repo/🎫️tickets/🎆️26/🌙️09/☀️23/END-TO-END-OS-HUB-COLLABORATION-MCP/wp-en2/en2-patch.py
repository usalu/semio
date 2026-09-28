"""🩹️ EN2 prepared-patch tool: snapshot → edit in the overlay → hunks → idempotent apply on any root.

Usage:
  en2-patch.py begin <set> [--from-overlay] <path>...  copy the LIVE file (or, with --from-overlay, the overlay's current
                                                       file, for a set that builds on earlier sets) into payload/<set>/base/
                                                       and into the overlay
  en2-patch.py make <set> [--whole]                    diff every begun file (base vs overlay) into payload/<set>/hunks.json;
                                                       a new binary file is carried as base64; --whole carries each file as
                                                       one replacement guarded by its base's sha256 (generated outputs whose
                                                       repeated blocks make context hunks ambiguous)
  en2-patch.py apply <set>... [--write] [--root <dir>] dry run by default; the sets are chained in the given order in memory,
                                                       so a set that builds on an earlier one is judged on its result; every
                                                       hunk's old block must occur exactly once (or its new block already
                                                       once = applied); new files are created

A hunk is an exact old block (with up to 3 unchanged context lines each side) and its new block. Applying never
overwrites a file whole, so a peer's unrelated newer edits in the same file survive; a hunk whose old block is gone
and whose new block is absent is reported as a conflict and nothing is written.
"""
import base64
import difflib
import hashlib
import json
import shutil
import sys
from pathlib import Path

LIVE = Path("/Users/ueli/Documents/semio")
OVERLAY = LIVE / ".🧬semio/🌐hub/s14-en2-overlay"
PAYLOAD = Path(__file__).resolve().parent / "payload"
CONTEXT = 3


def begin(name, paths, from_overlay):
    base = PAYLOAD / name / "base"
    for rel in paths:
        source = (OVERLAY if from_overlay else LIVE) / rel
        target = base / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        if source.exists():
            shutil.copy2(source, target)
            (OVERLAY / rel).parent.mkdir(parents=True, exist_ok=True)
            if not from_overlay:
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


def make(name, whole=False):
    hunks = []
    for rel, created in begun_files(name):
        if whole and not created:
            base = (PAYLOAD / name / "base" / rel).read_bytes()
            hunks.append({"path": rel, "replace_base_sha256": hashlib.sha256(base).hexdigest(), "replace": (OVERLAY / rel).read_text(encoding="utf-8")})
            continue
        if created:
            raw = (OVERLAY / rel).read_bytes()
            try:
                hunks.append({"path": rel, "create": raw.decode("utf-8")})
            except UnicodeDecodeError:
                hunks.append({"path": rel, "create_b64": base64.b64encode(raw).decode("ascii")})
            continue
        edited = (OVERLAY / rel).read_text(encoding="utf-8")
        before = (PAYLOAD / name / "base" / rel).read_text(encoding="utf-8").splitlines(keepends=True)
        after = edited.splitlines(keepends=True)
        for group in difflib.SequenceMatcher(None, before, after, autojunk=False).get_grouped_opcodes(CONTEXT):
            first, last = group[0], group[-1]
            old = "".join(before[first[1] : last[2]])
            new = "".join(after[first[3] : last[4]])
            hunks.append({"path": rel, "old": old, "new": new})
    (PAYLOAD / name / "hunks.json").write_text(json.dumps(hunks, indent=1, ensure_ascii=False) + "\n", encoding="utf-8")
    print("%s: %d hunks over %d files" % (name, len(hunks), len({hunk["path"] for hunk in hunks})))


def apply(names, write, root):
    texts, total_problems = {}, 0
    for name in names:
        hunks = json.loads((PAYLOAD / name / "hunks.json").read_text(encoding="utf-8"))
        problems, applied, pending = [], 0, 0
        for hunk in hunks:
            path = root / hunk["path"]
            if "create" in hunk or "create_b64" in hunk:
                wanted = hunk["create"].encode("utf-8") if "create" in hunk else base64.b64decode(hunk["create_b64"])
                current = texts.get(hunk["path"])
                current = current if current is not None else (path.read_bytes() if path.exists() else None)
                if isinstance(current, str):
                    current = current.encode("utf-8")
                if current is None:
                    texts[hunk["path"]] = wanted
                    pending += 1
                elif current == wanted:
                    applied += 1
                else:
                    problems.append("%s: exists with different content" % hunk["path"])
                continue
            if "replace" in hunk:
                current = texts.get(hunk["path"])
                current = current if current is not None else (path.read_text(encoding="utf-8") if path.exists() else None)
                if current == hunk["replace"]:
                    applied += 1
                elif current is not None and hashlib.sha256(current.encode("utf-8")).hexdigest() == hunk["replace_base_sha256"]:
                    texts[hunk["path"]] = hunk["replace"]
                    pending += 1
                else:
                    problems.append("%s: changed since its base was recorded" % hunk["path"])
                continue
            text = texts.get(hunk["path"])
            if text is None:
                if not path.exists():
                    problems.append("%s: missing" % hunk["path"])
                    continue
                text = path.read_text(encoding="utf-8")
            new_count = text.count(hunk["new"])
            old_count = text.count(hunk["old"]) - new_count * hunk["new"].count(hunk["old"])
            if old_count == 1 and new_count == 0:
                texts[hunk["path"]] = text.replace(hunk["old"], hunk["new"], 1)
                pending += 1
            elif new_count >= 1 and old_count == 0:
                applied += 1
            else:
                problems.append("%s: old block found %d times, new block %d times\n--- old\n%s" % (hunk["path"], old_count, new_count, hunk["old"][:600]))
        print("%s on %s: %d hunks, %d pending, %d already applied, %d problems" % (name, root, len(hunks), pending, applied, len(problems)))
        for problem in problems:
            print("PROBLEM", problem)
        total_problems += len(problems)
    if total_problems or not write:
        print("nothing written" if not write else "refused: problems present, nothing written")
        return 1 if total_problems else 0
    for rel, text in texts.items():
        target = root / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        if isinstance(text, bytes):
            target.write_bytes(text)
        else:
            target.write_text(text, encoding="utf-8")
        print("wrote", rel)
    return 0


def main(argv):
    command, name = argv[0], argv[1]
    if command == "begin":
        begin(name, [arg for arg in argv[2:] if arg != "--from-overlay"], "--from-overlay" in argv)
        return 0
    if command == "make":
        make(name, "--whole" in argv)
        return 0
    if command == "apply":
        root = Path(argv[argv.index("--root") + 1]) if "--root" in argv else LIVE
        names = [arg for index, arg in enumerate(argv[1:], 1) if not arg.startswith("--") and argv[index - 1] != "--root"]
        return apply(names, "--write" in argv, root)
    raise SystemExit("unknown command %s" % command)


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
