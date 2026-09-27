"""🧰️ AV1 patch authoring: snapshot overlay files before editing (`base`), then derive anchored hunks + new-file payloads (`make`).

Usage:
  python3 av1-patch.py base <rel> [<rel> ...]   # copy overlay/<rel> to base/<rel> once, before the first edit
  python3 av1-patch.py new <rel> [<rel> ...]    # declare overlay/<rel> as a NEW file (payload copy on make)
  python3 av1-patch.py make                     # write payload/hunks.json + payload/files/<rel>
"""
import difflib, json, os, shutil, sys

ROOT = "/Users/ueli/Documents/semio"
OVERLAY = os.path.join(ROOT, ".🧬semio/🌐hub/s13-av1-overlay")
BASE = os.path.join(ROOT, ".🧬semio/🌐hub/s13-av1-base")
HERE = os.path.dirname(os.path.abspath(__file__))
PAYLOAD = os.path.join(HERE, "payload")
MANIFEST = os.path.join(HERE, "payload", "manifest.json")


def load_manifest() -> dict:
    if os.path.exists(MANIFEST):
        return json.load(open(MANIFEST, encoding="utf-8"))
    return {"edited": [], "new": []}


def save_manifest(manifest: dict) -> None:
    os.makedirs(PAYLOAD, exist_ok=True)
    json.dump(manifest, open(MANIFEST, "w", encoding="utf-8"), ensure_ascii=False, indent=1)


def base(rels: list) -> None:
    manifest = load_manifest()
    for rel in rels:
        dst = os.path.join(BASE, rel)
        if not os.path.exists(dst):
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copy2(os.path.join(OVERLAY, rel), dst)
        if rel not in manifest["edited"]:
            manifest["edited"].append(rel)
    save_manifest(manifest)


def new(rels: list) -> None:
    manifest = load_manifest()
    for rel in rels:
        if rel not in manifest["new"]:
            manifest["new"].append(rel)
    save_manifest(manifest)


def unique_anchor(old_lines: list, start: int, end: int) -> tuple:
    lo, hi = (start, end) if end > start else (max(0, start - 1), min(len(old_lines), start + 1))
    text = "".join(old_lines)
    grow_before = True
    while True:
        block = "".join(old_lines[lo:hi])
        if block.strip() and text.count(block) == 1:
            return lo, hi
        if lo == 0 and hi == len(old_lines):
            return lo, hi
        if (grow_before and lo > 0) or hi == len(old_lines):
            lo -= 1
        else:
            hi += 1
        grow_before = not grow_before


def file_hunks(rel: str, old_lines: list, new_lines: list) -> list:
    ops = [op for op in difflib.SequenceMatcher(a=old_lines, b=new_lines, autojunk=False).get_opcodes() if op[0] != "equal"]
    groups = []
    for op in ops:
        if groups and op[1] - groups[-1][2] <= 6:
            groups[-1] = [groups[-1][0], groups[-1][1], op[2], groups[-1][3], op[4]]
        else:
            groups.append([op[0], op[1], op[2], op[3], op[4]])
    while True:
        spans = [(g, unique_anchor(old_lines, g[1], g[2])) for g in groups]
        merged = False
        for index in range(len(spans) - 1):
            if spans[index][1][1] > spans[index + 1][1][0]:
                a, b = groups[index], groups[index + 1]
                groups[index : index + 2] = [["replace", a[1], b[2], a[3], b[4]]]
                merged = True
                break
        if not merged:
            break
    hunks = []
    for (_, i1, i2, j1, j2), (lo, hi) in spans:
        hunks.append({"file": rel, "old": "".join(old_lines[lo:hi]), "new": "".join(old_lines[lo:i1]) + "".join(new_lines[j1:j2]) + "".join(old_lines[i2:hi])})
    return hunks


def make() -> None:
    manifest = load_manifest()
    hunks = []
    for rel in manifest["edited"]:
        old_lines = open(os.path.join(BASE, rel), encoding="utf-8").read().splitlines(keepends=True)
        new_lines = open(os.path.join(OVERLAY, rel), encoding="utf-8").read().splitlines(keepends=True)
        hunks.extend(file_hunks(rel, old_lines, new_lines))
    files_dir = os.path.join(PAYLOAD, "files")
    if os.path.exists(files_dir):
        shutil.rmtree(files_dir)
    for rel in manifest["new"]:
        dst = os.path.join(files_dir, rel)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copy2(os.path.join(OVERLAY, rel), dst)
    json.dump(hunks, open(os.path.join(PAYLOAD, "hunks.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(f"hunks={len(hunks)} edited_files={len(manifest['edited'])} new_files={len(manifest['new'])}")


if __name__ == "__main__":
    command, rest = sys.argv[1], sys.argv[2:]
    {"base": base, "new": new}.get(command, lambda _: make())(rest)
