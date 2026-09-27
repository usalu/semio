#!/usr/bin/env python3
"""🆔️ F9 carrier census: every tracked or untracked-unignored file that carries a composed-child/scene id minted today by
`DefaultHasher` (`<prefix>-<16 hex>`), grouped by owner plugin and carrier class. `--json <path>` writes the full list."""
import json
import re
import subprocess
import sys
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
FIXED = ["jack-content", "dag-content", "sequence-content", "document", "gisterrain-mesh", "background-drawing", "mesh",
         "kind-catalogs", "catalog", "presentation", "animation", "imperative-flow", "imperative-text", "playbook-flow",
         "playbook-document", "architect-benchmarks", "architect-knowledge", "forms-scene", "shooting-emblem", "steps-flow",
         "note-text", "wires-content", "raster-asset", "remodeling-asset", "din18599-climate"]
PATTERNS = {p: re.compile(rb"(?<![A-Za-z0-9_-])" + re.escape(p.encode()) + rb"-([0-9a-f]{16})(?![0-9a-zA-Z])") for p in FIXED}
PATTERNS["<slug>-brep"] = re.compile(rb"-brep-([0-9a-f]{16})(?![0-9a-zA-Z])")
PATTERNS["<pane>-model"] = re.compile(rb"(?<![A-Za-z0-9_-])(shape|building|energy|structure-classic)-model-([0-9a-f]{16})(?![0-9a-zA-Z])")
PATTERNS["remodeling-mesh-io"] = re.compile(rb"remodeling-mesh-io-([0-9a-f]{16})-([0-9a-f]{16})")
CANDIDATE = r"(content|document|mesh|catalogs?|presentation|animation|flow|text|benchmarks|knowledge|scene|emblem|brep|model|asset|climate|drawing|io-[0-9a-f]{16})-[0-9a-f]{16}"
listing = subprocess.run(["git", "-c", "core.quotepath=off", "grep", "-l", "-z", "--untracked", "-E", CANDIDATE], cwd=ROOT, capture_output=True).stdout.decode().split("\0")
hits, classes, owners = {}, Counter(), defaultdict(Counter)


def owner_of(rel):
    parts = rel.split("/")
    if parts[:2] == ["✏️s", "🔌️plugins"] and len(parts) > 2:
        return parts[2]
    return "/".join(parts[:3])


def class_of(rel):
    name = rel.rsplit("/", 1)[-1]
    if "/🧪️tests/" in rel:
        return "test-source" if name.split(".")[-1] in ("rs", "py", "ts", "feature", "cs", "go") else "test-data"
    if "/🧫️fixtures/" in rel:
        return "fixture"
    if "/📚️examples/" in rel or "/🖼️assets/" in rel:
        return "asset/example"
    if name == "🛂️.descriptor.semio":
        return "descriptor"
    if name == "🔣️.json":
        return "manifest-json"
    if name.endswith(".rs"):
        return "production-rs"
    return "other"


for rel in filter(None, listing):
    path = ROOT / rel
    if rel.startswith(".tmp-ticket") or rel.startswith(".🧬semio/") or not path.is_file():
        continue
    try:
        data = path.read_bytes()
    except OSError:
        continue
    found = Counter()
    for prefix, pattern in PATTERNS.items():
        for match in pattern.finditer(data):
            if prefix == "<pane>-model" and match.group(1).endswith(b"-brep"):
                continue
            found[prefix] += 1
    if found:
        hits[rel] = dict(found)
        classes[class_of(rel)] += 1
        owners[owner_of(rel)][class_of(rel)] += 1
by_prefix = Counter()
for found in hits.values():
    for prefix in found:
        by_prefix[prefix] += 1
print(f"files {len(hits)}")
print("classes", dict(classes.most_common()))
print("prefixes (files)", dict(by_prefix.most_common()))
for owner, counter in sorted(owners.items(), key=lambda item: -sum(item[1].values())):
    print(f"  {sum(counter.values()):5d} {owner} {dict(counter.most_common())}")
if "--json" in sys.argv:
    Path(sys.argv[sys.argv.index("--json") + 1]).write_text(json.dumps(hits, ensure_ascii=False, indent=1), encoding="utf-8")
