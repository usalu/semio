#!/usr/bin/env python3
"""🌳️ Authoritative census of kind-only leaf-name conformance.

Classifies every tracked file against 🔣️taxonomy.json:
  EXEMPT   — matched by fixedFilenameContracts / scopedFileKinds / pathExclusions
  OK       — basename is already <kindEmoji><extensionChain>
  RENAME   — emoji-prefixed leaf with a stem; kind-only target free in its directory
  SPLIT    — kind-only target collides with a sibling; needs a semantic child directory
  UNKNOWN  — extension not registered in fileKinds
"""
import json, subprocess, collections, fnmatch, os, sys

TAX = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
tax = json.load(open(TAX))
kinds = tax["fileKinds"]

ext_emoji = sorted(
    ((ext, spec["emoji"], kid) for kid, spec in kinds.items() for ext in spec["extensionChains"]),
    key=lambda x: -len(x[0]),
)
emojis = {spec["emoji"] for spec in kinds.values()}

fixed = [(k, v["pathPattern"]) for k, v in tax["fixedFilenameContracts"].items()]
scoped = [(k, v["pathPattern"]) for k, v in tax["scopedFileKinds"].items()]
excl = [v["path"] for v in tax["pathExclusions"].values()]


def exempt(path):
    for p in excl:
        if path.startswith(p):
            return "pathExclusion"
    for k, pat in fixed:
        if fnmatch.fnmatch(path, pat) or fnmatch.fnmatch("/" + path, "/" + pat):
            return "fixed:" + k
    for k, pat in scoped:
        if fnmatch.fnmatch(path, pat):
            return "scoped:" + k
    return None


def kind_of(base):
    for ext, emoji, kid in ext_emoji:
        if base.endswith(ext):
            return emoji + ext, kid, base[: -len(ext)], emoji
    return None, None, None, None


files = [f for f in subprocess.run(["git", "ls-files", "-z"], capture_output=True).stdout.decode().split("\0") if f]

rows = []
bydir = collections.defaultdict(list)
for f in files:
    ex = exempt(f)
    d, _, base = f.rpartition("/")
    tgt, kid, stem, emoji = kind_of(base)
    rows.append({"path": f, "dir": d, "base": base, "exempt": ex, "target": tgt, "kind": kid, "stem": stem, "emoji": emoji})
    if ex is None and tgt is not None:
        bydir[d].append(tgt)

counts = collections.Counter(t for d in bydir for t in bydir[d])
dirtgt = {d: collections.Counter(v) for d, v in bydir.items()}

klass = collections.Counter()
buckets = collections.defaultdict(list)
for r in rows:
    if r["exempt"]:
        r["class"] = "EXEMPT"
    elif r["target"] is None:
        r["class"] = "UNKNOWN"
    elif r["base"] == r["target"]:
        r["class"] = "OK"
    elif dirtgt.get(r["dir"], {}).get(r["target"], 0) > 1:
        r["class"] = "SPLIT"
    else:
        r["class"] = "RENAME"
    klass[r["class"]] += 1
    buckets[r["class"]].append(r)

print("=== classification ===")
for k, c in klass.most_common():
    print(f"{c:8d}  {k}")
print()


def area(p):
    parts = p.split("/")
    if parts[0].startswith(".🧬semio"):
        return ".🧬semio"
    return parts[0]


for cls in ("RENAME", "SPLIT", "UNKNOWN"):
    print(f"=== {cls}: by area ===")
    for a, c in collections.Counter(area(r["path"]) for r in buckets[cls]).most_common(12):
        print(f"  {c:8d}  {a}")
    print(f"=== {cls}: by stem (top 30) ===")
    for s, c in collections.Counter(r["stem"] or r["base"] for r in buckets[cls]).most_common(30):
        print(f"  {c:8d}  {s!r}")
    print()

if "--emit" in sys.argv:
    out = sys.argv[sys.argv.index("--emit") + 1]
    with open(out, "w") as fh:
        for r in rows:
            if r["class"] in ("RENAME", "SPLIT", "UNKNOWN"):
                fh.write(json.dumps(r, ensure_ascii=False) + "\n")
    print("emitted", out)
