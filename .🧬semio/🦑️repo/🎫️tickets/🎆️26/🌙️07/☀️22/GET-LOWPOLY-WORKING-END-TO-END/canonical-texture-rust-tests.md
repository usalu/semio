# Canonical Texture Rust Test Sources

These are test-source edits against the runtime owner's agreed canonical APIs. No new Native run, Rust compile or Rust assertion pass is claimed.

## Nine Explicit Sites

Eight positive malformed layer-data sites now author complete256² RGBA canvases:
- Binary snapshot schema-identity fixture: the original [1,2,3,255] color fills the full canvas; decoded layers must equal the source in addition to preserving the mesh child.
- IO fixture: a full alternating two-color canvas preserves the original8-byte pattern's field-fidelity intent. The misleading deliberately-noncanonical fixture doc was removed.
- SQLite snapshot fixture constructor: an explicit canonical white canvas embeds the neutral0,1,127,128,255 sentinel prefix. Independent Bun SQLite queries now check262,144 octets, the first5 sentinel values and an all-white remaining tail. The second empty white layer remains valid.
- SQLite controlled native-work fixture: the17-byte-channel pattern fills the complete canonical canvas, preserving cancellation/row-admission intent.
- Four model compositing layers now use complete uniform canvases. Tests check output extent and every pixel, rather than a malformed first-pixel-only prefix.

The ninth site is the engine's former silent-repair test. It now observes LowpolyCoreError::Paint with actual_bytes3, and requires exact unchanged snapshot state. A different object's paint layer is missing at invocation, proving malformed validation occurs before inserting missing layers or materializing white.

## Model Refusal Cases

Two new model tests consume the owning neutral 🎨️texture/🧪️tests/🔣️.json corpus:
- Layer validation/materialization accepts only empty opaque-white shorthand or complete data, preserves the source and exposes the canonical typed refusal code and actual byte length.
- Malformed invisible layers also refuse compositing; visibility must not hide invalid documents.
- Invalid raw brush/fill/sample inputs refuse without panic, crop/pad, replacement or partial mutation. Inputs include partial pixels, other-size square buffers, N−4, N+4 and4MiB.

These tests target the agreed model APIs returning PaintTextureError and engine Result<(),LowpolyCoreError>. The runtime owner still owns production validation and all19 nonwhite JSON records. Light owns79 white JSON records. There are no runtime adapters or migration scripts.

Test-only callers in editor paint/session/mounted now explicitly expect successful materialized_pixels results so a pair of errors cannot accidentally satisfy positive value comparisons. Private retained-fill/comb canonical buffers and the generic pixels corpus were not changed.

## Files

All paths are beneath the Lowpoly subset owner:
- 🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs
- 🚪️io/🧪️tests/🔬️unit/🦀️.rs
- 🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
- 🧬️schema/🧪️tests/🔬️unit/🦀️.rs
- ✏️editor/⚙️engine/🧪️tests/🔬️unit/🦀️.rs
- ✏️editor/🎮️commands/🖌️paint/🧪️tests/🔬️unit/🦀️.rs
- ✏️editor/🖌️session/🧪️tests/🔬️unit/🦀️.rs
- ✏️editor/🧪️tests/🔌️mounted/🦀️.rs

The owning neutral fixture include paths were physically checked to exist. No production source was edited for this task. Rust assertions remain unexecuted.

Source-only compile risk reported to root: the existing SQLite unit file imports SqliteSnapshotControl, SqliteSnapshotPhase and SnapshotEncoding twice in separate use statements. These imports were left unchanged as unrelated to authored texture data. This is not an actual compiler diagnostic; the owning source/test mount must resolve it before its Native selection.

Root-authorized scoped compile correction: after a fresh read, removed only the redundant second imports of SqliteSnapshotControl, SnapshotEncoding and SqliteSnapshotPhase; their first imports remain. SqliteDatabaseLimits/export_sqlite_database/import_sqlite_database remain imported. No production refactor or peer test changes. This resolves a deterministic source-level duplicate-name risk without claiming a Rust compiler execution.

## Current Production Caller Read-Only Review

Fresh source confirms composite validates every layer before allocation/visibility and ensure_all_paint_buffers validates the whole snapshot before insertion/materialization. Paint tick now returns Result and all4 current callers propagate it. ApplyPaintStroke.runs returns Result<Option<Vec<PixelRun>>,ValueError>; diff maps typed refusal to fatal and inverse propagates Result.

Actionable issue sent to runtime owner: Fault::from(string) produces app.message according to framework diagnostic lines651-659. New paint maps using Fault::from(canonical-code-string) therefore lose the canonical fault code; use an explicit typed fault constructor retaining actual bytes/message. Also reported obsolete irrefutable Some(runs) branch/square-inference docs in ApplyPaintStroke diff, unreachable fallback channel getters in validated composite, and session texture-cache silent-skip policy. Runtime owns these production corrections. Decode/admission checks, localized notice and infallible inverse boundary were explicitly acknowledged as pending by the production owner; this report does not claim those boundaries finished.
