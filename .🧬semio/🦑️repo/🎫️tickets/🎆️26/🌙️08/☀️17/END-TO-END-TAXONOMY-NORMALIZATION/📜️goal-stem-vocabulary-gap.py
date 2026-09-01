#!/usr/bin/env python3
"""🗣️ Repo-wide gap census: leaf stems with no registered semantic directory kind.

Mirrors canonicalFile() in 🧹️normalization/🟦️.ts:
  - strip longest registered extension chain -> sourceStem
  - drop a trailing ".test" -> semanticStem
  - empty / generic stem            -> canonical kind-only leaf, no directory needed
  - semanticStem == parent dir slug -> canonical kind-only leaf, no directory needed
  - semanticStem matches a semanticDirectoryKinds slugPattern -> becomes <emoji><stem>/<kind-only>
  - otherwise                       -> semantic-stem-unresolved (VOCABULARY GAP)
"""
import json, subprocess, collections, re, fnmatch, sys

TAX = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
tax = json.load(open(TAX))
GENERIC = {"asset", "assets", "component", "components", "glue", "test", "tests", "implementation", "impl", "index"}
kinds = tax["fileKinds"]
ext_chains = sorted({e for s in kinds.values() for e in s["extensionChains"]}, key=len, reverse=True)
sd = tax["semanticDirectoryKinds"]
patterns = [(kid, spec["emoji"], re.compile(spec["slugPattern"])) for kid, spec in sd.items()]
fixed = [(k, v["pathPattern"]) for k, v in tax["fixedFilenameContracts"].items()]
EMOJI = re.compile(r"^[^\w\s.\-]+")


def is_fixed(path):
    return any(fnmatch.fnmatch(path, p) or fnmatch.fnmatch("/" + path, "/" + p) for _, p in fixed)


def split_emoji(name):
    m = EMOJI.match(name)
    return (m.group(0), name[m.end():]) if m else ("", name)


files = [f for f in subprocess.run(["git", "ls-files", "-z"], capture_output=True).stdout.decode().split("\0") if f]

gap = collections.Counter()
gap_ex = collections.defaultdict(list)
resolved = collections.Counter()
kindonly = 0
fixedn = 0
noext = 0

for f in files:
    if is_fixed(f):
        fixedn += 1
        continue
    d, _, base = f.rpartition("/")
    ext = next((e for e in ext_chains if base.endswith(e)), None)
    if ext is None:
        noext += 1
        continue
    stem = base[: -len(ext)]
    _, rest = split_emoji(stem)
    semantic = rest[:-5] if rest.endswith(".test") else rest
    if not semantic or semantic.lower() in GENERIC:
        kindonly += 1
        continue
    parent_slug = split_emoji(d.rsplit("/", 1)[-1])[1] if d else ""
    if parent_slug and parent_slug.lower() == semantic.lower():
        kindonly += 1
        continue
    hit = next((kid for kid, emoji, rx in patterns if rx.match(semantic)), None)
    if hit:
        resolved[hit] += 1
        continue
    gap[semantic] += 1
    if len(gap_ex[semantic]) < 2:
        gap_ex[semantic].append(f)

print(f"kind-only after normalization : {kindonly}")
print(f"fixed-filename contract       : {fixedn}")
print(f"unregistered extension        : {noext}")
print(f"resolved to existing directory: {sum(resolved.values())} across {len(resolved)} kinds")
print(f"VOCABULARY GAP                : {sum(gap.values())} files across {len(gap)} distinct stems")
print()
print("=== top 70 gap stems ===")
for s, c in gap.most_common(70):
    print(f"{c:7d}  {s!r}")
    for e in gap_ex[s][:1]:
        print(f"          {e}")
