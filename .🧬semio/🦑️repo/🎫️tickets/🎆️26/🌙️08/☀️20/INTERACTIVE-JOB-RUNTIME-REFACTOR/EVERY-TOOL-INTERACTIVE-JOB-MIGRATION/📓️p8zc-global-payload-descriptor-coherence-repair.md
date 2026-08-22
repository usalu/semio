# P8zc Global Payload Descriptor Coherence Repair

## Verdict target

Source/static audit-ready for both blocking findings in
`📓️p8zb-independent-global-payload-authority-final-audit.md`. This is not a
compile, generated-code discovery or runtime-pass claim.

## CAD lossless generation domain

The persisted engagement-preview counter now has one cross-surface exact
domain: nonnegative signed 32-bit integer, `0..=2_147_483_647`.

Changed source and descriptors:

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs`
  defines `CAD_PREVIEW_GENERATION_MAX = i32::MAX`, persists `i32`, and rejects
  negative JSON-backed ingestion.
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
  mirrors `i32` in `CadPlayRuntime` and
  `CadPreviewStamp`, rejects negative preview reads, checks the persisted base,
  and uses `checked_add(1)` before emitting a config snapshot. At maximum it
  returns `cad.preview.conflict: engagement preview generation exhausted`.
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️component.rs`
  declares `i32`.
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️component.proto`
  declares `int32`.
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔗️component.graphql`
  declares
  GraphQL `Int` and documents the nonnegative maximum.
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🟦️component.ts`
  retains the exact
  JavaScript `number` representation and documents minimum/maximum.
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️component.json`
  declares integer,
  minimum `0`, maximum `2147483647`.

No preview-generation `u64` or Proto `uint64` remains. The operation identity's
framework-sourced `operation_id` and `operation_generation` remain `u64`; they
are serialized inside the exact operation-identity JSON string and are not the
cross-surface GraphQL/TypeScript preview counter rejected by P8zb.

The source fixture
`preview_generation_cross_surface_domain_round_trips_max_and_rejects_plus_one`
pins:

- exact maximum Rust/JSON round trip and preview reconstruction;
- maximum + 1 rejection during `CadConfig` deserialization;
- checked-increment conflict at maximum;
- JSON descriptor min/max;
- Rust, Proto, GraphQL and TypeScript descriptor type declarations.

The pre-existing exact operation identity, equal-read, +1, ABA, cross-app and
cold-reopen fixtures remain intact.

## Note and Layout lifecycle vocabulary

Changed Note locations:

- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️component.rs` now calls
  the durable accessor region `TextChildren`; its comments and fixtures state
  that `NoteTextChild.paragraphs` owns content. Cache-miss, uncached-handle and
  staleness-gap language is removed.
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
  describes duplication as copying durable paragraphs before reminting the
  child record.
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️edit-block-text/🧪️tests/replaces-the-intro-paragraphs/🦀️component.rs`
  describes
  inverse and diff behavior solely in terms of snapshot-owned records.

Changed Layout location:

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
  describes DWG import and SVG export as retaining and consuming
  `LayoutDrawingChild.content` from the snapshot-owned record. Test names,
  locals and messages use `durable`, `owned`, `content` and `retained`, not
  scratch/cache lifecycle terms.

Exact scans across both plugin trees are clean for `working-scene`,
`working_scene`, `scratch-cache`, `scratch_cache`, `cache-miss`, `cache_miss`,
`uncached`, `never-cached`, the removed cache-oriented test names and the dead
`WorkingScene` region name.

## Static gate results

```text
rustfmt --edition 2021 <seven final-corrective Rust files>
=> exit 0

rustfmt --edition 2021 --check <same files>
=> exit 0

bun -e JSON.parse(CAD config JSON descriptor) + generation assertions
=> parsed; integer, minimum 0, maximum 2147483647

exact CAD Rust/Proto/GraphQL/TypeScript/JSON coherence scan
=> clean; no preview generation u64/uint64 remains

exact Note/Layout stale lifecycle vocabulary scan
=> clean

bun ./📜️script.ts verify interactivity
=> exit 0; DENY mode clean

bun ./📜️script.ts verify interactivity tool-jobs --format json
=> exit 1 expected repository-wide residual:
   34 process-global payload candidates
   12 framework-reserved routes
   875 live command registrations
   bounded rows 9; factories 6; registrations 1; dispatches 1; aliases 4;
   self-tests 12

git diff --check -- <seven repaired plugin cohorts>
=> exit 0

corrected rg --pcre2 added-line debug scan over seven repaired cohorts
=> clean
```

## Unrun gates

Per instruction, no Cargo command, compilation/type/borrow/Send check, build,
unit/integration test execution, native/release/Wasm runtime, actual worker
launch, generated-code regeneration or descriptor discovery, cache deletion,
git mutation, or ticket metadata operation ran.

The complete P8za implementation still requires the independent authorized
native/release/Wasm and isolated-worker execution gates listed in
`📓️p8za-global-payload-authority-repair.md`. The repository-wide tool-job
ledger also remains independently expected-red and mandatory to close outside
this final CAD descriptor/Note/Layout documentation cohort.

## P8zf transition-closure addendum

The later P8zf audit found utility-switch and scene-import session clears that
bypassed the checked preview transition authority. The corrective closure also
found the adjacent active-example reset. All engagement handlers and all three
reset cohorts now route changed checkpoints through one
preview_transition_snapshot_of helper. The ordinary snapshot_of path fails
closed if its serialized session differs, and the raw runtime-to-config packer
is no longer used from any command module.

Production-dispatch source fixtures now cover ordinary transition, utility
clear, import clear, active-example clear, equal-checkpoint no-op, two-app
A→B→A isolation, missing operation context, maximum-generation exhaustion and
direct ordinary-snapshot bypass rejection. See
📓️p8zg-cad-preview-transition-closure.md for the exhaustive call-site
inventory and final permitted static gates.
