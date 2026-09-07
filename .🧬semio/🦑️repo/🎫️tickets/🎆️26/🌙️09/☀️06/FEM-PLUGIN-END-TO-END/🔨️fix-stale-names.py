#!/usr/bin/env python3
"""🗺️ Rewrite every stale reference to a pre-truncation fem test-case directory name.

Commit `b0dfa0f09b` (2026-09-05, Windows path-length truncation) renamed 43 mutation test-case
directories under `…/🧬️schema/🧬️mutations/<kind>/🧪️tests/<case>/` without updating the literals
that name them. `🗺️truncated-names.tsv` carries `old<TAB>new` for all 43; the on-disk truncated
names are authoritative.

Two literal forms are rewritten, in this order:

* the FULL directory identity (`🚫️removes-the-unreferenced-timber-material`), which appears in
  `include_str!` paths, Gherkin `Examples` fixture columns, oracle `directoryName` fields, module
  docstrings and the `members-of-tests` registry of `🔣️taxonomy.json`;
* the EMOJI-STRIPPED identity (`removes-the-unreferenced-timber-material`) — the scenario `id` the
  test coordinator requires to equal `leadingEmojiIdentity(directoryName).rest`
  (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`), plus the
  `<kind>/<case>` labels Rust assertion messages and JSON `notes` carry.

Full names are replaced before stems so a rewritten path is never rewritten twice: no new stem is
an old stem, and no old name or stem is a substring of another (asserted at startup).

`🔣️taxonomy.json` takes FULL replacements only, and skips any target name without a leading emoji
— `🧹️normalization/🟦️.ts` THROWS on an emoji-less `memberNames` entry, which would take the whole
taxonomy, and every registry tool and dev boot with it, offline.

Gherkin `Examples` tables whose cells changed are re-padded so the pipes stay aligned.

Usage:
    python3 🔨️fix-stale-names.py [--check] [--verbose]

`--check` reports what would change and writes nothing.
"""

import os
import re
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *([".."] * 7)))
MAP = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗺️truncated-names.tsv")

PLUGIN_ROOT = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem")
TAXONOMY = os.path.join(REPO, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🔣️taxonomy.json")

SKIP_DIRS = {".git", "node_modules", "target", "dist", "🗑️generated"}
TEXT_SUFFIXES = (".rs", ".ts", ".py", ".feature", ".json", ".md", ".toml", ".semio", ".mjs", ".grammar")

CLASSES = ("path", "key", "prose")


def stem(name):
    """✂️ The identity a leading-emoji directory name renders — everything from its first ASCII letter."""
    match = re.search(r"[a-z0-9]", name)
    return name[match.start():] if match else name


def leads_with_emoji(name):
    """😀️ Whether a registry member name carries the leading emoji `🧹️normalization/🟦️.ts` demands."""
    return bool(name) and not re.match(r"[\x00-\x7f]", name)


def load_pairs():
    pairs = []
    with open(MAP, encoding="utf-8") as handle:
        for line in handle:
            if not line.strip():
                continue
            old, new = line.rstrip("\n").split("\t")
            pairs.append((old, new))
    olds = [old for old, _ in pairs]
    news = [new for _, new in pairs]
    old_stems = [stem(old) for old in olds]
    new_stems = [stem(new) for new in news]
    assert len(set(olds)) == len(olds) and len(set(news)) == len(news), "names must be unique"
    assert len(set(old_stems)) == len(old_stems), "stems must be unique"
    for group in (olds, old_stems):
        for left in group:
            for right in group:
                assert left == right or left not in right, f"{left!r} is a substring of {right!r}"
    for candidate in new_stems:
        assert candidate not in old_stems, f"new stem {candidate!r} collides with an old stem"
    return pairs


def classify(text, start, end, form):
    """🏷️ Which of the three reference classes this occurrence belongs to.

    A FULL identity is path-bearing when it sits inside a `/`-joined literal, fills a
    `directoryName` field, or fills a Gherkin `Examples` cell that a step interpolates into an
    `asset://…` URI; it is an identity key when it is one bare element of a registry array. A
    STEM never spells a directory, so only the coordinator's `id` field makes it a key.
    """
    line_start = text.rfind("\n", 0, start) + 1
    line_end = text.find("\n", end)
    line = text[line_start:line_end if line_end != -1 else len(text)]
    if form == "stem":
        return "key" if re.search(r'"id"\s*:', line) else "prose"
    before = text[start - 1] if start else ""
    after = text[end] if end < len(text) else ""
    if before == "/" or after == "/" or re.search(r'"directoryName"\s*:', line):
        return "path"
    if line.lstrip().startswith("|") and line.rstrip().endswith("|"):
        return "path"
    if re.fullmatch(r'\s*"[^"]*",?', line):
        return "key"
    return "prose"


def substitute(text, pairs, path, counts):
    """🔁️ Applies full-name replacements, then stem replacements, tallying each occurrence's class."""
    full_only = os.path.abspath(path) == TAXONOMY
    for old, new in pairs:
        if full_only and not leads_with_emoji(new):
            continue
        index = 0
        while True:
            found = text.find(old, index)
            if found == -1:
                break
            counts[classify(text, found, found + len(old), "full")] += 1
            text = text[:found] + new + text[found + len(old):]
            index = found + len(new)
    if full_only:
        return text
    for old, new in pairs:
        old_stem, new_stem = stem(old), stem(new)
        index = 0
        while True:
            found = text.find(old_stem, index)
            if found == -1:
                break
            counts[classify(text, found, found + len(old_stem), "stem")] += 1
            text = text[:found] + new_stem + text[found + len(old_stem):]
            index = found + len(new_stem)
    return text


def is_table_row(line):
    stripped = line.strip()
    return stripped.startswith("|") and stripped.endswith("|") and len(stripped) > 1


def realign_tables(before, after):
    """📐️ Re-pads every Gherkin `Examples` table block that this rewrite changed."""
    old_lines = before.split("\n")
    new_lines = after.split("\n")
    if len(old_lines) != len(new_lines):
        return after
    result = list(new_lines)
    index = 0
    while index < len(result):
        if not is_table_row(result[index]):
            index += 1
            continue
        start = index
        while index < len(result) and is_table_row(result[index]):
            index += 1
        block = range(start, index)
        if all(old_lines[row] == new_lines[row] for row in block):
            continue
        cells = [[cell.strip() for cell in result[row].strip()[1:-1].split("|")] for row in block]
        if len({len(row) for row in cells}) != 1:
            continue
        indent = result[start][: len(result[start]) - len(result[start].lstrip())]
        widths = [max(len(row[column]) for row in cells) for column in range(len(cells[0]))]
        for offset, row in enumerate(block):
            padded = " | ".join(cells[offset][column].ljust(widths[column]) for column in range(len(widths)))
            result[row] = f"{indent}| {padded} |"
    return "\n".join(result)


def targets():
    for root, directories, files in os.walk(PLUGIN_ROOT):
        directories[:] = [name for name in directories if name not in SKIP_DIRS]
        for name in files:
            if name.endswith(TEXT_SUFFIXES):
                yield os.path.join(root, name)
    yield TAXONOMY


def main():
    check = "--check" in sys.argv[1:]
    verbose = "--verbose" in sys.argv[1:]
    pairs = load_pairs()
    skipped = [new for _, new in pairs if not leads_with_emoji(new)]
    totals = dict.fromkeys(CLASSES, 0)
    changed = 0
    for path in sorted(targets()):
        try:
            with open(path, encoding="utf-8") as handle:
                original = handle.read()
        except (UnicodeDecodeError, FileNotFoundError):
            continue
        counts = dict.fromkeys(CLASSES, 0)
        rewritten = substitute(original, pairs, path, counts)
        if path.endswith(".feature"):
            rewritten = realign_tables(original, rewritten)
        if rewritten == original:
            continue
        changed += 1
        for key in CLASSES:
            totals[key] += counts[key]
        rel = os.path.relpath(path, REPO)
        print(f"{sum(counts.values()):4d}  path={counts['path']:<3d} key={counts['key']:<3d} prose={counts['prose']:<3d}  {rel}")
        if not check:
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(rewritten)
    print(f"\nfiles changed: {changed}")
    print(f"occurrences:   path={totals['path']} key={totals['key']} prose={totals['prose']} total={sum(totals.values())}")
    if skipped:
        print(f"REGISTRY SKIPPED (no leading emoji, would break taxonomy normalization): {', '.join(skipped)}")
    if verbose:
        for old, new in pairs:
            print(f"    {old}  →  {new}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
