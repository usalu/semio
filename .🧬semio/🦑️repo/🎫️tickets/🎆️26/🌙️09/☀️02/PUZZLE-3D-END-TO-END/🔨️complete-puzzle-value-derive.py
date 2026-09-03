#!/usr/bin/env python3
"""🔨️ Completes the stalled value-derive migration in the puzzle plugin.

Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS converts serde derives to
`value_derive::ToValue`/`FromValue`. Fifteen puzzle types (artifact / diff / inference / config /
presence schema, x 2d/3d/5d) were given the `#[value(...)]` container attribute but never the
derives that register it, leaving the plugin with 201 errors: 15 x "cannot find attribute `value`"
(the attribute is registered by the derive, and the co-located `ArtifactSchema` derive registers
only `artifact_schema`/`state`/`derived`/`child`/`link_slot`) and 180 x
"`Puzzle{2,3,5}dDiff: ToValue/FromValue` is not satisfied" downstream.

Adds `value_derive::ToValue, value_derive::FromValue` to the `#[derive(...)]` that precedes each
item-level `#[value(...)]`. Idempotent: an item that already derives them is skipped.
"""
import re
import sys

TARGETS = [
    ("◻2d", "🧬️schema/🦀️.rs"), ("🧊️3d", "🧬️schema/🦀️.rs"), ("🖐️5d", "🧬️schema/🦀️.rs"),
    ("◻2d", "🧬️schema/🔺️diff/🦀️.rs"), ("🧊️3d", "🧬️schema/🔺️diff/🦀️.rs"), ("🖐️5d", "🧬️schema/🔺️diff/🦀️.rs"),
    ("◻2d", "🧬️schema/💡️inferences/🦀️.rs"), ("🧊️3d", "🧬️schema/💡️inferences/🦀️.rs"), ("🖐️5d", "🧬️schema/💡️inferences/🦀️.rs"),
    ("◻2d", "✏️editor/🎚️config/🧬️schema/🦀️.rs"), ("🧊️3d", "✏️editor/🎚️config/🧬️schema/🦀️.rs"), ("🖐️5d", "✏️editor/🎚️config/🧬️schema/🦀️.rs"),
    ("◻2d", "✏️editor/👥️presence/🧬️schema/🦀️.rs"), ("🧊️3d", "✏️editor/👥️presence/🧬️schema/🦀️.rs"), ("🖐️5d", "✏️editor/👥️presence/🧬️schema/🦀️.rs"),
]
BASE = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{dim}/🏅️standards/🔖️1/🪆️subsets/✳️any/{rest}"
DERIVES = "value_derive::ToValue, value_derive::FromValue"


def patch(path, apply):
    lines = open(path, encoding="utf-8").read().split("\n")
    changed = 0
    for index, line in enumerate(lines):
        if not line.lstrip().startswith("#[value("):
            continue
        cursor = index - 1
        derive = None
        while cursor >= 0 and (lines[cursor].lstrip().startswith("#[") or lines[cursor].lstrip().startswith("///") or not lines[cursor].strip()):
            if re.match(r"\s*#\[derive\(", lines[cursor]):
                derive = cursor
                break
            cursor -= 1
        if derive is None or DERIVES.split(",")[0] in lines[derive]:
            continue
        lines[derive] = re.sub(r"\)\]\s*$", f", {DERIVES})]", lines[derive])
        changed += 1
    if changed and apply:
        open(path, "w", encoding="utf-8").write("\n".join(lines))
    return changed


if __name__ == "__main__":
    apply = "--apply" in sys.argv
    total = 0
    for dim, rest in TARGETS:
        path = BASE.format(dim=dim, rest=rest)
        count = patch(path, apply)
        total += count
        if count:
            print(f"{'patched' if apply else 'would patch'} {count}: {path}")
    print(f"total items: {total}")
