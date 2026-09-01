"""🌳️ Authoritative basename-breach scan: canonical = fileKind emoji + extension chain."""
import json, os, re, collections, fnmatch, sys

TAX = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
d = json.load(open(TAX, encoding="utf8"))
canon = {f"{v['emoji']}{e}" for v in d["fileKinds"].values() for e in v["extensionChains"]}
patterns = [(k, v["pathPattern"], v.get("configurability", "?")) for k, v in d["fixedFilenameContracts"].items() if "pathPattern" in v]
entry = {c["filename"] for c in d["configurableEntryContracts"].values()}

SKIP = {"node_modules", "target", ".git", "dist", "build", "pkg", "__pycache__", "coverage", ".venv",
        "partial_movie_files", ".nx", ".cursor", "temp", ".pytest_cache", ".ruff_cache"}

def covered(rel):
    for _, pat, _ in patterns:
        if fnmatch.fnmatch(rel, pat) or fnmatch.fnmatch("/" + rel, pat) or fnmatch.fnmatch(rel, pat.lstrip("*/")):
            return True
    return False

shapes = collections.Counter(); by_area = collections.Counter()
canonical = covered_n = total = 0
breaches = []
for dp, dn, fn in os.walk("."):
    dn[:] = [x for x in dn if x not in SKIP and not x.startswith(".git")]
    for n in fn:
        rel = os.path.relpath(os.path.join(dp, n), ".")
        total += 1
        if n in canon or n in entry: canonical += 1; continue
        if covered(rel): covered_n += 1; continue
        shapes[n] += 1
        breaches.append(rel)
        by_area[rel.split(os.sep)[0]] += 1

print(f"total={total}  canonical={canonical}  contract-covered={covered_n}  BREACH={len(breaches)}")
print(f"\ndistinct breach basenames: {len(shapes)}")
for n, c in shapes.most_common(25): print(f"  {c:6d}  {n}")
print("\nby top-level area:")
for a, c in by_area.most_common(12): print(f"  {c:6d}  {a}")
if len(sys.argv) > 1:
    open(sys.argv[1], "w", encoding="utf8").write("\n".join(sorted(breaches)))
    print(f"\nwrote {len(breaches)} paths -> {sys.argv[1]}")
