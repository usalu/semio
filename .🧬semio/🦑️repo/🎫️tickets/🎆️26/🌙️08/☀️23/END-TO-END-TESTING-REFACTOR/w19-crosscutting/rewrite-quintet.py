#!/usr/bin/env python3
"""Replaces the closing paragraph shared verbatim by ten `@no-oracle-` mutation cases across five
plugins with one that states each case's own counts, its own identity input and its own evidence
ceiling."""
import textwrap

CASES = {
    "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🧪️tests/mutate-cad-1/component.feature": (
        "Concretely for this subset: twenty kinds, so twenty mutate rows and twenty inverse rows plus the one "
        "identity-round-trip scenario, and every `(before, mutation, diff, outcome, after)` quintet is the same committed "
        "bytes the production crate's own fixture test beside that leaf already asserts against — this case adds the "
        "end-to-end platform path, not a second copy of the data. `identity-round-trip` re-encodes "
        "`📎replace-references/🧪️tests/swaps-the-shape-reference-list`'s before-snapshot, chosen because it is the "
        "one committed CAD document that carries a populated `referencesByModelDefinitionId` map. The ceiling on all "
        "of it: there is no second producer anywhere, so nothing here can catch a mistake that the handcrafted vector "
        "and the production code make together."
    ),
    "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🧪️tests/mutate-block-3d-1/component.feature": (
        "Concretely for this subset: thirty-seven kinds — thirty-seven mutate rows, thirty-seven inverse rows and one "
        "identity-round-trip scenario, the second-largest vocabulary in this family — each replaying the same committed `(before, mutation, diff, outcome, after)` bytes the "
        "production crate's own fixture test beside that leaf asserts against, so this case adds the end-to-end "
        "platform path rather than a second copy of the data. `identity-round-trip` re-encodes "
        "`✏️rename-object-kind/🧪️tests/renames-object-kind-to-pod`'s before-snapshot, the one committed 3d document "
        "that carries both a catalogue child handle and local extras. The ceiling on all of it: no second producer "
        "exists, so a mistake shared by the handcrafted vector and the production code passes unseen."
    ),
    "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🧪️tests/mutate-block-5d-1/component.feature": (
        "Concretely for this subset: forty-one kinds — forty-one mutate rows, forty-one inverse rows and one "
        "identity-round-trip scenario, the largest vocabulary in this family — each replaying the same committed `(before, mutation, diff, outcome, after)` bytes the production "
        "crate's own fixture test beside that leaf asserts against, so this case adds the end-to-end platform path "
        "rather than a second copy of the data. `identity-round-trip` re-encodes "
        "`✏️rename-part-kind/🧪️tests/renames-part-kind-to-pod`'s before-snapshot, the one committed 5d document that "
        "carries BOTH facets populated at once. The ceiling on all of it: no second producer exists, so a mistake "
        "shared by the handcrafted vector and the production code passes unseen."
    ),
    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🧪️tests/mutate-assembly-1/component.feature": (
        "Concretely for this subset: nine kinds — nine mutate rows, nine inverse rows and one identity-round-trip "
        "scenario: the SMALLEST vocabulary in this family, "
        "which matters because nine kinds cannot cover a wave-function-collapse solver's state space and this case "
        "does not pretend otherwise. Each row replays the same committed `(before, mutation, diff, outcome, after)` "
        "bytes the production crate's own fixture test beside that leaf asserts against, so what is added here is the "
        "end-to-end platform path, not a second copy of the data. `identity-round-trip` re-encodes "
        "`🔗connect-slots/🧪️tests/joins-slot-b-to-slot-c-at-index-1`'s before-snapshot, the committed three-slot "
        "assembly. The ceiling: no second producer, so a mistake shared by the vector and the production code passes "
        "unseen."
    ),
    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🧪️tests/mutate-procedural-2d-1/component.feature": (
        "Concretely for this subset: fourteen kinds — fourteen mutate rows, fourteen inverse rows and one "
        "identity-round-trip scenario — each replaying the same committed "
        "`(before, mutation, diff, outcome, after)` bytes the production crate's own fixture test beside that leaf "
        "asserts against — the end-to-end platform path is what this case adds, not a second copy of the data. "
        "`identity-round-trip` re-encodes "
        "`➖delete-generation/🧪️tests/removes-the-selected-generation-2-and-falls-back-to-generation-1`'s "
        "before-snapshot, the one committed 2d document carrying a two-generation history rather than a single "
        "current graph. Two ceilings, both real: there is no second producer, and this subset shares its snapshot "
        "SHAPE with `🧊️procedural3d`, whose case reads a structurally parallel document — agreement between the two "
        "is therefore not independent evidence about either."
    ),
    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🧪️tests/mutate-procedural-3d-1/component.feature": (
        "Concretely for this subset: fourteen kinds — fourteen mutate rows, fourteen inverse rows and one "
        "identity-round-trip scenario — each replaying the same committed "
        "`(before, mutation, diff, outcome, after)` bytes the production crate's own fixture test beside that leaf "
        "asserts against — the end-to-end platform path is what this case adds, not a second copy of the data. "
        "`identity-round-trip` re-encodes "
        "`🗑delete-generation/🧪️tests/removes-the-selected-generation-2-and-falls-back`'s before-snapshot, the one "
        "committed 3d document carrying a two-generation history. Two ceilings, both real: there is no second "
        "producer, and this subset shares its snapshot SHAPE with `🌀️procedural2d`, whose case reads a structurally "
        "parallel document — agreement between the two is therefore not independent evidence about either."
    ),
    "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🧪️tests/mutate-puzzle-3d-1/component.feature": (
        "Concretely for this subset: thirty-five kinds — thirty-five mutate rows, thirty-five inverse rows and one "
        "identity-round-trip scenario — each replaying the same "
        "committed `(before, mutation, diff, outcome, after)` bytes the production crate's own fixture test beside "
        "that leaf asserts against, so this case adds the end-to-end platform path rather than a second copy of the "
        "data. `identity-round-trip` re-encodes `🌱create-object/🧪️tests/appends-object-c`'s before-snapshot — the "
        "committed FOUR-collection scene, chosen so the round trip has to preserve four independent id-keyed "
        "orderings and not one. The ceiling: no second producer exists, so a mistake shared by the handcrafted vector "
        "and the production code passes unseen."
    ),
    "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🧪️tests/mutate-puzzle-5d-1/component.feature": (
        "Concretely for this subset: twenty-eight kinds — twenty-eight mutate rows, twenty-eight inverse rows and one "
        "identity-round-trip scenario — each replaying the same "
        "committed `(before, mutation, diff, outcome, after)` bytes the production crate's own fixture test beside "
        "that leaf asserts against, so this case adds the end-to-end platform path rather than a second copy of the "
        "data. `identity-round-trip` re-encodes `🌱create-part/🧪️tests/appends-part-c`'s before-snapshot — the "
        "committed two-part, one-fastener assembly, the smallest document in which a fastener still has two distinct "
        "parts to bind. The ceiling: no second producer exists, so a mistake shared by the handcrafted vector and the "
        "production code passes unseen."
    ),
    "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🧪️tests/mutate-puzzle-2d-1/component.feature": (
        "Concretely for this subset: twenty-six kinds — twenty-six mutate rows, twenty-six inverse rows and one "
        "identity-round-trip scenario — each replaying the same committed "
        "`(before, mutation, diff, outcome, after)` bytes the production crate's own fixture test beside that leaf "
        "asserts against, so this case adds the end-to-end platform path rather than a second copy of the data. "
        "`identity-round-trip` re-encodes `🌱create-node/🧪️tests/appends-node-c`'s before-snapshot — the committed "
        "two-node, one-edge drawing, the smallest document in which the node/edge split described above is actually "
        "present. Two ceilings: no second producer exists, and one of the twenty-six kinds "
        "(`replace-node-handle`) has no vector that moves anything, as recorded above."
    ),
    "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🧪️tests/mutate-lowpoly-1/component.feature": (
        "Concretely for this subset: seventeen kinds — seventeen mutate rows, seventeen inverse rows and one "
        "identity-round-trip scenario — each replaying the same committed "
        "`(before, mutation, diff, outcome, after)` bytes the production crate's own fixture test beside that leaf "
        "asserts against, so this case adds the end-to-end platform path rather than a second copy of the data. "
        "`identity-round-trip` re-encodes `➖️remove-paint-layer/🧪️tests/drops-the-detail-layer-at-index-1`'s "
        "before-snapshot — the committed two-object document with STACKED paint layers, the only one in which the "
        "index-keyed sub-collection has more than one entry to get wrong. The ceiling: no second producer exists, so "
        "a mistake shared by the handcrafted vector and the production code passes unseen."
    ),
}

OLD_START = "Every scenario replays one committed `(before, mutation, diff, outcome, after)` quintet"
ROOT = "/Users/ueli/Documents/semio/"


def wrap(text: str) -> str:
    return "\n".join("  " + line for line in textwrap.wrap(text, width=98, break_long_words=False, break_on_hyphens=False))


changed = 0
for rel, replacement in CASES.items():
    path = ROOT + rel
    original = open(path, encoding="utf-8").read()
    lines = original.split("\n")
    start = next(i for i, l in enumerate(lines) if l.startswith("Feature:"))
    end = next(i for i in range(start + 1, len(lines)) if lines[i].lstrip().startswith("@"))
    head, body, tail = lines[: start + 1], "\n".join(lines[start + 1 : end]).rstrip("\n"), lines[end:]
    out = []
    hit = False
    for para in body.split("\n\n"):
        if " ".join(para.split()).startswith(OLD_START):
            out.append(wrap(replacement))
            hit = True
        else:
            out.append(para)
    assert hit, rel
    rewritten = "\n".join(head + ["\n\n".join(out), ""] + tail)
    if rewritten != original:
        open(path, "w", encoding="utf-8").write(rewritten)
        changed += 1
print(f"rewrote {changed} of {len(CASES)}")
