# GIS Terrain Command Cohort

## Scope

The GIS terrain editor's `setExaggeration`, `setCamera`, and host-pushed `setLocale` rows now use one exact app-owned bounded factory. The media-import route is deliberately not claimed here and remains in the official import ledger.

## Runtime Shape

- All three routes are registered under `s.gis.gisterrain@1/*#editor` and `gis.terrain.tool-command.v1`.
- Scalar exaggeration work is constant. Camera and locale payloads admit at most 8,192 bytes; max+1 fails both the action bridge and retained-work preflight.
- One retained work item dispatches the typed command with the exact app operation context. Oversized wire/checkpoint owners are returned on admission failure.
- The language-neutral JSON limits fixture pins tool ids, exact text bytes, max+1 delta, and work extent; Rust reads it with `serde_json`, while the coordinator independently parsed it with Bun.

## Evidence

- `rustfmt --edition 2021 <GIS terrain editor component>`: exit 0.
- Bun fixture parse/value law: exit 0.
- `git diff --check` for the cohort: exit 0.
- Official tool-job verifier: expected workspace exit 1 for open global/scan/import/remaining ledgers; command remainder fell from 733 to 730, with no GIS terrain command row remaining and no forged proof/factory error.

## Pending Runtime Gate

Focused GIS Rust tests are queued behind the single compiler lease. This packet has static closure only until those tests execute.
