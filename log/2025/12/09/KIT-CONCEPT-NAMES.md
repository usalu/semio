---
date:
  created: '2025-12-09T18:14:57.864Z'
  updated: '2025-12-09T18:14:57.864Z'
slug: KIT-CONCEPT-NAMES
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Display concept names in kit app rows
model: claude-opus-4.5
prompts: []
commit: unknown
affectedFiles: []
lines:
  added: 0
  removed: 0
---
# Previously

# Concept rows in the kit app rendered GUIDs because KitStore snapshots only kept `{guid, name: guid}` placeholders and ignored concept diffs, so names never flowed through UI consumers.
# Home screen concept filters expected KitShallow concepts as GUID strings and could not resolve concept names when snapshots supplied objects.

# Plan

# Locate where kit concepts are stored and surfaced to the UI.
# Refactor KitStore to persist full Concept data and process concept diffs, then ensure consumers read names correctly.
# Update developer docs and log the work.

# Changes

# Added a ConceptStore layer in `Sketchpad.tsx` with Y.js backing, wired KitStore to use it for snapshotting, diff handling, and persistence reconstruction.
# Updated Sketchpad persistence bootstrap to rebuild concepts from the `yConcepts` array (with legacy GUID fallback) and cleared caches on concept edits.
# Hardened Home concept filtering to handle concept entries as GUIDs or objects so name resolution stays stable.
# Documented the concept storage mechanism in `README.md` and `AGENTS.md`.
