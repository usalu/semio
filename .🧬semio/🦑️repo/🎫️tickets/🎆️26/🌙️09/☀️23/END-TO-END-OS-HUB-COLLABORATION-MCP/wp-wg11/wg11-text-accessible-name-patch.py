#!/usr/bin/env python3
"""♿️ WG11 session 14 — prepared patch for window 3: a painted text paragraph speaks its text to assistive technology.

Measured (run wr-1/wr-2, 7800 B3, wasm32 block2d): the board paints `6 Handle Kinds, 11 Handles` (`dumpStructure` text) while its
accessibility projection announces `block2d-play-board.summary` / `.counts` as `paragraph` nodes with NO name — the mirror
(`#semio-wgpu-accessibility`) and the native AccessKit tree hand a screen reader an empty paragraph; React renders the same text
as DOM text. Root: the shared projection (`accessibility_projection_node`, Rust, and its TypeScript twin
`uiAccessibilityProjectionNodeV1`) derives a component's own name only for section/group containers, buttons, tree items and
tables — a `Text` component's `value` was never its name. Fix (both twins): a Text node without an explicit
`AccessibilitySpec.label` is named by its value (an explicit label still wins, the existing `#status` row). Shared fixture
`🧬️contract/🧫️fixtures/♿️accessibility-projection.json` gains `#caption` (a text node with no label → named by its value; not
reachable by name, `announced` unchanged); the contract's Rust law, the wgpu target's projection law and the TypeScript runner all
answer it.

Dry run by default; `--apply` writes; `--revert` restores (every anchor asserted exactly once).
"""

import difflib
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
CONTRACT = ROOT / "🧰️framework/🔨️modules/🖱️ui/🧬️contract"
RUST = CONTRACT / "♿️accessibility/🦀️.rs"
TS = CONTRACT / "♿️accessibility/🟦️.ts"
FIXTURE = CONTRACT / "🧫️fixtures/♿️accessibility-projection.json"

EDITS = {
    RUST: [
        (
            """        crate::Component::Table(props) => Some(&props.label),
        _ => None,
    });""",
            """        crate::Component::Table(props) => Some(&props.label),
        crate::Component::Text(props) => Some(&props.value),
        _ => None,
    });""",
        ),
    ],
    TS: [
        (
            """  const componentLabel = record.component.type === "button" || record.component.type === "treeItem" || record.component.type === "table" || (record.component.type === "container" && (record.component.role === "section" || record.component.role === "group")) ? record.component.label : null;""",
            """  const componentLabel =
    record.component.type === "text"
      ? record.component.value
      : record.component.type === "button" || record.component.type === "treeItem" || record.component.type === "table" || (record.component.type === "container" && (record.component.role === "section" || record.component.role === "group"))
        ? record.component.label
        : null;""",
        ),
    ],
    FIXTURE: [
        (
            """        "children": [1, 3, 4, 5, 6, 7, 8, 9, 10]""",
            """        "children": [1, 3, 4, 5, 6, 7, 8, 9, 10, 11]""",
        ),
        (
            """        "accessibility": { "label": "Explicit override" }
      }
    ]
  },""",
            """        "accessibility": { "label": "Explicit override" }
      },
      {
        "id": 11,
        "key": "#caption",
        "component": { "type": "text", "value": "6 Handle Kinds, 11 Handles" },
        "layout": { "kind": "leaf", "width": "fill", "height": "hug" },
        "style": {},
        "activity": "idle",
        "accessibility": {}
      }
    ]
  },""",
        ),
        (
            """    { "nodeId": 10, "key": "#explicit-tree-item", "depth": 1, "role": "treeitem", "label": "Explicit override", "description": null, "live": "off", "shortcut": null, "hidden": false, "disabled": false, "focusable": true, "tabbable": true, "actionable": false }
  ],""",
            """    { "nodeId": 10, "key": "#explicit-tree-item", "depth": 1, "role": "treeitem", "label": "Explicit override", "description": null, "live": "off", "shortcut": null, "hidden": false, "disabled": false, "focusable": true, "tabbable": true, "actionable": false },
    { "nodeId": 11, "key": "#caption", "depth": 1, "role": "paragraph", "label": "6 Handle Kinds, 11 Handles", "description": null, "live": "off", "shortcut": null, "hidden": false, "disabled": false, "focusable": false, "tabbable": false, "actionable": false }
  ],""",
        ),
    ],
}


def replaced(path, source, edits):
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:90]!r}")
        source = source.replace(old, new)
    return source


def main():
    revert = "--revert" in sys.argv
    apply = "--apply" in sys.argv or revert
    for path, edits in EDITS.items():
        before = path.read_text(encoding="utf-8")
        after = replaced(path, before, [(new, old) for old, new in edits] if revert else edits)
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
        if apply:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'REVERTED' if revert else 'APPLIED' if apply else 'DRY RUN'}: {len(EDITS)} files")


if __name__ == "__main__":
    main()
