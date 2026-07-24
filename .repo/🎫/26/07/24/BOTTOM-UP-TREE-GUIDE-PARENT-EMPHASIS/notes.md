# Notes

## Problem

Bottom-anchored trees (`direction="up"`) rendered children above parents, but:

1. Branch stems still extended from the row mid-line toward the **bottom**, leaving a gap at the parent junction.
2. Hover/selection path helpers only resolved the owner row as the **previous** sibling of a branch (and terminal branch as the **next** sibling of a row), so parents above/below the flipped content never received path emphasis.

## Fix

- `TreeDocumentGutter` reads `TreeContext.direction` and extends `tree-branch-stem` toward children (`top→anchor` when up, `anchor→bottom` when down). Prop renamed to `extendBranchStem`.
- `rowForBranch` / `markTerminalBranch` inspect both sibling sides (and collapsible-content both sides for sections).
- `WindowMeasuresTree` / nested providers pass `direction` through `TreeContext`.
