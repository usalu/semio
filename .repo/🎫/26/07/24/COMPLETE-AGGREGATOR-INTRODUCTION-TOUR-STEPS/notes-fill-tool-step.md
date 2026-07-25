# Fix fill-tool intro: wrong tree, premature celebrate, covered panel

## Cause
`introduce: "tool.fill"` was treated as a panel tab id, so the shell drilled into the fill leaf **without** going through tab-selection activation. That rendered the inactive activate-toggle tree (not the normal options tree), could complete/celebrate early, and `placement: "auto"` parked the info box on top of the bottom panel.

## Fix
- Brand `fill-tool`: no `show` of the fill panel; `placement: "top"`.
- Shell: stop opening `tool.*` leaves from introduce ids; for tool-pick interactions, open only the Tool category, clear an already-active tool, and let the user press the leaf (real activation).
- `fill-distribution` still opens the fill panel + keeps fill active via measure targets; also `placement: "top"`.

## Verify
- vitest brand + tool tabs: pass (`vitest-fill-tool-step.txt`)
