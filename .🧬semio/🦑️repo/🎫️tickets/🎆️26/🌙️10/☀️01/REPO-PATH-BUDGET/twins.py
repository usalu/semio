"""Move schema mutation twins onto the fixture directory names already renamed.

python3 twins.py
python3 twins.py --apply
"""
import os
import subprocess
import sys

ROOT = "/Users/ueli/Documents/semio"
MAP = os.path.dirname(os.path.abspath(__file__))
for name in os.listdir(os.path.dirname(os.path.abspath(__file__))):
    pass
HERE = os.path.dirname(os.path.abspath(__file__))
BOUND = set("/\"'` \n\t,]}|><()=:;[]{}")
TEXT_EXT = {".rs", ".ts", ".tsx", ".js", ".mjs", ".cjs", ".py", ".json", ".feature", ".toml", ".md"}


def ascii_tail(part):
    return "".join(char for char in part if ord(char) < 128)


def fixtures_index(parts):
    for index, part in enumerate(parts):
        if ascii_tail(part) == "fixtures" and index + 1 < len(parts) and ascii_tail(parts[index + 1]) == "mutations":
            return index
    return None


def schema_segment(parent):
    absolute = os.path.join(ROOT, parent)
    if not os.path.isdir(absolute):
        return None
    for name in os.listdir(absolute):
        if ascii_tail(name) == "schema":
            return name
    return None


def load_pairs():
    map_path = os.path.join(HERE, "generated-missing")
    for name in os.listdir(HERE):
        candidate = os.path.join(HERE, name, "rename-map.txt")
        if os.path.exists(candidate):
            map_path = candidate
    lines = open(map_path, encoding="utf-8").read().splitlines()
    return [(lines[i], lines[i + 1].strip()) for i in range(0, len(lines), 2)]


def moves():
    leaf_moves = []
    case_moves = []
    for old, new in load_pairs():
        parts = old.split("/")
        index = fixtures_index(parts)
        if index is None:
            continue
        rel = parts[index + 2:]
        new_rel = new.split("/")[index + 2:]
        if not rel or rel == new_rel:
            continue
        parent = "/".join(parts[:index])
        schema_name = schema_segment(parent)
        if not schema_name:
            continue
        if len(rel) == 1:
            source = os.path.join(ROOT, parent, schema_name, parts[index + 1], rel[0])
            dest = os.path.join(ROOT, parent, schema_name, parts[index + 1], new_rel[0])
            if os.path.isdir(source) and source != dest and not os.path.exists(dest):
                leaf_moves.append((source, dest))
            continue
        schema_leaf = os.path.join(ROOT, parent, schema_name, parts[index + 1], rel[0])
        if not os.path.isdir(schema_leaf):
            continue
        tests = next((name for name in os.listdir(schema_leaf) if ascii_tail(name) == "tests"), None)
        if not tests:
            continue
        source = os.path.join(schema_leaf, tests, *rel[1:])
        dest = os.path.join(schema_leaf, tests, *new_rel[1:])
        if os.path.isdir(source) and source != dest and not os.path.exists(dest):
            case_moves.append((source, dest))
    ordered = sorted(case_moves + leaf_moves, key=lambda item: item[0].count(os.sep), reverse=True)
    return ordered


def rewrite(ordered):
    pairs = []
    for source, dest in ordered:
        old = os.path.relpath(source, ROOT)
        new = os.path.relpath(dest, ROOT)
        pairs.append((old, new))
        # Mounts name the directory from the mutations segment down.
        old_parts = old.split("/")
        new_parts = new.split("/")
        for index, part in enumerate(old_parts):
            if ascii_tail(part) == "mutations":
                pairs.append(("/".join(old_parts[index:]), "/".join(new_parts[index:])))
                break
    pairs.sort(key=lambda item: len(item[0]), reverse=True)
    mapping = {}
    for old, new in pairs:
        mapping.setdefault(old, new)
    needle_path = os.path.join(HERE, "twin-needles.txt")
    open(needle_path, "w", encoding="utf-8").write("\n".join(mapping) + "\n")
    listed = subprocess.run(
        ["rg", "-l", "-F", "-f", needle_path, ROOT],
        capture_output=True, text=True, check=False,
    ).stdout.splitlines()
    changed = 0
    for full in listed:
        rel = os.path.relpath(full, ROOT)
        parts = rel.split("/")
        if any(ascii_tail(part) == "tickets" for part in parts):
            continue
        if len(parts) > 2 and ascii_tail(parts[1]) == "plugins" and ascii_tail(parts[2]) == "norm":
            continue
        if os.path.splitext(full)[1] not in TEXT_EXT:
            continue
        if os.path.getsize(full) > 5_000_000:
            continue
        try:
            text = open(full, encoding="utf-8").read()
        except UnicodeError:
            continue
        present = [(old, new) for old, new in mapping.items() if old in text]
        if not present:
            continue
        updated = text
        for old, new in sorted(present, key=lambda item: len(item[0]), reverse=True):
            pieces = []
            cursor = 0
            while True:
                found = updated.find(old, cursor)
                if found < 0:
                    pieces.append(updated[cursor:])
                    break
                before = updated[found - 1] if found else "/"
                end = found + len(old)
                after = updated[end] if end < len(updated) else "/"
                pieces.append(updated[cursor:found])
                pieces.append(new if before in BOUND and after in BOUND else old)
                cursor = end
            updated = "".join(pieces)
        if updated != text:
            open(full, "w", encoding="utf-8").write(updated)
            changed += 1
    return changed


def main():
    ordered = moves()
    print("schema twins", len(ordered), flush=True)
    if "--apply" not in sys.argv:
        for source, dest in ordered[:8]:
            print(os.path.relpath(source, ROOT)[-90:])
            print(" ->", os.path.relpath(dest, ROOT)[-70:])
        return
    for source, dest in ordered:
        if os.path.exists(dest):
            raise SystemExit(f"destination exists: {dest}")
        os.rename(source, dest)
    print("renamed", len(ordered), flush=True)
    print("rewrote", rewrite(ordered), flush=True)


if __name__ == "__main__":
    main()
