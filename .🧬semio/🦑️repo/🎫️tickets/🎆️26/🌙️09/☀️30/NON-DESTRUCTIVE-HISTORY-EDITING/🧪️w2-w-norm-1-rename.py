#!/usr/bin/env python3
"""🧪️ W2-W norm-1 follow-up: gives the 17 EN 1991 mutation leaves that lost their emoji (directory and descriptor named by a
bare U+FE0F) their taxonomy names back, and moves every committed vector bundle and canonical case onto the short scenario
names of `🧪️w2-w-norm-1-vectors.py`, so the canonical mutation pair fits the taxonomy path budget. Every reference outside the
generated sources (lib-root `#[path]`s, descriptor owners, the schema catalog, the norm mutation-leaf taxonomy fixture) is
rewritten in place; the generated sources are rewritten afterwards by `🧪️w2-w-norm-1-vectors.py sources <std>`.

    python3 🧪️w2-w-norm-1-rename.py [--dry-run]
"""
import importlib.util
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
TICKET = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("vectors", TICKET / "🧪️w2-w-norm-1-vectors.py")
vectors = importlib.util.module_from_spec(spec)
spec.loader.exec_module(vectors)

LEAF_EMOJI = {
    "change-accidental-assumed-force": "🚗",
    "change-bridge-lane-width": "↔️",
    "change-coast-or-island": "🏝️",
    "change-crane-claimed": "🏗️",
    "change-en-sk": "❄️",
    "change-floor-assumed-qk": "🏢",
    "change-hoisting-speed": "⏫",
    "change-mixed-terrain-distance": "📏",
    "change-roof-assumed-sk": "🌨️",
    "change-silo-bulk-density": "🌾",
    "change-silo-claimed": "🏭",
    "change-silo-hydraulic-radius": "⭕",
    "change-silo-k": "⚙️",
    "change-snow-zone": "🗺️",
    "change-structure-kind": "🌉",
    "change-terrain-category": "🏞️",
    "change-wind-zone": "🪁",
}
LIBRARY = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
TEXT_REFERENCES = [
    ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🦀️.rs",
    LIBRARY / "🔣️schema-catalog.json",
    LIBRARY / "📓️schema-catalog.md",
    ROOT / "✏️s/🔌️plugins/📕️norm/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json",
]
FIXTURE_PREFIX_BYTES = 150
FIXTURE_ROOT_BUDGET = 240 - 42


def move(source, destination, dry):
    """🚚️ One directory move, refusing to overwrite."""
    if source == destination:
        return
    assert source.is_dir(), source
    assert not destination.exists(), destination
    print(f"mv {source.relative_to(ROOT)} -> {destination.name}")
    if not dry:
        destination.parent.mkdir(parents=True, exist_ok=True)
        source.rename(destination)


def rename_leaves(dry):
    """🍃️ The 17 leaf directories, their fixtures, descriptors and every textual reference."""
    subset = vectors.subset_root("en1991")
    for kind, emoji in LEAF_EMOJI.items():
        old, new = f"️{kind}", f"{emoji}{kind}"
        for tree in ("🧬️schema/🧬️mutations", "🧫️fixtures/🧬️mutations"):
            if (subset / tree / old).is_dir():
                move(subset / tree / old, subset / tree / new, dry)
        descriptor = subset / "🧬️schema/🧬️mutations" / (old if dry else new) / "🔣️.json"
        text = descriptor.read_text(encoding="utf-8")
        declared = json.loads(text)
        assert declared["semanticKind"] == kind and declared["emoji"] == "️", descriptor
        text = text.replace(f'/{old}"', f'/{new}"').replace('"emoji": "️"', f'"emoji": "{emoji}"')
        if not dry:
            descriptor.write_text(text, encoding="utf-8")
    for path in TEXT_REFERENCES:
        text = path.read_text(encoding="utf-8")
        updated = text
        for kind, emoji in LEAF_EMOJI.items():
            old, new = f"️{kind}", f"{emoji}{kind}"
            updated = updated.replace(f"/{old}/", f"/{new}/").replace(f'/{old}"', f'/{new}"').replace(f'/{old}`', f'/{new}`').replace(f'"{old}"', f'"{new}"')
        assert not any(f"/️{kind}" in updated for kind in LEAF_EMOJI), path
        print(f"{'would rewrite' if dry else 'rewrote'} {path.relative_to(ROOT)} ({sum(updated.count(f'{e}{k}') - text.count(f'{e}{k}') for k, e in LEAF_EMOJI.items())} references)")
        if not dry and updated != text:
            path.write_text(updated, encoding="utf-8")


def rename_scenarios(standard, dry):
    """🎬️ Every committed bundle and canonical case of one standard onto its short scenario name."""
    subset = vectors.subset_root(standard)
    catalog = next(entry for entry in json.loads((subset / "🔮️oracles/🔣️.json").read_text(encoding="utf-8"))["mutationCatalogs"] if entry["id"] == vectors.STANDARDS[standard]["catalog"])
    committed = {vector["mutationId"]: vector["scenarios"][0]["directoryName"] for vector in catalog["vectors"]}
    for row in vectors.rows(standard):
        leaf, old, new = row["leafDir"], committed[row["kind"]], row["scenarioDir"]
        move(subset / "🧫️fixtures/🧬️mutations" / leaf / old, subset / "🧫️fixtures/🧬️mutations" / leaf / new, dry)
        move(subset / "🧬️schema/🧬️mutations" / leaf / "🧪️tests" / old, subset / "🧬️schema/🧬️mutations" / leaf / "🧪️tests" / new, dry)
        pair = len(f"{leaf}/{new}".encode())
        if FIXTURE_PREFIX_BYTES + pair > FIXTURE_ROOT_BUDGET:
            print(f"  over budget by {FIXTURE_PREFIX_BYTES + pair - FIXTURE_ROOT_BUDGET} bytes: {leaf}/{new}")


if __name__ == "__main__":
    dry = "--dry-run" in sys.argv
    rename_leaves(dry)
    if not dry:
        for standard in ("en1991", "en1990"):
            rename_scenarios(standard, dry)
