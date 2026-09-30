"""📏️ Path-budget measurement for `✏️s/🔌️plugins/📕️norm` (decision input for `📓️norm-path-budget.md`).

  python3 🧪️norm-path-budget.py

Measures every file under the norm plugin's artifacts (tracked and untracked, build output excluded), the byte length of
each path segment of the longest mutation-evidence paths, how many exceed the 240-byte taxonomy budget, and how many
would remain over budget under each candidate layout change. Also counts the repository-wide blast radius of each change.
"""

import collections
import os
import re
import subprocess

ROOT = "/Users/ueli/Documents/semio"
NORM = "✏️s/🔌️plugins/📕️norm"
BUDGET = 240
SKIP = {"dist", "node_modules", "target", ".git"}


def b(text):
    return len(text.encode("utf-8"))


def files(root):
    for directory, directories, names in os.walk(f"{ROOT}/{root}"):
        directories[:] = [name for name in directories if name not in SKIP]
        for name in names:
            yield os.path.relpath(os.path.join(directory, name), ROOT)


def repo_files():
    listed = subprocess.run(["git", "ls-files", "-co", "--exclude-standard"], cwd=ROOT, capture_output=True, text=True).stdout.splitlines()
    return [path for path in listed if not any(part in SKIP for part in path.split("/")) and os.path.isfile(os.path.join(ROOT, path))]


SUBSET = re.compile(r"^(?P<artifact>✏️s/🔌️plugins/📕️norm/🗿️artifacts/[^/]+)(?P<profile>/🏅️standards/[^/]+/🪆️subsets/[^/]+)(?P<rest>/.*)$")
EVIDENCE = re.compile(r"^(?P<owner>/🧫️fixtures/🧬️mutations)/(?P<leaf>[^/]+)/(?P<case>[^/]+)(?P<tail>/.*)$")


def main():
    norm = sorted(files(NORM))
    over = [path for path in norm if b(path) > BUDGET]
    print(f"norm files {len(norm)}, over {BUDGET} bytes {len(over)}")
    by_area = collections.Counter()
    for path in over:
        match = SUBSET.match(path)
        rest = match.group("rest") if match else path
        area = "🧫️fixtures/🧬️mutations" if rest.startswith("/🧫️fixtures/🧬️mutations") else "🧬️schema/🧬️mutations" if rest.startswith("/🧬️schema/🧬️mutations") else rest.split("/")[1] if match else "outside subsets"
        by_area[area] += 1
    print("over-budget files by area:", dict(by_area.most_common()))

    per_artifact = collections.defaultdict(lambda: [0, 0])
    segments = collections.defaultdict(list)
    evidence = []
    for path in norm:
        match = SUBSET.match(path)
        if not match:
            continue
        artifact = match.group("artifact").split("/")[-1]
        per_artifact[artifact][0] += 1
        per_artifact[artifact][1] += b(path) > BUDGET
        inner = EVIDENCE.match(match.group("rest"))
        if inner:
            evidence.append((path, match, inner))
            segments["prefix to artifact"].append(b(match.group("artifact")))
            segments["standard+subset"].append(b(match.group("profile")))
            segments["/🧫️fixtures/🧬️mutations"].append(b(inner.group("owner")))
            segments["leaf dir (+/)"].append(b(inner.group("leaf")) + 1)
            segments["case dir (+/)"].append(b(inner.group("case")) + 1)
            segments["evidence tail " + inner.group("tail")].append(b(inner.group("tail")))
    print("\nper artifact: files, over budget")
    for artifact, (count, overflow) in sorted(per_artifact.items()):
        print(f"  {artifact:16} {count:5} {overflow:5}")
    print(f"\nevidence files {len(evidence)}, over budget {sum(b(path) > BUDGET for path, _, _ in evidence)}")
    print("segment bytes (min / median / max):")
    for name, values in segments.items():
        values = sorted(values)
        print(f"  {name:48} {values[0]:4} {values[len(values) // 2]:4} {values[-1]:4}")
    longest = sorted(evidence, key=lambda row: -b(row[0]))[:5]
    print("\nlongest evidence paths:")
    for path, match, inner in longest:
        print(f"  {b(path)}: artifact {b(match.group('artifact'))} + profile {b(match.group('profile'))} + owner {b(inner.group('owner'))} + leaf {b(inner.group('leaf')) + 1} + case {b(inner.group('case')) + 1} + tail {b(inner.group('tail'))}")
        print(f"     {path}")

    def remaining(transform):
        return sum(b(transform(path, match, inner)) > BUDGET for path, match, inner in evidence)

    single = lambda path, match, inner: match.group("artifact") + match.group("rest")
    short_case = lambda path, match, inner: match.group("artifact") + match.group("profile") + inner.group("owner") + "/" + inner.group("leaf") + "/" + inner.group("case")[:1] + "️" + inner.group("tail")
    flat_tail = lambda path, match, inner: match.group("artifact") + match.group("profile") + inner.group("owner") + "/" + inner.group("leaf") + "/" + inner.group("case") + "/" + inner.group("tail").split("/")[-2] + ".json"
    no_vs16 = lambda path, match, inner: path.replace("️", "")
    print("\nremaining over budget under each option (evidence files):")
    print(f"  A  drop /🏅️standards/<v>/🪆️subsets/✳️any for single-subset artifacts  {remaining(single)}")
    print(f"  B  one-emoji case directories (e.g. `🎯️`)                              {remaining(short_case)}")
    print(f"  C  flat evidence files (<case>/🦠️mutation.json etc.)                   {remaining(flat_tail)}")
    print(f"  D  drop every U+FE0F variation selector                               {remaining(no_vs16)}")
    capped = lambda path, match, inner: match.group("artifact") + match.group("profile") + inner.group("owner") + "/" + inner.group("leaf") + "/" + inner.group("case").encode("utf-8")[:20].decode("utf-8", "ignore") + inner.group("tail")
    print(f"  E  case directories capped at 20 bytes (emoji + short slug)           {remaining(capped)}")
    schema_over = [path for path in over if "/🧬️schema/🧬️mutations/" in path]
    print(f"  schema-side over-budget files (🧬️schema/🧬️mutations/<leaf>/🧪️tests/<case>/…): {len(schema_over)}; with case capped at 20 bytes: {sum(b(re.sub(r'(/🧪️tests/)([^/]+)', lambda m: m.group(1) + m.group(2).encode('utf-8')[:20].decode('utf-8', 'ignore'), path)) > BUDGET for path in schema_over)}")
    combined = lambda path, match, inner: match.group("artifact") + inner.group("owner") + "/" + inner.group("leaf") + "/" + inner.group("case") + "/" + inner.group("tail").split("/")[-2] + ".json"
    print(f"  A+C                                                                  {remaining(combined)}")

    repo = repo_files()
    profiled = [path for path in repo if "/🏅️standards/" in path and "/🪆️subsets/" in path]
    bundles = [path for path in repo if re.search(r"/🦠️mutation/🔣️\.json$", path)]
    repo_over = [path for path in repo if b(path) > BUDGET]
    print(f"\nrepository: files {len(repo)}, over budget {len(repo_over)}, under standards/subsets {len(profiled)}, mutation bundles {len(bundles)}")
    owners = collections.Counter(path.split("/")[2] if path.startswith("✏️s/🔌️plugins/") else path.split("/")[0] for path in profiled)
    print("standards/subsets layout by plugin:", dict(owners.most_common(12)))
    over_owners = collections.Counter(path.split("/")[2] if path.startswith("✏️s/🔌️plugins/") else path.split("/")[0] for path in repo_over)
    print("over-budget files by plugin:", dict(over_owners.most_common(12)))
    bundle_owners = collections.Counter(path.split("/")[2] if path.startswith("✏️s/🔌️plugins/") else path.split("/")[0] for path in bundles)
    print("mutation bundles by plugin:", dict(bundle_owners.most_common(12)))

    subsets = collections.defaultdict(set)
    for path in profiled:
        match = re.match(r"^(.*?/🏅️standards/[^/]+)/🪆️subsets/([^/]+)/", path)
        if match:
            subsets[match.group(1)].add(match.group(2))
    single = [profile for profile, names in subsets.items() if names == {"✳️any"}]
    print(f"standard profiles {len(subsets)}, of which single-subset (only ✳️any) {len(single)}; files under them {sum(1 for path in profiled if any(path.startswith(profile + '/') for profile in single))}")


if __name__ == "__main__":
    main()
