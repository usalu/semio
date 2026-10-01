"""Rewrite references after directory renames. Reads rename-map.txt. Idempotent."""
import collections
import os
import re
import subprocess
import sys

ROOT = "/Users/ueli/Documents/semio"
TEXT_EXT = {
    ".rs", ".ts", ".tsx", ".js", ".mjs", ".cjs", ".py", ".json", ".feature",
    ".toml", ".graphql", ".gql", ".yml", ".yaml", ".sql", ".html", ".md",
}
BOUND = set("/\"'` \n\t,]}|><()=:;[]{}")


def prefix_len(part):
    if not part or ord(part[0]) < 128:
        return 0
    if len(part) > 1 and ord(part[1]) == 0xFE0F:
        return 2
    return 1


def ascii_tail(part):
    return part[prefix_len(part):]


def skipped_tree(parts):
    if any(ascii_tail(part) == "tickets" for part in parts):
        return True
    return len(parts) > 2 and ascii_tail(parts[1]) == "plugins" and ascii_tail(parts[2]) == "norm"


def load_map():
    here = os.path.dirname(os.path.abspath(__file__))
    map_path = None
    for name in os.listdir(here):
        candidate = os.path.join(here, name, "rename-map.txt")
        if os.path.exists(candidate):
            map_path = candidate
    lines = open(map_path, encoding="utf-8").read().splitlines()
    renames = {}
    for index in range(0, len(lines), 2):
        old = lines[index]
        new = lines[index + 1].strip()
        renames[old] = new
    pairs = []
    by_base = collections.defaultdict(set)
    for old, new in renames.items():
        if old == new:
            continue
        pairs.append((old, new))
        by_base[old.split("/")[-1]].add(new.split("/")[-1])
    for old_base, news in by_base.items():
        if len(news) == 1:
            new_base = next(iter(news))
            if old_base != new_base and prefix_len(old_base) > 0:
                pairs.append((old_base, new_base))
    pairs.sort(key=lambda pair: len(pair[0]), reverse=True)
    mapping = {}
    for old, new in pairs:
        mapping.setdefault(old, new)
    return mapping


def repo_files():
    listed = subprocess.run(
        ["git", "ls-files", "-co", "--exclude-standard"],
        cwd=ROOT, capture_output=True, text=True, check=True,
    ).stdout.splitlines()
    out = []
    for path in listed:
        if any(part in {"dist", "node_modules", "target", ".git"} for part in path.split("/")):
            continue
        if os.path.isfile(os.path.join(ROOT, path)):
            out.append(path)
    return out


def main():
    mapping = load_map()
    ordered = list(mapping.items())
    print("needles", len(ordered), flush=True)
    changed = 0
    scanned = 0
    for path in repo_files():
        parts = path.split("/")
        if skipped_tree(parts):
            continue
        if os.path.splitext(path)[1] not in TEXT_EXT:
            continue
        full = os.path.join(ROOT, path)
        if os.path.getsize(full) > 5_000_000:
            continue
        scanned += 1
        if scanned % 4000 == 0:
            print("scanned", scanned, "rewrote", changed, flush=True)
        try:
            text = open(full, encoding="utf-8").read()
        except UnicodeError:
            continue
        present = [(old, new) for old, new in ordered if old in text]
        if not present:
            continue
        pattern = re.compile("|".join(re.escape(old) for old, _new in present))

        def replace(match, source=text):
            old = match.group(0)
            start, end = match.span()
            before = source[start - 1] if start else "/"
            after = source[end] if end < len(source) else "/"
            if before not in BOUND or after not in BOUND:
                return old
            return mapping[old]

        updated = pattern.sub(replace, text)
        if updated != text:
            open(full, "w", encoding="utf-8").write(updated)
            changed += 1
    print("rewrote", changed, "of", scanned, flush=True)


if __name__ == "__main__":
    main()
