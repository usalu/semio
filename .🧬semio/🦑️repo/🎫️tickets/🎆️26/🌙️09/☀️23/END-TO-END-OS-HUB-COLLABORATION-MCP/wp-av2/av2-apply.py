"""🎥️ AV2 window-3 patch: animate video export host capability (`Effect::VideoRenderExport`).

Applies `payload/hunks.json` (uniquely anchored old→new replacements, re-read from disk right before writing)
and `payload/files/<rel>` (new files) onto a tree. Dry run is the default; `--write` writes only when EVERY hunk
and new file is admissible, so a partial landing never happens.

Usage: python3 av2-apply.py [--root <tree>] [--write]
"""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
PAYLOAD = os.path.join(HERE, "payload")


def main() -> int:
    args = sys.argv[1:]
    root = args[args.index("--root") + 1] if "--root" in args else "/Users/ueli/Documents/semio"
    write = "--write" in args
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
            with open(os.path.join(root, rel), "w", encoding="utf-8") as handle:
                handle.write(text)
        for target, payload in new_files:
            os.makedirs(os.path.dirname(target), exist_ok=True)
            with open(target, "wb") as handle:
                handle.write(payload)
        print("[av2-apply] written")
    return 0


if __name__ == "__main__":
    sys.exit(main())
