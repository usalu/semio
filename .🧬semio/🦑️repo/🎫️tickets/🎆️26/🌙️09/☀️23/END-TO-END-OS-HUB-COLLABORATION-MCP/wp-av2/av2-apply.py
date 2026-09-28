"""🎥️ AV2 window-3 patch: animate video export host capability (`Effect::VideoRenderExport`).

Applies `payload/hunks.json` (uniquely anchored old→new replacements, re-read from disk right before writing)
and `payload/files/<rel>` (new files) onto a tree. Dry run is the default; `--write` writes only when EVERY hunk
and new file is admissible, so a partial landing never happens.

Usage: python3 av2-apply.py [--root <tree>] [--write] [--revert]
`--write` first copies every file it edits to `BACKUP` (pre-landing bytes); `--revert` restores them and removes the new
files, but only when no file changed since the landing (all-or-nothing, like the landing).
"""
import json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
PAYLOAD = os.path.join(HERE, "payload")
BACKUP = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14b-av2-prelanding"


def main() -> int:
    args = sys.argv[1:]
    root = args[args.index("--root") + 1] if "--root" in args else "/Users/ueli/Documents/semio"
    write = "--write" in args
    if "--revert" in args:
        return revert(root)
    hunks = json.load(open(os.path.join(PAYLOAD, "hunks.json"), encoding="utf-8"))
    manifest = json.load(open(os.path.join(PAYLOAD, "manifest.json"), encoding="utf-8"))
    texts, problems = {}, []
    for index, hunk in enumerate(hunks):
        rel = hunk["file"]
        if rel not in texts:
            path = os.path.join(root, rel)
            if not os.path.exists(path):
                problems.append(f"hunk {index}: missing file {rel}")
                continue
            texts[rel] = open(path, encoding="utf-8").read()
        text = texts[rel]
        count = text.count(hunk["old"])
        if count == 1:
            texts[rel] = text.replace(hunk["old"], hunk["new"], 1)
        elif text.count(hunk["new"]) == 1 and hunk["new"] != hunk["old"]:
            problems.append(f"hunk {index}: {rel} already carries this hunk")
        else:
            first = hunk["old"].strip().splitlines()[0][:100] if hunk["old"].strip() else "<empty>"
            problems.append(f"hunk {index}: {rel} anchor matches {count}x (first line: {first!r})")
    new_files = []
    for rel in manifest["new"]:
        source = os.path.join(PAYLOAD, "files", rel)
        target = os.path.join(root, rel)
        payload = open(source, "rb").read()
        if os.path.exists(target) and open(target, "rb").read() != payload:
            problems.append(f"new file {rel} exists with different content")
        new_files.append((target, payload))
    print(f"[av2-apply] root={root} hunks={len(hunks)} files_edited={len(texts)} new_files={len(new_files)} problems={len(problems)} mode={'write' if write else 'dry-run'}")
    for problem in problems:
        print(f"[av2-apply] PROBLEM {problem}")
    if problems:
        return 1
    if write:
        for rel, text in texts.items():
            saved = os.path.join(BACKUP, "before", rel)
            os.makedirs(os.path.dirname(saved), exist_ok=True)
            shutil.copy2(os.path.join(root, rel), saved)
            written = os.path.join(BACKUP, "after", rel)
            os.makedirs(os.path.dirname(written), exist_ok=True)
            open(written, "w", encoding="utf-8").write(text)
            with open(os.path.join(root, rel), "w", encoding="utf-8") as handle:
                handle.write(text)
        for target, payload in new_files:
            os.makedirs(os.path.dirname(target), exist_ok=True)
            with open(target, "wb") as handle:
                handle.write(payload)
        print("[av2-apply] written")
    return 0


def revert(root: str) -> int:
    manifest = json.load(open(os.path.join(PAYLOAD, "manifest.json"), encoding="utf-8"))
    problems = []
    for rel in manifest["edited"]:
        written = os.path.join(BACKUP, "after", rel)
        if os.path.exists(written) and open(os.path.join(root, rel), "rb").read() != open(written, "rb").read():
            problems.append(f"{rel} changed since the landing")
    for rel in manifest["new"]:
        target = os.path.join(root, rel)
        if os.path.exists(target) and open(target, "rb").read() != open(os.path.join(PAYLOAD, "files", rel), "rb").read():
            problems.append(f"new file {rel} changed since the landing")
    print(f"[av2-apply] revert root={root} problems={len(problems)}")
    for problem in problems:
        print(f"[av2-apply] PROBLEM {problem}")
    if problems:
        return 1
    for rel in manifest["edited"]:
        saved = os.path.join(BACKUP, "before", rel)
        if os.path.exists(saved):
            shutil.copy2(saved, os.path.join(root, rel))
    for rel in manifest["new"]:
        target = os.path.join(root, rel)
        if os.path.exists(target):
            os.remove(target)
        parent = os.path.dirname(target)
        while parent != root and os.path.isdir(parent) and not os.listdir(parent):
            os.rmdir(parent)
            parent = os.path.dirname(parent)
    print("[av2-apply] reverted")
    return 0


if __name__ == "__main__":
    sys.exit(main())
