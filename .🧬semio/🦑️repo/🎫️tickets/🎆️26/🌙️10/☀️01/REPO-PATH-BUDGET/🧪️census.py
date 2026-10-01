"""Census of repo paths over the 240-byte taxonomy budget, excluding the norm plugin.

  python3 🧪️census.py
"""

import collections
import os
import re
import subprocess

ROOT = "/Users/ueli/Documents/semio"
BUDGET = 240
SKIP = {"dist", "node_modules", "target", ".git"}
NORM = "✏️s/🔌️plugins/📕️norm/"


def nbytes(text):
    return len(text.encode("utf-8"))


def repo_files():
    listed = subprocess.run(
        ["git", "ls-files", "-co", "--exclude-standard"],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=True,
    ).stdout.splitlines()
    out = []
    for path in listed:
        if any(part in SKIP for part in path.split("/")):
            continue
        if not os.path.isfile(os.path.join(ROOT, path)):
            continue
        out.append(path)
    return out


MUTATION = re.compile(
    r"^(?P<head>.*/(?:🧫️fixtures|🧬️schema)/🧬️mutations)/(?P<leaf>[^/]+)/(?P<tests>🧪️tests/)?(?P<case>[^/]+)(?P<tail>/.*)$"
)


def plugin_of(path):
    parts = path.split("/")
    if path.startswith("✏️s/🔌️plugins/") and len(parts) > 2:
        return parts[2]
    if len(parts) > 0:
        return parts[0]
    return "?"


def main():
    files = repo_files()
    over = [path for path in files if nbytes(path) > BUDGET and not path.startswith(NORM)]
    print(f"repo files {len(files)}")
    print(f"over budget outside norm {len(over)}")

    by_plugin = collections.Counter(plugin_of(path) for path in over)
    print("\nby plugin:")
    for name, count in by_plugin.most_common():
        print(f"  {count:5} {name}")

    areas = collections.Counter()
    parsed = []
    unparsed = []
    for path in over:
        match = MUTATION.match(path)
        if match:
            parsed.append((path, match))
            area = "schema" if "/🧬️schema/" in path else "fixtures"
            areas[area] += 1
        else:
            unparsed.append(path)
            areas["other"] += 1
    print("\nareas:", dict(areas))

    # How much of the overflow is the case directory?
    case_bytes = []
    leaf_bytes = []
    head_bytes = []
    tail_bytes = []
    for path, match in parsed:
        case_bytes.append(nbytes(match.group("case")))
        leaf_bytes.append(nbytes(match.group("leaf")))
        head_bytes.append(nbytes(match.group("head")))
        tail_bytes.append(nbytes(match.group("tail")))

    def stats(label, values):
        if not values:
            print(f"  {label}: none")
            return
        values = sorted(values)
        print(
            f"  {label:20} n={len(values):5} min={values[0]:3} med={values[len(values)//2]:3} max={values[-1]:3}"
        )

    print("\nmutation segment bytes (directory name, no slash):")
    stats("case", case_bytes)
    stats("leaf", leaf_bytes)
    stats("head", head_bytes)
    stats("tail", tail_bytes)

    # Option B simulation: replace case with a 12-byte placeholder.
    PLACEHOLDER = "🎯️shortslug"  # measured below
    print(f"\nplaceholder {PLACEHOLDER!r} bytes {nbytes(PLACEHOLDER)}")

    def with_case(path, match, case):
        tests = match.group("tests") or ""
        return match.group("head") + "/" + match.group("leaf") + "/" + tests + case + match.group("tail")

    remain_12 = []
    remain_emoji = []
    for path, match in parsed:
        case = match.group("case")
        # leading emoji grapheme: first codepoint plus optional FE0F
        # approximate: take chars until we pass the emoji
        emoji = case[0]
        if len(case) > 1 and case[1] == "️":
            emoji = case[:2]
        shortened = with_case(path, match, emoji + "slug")  # emoji + 4 byte slug ~ 
        if nbytes(shortened) > BUDGET:
            remain_12.append((nbytes(shortened), path, match))
        emoji_only = with_case(path, match, emoji)
        if nbytes(emoji_only) > BUDGET:
            remain_emoji.append((nbytes(emoji_only), path, match))

    print(f"mutation files still over with emoji+slug(4): {len(remain_12)}")
    print(f"mutation files still over with emoji only: {len(remain_emoji)}")

    # Group remaining by plugin and by whether leaf or head is the problem.
    by_p = collections.Counter(plugin_of(path) for _, path, _ in remain_12)
    print("still over after emoji+4slug, by plugin:")
    for name, count in by_p.most_common():
        print(f"  {count:5} {name}")

    print("\nlongest still-over after emoji+4slug (up to 30):")
    for length, path, match in sorted(remain_12, key=lambda row: -row[0])[:30]:
        leaf = match.group("leaf")
        print(f"  {length} leaf={nbytes(leaf):3} {leaf} :: {path[:180]}")

    # Non-mutation over-budget paths
    print(f"\nnon-mutation over-budget: {len(unparsed)}")
    by_p = collections.Counter(plugin_of(path) for path in unparsed)
    print("by plugin:")
    for name, count in by_p.most_common():
        print(f"  {count:5} {name}")

    # Classify non-mutation by a coarse key: strip the filename and the last 2 dirs if long
    shapes = collections.Counter()
    for path in unparsed:
        parts = path.split("/")
        # replace long segments with LEN
        shaped = []
        for part in parts:
            if nbytes(part) > 24:
                shaped.append(f"<{nbytes(part)}>")
            else:
                shaped.append(part)
        # collapse repeated numeric-looking tails
        shapes["/".join(shaped[:8])] += 1
    print("\nnon-mutation shape prefixes:")
    for shape, count in shapes.most_common(40):
        print(f"  {count:5} {shape}")

    print("\nlongest non-mutation (40):")
    for path in sorted(unparsed, key=lambda p: -nbytes(p))[:40]:
        print(f"  {nbytes(path):3} {path}")

    # Case name patterns: emoji + rest, how the rest relates to the leaf
    patterns = collections.Counter()
    samples = collections.defaultdict(list)
    for path, match in parsed:
        case = match.group("case")
        leaf = match.group("leaf")
        emoji = case[0]
        rest = case[1:]
        if rest.startswith("️"):
            emoji = case[:2]
            rest = case[2:]
        leaf_rest = leaf[1:]
        if leaf_rest.startswith("️"):
            leaf_rest = leaf[2:]
        if rest == leaf_rest or rest.endswith(leaf_rest):
            kind = "repeats-leaf"
        elif rest in ("applies", "refuses", "inverse", "roundtrip"):
            kind = "already-short"
        else:
            kind = "other"
        patterns[kind] += 1
        if len(samples[kind]) < 8:
            samples[kind].append(f"{case}  |  leaf={leaf}")
    print("\ncase vs leaf:")
    for kind, count in patterns.most_common():
        print(f"  {count:5} {kind}")
        for sample in samples[kind]:
            print(f"         {sample}")

    # Distinct case slugs (text after emoji) frequency — to design the short map
    slugs = collections.Counter()
    for path, match in parsed:
        case = match.group("case")
        rest = case[2:] if len(case) > 1 and case[1] == "️" else case[1:]
        # take the verb-ish prefix before the leaf echo: first two hyphen words if long
        slugs[rest] += 1
    print(f"\ndistinct case rests: {len(slugs)}")
    print("most common rests:")
    for rest, count in slugs.most_common(40):
        print(f"  {count:5} {nbytes(rest):3} {rest[:80]}")

    # Per plugin: if we shorten every case rest to its first hyphen-word (the verb), how many remain?
    print("\nper plugin option B (case = emoji + first hyphen word, max 12 bytes of that word):")
    per = collections.defaultdict(lambda: [0, 0, 0])  # total over, remain after B, remain after B+leaf cap
    for path, match in parsed:
        plug = plugin_of(path)
        per[plug][0] += 1
        case = match.group("case")
        emoji = case[:2] if len(case) > 1 and case[1] == "️" else case[:1]
        rest = case[len(emoji):]
        word = rest.split("-")[0] if rest else ""
        # cap the word so emoji+word <= 12 bytes if possible, else emoji only
        slug = word
        while slug and nbytes(emoji + slug) > 12:
            slug = slug[:-1]
        shortened = with_case(path, match, emoji + slug)
        if nbytes(shortened) > BUDGET:
            per[plug][1] += 1
        # also try capping leaf text (after emoji) at 20 bytes
        leaf = match.group("leaf")
        lemoji = leaf[:2] if len(leaf) > 1 and leaf[1] == "️" else leaf[:1]
        lrest = leaf[len(lemoji):]
        while lrest and nbytes(lemoji + lrest) > 24:
            lrest = lrest[:-1]
        tests = match.group("tests") or ""
        both = match.group("head") + "/" + lemoji + lrest + "/" + tests + emoji + slug + match.group("tail")
        if nbytes(both) > BUDGET:
            per[plug][2] += 1
    for plug, (total, remain, both) in sorted(per.items(), key=lambda kv: -kv[1][0]):
        print(f"  {plug:16} over={total:5} after_B={remain:5} after_B_and_leaf24={both:5}")

    # non-mutation per plugin already printed; simulate nothing


if __name__ == "__main__":
    main()
