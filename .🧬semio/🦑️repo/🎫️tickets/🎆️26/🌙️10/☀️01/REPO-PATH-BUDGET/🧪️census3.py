"""Decide, per plugin, which shortening clears the 240-byte budget outside norm.

  python3 🧪️census3.py
"""

import collections
import json
import os
import subprocess

ROOT = "/Users/ueli/Documents/semio"
BUDGET = 240
SKIP = {"dist", "node_modules", "target", ".git"}
NORM = "✏️s/🔌️plugins/📕️norm/"
ROLES = {
    "📸️snapshot", "⬅️before", "➡️after", "🦠️mutation", "🎯️outcome", "🔺️diff",
    "🧬️schema", "🧪️tests", "🔬️unit", "🧬️schem", "🔣️.json", "🦀️.rs", "🟦️.ts",
    "🐍️.py", "🗄️.sql", "🚫️.absent",
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


def plugin_of(path):
    parts = path.split("/")
    if path.startswith("✏️s/🔌️plugins/") and len(parts) > 2:
        return parts[2]
    return parts[0]


def is_case(part):
    if part in ROLES or "." in part:
        return False
    return True


def apply_b(parts):
    """Shorten case-like directories under 🧬️mutations or 🧫️fixtures."""
    new = parts[:]
    anchors = []
    for marker in ("🧬️mutations", "🧫️fixtures"):
        if marker in new:
            anchors.append(new.index(marker))
    if not anchors:
        return new
    start = min(anchors) + 1
    # keep the leaf (first segment) unless we are inside a second anchor; shorten every later case dir
    for i in range(start + 1, len(new)):
        if is_case(new[i]):
            new[i] = short_name(new[i])
    return new


def apply_a(parts):
    """Drop /🏅️standards/<v>/🪆️subsets/<s> when the subset is ✳️any or 🌐️any."""
    if "🏅️standards" not in parts or "🪆️subsets" not in parts:
        return parts
    i = parts.index("🏅️standards")
    j = parts.index("🪆️subsets")
    if j != i + 2:
        return parts
    subset = parts[j + 1] if j + 1 < len(parts) else ""
    if subset not in {"✳️any", "🌐️any"}:
        return parts
    return parts[:i] + parts[j + 2:]


def apply_leaf(parts, leaf_limit):
    new = parts[:]
    if "🧬️mutations" not in new:
        return new
    i = new.index("🧬️mutations") + 1
    if i < len(new) and nbytes(new[i]) > leaf_limit:
        new[i] = short_name(new[i], leaf_limit)
    return new


def main():
    print("budget", BUDGET, "fixture reserve 42 so a case directory must be <= 198")

    over = [p for p in repo_files() if nbytes(p) > BUDGET and not p.startswith(NORM)]
    print("over", len(over))

    plans = {
        "B": lambda parts: apply_b(parts),
        "B+leaf20": lambda parts: apply_leaf(apply_b(parts), 20),
        "A": lambda parts: apply_a(parts),
        "A+B": lambda parts: apply_b(apply_a(parts)),
        "A+B+leaf24": lambda parts: apply_leaf(apply_b(apply_a(parts)), 24),
    }
    for name, fn in plans.items():
        remain = collections.Counter()
        cleared = 0
        for path in over:
            parts = path.split("/")
            shortened = "/".join(fn(parts))
            if nbytes(shortened) > BUDGET:
                remain[plugin_of(path)] += 1
            else:
                cleared += 1
        print(f"\n{name}: cleared {cleared} remain {sum(remain.values())}")
        for plug, count in remain.most_common(12):
            print(f"  {count:5} {plug}")

    # Files B does not clear: what class are they?
    print("\n== still over after B, classes ==")
    classes = collections.Counter()
    samples = collections.defaultdict(list)
    for path in over:
        shortened = "/".join(apply_b(path.split("/")))
        if nbytes(shortened) <= BUDGET:
            continue
        parts = path.split("/")
        if "/✏️editor/" in path or "/👁️viewer/" in path:
            kind = "editor"
        elif "🎫️tickets" in parts:
            kind = "ticket"
        elif path.startswith("✏️s/🧑‍💻dev/"):
            kind = "dev-laws"
        elif "🧬️mutations" in parts:
            # extra depth under mutations?
            i = parts.index("🧬️mutations")
            depth = len(parts) - i - 1
            kind = f"mutation-depth-{depth}"
        elif "🧫️fixtures" in parts:
            kind = "fixture-other"
        else:
            kind = "other"
        classes[f"{plugin_of(path)}|{kind}"] += 1
        key = f"{plugin_of(path)}|{kind}"
        if len(samples[key]) < 1:
            samples[key].append((nbytes(shortened), shortened))
    for key, count in classes.most_common(40):
        length, sample = samples[key][0]
        print(f"  {count:4} {key} eg {length}")
        print(f"       {sample}")


if __name__ == "__main__":
    main()
