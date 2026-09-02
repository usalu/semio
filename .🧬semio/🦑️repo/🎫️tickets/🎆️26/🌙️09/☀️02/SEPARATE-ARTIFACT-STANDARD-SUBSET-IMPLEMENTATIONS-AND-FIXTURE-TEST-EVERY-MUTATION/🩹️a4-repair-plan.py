import json
import os
import re
import subprocess

TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"

resolved = json.load(open(f"{TICKET}/🗑️generated/a4-resolve.json"))
territory = json.load(open(f"{TICKET}/🗑️generated/a4-territory-before.json"))
scope_by_uri_scheme = {(b["scope"], None): b for b in territory}

# manual overrides for the two ambiguous compound-extension / multi-fixture cases
OVERRIDES = {
    ("✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🧪️tests/mutate-en1990-1/🥒️.feature",
     "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️high-consequence-office/🖼️assets/🗣️high-consequence-office.dsl.semio"):
        "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️high-consequence-office/🖼️assets/🧪️high-consequence-office/🗣️.dsl.semio",
    ("✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🧪️tests/mutate-en1991-1/🥒️.feature",
     "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️retail-hydrocarbon-fire/🖼️assets/🗣️retail-hydrocarbon-fire.dsl.semio"):
        "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️retail-hydrocarbon-fire/🖼️assets/🧪️retail-hydrocarbon-fire/🗣️.dsl.semio",
    ("✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🧪️tests/mutate-en1992-1/🥒️.feature",
     "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️liquid-retaining-fem-anchor/🖼️assets/🗣️liquid-retaining-fem-anchor.dsl.semio"):
        "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️liquid-retaining-fem-anchor/🖼️assets/🧪️liquid-retaining-fem-anchor/🗣️.dsl.semio",
    ("✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🧪️tests/mutate-en1993-1/🥒️.feature",
     "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️high-strength-connection/🖼️assets/🗣️high-strength-connection.dsl.semio"):
        "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️high-strength-connection/🖼️assets/🧪️high-strength-connection/🗣️.dsl.semio",
    ("✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/🧪️tests/mutate-en1994-1/🥒️.feature",
     "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️composite-bridge-girder/🖼️assets/🗣️composite-bridge-girder.dsl.semio"):
        "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️composite-bridge-girder/🖼️assets/🧪️composite-bridge-girder/🗣️.dsl.semio",
    ("✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🧪️tests/mutate-en1995-1/🥒️.feature",
     "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️glulam-footbridge/🖼️assets/🗣️glulam-footbridge.dsl.semio"):
        "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️glulam-footbridge/🖼️assets/🧪️glulam-footbridge/🗣️.dsl.semio",
    ("✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🧪️tests/mutate-en1996-1/🥒️.feature",
     "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️loadbearing-wall/🖼️assets/🗣️loadbearing-wall.dsl.semio"):
        "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️loadbearing-wall/🖼️assets/🧪️loadbearing-wall/🗣️.dsl.semio",
    ("✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🧪️tests/mutate-en1998-1/🥒️.feature",
     "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️seismic-rc-frame/🖼️assets/🗣️seismic-rc-frame.dsl.semio"):
        "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️seismic-rc-frame/🖼️assets/🧪️seismic-rc-frame/🗣️.dsl.semio",
    ("✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🧪️tests/mutate-en1999-1/🥒️.feature",
     "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️aluminium-roof-purlin/🖼️assets/🗣️aluminium-roof-purlin.dsl.semio"):
        "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️aluminium-roof-purlin/🖼️assets/🧪️aluminium-roof-purlin/🗣️.dsl.semio",
    ("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🧪️tests/mutate-rewrite-1/🥒️.feature",
     "♻️nakagin-ground-floor.snapshot.json"):
        "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🧪️tests/mutate-rewrite-1/🧫️fixtures/🧪️nakagin-ground-floor/🔣️.snapshot.json",
}

plan = []
skipped = []
for r in resolved:
    scope = r["scope"]
    uri = r.get("uri")
    scheme = r.get("scheme")
    if not uri:
        skipped.append(r)
        continue
    key = (scope, uri)
    if key in OVERRIDES:
        target_abs = OVERRIDES[key]
    else:
        cands = r.get("candidates", [])
        if not cands:
            skipped.append(r)
            continue
        target_abs = cands[0]["path"]

    feature_path = scope
    case_dir = os.path.dirname(feature_path)
    tests_dir = os.path.dirname(case_dir)
    owner_dir = os.path.dirname(tests_dir)

    if scheme == "asset":
        new_uri = os.path.relpath(target_abs, owner_dir)
    elif scheme == "shared":
        new_uri = os.path.relpath(target_abs, os.path.join(owner_dir, "🧫️fixtures"))
    elif scheme == "local":
        new_uri = os.path.relpath(target_abs, os.path.join(case_dir, "🧫️fixtures"))
    else:
        skipped.append(r)
        continue

    plan.append({
        "scope": scope,
        "scheme": scheme,
        "old_uri": uri,
        "new_uri": new_uri,
        "target_abs": target_abs,
        "target_exists": os.path.exists(target_abs),
    })

with open(f"{TICKET}/🗑️generated/a4-repair-plan.json", "w") as f:
    json.dump({"plan": plan, "skipped": skipped}, f, indent=2, ensure_ascii=False)

print("plan entries:", len(plan), "skipped:", len(skipped))
for s in skipped:
    print("SKIP:", s.get("scope"), s.get("uri"), s.get("status"))
bad = [p for p in plan if not p["target_exists"]]
print("plan entries whose target does NOT exist on disk:", len(bad))
for b in bad:
    print("  BAD TARGET:", b)
