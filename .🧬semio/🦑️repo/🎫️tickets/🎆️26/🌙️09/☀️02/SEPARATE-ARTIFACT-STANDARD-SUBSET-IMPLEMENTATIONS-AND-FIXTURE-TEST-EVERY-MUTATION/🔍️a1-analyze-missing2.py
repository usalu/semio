#!/usr/bin/env python3
"""🔍️ Per-breach analysis of missing-fixture entries under 🧿️semio, using the actual (id-expanded)
breach dump rather than raw templated feature text, so `<id>` placeholders are never mistaken for
literal directory names.
"""
import json
import os
import re

REPO = "/Users/ueli/Documents/semio"
TICKET = os.path.join(
    REPO,
    ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION",
)
TESTS_ROOT = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests")

mf = json.load(open(os.path.join(TICKET, "🗑️generated/a1/current-missing-fixture.json")))

SCOPE_RE = re.compile(r"mutate-semio-([a-z]+)/")
URI_RE = re.compile(r"Fixture (\S+) does not resolve")


def exists_rel(base, rel):
    return os.path.isfile(os.path.join(base, rel))


rows = []
for b in mf:
    scope = b["scope"]
    sm = SCOPE_RE.search(scope)
    if not sm:
        continue
    subset = sm.group(1)
    um = URI_RE.search(b["summary"])
    uri = um.group(1) if um else None
    rows.append((subset, uri, scope))

for subset, uri, scope in rows:
    if uri is None or not uri.startswith("local://"):
        print(f"NONLOCAL  {subset:14s} {uri}")
        continue
    name = uri[len("local://") :]
    case_dir = os.path.join(TESTS_ROOT, f"mutate-semio-{subset}")
    fixture_base = os.path.join(case_dir, "🧫️fixtures")
    if exists_rel(fixture_base, name):
        print(f"ALREADY-OK {subset:14s} {uri}")
        continue
    candidates = []
    if name.endswith(".json"):
        stem = name[: -len(".json")]
        candidates.append(f"{stem}/🔣️.json")
        if stem.startswith("🦠️"):
            mid = stem[len("🦠️") :]
            candidates.append(f"🧫️{mid}/🦠️mutation/🔣️.json")
            candidates.append(f"{mid}/🦠️mutation/🔣️.json")
    m = re.match(r"^(.*)\.([A-Za-z0-9.]+)$", name)
    if m and not name.endswith(".json"):
        stem, ext = m.group(1), m.group(2)
        candidates.append(f"{stem}/🔣️.{ext}")

    found = [c for c in candidates if exists_rel(fixture_base, c)]
    if found:
        print(f"FIX        {subset:14s} local://{name}  ->  local://{found[0]}" + (f"  (also matched: {found[1:]})" if len(found) > 1 else ""))
    else:
        print(f"MANUAL     {subset:14s} local://{name}   (tried: {candidates})")
