# -*- coding: utf-8 -*-
"""📝️ S18 §14c: landing row for the tree disclosure-label element id (idempotent)."""
import pathlib

LANDING = pathlib.Path("/Users/ueli/Documents/semio/.tmp-ticket/📓️landing.md")
ROW = "| S18 | 14c — engine-contract `per-window element ids` reds (world projection panes): a tree row's disclosure label (the fold button's `aria-labelledby` target) carried React's raw `useId()` (`_r_3_`) as its DOM id — not an element id, not addressable; the 2nd red was a cascade (the 1st failed before `cleanup()`, the next mount's `getElementById` hit the stale pane's toggle). Now `childElementId(<row id>, \"disclosureLabel\")`, else `ui.tree.disclosureLabel.<generated>`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx`; temporary `[DEBUG] s18 ids` in the engine-contract law removed in the same pass; codemod `wp-s18/s18-14c-tree-label-id.py`. NOT the 17:47 World3dHost edit (that one only memoizes leftover overlays) | TS only (ui + host) | tsc 0 (`.🧬semio/🌐hub/s14-s18-captures/s18-14c-tsc-tree-1.txt`); laws: see `s18-14c-law-table-5.txt` | 29 01:4x |"


def main() -> None:
    text = LANDING.read_text(encoding="utf-8")
    if ROW[:70] not in text:
        LANDING.write_text(text.rstrip("\n") + "\n" + ROW + "\n", encoding="utf-8")
        print("added 1")
    else:
        print("added 0")


main()
