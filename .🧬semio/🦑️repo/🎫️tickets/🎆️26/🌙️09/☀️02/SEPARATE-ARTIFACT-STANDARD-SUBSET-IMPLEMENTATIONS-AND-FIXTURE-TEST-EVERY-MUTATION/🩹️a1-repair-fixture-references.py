#!/usr/bin/env python3
"""🩹️ Repairs dead `local://`/`asset://` fixture references under 🧿️semio/🧪️tests/mutate-semio-*.

Root cause (verified by hand, not assumed): a kind-only-basename migration renamed case-local
fixture leaves from flat named files (`🦠️insert-channel.json`, `🗣️example.dsl.semio`, …) to
kind-only files nested one directory deeper (`🧫️insert-channel/🦠️mutation/🔣️.json`,
`🖼️assets/🗣️.dsl.semio`, …) without touching the `🥒️.feature` references that name them. This
script repoints every reference at the file that now actually holds the data, verified case by
case against the current breach dump before being written here. It does not delete or invent any
fixture content; every target path already exists on disk except where a `mkdir`/`cp` is called
out explicitly below (each one previously verified to be a straight relocation of an existing
file, never a fabrication).
"""
import os
import re
import shutil

REPO = "/Users/ueli/Documents/semio"
TESTS_ROOT = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests")

TIER_A = ["animation", "audio", "cad", "document", "flow", "image", "video", "mesh"]
STRAY_NO_MUTATION_SUBSETS = ["animation", "audio", "cad", "document", "flow", "image", "video"]

changed_files = set()


def read(path):
    with open(path, encoding="utf8") as f:
        return f.read()


def write(path, text):
    with open(path, "w", encoding="utf8") as f:
        f.write(text)


def sub_all(path, mapping_or_pattern, repl=None, *, regex=False):
    text = read(path)
    original = text
    if regex:
        text = re.sub(mapping_or_pattern, repl, text)
    else:
        text = text.replace(mapping_or_pattern, repl)
    if text != original:
        write(path, text)
        changed_files.add(path)
    return text != original


# 1. Relocate the case-local "no-mutation" vector that a bare migration left at the fixtures ROOT
#    (🧫️fixtures/🦠️mutation/🔣️.json) into the same 🧫️<id>/🦠️mutation/🔣️.json shape every other
#    mutation in these Tier-A (combined-vector) subsets already uses, so one uniform template
#    substitution covers every id including no-mutation.
for sub in STRAY_NO_MUTATION_SUBSETS:
    base = os.path.join(TESTS_ROOT, f"mutate-semio-{sub}", "🧫️fixtures")
    src = os.path.join(base, "🦠️mutation", "🔣️.json")
    dst_dir = os.path.join(base, "🧫️no-mutation", "🦠️mutation")
    dst = os.path.join(dst_dir, "🔣️.json")
    if os.path.isfile(src) and not os.path.isfile(dst):
        os.makedirs(dst_dir, exist_ok=True)
        shutil.move(src, dst)
        os.rmdir(os.path.dirname(src))
        print(f"MOVE  {sub:14s} 🦠️mutation/🔣️.json -> 🧫️no-mutation/🦠️mutation/🔣️.json")

# 2. presentation's "no-mutation" mutation payload is needed at BOTH the bare split-triple location
#    (spec-vector scenario) and the doubled 🧫️-prefixed location every other presentation mutation
#    id has (mutate/inverse scenario against the real derived deck). It is trivial ({"mutation":
#    "noMutation"}) and identical in both roles, so it is copied rather than invented.
pres_base = os.path.join(TESTS_ROOT, "mutate-semio-presentation", "🧫️fixtures")
pres_src = os.path.join(pres_base, "no-mutation", "🦠️mutation", "🔣️.json")
pres_dst_dir = os.path.join(pres_base, "🧫️no-mutation", "🦠️mutation")
pres_dst = os.path.join(pres_dst_dir, "🔣️.json")
if os.path.isfile(pres_src) and not os.path.isfile(pres_dst):
    os.makedirs(pres_dst_dir, exist_ok=True)
    shutil.copyfile(pres_src, pres_dst)
    print("COPY  presentation    no-mutation/🦠️mutation/🔣️.json -> 🧫️no-mutation/🦠️mutation/🔣️.json")

# 3. Text fixes per feature file.
FIXTURE_URI_RE = re.compile(r"local://🦠️([^\s\"'`,;)\]]+)\.json")


def tier_a_regex_fix(feat_path):
    return sub_all(feat_path, FIXTURE_URI_RE, r"local://🧫️\1/🦠️mutation/🔣️.json", regex=True)


for sub in TIER_A + ["presentation"]:
    feat = os.path.join(TESTS_ROOT, f"mutate-semio-{sub}", "🥒️.feature")
    if os.path.isfile(feat):
        tier_a_regex_fix(feat)

# split-triple subsets: local://<id>/⬅️before.json etc -> .../🔣️.json nesting
for sub in ["presentation", "model"]:
    feat = os.path.join(TESTS_ROOT, f"mutate-semio-{sub}", "🥒️.feature")
    sub_all(feat, "local://<id>/⬅️before.json", "local://<id>/⬅️before/🔣️.json")
    sub_all(feat, "local://<id>/🦠️mutation.json", "local://<id>/🦠️mutation/🔣️.json")
    sub_all(feat, "local://<id>/➡️after.json", "local://<id>/➡️after/🔣️.json")

# asset:// example-dsl references: the leaf was renamed to kind-only (🗣️.dsl.semio), never "example"
for sub in os.listdir(TESTS_ROOT):
    feat = os.path.join(TESTS_ROOT, sub, "🥒️.feature")
    if os.path.isfile(feat):
        sub_all(feat, "🖼️assets/🗣️example.dsl.semio", "🖼️assets/🗣️.dsl.semio")

# drawing: 6 mutation-dir Examples-table cells are missing their real directory's "-node"/"-nodes"
# suffix (verified against ✳️drawing/🧬️schema/🧬️mutations/ on disk).
drawing_feat = os.path.join(TESTS_ROOT, "mutate-semio-drawing", "🥒️.feature")
for old, new in [
    ("| flatten             | 🫓flatten              |", "| flatten             | 🫓flatten-node         |"),
    ("| group               | 🧷group                |", "| group               | 🧷group-nodes          |"),
    ("| rotate              | 🔄rotate               |", "| rotate              | 🔄rotate-node          |"),
    ("| scale               | 📏scale                |", "| scale               | 📏scale-node           |"),
    ("| unflatten           | 🎈unflatten            |", "| unflatten           | 🎈unflatten-node       |"),
    ("| ungroup             | 💫ungroup              |", "| ungroup             | 💫ungroup-node         |"),
]:
    sub_all(drawing_feat, old, new)
sub_all(drawing_feat, "local://🗣️artifact.dsl.semio", "local://🗣️.dsl.semio")

# image, mesh: same root-level kind-only artifact, no case subdirectory
for sub in ["image", "mesh"]:
    feat = os.path.join(TESTS_ROOT, f"mutate-semio-{sub}", "🥒️.feature")
    sub_all(feat, "local://🗣️artifact.dsl.semio", "local://🗣️.dsl.semio")

# graph, kit: real capsule tower dsl moved under a case-named directory
for sub in ["graph", "kit"]:
    feat = os.path.join(TESTS_ROOT, f"mutate-semio-{sub}", "🥒️.feature")
    sub_all(feat, "local://🗣️nakagin-capsule-tower.dsl.semio", "local://🧪️nakagin-capsule-tower/🗣️.dsl.semio")

# text
text_feat = os.path.join(TESTS_ROOT, "mutate-semio-text", "🥒️.feature")
sub_all(text_feat, "local://🗣️zukunft-bau-entwerfen-mit-bestand.dsl.semio", "local://🧪️zukunft-bau-entwerfen-mit-bestand/🗣️.dsl.semio")

# table
table_feat = os.path.join(TESTS_ROOT, "mutate-semio-table", "🥒️.feature")
sub_all(table_feat, "local://📊️reuse-marketplaces.csv", "local://🧪️reuse-marketplaces/📊️.csv")

# audio, video: real derived dsl moved under a case-named directory
for sub in ["audio", "video"]:
    feat = os.path.join(TESTS_ROOT, f"mutate-semio-{sub}", "🥒️.feature")
    sub_all(feat, "local://🗣️bauen-mit-bestand-ausschnitt.dsl.semio", "local://🧪️bauen-mit-bestand-ausschnitt/🗣️.dsl.semio")

# brep
brep_feat = os.path.join(TESTS_ROOT, "mutate-semio-brep", "🥒️.feature")
sub_all(brep_feat, "local://🗣️hexagonal-cut-concrete-forest-left.dsl.semio", "local://🧪️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio")

# value: ⬅️before.json nesting, the real source model.json sits bare at fixtures root, and the
# derived dsl/pack twins sit under a case-named directory
value_feat = os.path.join(TESTS_ROOT, "mutate-semio-value", "🥒️.feature")
sub_all(value_feat, "local://⬅️before.json", "local://⬅️before/🔣️.json")
sub_all(value_feat, "local://🌲️hexagonal-cut-concrete-forest-left.model.json", "local://🔣️.json")
sub_all(value_feat, "local://🌲️hexagonal-cut-concrete-forest.dsl.semio", "local://🧪️hexagonal-cut-concrete-forest/🗣️.dsl.semio")
sub_all(value_feat, "local://🌲️hexagonal-cut-concrete-forest.pack.semio", "local://🧪️hexagonal-cut-concrete-forest/🎒️.pack.semio")

print()
print(f"feature files changed: {len(changed_files)}")
for p in sorted(changed_files):
    print(" ", os.path.relpath(p, REPO))
