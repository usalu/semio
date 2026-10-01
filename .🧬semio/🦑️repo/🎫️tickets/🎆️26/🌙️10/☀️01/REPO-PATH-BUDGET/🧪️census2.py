"""Second census: why paths exceed 240 bytes, and which shortening clears them.

  python3 🧪️census2.py
"""

import collections
import os
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
    return [
        path
        for path in listed
        if not any(part in SKIP for part in path.split("/"))
        and os.path.isfile(os.path.join(ROOT, path))
    ]


def emoji_len(part):
    if len(part) > 1 and part[1] == "️":
        return 2
    return 1


def short_case(part, limit=12):
    """One emoji plus a slug, whole name <= limit bytes."""
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


def classify(parts):
    if "🧬️mutations" in parts:
        return "mutation"
    if "🧫️fixtures" in parts:
        return "fixture-other"
    if "🎫️tickets" in parts:
        return "ticket"
    return "other"


def main():
    over = [
        path
        for path in repo_files()
        if nbytes(path) > BUDGET and not path.startswith(NORM)
    ]
    print(f"over {len(over)}")

    # Depth histogram of segment counts and the longest segments.
    depths = collections.Counter(len(path.split("/")) for path in over)
    print("depth histogram (segments):")
    for depth, count in sorted(depths.items()):
        print(f"  {depth:3} {count}")

    # For mutation paths, fixed prefix length up to and including 🧬️mutations.
    buckets = collections.defaultdict(list)
    for path in over:
        parts = path.split("/")
        kind = classify(parts)
        buckets[kind].append(path)

    for kind, paths in buckets.items():
        print(f"\n== {kind} {len(paths)} ==")

    # Mutation: measure prefix through 🧬️mutations, leaf, case, tail.
    print("\nmutation prefix-through-mutations bytes (min/med/max) and remain after short case:")
    per = collections.defaultdict(lambda: collections.Counter())
    samples = collections.defaultdict(list)
    for path in buckets["mutation"]:
        parts = path.split("/")
        idx = parts.index("🧬️mutations")
        prefix = "/".join(parts[: idx + 1])
        leaf = parts[idx + 1] if len(parts) > idx + 1 else ""
        # case is next, unless 🧪️tests sits between
        rest = parts[idx + 2 :]
        if rest and rest[0] == "🧪️tests":
            case = rest[1] if len(rest) > 1 else ""
            tail = "/".join(rest[2:])
            case_slot = idx + 3
        else:
            case = rest[0] if rest else ""
            tail = "/".join(rest[1:])
            case_slot = idx + 2
        plug = plugin_of(path)
        new_parts = parts[:]
        if case:
            new_parts[case_slot] = short_case(case)
        shortened = "/".join(new_parts)
        per[plug]["n"] += 1
        per[plug]["prefix_max"] = max(per[plug]["prefix_max"], nbytes(prefix))
        if nbytes(shortened) > BUDGET:
            per[plug]["remain"] += 1
            # how many bytes over, and which segment to blame
            overflow = nbytes(shortened) - BUDGET
            if len(samples[plug]) < 3:
                samples[plug].append((overflow, nbytes(prefix), nbytes(leaf), nbytes(new_parts[case_slot]) if case else 0, nbytes(tail), shortened))
        else:
            per[plug]["cleared"] += 1
    for plug, counter in sorted(per.items(), key=lambda kv: -kv[1]["n"]):
        print(
            f"  {plug:16} n={counter['n']:5} cleared={counter['cleared']:5} remain={counter['remain']:5} prefix_max={counter['prefix_max']}"
        )
        for row in samples[plug]:
            overflow, prefix, leaf, case, tail, shortened = row
            print(f"      +{overflow:3} prefix={prefix} leaf={leaf} case={case} tail={tail}")
            print(f"         {shortened}")

    # fixture-other: same but anchor on 🧫️fixtures
    print("\nfixture-other after short case (anchor 🧫️fixtures):")
    per = collections.defaultdict(lambda: collections.Counter())
    samples = collections.defaultdict(list)
    for path in buckets["fixture-other"]:
        parts = path.split("/")
        idx = parts.index("🧫️fixtures")
        prefix = "/".join(parts[: idx + 1])
        rest = parts[idx + 1 :]
        plug = plugin_of(path)
        # shorten every segment after fixtures that is a case-like directory (not a known file role)
        roles = {
            "📸️snapshot",
            "⬅️before",
            "➡️after",
            "🦠️mutation",
            "🎯️outcome",
            "🔺️diff",
            "🧬️schema",
            "🧪️tests",
            "🔬️unit",
        }
        new_parts = parts[:]
        # treat the second directory under fixtures as the case when present
        if len(rest) >= 2 and rest[1] not in roles and "." not in rest[1]:
            new_parts[idx + 2] = short_case(rest[1])
        shortened = "/".join(new_parts)
        per[plug]["n"] += 1
        per[plug]["prefix_max"] = max(per[plug]["prefix_max"], nbytes(prefix))
        if nbytes(shortened) > BUDGET:
            per[plug]["remain"] += 1
            if len(samples[plug]) < 2:
                samples[plug].append((nbytes(shortened) - BUDGET, nbytes(prefix), shortened))
        else:
            per[plug]["cleared"] += 1
    for plug, counter in sorted(per.items(), key=lambda kv: -kv[1]["n"]):
        print(
            f"  {plug:16} n={counter['n']:5} cleared={counter['cleared']:5} remain={counter['remain']:5} prefix_max={counter['prefix_max']}"
        )
        for overflow, prefix, shortened in samples[plug]:
            print(f"      +{overflow:3} prefix={prefix} {shortened}")

    # Where does the depth come from? Count occurrences of known deep segments among over-budget files.
    markers = collections.Counter()
    for path in over:
        for part in path.split("/"):
            if part in {
                "✏️editor",
                "👁️viewer",
                "🎭️modes",
                "✏️edit",
                "👁️view",
                "🪟️windows",
                "🎚️config",
                "🫧️transient",
                "🏅️standards",
                "🪆️subsets",
                "✳️any",
                "🌐️any",
                "🧬️mutations",
                "🧫️fixtures",
            }:
                markers[part] += 1
    print("\nmarker hits among over-budget files:")
    for name, count in markers.most_common():
        print(f"  {count:5} {name}")

    # editor/viewer nesting: how many over-budget files contain ✏️editor or 👁️viewer
    editor = [p for p in over if "/✏️editor/" in p or "/👁️viewer/" in p]
    plain = [p for p in over if "/✏️editor/" not in p and "/👁️viewer/" not in p]
    print(f"\neditor/viewer nested: {len(editor)}")
    print(f"artifact-root (no editor/viewer): {len(plain)}")

    # For artifact-root mutation files, does option B clear them?
    print("\nartifact-root mutation, option B:")
    cleared = remain = 0
    remain_samples = []
    leaf_need = collections.Counter()
    for path in plain:
        parts = path.split("/")
        if "🧬️mutations" not in parts:
            continue
        idx = parts.index("🧬️mutations")
        rest = parts[idx + 2 :]
        new_parts = parts[:]
        if rest and rest[0] == "🧪️tests" and len(rest) > 1:
            new_parts[idx + 3] = short_case(rest[1])
        elif rest:
            new_parts[idx + 2] = short_case(rest[0])
        shortened = "/".join(new_parts)
        if nbytes(shortened) > BUDGET:
            remain += 1
            leaf = parts[idx + 1]
            leaf_need[plugin_of(path)] += 1
            if len(remain_samples) < 25:
                remain_samples.append((nbytes(shortened), nbytes(leaf), leaf, shortened))
        else:
            cleared += 1
    print(f"  cleared {cleared} remain {remain}")
    print("  remain by plugin", dict(leaf_need))
    for row in sorted(remain_samples, key=lambda r: -r[0])[:25]:
        print(f"    {row[0]} leaf_bytes={row[1]} {row[2]}")
        print(f"      {row[3]}")

    # artifact-root NON mutation
    print("\nartifact-root non-mutation count", sum(1 for p in plain if "🧬️mutations" not in p.split("/")))
    shapes = collections.Counter()
    for path in plain:
        if "🧬️mutations" in path.split("/"):
            continue
        parts = path.split("/")
        # show from subsets onward, collapsing long names
        if "🪆️subsets" in parts:
            i = parts.index("🪆️subsets")
            tail = parts[i + 2 :]
        else:
            tail = parts[-6:]
        shaped = "/".join(f"<{nbytes(p)}>" if nbytes(p) > 20 else p for p in tail)
        shapes[shaped] += 1
    print("artifact-root non-mutation tails:")
    for shape, count in shapes.most_common(30):
        print(f"  {count:4} {shape}")

    # editor nested: bytes of the editor infix
    print("\neditor infix bytes (from after subset to before 🧫️fixtures or 🧬️schema):")
    infix_stats = []
    for path in editor[:]:
        parts = path.split("/")
        if "🪆️subsets" not in parts:
            continue
        i = parts.index("🪆️subsets")
        # subset name is i+1
        start = i + 2
        end = None
        for marker in ("🧫️fixtures", "🧬️schema", "🧬️mutations"):
            if marker in parts:
                end = parts.index(marker)
                break
        if end is None:
            continue
        infix = "/".join(parts[start:end])
        infix_stats.append(nbytes(infix))
    if infix_stats:
        infix_stats.sort()
        print(
            f"  n={len(infix_stats)} min={infix_stats[0]} med={infix_stats[len(infix_stats)//2]} max={infix_stats[-1]}"
        )

    # What if editor infix is collapsed by dropping 🎭️modes/<mode> (two segments)?
    print("\nsimulation: short case AND drop /🎭️modes/<mode> from editor paths")
    cleared = remain = 0
    by = collections.Counter()
    for path in over:
        parts = path.split("/")
        new = []
        skip_next = False
        for i, part in enumerate(parts):
            if skip_next:
                skip_next = False
                continue
            if part == "🎭️modes" and i + 1 < len(parts):
                skip_next = True
                continue
            new.append(part)
        # short case under fixtures/mutations
        if "🧬️mutations" in new:
            idx = new.index("🧬️mutations")
            slot = idx + 2
            if slot < len(new) and new[slot] == "🧪️tests":
                slot += 1
            if slot < len(new) and "." not in new[slot]:
                new[slot] = short_case(new[slot])
        elif "🧫️fixtures" in new:
            idx = new.index("🧫️fixtures")
            slot = idx + 2
            if slot < len(new) and "." not in new[slot]:
                new[slot] = short_case(new[slot])
        shortened = "/".join(new)
        if nbytes(shortened) > BUDGET:
            remain += 1
            by[plugin_of(path)] += 1
        else:
            cleared += 1
    print(f"  cleared {cleared} remain {remain}")
    print("  remain by plugin", dict(by.most_common()))

    # tickets and framework leftovers
    print("\nnon-plugin leftovers:")
    for path in over:
        if not path.startswith("✏️s/🔌️plugins/"):
            print(f"  {nbytes(path):3} {path}")


if __name__ == "__main__":
    main()
