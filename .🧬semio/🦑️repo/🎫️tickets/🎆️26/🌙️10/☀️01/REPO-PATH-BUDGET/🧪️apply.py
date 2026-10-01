"""Rename over-budget directories outside the norm plugin and rewrite references.

python3 🧪️apply.py
python3 🧪️apply.py --apply
"""

import collections
import os
import re
import subprocess
import sys

ROOT = "/Users/ueli/Documents/semio"
BUDGET = 240
SKIP = {"dist", "node_modules", "target", ".git"}
TEXT_EXT = {
    ".rs", ".ts", ".tsx", ".js", ".mjs", ".cjs", ".py", ".json", ".feature",
    ".toml", ".graphql", ".gql", ".yml", ".yaml", ".sql", ".html", ".md",
}
FIXED_TAILS = {
    "s", "plugins", "artifacts", "standards", "subsets", "mutations", "fixtures",
    "tests", "schema", "snapshot", "before", "after", "outcome", "diff", "mutation",
    "modes", "windows", "transient", "config", "unit", "contract",
}
COORD_PARENTS = {"artifacts", "standards", "subsets", "plugins"}
CASE_SLUG_BYTES = 12
SLUG = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
BOUND = set("/\"'` \n\t,]}|><()=:;[]{}")


def nbytes(text):
    return len(text.encode("utf-8"))


def prefix_len(part):
    if not part or ord(part[0]) < 128:
        return 0
    if len(part) > 1 and ord(part[1]) == 0xFE0F:
        return 2
    return 1


def ascii_tail(part):
    return part[prefix_len(part):]


def repo_files():
    listed = subprocess.run(
        ["git", "ls-files", "-co", "--exclude-standard"],
        cwd=ROOT, capture_output=True, text=True, check=True,
    ).stdout.splitlines()
    out = []
    for path in listed:
        if any(part in SKIP for part in path.split("/")):
            continue
        if os.path.isfile(os.path.join(ROOT, path)):
            out.append(path)
    return out


def skipped_tree(parts):
    if any(ascii_tail(part) == "tickets" for part in parts):
        return True
    return len(parts) > 2 and ascii_tail(parts[1]) == "plugins" and ascii_tail(parts[2]) == "norm"


def is_fixed(parts, index):
    if "." in parts[index]:
        return True
    tail = ascii_tail(parts[index])
    parent = ascii_tail(parts[index - 1]) if index else ""
    if parent == "windows":
        return False
    if parent == "modes":
        return True
    if tail in {"editor", "viewer"} and index >= 2 and ascii_tail(parts[index - 2]) == "subsets":
        return True
    if tail in FIXED_TAILS:
        return True
    return parent in COORD_PARENTS


def is_leaf(parts, index):
    if index <= 0:
        return False
    parent = ascii_tail(parts[index - 1])
    if parent == "mutations":
        return True
    if parent == "fixtures" and ascii_tail(parts[index]) != "mutations":
        return True
    return False


def legal_tail(tail, max_ascii):
    chars = []
    for char in tail.lower():
        if char.isascii() and (char.isalnum() or char == "-"):
            chars.append(char)
    text = "".join(chars).strip("-")
    while "--" in text:
        text = text.replace("--", "-")
    if not text:
        return ""
    if len(text.encode()) <= max_ascii and SLUG.fullmatch(text):
        return text
    words = [word for word in text.split("-") if word]
    while len(words) > 1 and len("-".join(words).encode()) > max_ascii:
        words.pop()
    kept = "-".join(words)
    if kept and len(kept.encode()) <= max_ascii and SLUG.fullmatch(kept):
        return kept
    return ""


def pure_short(part, limit):
    emoji = part[:prefix_len(part)]
    word = legal_tail(ascii_tail(part).split("-")[0], max(1, limit - nbytes(emoji)))
    if not word:
        return part
    return emoji + word


def unique_name(part, limit, taken):
    name = pure_short(part, limit)
    if name == part or nbytes(name) > limit:
        return part
    if name not in taken:
        return name
    emoji = name[:prefix_len(name)]
    base = ascii_tail(name)
    n = 2
    while n < 100:
        suffix = str(n)
        if len((base + suffix).encode()) > max(1, limit - nbytes(emoji)):
            return part
        candidate = emoji + base + suffix
        if candidate not in taken:
            return candidate
        n += 1
    return part


def children_of(files):
    children = collections.defaultdict(set)
    for path in files:
        built = []
        for part in path.split("/")[:-1]:
            children["/".join(built)].add(part)
            built.append(part)
    return children


def compose(path, renames):
    built = []
    original = []
    for part in path.split("/"):
        original.append(part)
        built.append(renames.get("/".join(original), part))
    return "/".join(built)


def slug_floor(name):
    # Directory kinds reject an emoji with no ascii slug.
    prefix = prefix_len(name)
    if ascii_tail(name):
        return name[: prefix + 1]
    return name[:prefix] or name[:1]


def spare_bytes(name):
    return max(0, nbytes(name) - nbytes(slug_floor(name)))


def trim_to(name, limit, taken):
    emoji = name[:prefix_len(name)]
    word = legal_tail(ascii_tail(name), max(0, limit - nbytes(emoji)))
    if not word:
        return name
    new = emoji + word
    if new in taken or new == name:
        return name
    return new


def main():
    do_apply = "--apply" in sys.argv
    files = [path for path in repo_files() if not skipped_tree(path.split("/"))]
    print("files in scope", len(files), flush=True)
    over = [path for path in files if nbytes(path) > BUDGET]
    print("over", len(over), flush=True)

    def min_length(path):
        parts = path.split("/")
        total = 0
        for index, part in enumerate(parts):
            if index:
                total += 1
            if index < len(parts) - 1 and not is_fixed(parts, index):
                total += nbytes(slug_floor(part))
            else:
                total += nbytes(part)
        return total

    possible = [path for path in over if min_length(path) <= BUDGET]
    impossible = [path for path in over if path not in set(possible)]
    print("can fit by renaming", len(possible), "need a shorter spine", len(impossible), flush=True)
    children = children_of(files)
    hot = set()
    for path in possible:
        built = []
        for part in path.split("/")[:-1]:
            built.append(part)
            hot.add("/".join(built))

    renames = {}
    by_parent = collections.defaultdict(list)
    for directory in hot:
        parts = directory.split("/")
        index = len(parts) - 1
        if is_fixed(parts, index) or is_leaf(parts, index):
            continue
        if not any(ascii_tail(part) in {"mutations", "fixtures"} for part in parts):
            continue
        if nbytes(ascii_tail(parts[index])) <= CASE_SLUG_BYTES:
            continue
        by_parent["/".join(parts[:-1])].append(parts[index])

    for parent, names in by_parent.items():
        taken = set(children.get(parent, ()))
        for name in set(names):
            taken.discard(name)
        assigned = {}
        for name in sorted(set(names)):
            assigned[name] = unique_name(name, nbytes(name[:prefix_len(name)]) + CASE_SLUG_BYTES, taken | set(assigned.values()))
        for name, new in assigned.items():
            if new != name:
                renames[f"{parent}/{name}" if parent else name] = new
    print("after case rename plan", sum(nbytes(compose(path, renames)) > BUDGET for path in over), "dirs", len(renames), flush=True)

    for round_index in range(8):
        left = [path for path in possible if nbytes(compose(path, renames)) > BUDGET]
        if not left:
            break
        changed = 0
        for path in left:
            overflow = nbytes(compose(path, renames)) - BUDGET
            parts = path.split("/")
            ranked = []
            for index, part in enumerate(parts[:-1]):
                if is_fixed(parts, index):
                    continue
                key = "/".join(parts[: index + 1])
                current = renames.get(key, part)
                spare = spare_bytes(current)
                if spare <= 0:
                    continue
                priority = spare + (0 if is_leaf(parts, index) else 1000)
                ranked.append((priority, spare, key, current))
            ranked.sort(reverse=True)
            for _priority, spare, key, current in ranked:
                if overflow <= 0:
                    break
                current = renames.get(key, current)
                spare = spare_bytes(current)
                if spare <= 0:
                    continue
                allowed = nbytes(current) - min(spare, overflow)
                parent = key.rsplit("/", 1)[0] if "/" in key else ""
                taken = set()
                for sibling in children.get(parent, ()):
                    sibling_path = f"{parent}/{sibling}" if parent else sibling
                    taken.add(renames.get(sibling_path, sibling))
                taken.discard(current)
                new = trim_to(current, allowed, taken)
                if nbytes(new) < nbytes(current):
                    renames[key] = new
                    overflow -= nbytes(current) - nbytes(new)
                    changed += 1
        left_after = sum(nbytes(compose(path, renames)) > BUDGET for path in over)
        print("round", round_index, "left", left_after, "changed", changed, flush=True)
        if changed == 0:
            break

    cleared = [path for path in over if nbytes(compose(path, renames)) <= BUDGET]
    useful = set()
    for path in cleared:
        built = []
        for part in path.split("/")[:-1]:
            built.append(part)
            key = "/".join(built)
            if key in renames:
                useful.add(key)
    renames = {key: value for key, value in renames.items() if key in useful}
    left = [path for path in over if nbytes(compose(path, renames)) > BUDGET]
    print("cleared", len(cleared), "kept renames", len(renames), flush=True)
    print("final over", len(left), "of which untouched spine", sum(path in set(impossible) for path in left), flush=True)
    profiles = set()
    for path in left:
        parts = path.split("/")
        for index, part in enumerate(parts):
            if ascii_tail(part) == "subsets" and index >= 2:
                profiles.add("/".join(parts[: index - 2]))
                break
    print("profiles still over", len(profiles), flush=True)
    under = 0
    for path in files:
        if any(path.startswith(profile + "/") for profile in profiles):
            under += 1
    print("files under those artifact roots", under, flush=True)
    worst = sorted(left, key=lambda item: -nbytes(compose(item, renames)))
    for path in worst[:8]:
        print(nbytes(compose(path, renames)), compose(path, renames))
    if worst:
        print("WORST SEGMENTS")
        for index, part in enumerate(compose(worst[0], renames).split("/")):
            print(f"{index:2} {nbytes(part):3} {ascii_tail(part)}")
        kinds = {}
        for path in left:
            parts = path.split("/")
            tails = {ascii_tail(part) for part in parts}
            if "editor" in tails or "viewer" in tails:
                kind = "editor"
            elif "export" in tails or "import" in tails:
                kind = "io"
            elif "examples" in tails:
                kind = "examples"
            else:
                kind = "other"
            kinds[kind] = kinds.get(kind, 0) + 1
        print("LEFT KINDS", kinds)
        plugins = {}
        shown = {"other": 0}
        for path in left:
            parts = path.split("/")
            tails = {ascii_tail(part) for part in parts}
            if "editor" in tails or "viewer" in tails:
                kind = "editor"
            elif "export" in tails or "import" in tails:
                kind = "io"
            elif "examples" in tails:
                kind = "examples"
            else:
                kind = "other"
                if shown["other"] < 5:
                    composed = compose(path, renames)
                    print("OTHER", nbytes(composed))
                    for part in composed.split("/"):
                        print("   ", nbytes(part), ascii_tail(part))
                    shown["other"] += 1
            plug = parts[2] if len(parts) > 2 and ascii_tail(parts[1]) == "plugins" else parts[0]
            plugins[plug] = plugins.get(plug, 0) + 1
        for plug, count in sorted(plugins.items(), key=lambda item: -item[1])[:20]:
            print(f"  {count:4} {ascii_tail(plug)}")

    here = os.path.dirname(os.path.abspath(__file__))
    generated = here
    for name in os.listdir(here):
        if os.path.isdir(os.path.join(here, name)) and os.path.exists(os.path.join(here, name, "census.txt")):
            generated = os.path.join(here, name)
    map_path = os.path.join(generated, "rename-map.txt")
    with open(map_path, "w", encoding="utf-8") as handle:
        for old in sorted(renames):
            handle.write(f"{old}\n  {compose(old, renames)}\n")
    leaves = [old for old in renames if is_leaf(old.split("/"), len(old.split("/")) - 1)]
    print("map", len(renames), "leaf renames", len(leaves), "->", map_path, flush=True)
    if not do_apply:
        return
    blocked = [path for path in cleared if nbytes(compose(path, renames)) > BUDGET]
    if blocked:
        raise SystemExit(f"refusing to apply: {len(blocked)} cleared paths lost their renames")
    for old in sorted(renames, key=lambda item: item.count("/"), reverse=True):
        src = os.path.join(ROOT, old)
        parent = os.path.dirname(src)
        dst = os.path.join(parent, renames[old])
        if not os.path.isdir(src):
            print("missing", old, flush=True)
            continue
        if os.path.exists(dst):
            raise SystemExit(f"destination exists: {dst}")
        os.rename(src, dst)
    print("renamed", len(renames), flush=True)
    rewrite(renames)


def rewrite(renames):
    pairs = []
    by_base = collections.defaultdict(set)
    for old in renames:
        new = compose(old, renames)
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
    pattern = re.compile("|".join(re.escape(old) for old in mapping))
    changed_files = 0
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
        try:
            text = open(full, encoding="utf-8").read()
        except UnicodeError:
            continue
        if not pattern.search(text):
            continue

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
            changed_files += 1
    print("rewrote", changed_files, "of", scanned, flush=True)


if __name__ == "__main__":
    main()
