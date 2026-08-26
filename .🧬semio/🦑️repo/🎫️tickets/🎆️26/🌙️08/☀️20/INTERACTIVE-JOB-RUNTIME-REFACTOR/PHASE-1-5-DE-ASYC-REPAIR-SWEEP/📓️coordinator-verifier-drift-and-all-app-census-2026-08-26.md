# Coordinator Verifier Drift and All-App Census

Date: 2026-08-26

## Scope

Revalidate the root interactivity gates after the shared retained-command checkpoint changed from ARC1 v1/32 bytes to ARC1 v2/40 bytes and app-owned immutable request context became part of checkpoint identity.

## Repairs

- Updated the retained-command verifier to require:
  - the 40-byte ARC1 v2 header;
  - exact v2 decoding;
  - context-digest capture and fail-closed mismatch;
  - app-owned context retention;
  - the schema fixture plus byteorder oracle;
  - the live v2 replay cursor at byte 40.
- Added hostile mutations for checkpoint version and context-identity comparison.
- Updated the typed-operation publication verifier to recognize the formatted two-line exact-owner detachment.
- Updated child-content capture to require the new Arc request-context lane.
- Updated the Draw render-owner law and its hostile fixture to require the request-context renderer.
- Restored explicit Migrated action declarations for three setContributions proof rows instead of relying on constructor mutation.

## Executed Evidence

Command: bun ./📜️script.ts verify interactivity tool-jobs --self-test

- Exit: 0
- Result: self-tests=466 clean

Command: bun ./📜️script.ts verify interactivity tool-jobs --format json --output <ticket>/📊️coordinator-official-tool-jobs-current-2026-08-26.json

- Exit: 1, expected while the migration ledger remains open.
- The four stale framework failures for Draw ownership, typed full-operation bounding, persistent preparation, and child-root capture are gone.
- The three stray-proof failures are gone.
- Current truthful open ledger:
  - 36 process-global payload-store candidates;
  - 35 app-owned import-media routes;
  - 743 live command registrations.

Command: bun ./📜️script.ts verify interactivity apps actions

- Exit: 0
- Descriptors: 32
- Extensions: 4
- Apps: 101
- Launch-only products: 57
- Surfaces: 158
- Actions: 4,760
- Migrated actions: 1,979
- Missing actions: 2,781
- Launch-covered apps: 101
- Launch-missing apps: 0
- Launches: 237
- Failures: 0
- Self-tests: 25

## Evidence Files

- 📊️coordinator-official-tool-jobs-current-2026-08-26.json
- This report

## Status

The root gates are internally consistent again. The phase remains open because the reported command, importer, and process-global inventories are real implementation work, not verifier drift.
