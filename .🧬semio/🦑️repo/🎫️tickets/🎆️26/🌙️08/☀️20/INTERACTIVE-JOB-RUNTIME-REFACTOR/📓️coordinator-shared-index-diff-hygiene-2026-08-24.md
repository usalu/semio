# Coordinator Shared Index Diff Hygiene — 2026-08-24

## Current Result

`git diff --check` on the working tree exits zero, apart from Git's informational warning that an
unrelated end-to-end DXF fixture has CRLF content.

`git diff --cached --check` still exits nonzero for six trailing-space lines in the older staged
snapshot of three coordinator reports:

- P3m Engine GPU surface census, date and owner lines;
- P3n Engine surface retirement audit, date and owner lines; and
- Phase 10 mandated orchestration boundary, date and owner lines.

The current working files have been repaired with `apply_patch` and no longer contain those spaces.
The cached index still holds its earlier version.

## Preservation Rule

Repository instructions forbid agents from running modifying Git commands, including staging.
Therefore the coordinator did not run `git add`, reset, checkout, restore, stash, or any index
mutation to hide the cached failure.

The final serialized gate owner must rerun both working and cached checks after the human/shared
index owner naturally refreshes the staged snapshot. Until then, reports must distinguish:

- current file content: clean; and
- cached staged snapshot: stale and RED.

The DXF warning belongs to the concurrent end-to-end testing ticket and is not modified by this
master refactor.
