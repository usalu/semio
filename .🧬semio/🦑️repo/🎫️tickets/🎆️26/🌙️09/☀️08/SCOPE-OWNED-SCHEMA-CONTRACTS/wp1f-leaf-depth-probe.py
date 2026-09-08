#!/usr/bin/env python3
"""🔬️ What a depth-1|2 mutation-leaf walk classifies, against what the harness classified before.

BEFORE is the rule the harness actually ran: `isMutationLeafDirectory` reads
`mutationDirectoryPattern` off `testTaxonomy()`, which strips every key not on its `required` list —
so the declared pattern never arrived and the structural fallback (a kebab tail, minus three facet
names, at ONE directory level) was always the live rule.

AFTER is the rule implemented in `mutationLeafDirectories`: a leaf is a directory under
`🧬️schema/🧬️mutations`, at depth 1 or 2, that carries a descriptor OR matches the taxonomy's declared
`mutationDirectoryPattern`; a directory with no descriptor whose children qualify is a GROUPING
directory and never a leaf. Facet and collection names come from the taxonomy.
"""
import fnmatch
import json
import os
import re
import unicodedata
from collections import Counter

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
TAXONOMY = json.load(open(os.path.join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json")))
PATTERN = re.compile(TAXONOMY["mutationDirectoryPattern"])
FALLBACK = re.compile("[a-z][a-z0-9]*(?:-[a-z0-9]+)+$")
FACETS = set(TAXONOMY["mutationOrganizationalFacetDirs"]) | set(TAXONOMY["mutationBehaviorFacetDirs"])
LEGACY_FACETS = {"💾️binary", "📝️text", "🧬️schema"}
COLLECTIONS = [p.rsplit("/", 1)[-1] for p in TAXONOMY["schemaScopeOwnerLevels"]["fixtureOwnerPathPatterns"] if not p.endswith("/**")]
MUT = TAXONOMY["schemaScopeOwnerLevels"]["mutationsFacetDirName"]
SCHEMA = TAXONOMY["schemaScopeOwnerLevels"]["facetDirName"]
DESCRIPTOR = "🔣️.json"
SKIP = {"node_modules", ".git", "target", "dist", "📤️dist", "build", "out", "storybook-static", ".venv", "__pycache__", "obj", "bin"}


def nfc(text):
    return unicodedata.normalize("NFC", text)


def collection(name):
    return any(fnmatch.fnmatch(nfc(name), nfc(p)) for p in COLLECTIONS)


def mutation_dirs():
    for base, dirs, _ in os.walk(ROOT):
        dirs[:] = [d for d in dirs if not d.startswith(".") and d not in SKIP]
        if os.path.basename(base) == MUT and os.path.basename(os.path.dirname(base)) == SCHEMA:
            yield base


def subdirs(path):
    try:
        return sorted(e.name for e in os.scandir(path) if e.is_dir(follow_symlinks=False))
    except OSError:
        return []


def described(path):
    return os.path.isfile(os.path.join(path, DESCRIPTOR))


def before_leaf(name):
    return FALLBACK.search(nfc(name)) is not None and nfc(name) not in LEGACY_FACETS


def structural(name):
    return not collection(name) and nfc(name) not in FACETS


def qualifies(parent, name):
    return structural(name) and (described(os.path.join(parent, name)) or PATTERN.match(nfc(name)) is not None)


before, after, grouping = [], [], []
for mutations in mutation_dirs():
    rel_root = os.path.relpath(mutations, ROOT)
    for name in subdirs(mutations):
        here = os.path.join(mutations, name)
        if before_leaf(name):
            before.append(f"{rel_root}/{name}")
        if not structural(name):
            continue
        if described(here):
            after.append(f"{rel_root}/{name}")
            continue
        children = [c for c in subdirs(here) if qualifies(here, c)]
        if children:
            grouping.append(f"{rel_root}/{name}")
            after.extend(f"{rel_root}/{name}/{c}" for c in children)
            continue
        if PATTERN.match(nfc(name)):
            after.append(f"{rel_root}/{name}")

before_set, after_set = set(before), set(after)
dropped = sorted(before_set - after_set)
added = sorted(after_set - before_set)
print(json.dumps({
    "before_leaves": len(before_set),
    "after_leaves": len(after_set),
    "grouping_dirs_reclassified": len(grouping),
    "grouping_by_owner": dict(Counter(p.split("/")[2] if p.startswith("✏️s/") else p.split("/")[0] for p in grouping)),
    "dropped_count": len(dropped),
    "dropped_described": len([p for p in dropped if described(os.path.join(ROOT, p))]),
    "dropped_sample": dropped[:6],
    "added_count": len(added),
    "added_depth2": len([p for p in added if p.split(f"/{MUT}/", 1)[-1].count("/") == 1]),
    "added_depth1_sample": [p for p in added if p.split(f"/{MUT}/", 1)[-1].count("/") == 0][:6],
}, indent=2, ensure_ascii=False))
