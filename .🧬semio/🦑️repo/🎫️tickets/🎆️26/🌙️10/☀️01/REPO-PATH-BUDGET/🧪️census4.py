"""Segment contribution on paths still over 240 after option B.

  python3 🧪️census4.py
"""

import collections
import os
import subprocess

ROOT = "/Users/ueli/Documents/semio"
BUDGET = 240
SKIP = {"dist", "node_modules", "target", ".git"}
NORM = "✏️s/🔌️plugins/📕️norm/"
ROLES = {
    "📸️snapshot", "⬅️before", "➡️after", "🦠️mutation", "🎯️outcome", "🔺️diff",
    "🧬️schema", "🧪️tests", "🔬️unit", "🔬️contract", "🔬️mutation-vectors",
    "🔬️window-ownership",
}


def nbytes(text):
    return len(text.encode("utf-8"))


def repo_files():
    listed = subprocess.run(
        ["git", "ls-files", "-co", "--exclude-standard"],
        cwd=ROOT, capture_output=True, text=True, check=True,
    ).stdout.splitlines()
    return [
        path for path in listed
        if not any(part in SKIP for part in path.split("/"))
        and os.path.isfile(os.path.join(ROOT, path))
    ]


def emoji_len(part):
    return 2 if len(part) > 1 and part[1] == "️" else 1


def short_name(part, limit=12):
    n = emoji_len(part)
    emoji = part[:n]
    rest = part[n:]
    word = rest.split("-")[0] if rest else ""
    while word and nbytes(emoji + word) > limit:
        word = word[:-1]
    return emoji + word


def apply_b(parts):
    new = parts[:]
    anchors = [new.index(m) for m in ("🧬️mutations", "🧫️fixtures") if m in new]
    if not anchors:
        return new
    start = min(anchors) + 1
    for i in range(start + 1, len(new)):
        if new[i] not in ROLES and "." not in new[i]:
            new[i] = short_name(new[i])
    return new


def drop_following(parts, marker):
    new = []
    skip = False
    for part in parts:
        if skip:
            skip = False
            continue
        if part == marker:
            skip = True
            continue
        new.append(part)
    return new


def plugin_of(parts):
    path = "/".join(parts)
    return parts[2] if path.startswith("✏️s/🔌️plugins/") and len(parts) > 2 else parts[0]


def family(parts):
    path = "/".join(parts)
    if "🎫️tickets" in parts:
        return "ticket"
    if "/✏️editor/" in path or "/👁️viewer/" in path:
        return "editor"
    if "/🚪️io/" in path:
        return "io"
    if "/📚️examples/" in path:
        return "examples"
    if "🧬️mutations" in parts:
        return "mutation"
    return "other"


def main():
    remain = []
    for path in repo_files():
        if path.startswith(NORM) or nbytes(path) <= BUDGET:
            continue
        shortened = apply_b(path.split("/"))
        if nbytes("/".join(shortened)) > BUDGET:
            remain.append(shortened)
    print("remain after B", len(remain))
    by = collections.Counter(f"{plugin_of(p)}|{family(p)}" for p in remain)
    print("classes:")
    for key, count in by.most_common():
        print(f"  {count:4} {key}")

    # Longest segment on each remaining path, grouped.
    seg = collections.Counter()
    for parts in remain:
        longest = max(parts, key=nbytes)
        seg[longest] += 1
    print("\nlongest segment on remaining paths:")
    for name, count in seg.most_common(25):
        print(f"  {count:4} {nbytes(name):3} {name}")

    def count_ok(transform):
        cleared = collections.Counter()
        left = collections.Counter()
        for parts in remain:
            new = transform(parts)
            if nbytes("/".join(new)) <= BUDGET:
                cleared[family(parts)] += 1
            else:
                left[family(parts)] += 1
        return cleared, left

    plans = [
        ("drop 🎭️modes/<mode>", lambda p: drop_following(p, "🎭️modes")),
        ("drop 🏅️standards/<v>", lambda p: drop_following(p, "🏅️standards")),
        ("drop both standards and subsets pair", lambda p: drop_following(drop_following(p, "🏅️standards"), "🪆️subsets")),
        ("cap every segment at 24 bytes", lambda p: [short_name(x, 24) if nbytes(x) > 24 and "." not in x else x for x in p]),
        ("cap segments at 18", lambda p: [short_name(x, 18) if nbytes(x) > 18 and "." not in x else x for x in p]),
    ]
    # nested io: second 🏅️standards occurrence
    def drop_inner_profile(parts):
        indexes = [i for i, part in enumerate(parts) if part == "🏅️standards"]
        if len(indexes) < 2:
            return parts
        i = indexes[1]
        # drop standards/<v>/subsets/<s>
        if i + 3 < len(parts) and parts[i + 2] == "🪆️subsets":
            return parts[:i] + parts[i + 4 :]
        return parts

    plans.append(("drop inner standards/subsets", drop_inner_profile))
    plans.append(("B-remain + drop modes + drop inner profile", lambda p: drop_inner_profile(drop_following(p, "🎭️modes"))))
    plans.append(("cap 24 + drop modes + drop inner", lambda p: drop_inner_profile(drop_following([short_name(x, 24) if nbytes(x) > 24 and "." not in x else x for x in p], "🎭️modes"))))

    for name, fn in plans:
        cleared, left = count_ok(fn)
        print(f"\n{name}: cleared {sum(cleared.values())} left {sum(left.values())} {dict(left)}")

    print("\nstill over after cap18, up to 15:")
    shown = 0
    for parts in sorted(remain, key=lambda p: -nbytes("/".join(p))):
        new = [short_name(x, 18) if nbytes(x) > 18 and "." not in x else x for x in parts]
        path = "/".join(new)
        if nbytes(path) <= BUDGET:
            continue
        print(f"  {nbytes(path)} {path}")
        shown += 1
        if shown >= 15:
            break


if __name__ == "__main__":
    main()
