# B1 — spr/vcs (grouping + composition-aware checkpoints) report

Scope: `semio-framework-os-kernel`. Primary files —
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` and
`🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs` — plus the persistence path
(`📡️spr/📜️history/🦀️component.rs`) and the collateral one-line struct-literal fixes the ticket's
"COLLATERAL-FIX RIGHTS" section pre-authorized in `🏪️store`, `📡️spr/🧪️testkit`, `📡️spr/🔀️crdt`,
`📡️spr/🔗️causal`, and (self-discovered, same category) `🏪️store/🔄️sync`.

## TASK 1 — `MutationMeta.group_id`

- `📡️spr/🎮️command/🦀️component.rs:389-425` — `MutationMeta` gains
  `pub group_id: Option<String>` (`:424`), `#[serde(default, skip_serializing_if = "Option::is_none")]`
  — attributed and doc-styled identically to the sibling `semantic_kind`/`label` fields immediately
  above it. Doc comment states the semantics (composite-gesture stamp; `Some(id)` = one member of a
  multi-document gesture the future `CompositionCoordinator` dispatches; `None` = solitary edit) and
  records the import-direction decision inline (see below).
- **Persistence (`.spr`)**: `📡️spr/📜️history/🦀️component.rs`
  - `HistoryOpMeta` (`:83-97`) gains `pub group_id: Option<String>`, dict-interned like
    `op_id`/`author_id`/`dependencies` (design note in its doc comment: every sibling member of one
    composite gesture shares the identical string, so the dictionary compresses it almost for free
    across a whole edit/checkpoint).
  - `write_op_meta`/`read_op_meta` (`:602`/`:651`) — new presence bit 4 on the existing single
    presence byte (bits 0-3 already covered `op_id`/`author_id`/`hlt`/`payload_hash`); the field is
    written/read only when the bit is set, appended past the pre-existing tail. A decoder reading a
    byte-log written before this field existed sees bit4 unset (it never existed in the old presence
    byte) and recovers `group_id: None` — no format-version bump needed, matching the file's own
    "every `Option<T>` field... gets its own bitmask [bit]" convention documented above `write_op_meta`.
  - `sample_log()` (`:1652-1667`, the fixture every `history_encode_decode_identity_*` test in this
    file consumes via `assert_eq!(decoded, log)`) now populates `group_id: Some("group-composite-1")`
    on its one `HistoryOpMeta` entry — this is the round-trip proof the task asked for: it is
    exercised by the pre-existing `history_encode_decode_identity_standard` /
    `_full_verification` tests (both still pass, see Verification), not a narrow bolt-on test.
- **Bridge functions** (`🏪️store/🦀️component.rs`, the `command::MutationMeta ⇄ os_spr::HistoryOpMeta`
  translation layer `print_document_spr`/`parse_document_spr` call):
  - `history_op_meta_from_operation_meta` (`:1169-1181`) — `group_id: meta.group_id.clone()`.
  - `mutation_meta_from_history_op_meta` (`:1182-1194`) — `group_id: meta.group_id` (moved).
- **`mutation_envelope_from_edit`** (`📡️spr/🔗️causal/🦀️component.rs:281-309`) — verified, NOT
  extended. Its output type `MutationEnvelope` (`:33-41`, frozen-contract wire/causal envelope) does
  not carry `semantic_kind`/`label`/`payload_hash` either — those three fields are already dropped
  at this exact seam (confirmed by reading the function body: only `dependencies`/`author`/
  `timestamp` are pulled from `MutationMeta`). `group_id` follows the same established precedent and
  is intentionally NOT added to `MutationEnvelope`: this crate's real durable-persistence path is
  `.spr` (`HistoryOpMeta`, extended above), which is what "survives persistence and sync" binds to
  for this wire shape — `MutationEnvelope` is the transient per-op causal-ordering envelope, not a
  second persistence format. Documented at `command::MutationMeta.group_id`'s own doc comment.
- **Round-trip test**: extended (not duplicated) `operation_meta_serde_round_trip` at
  `📡️spr/🎮️command/🦀️component.rs:787-810` — asserts `group_id` serializes under its own field name
  (`MutationMeta` has no `rename_all`, unlike `Edit`), round-trips through `serde_json` byte-for-byte
  equal to the original, and that `None` is omitted from the wire form exactly like the sibling
  optional fields (`skip_serializing_if`).

### `InvocationId` import decision (prominent per task instruction)

`InvocationId` (`pub struct InvocationId(pub String)`) lives in `semio-framework`'s kernel module
(`🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs:46`), landed by A1. Checked the dependency direction
before importing: `🧰️framework/📦️packages/🦀️rust/Cargo.toml:26` declares
`semio-framework-os-kernel` as a dependency of `semio-framework` (not the reverse — confirmed
`semio-framework-os-kernel`'s own `Cargo.toml`
(`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`) has no `semio-framework` dependency at
all). Importing `InvocationId` into this crate (`semio-framework-os-kernel`, which I am editing)
would therefore invert that edge and create a cycle. Per the task's own fallback: `group_id` is
declared as `Option<String>` — the same primitive `InvocationId` wraps — not as `Option<InvocationId>`.
This is documented inline at `📡️spr/🎮️command/🦀️component.rs:412-423` (the field's doc comment) so a
future reader/agent sees the reasoning at the definition site, not only in this report.

## TASK 2 — `Checkpoint.composition_pins`

- `🌿️vcs/🦀️component.rs:99-131` (`🔖️Schemas` region):
  - New `pub struct CompositionPin { pub child_ref: String, pub checkpoint_id: String }`
    (`:109-113`), `#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]` +
    `#[serde(rename_all = "camelCase")]` — matches `Checkpoint`'s own derive/attr set exactly (the
    struct it's embedded in).
  - `Checkpoint` (`:116-131`) gains `pub composition_pins: Vec<CompositionPin>` (`:129`) with
    `#[serde(default, skip_serializing_if = "Vec::is_empty")]`, as specified.
- `content_addressed_checkpoint_id` (`:369-406`, was `:340-360`) — new trailing parameter
  `pins: &[CompositionPin]`. Hash-input extension is conditional: pin bytes are appended to the
  blake3 input **only when `pins` is non-empty**, so an empty-pins call reproduces byte-for-byte the
  exact same hash input the pre-existing (pre-this-ticket) function body produced — this is what
  keeps every checkpoint id ever minted for a non-composite artifact stable, not a version bump.
  When non-empty, pins are re-sorted by `child_ref` **inside the function** (never trusted in
  caller-supplied order) before hashing, each contributing `child_ref || 0 || checkpoint_id || 0` —
  documented inline as the deterministic-convergence guarantee for two peers whose local
  parallel-child dispatch discovers the same child set in different incidental order.
- `child_ref: String` holds the `ArtifactRef` wire URI (`"<artifact_id>!<kind>@<standard>/<subset>"`,
  `to_uri`/`parse_uri` on `ArtifactRef` in `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`, landed by A1),
  not the `ArtifactRef` struct itself — same cross-crate/dependency-direction consideration as
  `InvocationId` above (`ArtifactRef` also lives in `semio-framework`, which depends on this crate,
  not the other way). Documented at `CompositionPin`'s own doc comment (`🌿️vcs/🦀️component.rs:100-108`).
  The URI string doubles as the natural deterministic sort key the hashing function needs.
- `VcsError` (`🌿️vcs/🦀️component.rs:188-198`) gains two variants, styled identically to the
  neighboring `DialectMismatch`/`MigrationRequired`/`MigrationFailed` ("not yet raised by any call
  site — additive only" doc-comment pattern): `CompositionCycle(String)` (`:192`) and
  `OwnershipViolation(String)` (`:197`), both with `#[error("...")]` (this enum derives
  `thiserror::Error`, so `Display`/`Error` need no separate hand-written impl — confirmed no other
  file has an exhaustive `match VcsError { .. }` that would need a new arm; the only cross-file
  matches on `VcsError` are `Err(A) | Err(B) | Err(C) => ...` arms inside larger matches with other
  arms present, so adding variants does not break exhaustiveness anywhere. `fault_from_thiserror!`
  (`🗣️dsl/⚠️diagnostic/🦀️component.rs:297`) is generic over the whole enum via `.to_string()`, no
  per-variant code to extend).
- Unit tests, `🌿️vcs/🦀️component.rs:545-611`
  (`content_addressed_checkpoint_id_composition_pins_are_deterministic_and_backward_compatible`):
  (1) empty pins reproduce the *exact* pre-existing hash bytes — proven by reimplementing the
  pre-pins formula inline in the test and asserting byte-identical output, not just "doesn't
  panic"; (2) a non-empty pin set changes the id vs. no pins; (3) identical pins in identical order
  converge; (4) a different `checkpoint_id` for the same `child_ref` changes the id; (5) the same
  pin *set* in two different incidental orders still converges (proves the internal sort). The
  pre-existing `content_addressed_checkpoint_id_is_deterministic_and_content_sensitive` test
  (`:524-543`) was updated in place to pass the new trailing `&[]` argument at each of its 4 call
  sites — behavior unchanged.

## Collateral construction-site fixes

`MutationMeta { .. }` — ticket's own "known ones" list, one-line `group_id: <default>` each:
- `📡️spr/🧪️testkit/🦀️component.rs:947` (`meta_at` fixture) — `group_id: None`.
- `📡️spr/🔀️crdt/🦀️component.rs:179` (`meta_at` fixture) — `group_id: None`. (`:194`'s
  `meta_with_hash` uses `..meta_at(...)` struct-update syntax — auto-covered, no separate edit.)
- `📡️spr/🔗️causal/🦀️component.rs:713,725` (two-envelope `mutation_envelope_from_edit` test) —
  `group_id: None` each.
- `🏪️store/🦀️component.rs:1182` (`mutation_meta_from_history_op_meta`, the bridge fn itself) —
  `group_id: meta.group_id` (real threading, not a neutral default — this IS the bridge).
- `🏪️store/🦀️component.rs:1325,1471,2668,3008,4930` — `group_id: None` each (the five
  replay/decode-fallback/test sites the ticket listed as `:1181,1323,1468,2664,3003,4924`; line
  numbers shifted by the intervening edits, content matches).
- `📡️spr/🎮️command/🦀️component.rs:796,828` (was `:773,796`) — `:796` extended with
  `group_id: Some("invocation-1")` as part of the round-trip test rewrite above (not a neutral
  default — deliberate, to exercise the field); `:828` (`edit_serde_round_trip`) gets the neutral
  `group_id: None`.

`HistoryOpMeta { .. }` — self-discovered (my own additive field, not in the ticket's pre-enumerated
list, so I grepped for every construction site in the crate before/after my `write_op_meta`/
`read_op_meta` edits):
- `📡️spr/📜️history/🦀️component.rs:672` — the `read_op_meta` function's own `Ok(HistoryOpMeta { .. })`
  return, `group_id` added naturally while editing that function's body.
- `📡️spr/📜️history/🦀️component.rs:1652-1667` (`sample_log()`) — populated with a real
  `Some("group-composite-1")`, doubling as the round-trip proof (see TASK 1 above), not a neutral
  default.
- `🏪️store/🔄️sync/🦀️component.rs:375` (`history_edit_from_envelope`, builds from a
  `MutationEnvelope` which — per the TASK 1 design decision above — carries no `group_id` at all) —
  `group_id: None`.

`Checkpoint { .. }` — same category (additive field of mine, not pre-enumerated), all four real
struct-literal sites in the crate, `composition_pins: Vec::new()` each:
- `🏪️store/🦀️component.rs:743` (`reconcile_alternative`) — paired with updating its
  `content_addressed_checkpoint_id(...)` call to pass a trailing `&[]` (`:742`, comment: reconcile
  checkpoints carry no pins yet, the coordinator that would populate them is a later wave).
- `🏪️store/🦀️component.rs:1370-1383` (`parse_document_spr`'s `HistoryCheckpoint → Checkpoint` map) —
  `composition_pins: Vec::new()`, with an inline comment flagging that `HistoryCheckpoint` (the
  `.spr` durable form) does **not** carry composition pins yet, so this field is in-memory-only
  until a later wave extends that codec (see `## sharedFileRequests`).
- `🏪️store/🦀️component.rs:1534` (ops-text `OpsHeaderLine::Checkpoint` parse) —
  `composition_pins: Vec::new()`.
- `🏪️store/🦀️component.rs:2543-2544` (`ArtifactCommand::CommitCheckpoint` dispatch, the live commit
  path) — paired with updating its `content_addressed_checkpoint_id(...)` call (`:2543`, same `&[]`
  + comment: `ArtifactStore<P, Mutation>` has no notion of owned children yet).
- (`🏪️store/🦀️component.rs:968` is `OpsHeaderLine::Checkpoint { .. }`, an unrelated DSL enum variant
  — not `vcs::Checkpoint` — left untouched. `🛂️manifest/🦀️component.rs:1762` and
  `🧮️math/🧩️wfc/🐾️trail/🦀️component.rs:102` are also unrelated `Checkpoint` types in other crates.)

## Verification (actually run)

```
CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/🎯️target" cargo check -p semio-framework-os-kernel
```
Result: **clean, 0 errors.** Full output: `scratch-w1b1-check-1.txt`. `grep -c "^error"` → `0`.
Warning count: 49 (matches the stated pre-ticket baseline exactly — `warning: \`semio-framework-os-kernel\`
(lib) generated 49 warnings`). Finished in 3m00s (shared target dir, expected under concurrent load).

```
CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/🎯️target" cargo test -p semio-framework-os-kernel --lib
```
Result: **802 passed; 2 failed** (`scratch-w1b1-check-2.txt`, retried unchanged in
`scratch-w1b1-check-3.txt` and a third scoped retry in `scratch-w1b1-check-4.txt`). The 2 failures:
`os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::all_discovered_snapshot_grammars_recognize_their_shipped_fixtures`
and
`os_dsl::fixture_sweep::m5_production_coverage::all_discovered_grammars_report_uncovered_productions_for_their_shipped_fixture`.
Both fail on the SAME three artifacts: `🏗️fem::◻2d::🔖️1`, `📕️norm::📘️en1992::🔖️1`, `🕸️dag::🕸️dag::🔖️1`
— see `## Concurrent-churn observations` below for why these are not mine to fix and not caused by
my diff. All targeted tests for my own changes pass explicitly:
```
test os_vcs::tests::content_addressed_checkpoint_id_is_deterministic_and_content_sensitive ... ok
test os_spr::command::tests::edit_serde_round_trip ... ok
test os_spr::command::tests::operation_meta_serde_round_trip ... ok
test os_spr::history::tests::history_encode_decode_identity_full_verification ... ok
test os_spr::history::tests::history_encode_decode_identity_standard ... ok
test os_vcs::tests::content_addressed_checkpoint_id_composition_pins_are_deterministic_and_backward_compatible ... ok
test os_spr::testkit::tests::history_encode_decode_identity_handles_empty_edits_and_history ... ok
test os_spr::testkit::tests::history_encode_decode_identity_across_profiles ... ok
```
(8/8, `cargo test -p semio-framework-os-kernel --lib -- content_addressed_checkpoint_id
operation_meta_serde_round_trip history_encode_decode_identity edit_serde_round_trip`). Also ran the
full `os_vcs::` module (97/97 passed) as an extra check on my primary VCS file.

**Honest status**: `cargo check` is fully clean and matches baseline exactly. `cargo test --lib` is
NOT fully green (2/804 fail) — I am reporting this truthfully rather than claiming baseline parity.
Per my own analysis below, the 2 failures are pre-existing concurrent churn from a sibling session,
not a regression introduced by this wave's diff, but I have not personally verified a pristine
pre-ticket baseline run (no clean git state was available to compare against, and stashing is
forbidden by repo rules) — flagging this explicitly rather than asserting it as fact.

## `ArtifactRef`/`InvocationId` import decisions

Both cross-crate types (`InvocationId` at `🎠️kernel/🦀️component.rs:46`, `ArtifactRef` at
`🚪️io/🦀️component.rs:162`, both landed by A1) live in `semio-framework`, which depends on
`semio-framework-os-kernel` (confirmed via both crates' `Cargo.toml`: the former lists the latter as
a path dependency; the latter has no dependency on the former at all). Importing either into this
crate's files (`📡️spr/🎮️command`, `🌿️vcs`) would invert that edge and create a cycle. Both fields
therefore use the raw primitive representation instead: `group_id: Option<String>`
(`InvocationId(pub String)`'s wrapped type) and `CompositionPin.child_ref: String` (the `ArtifactRef`
wire URI form `to_uri()`/`parse_uri()` produce/consume). Both decisions are documented at their
field's own doc comment site as well as here.

## sharedFileRequests

1. **`📡️spr/📜️history/🦀️component.rs`'s `HistoryCheckpoint`** (the `.spr` durable form of a
   checkpoint) does not carry `composition_pins` — out of this wave's explicit TASK 2 scope (the
   task's file list for Task 2 named only `🌿️vcs/🦀️component.rs`; TASK 1 explicitly named the
   history file for persistence, TASK 2 did not). Consequence: `Checkpoint.composition_pins` is
   currently **in-memory-only** — a save/load round trip through `.spr` silently drops it (see
   `🏪️store/🦀️component.rs:1370-1383`'s inline comment). A later wave (likely whoever builds the
   `CompositionCoordinator`/commit-cascade that actually populates real pins, per the design doc's
   "commit cascades leaves-first over dirty children") should extend
   `HistoryCheckpoint`/`encode_checkpoint`/`decode_checkpoint` the same way this wave extended
   `HistoryOpMeta`/`write_op_meta`/`read_op_meta` for `group_id`. Region: `🔖️Checkpoint` in that
   file (mirrors the `🔖️Edit` region's op-meta bitmask pattern).
2. **`ArtifactVcs`/checkpoint construction sites outside this crate** (e.g. any UI/renderer code
   that constructs a `vcs::Checkpoint` literal directly, if any exists in `semio-framework` or a
   plugin crate) will need a one-line `composition_pins: Vec::new()` addition once that crate is
   rebuilt against this shape — I did not grep outside `semio-framework-os-kernel`'s own module tree
   (out of my crate boundary; `cargo check -p semio-framework-os-kernel` cannot observe downstream
   crates either).

## Concurrent-churn observations

- `✏️s/🔌️plugins/🕸️dag/**`, `✏️s/🔌️plugins/📕️norm/**`, and `✏️s/🔌️plugins/🏗️fem/**` are under live
  SMO (`SEMANTIC-MUTATIONS-OVERHAUL`) wave-2 fan-out as of this report: `git status --porcelain`
  shows `dag`'s old `📄set-snapshot`/`📋set-edges`/`📋set-nodes`/`🔗nodes` mutation triads deleted and
  new untracked triads (`↔️move-node`, `🌱create-node`, `🏷️rename-node`, etc.) mid-creation; `norm`
  shows dozens of `MM` (staged-then-modified-again, i.e. actively being edited right now) files
  including `en1992` itself; `fem`'s `◻2d` component was last modified today. `SEMANTIC-MUTATIONS-OVERHAUL/📓️wave2-reports/`
  already has `norm-en1992-1-any-report.md` (11:58 today) yet the underlying file shows `MM` — a
  later sub-wave is revising it further as I write this.
- `cargo test -p semio-framework-os-kernel --lib` fails exactly 2 tests
  (`os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::all_discovered_snapshot_grammars_recognize_their_shipped_fixtures`,
  `os_dsl::fixture_sweep::m5_production_coverage::all_discovered_grammars_report_uncovered_productions_for_their_shipped_fixture`),
  both auto-discovery sweeps that scan every plugin's shipped `.semio` fixture against its grammar
  on disk, both failing on exactly `🏗️fem::◻2d::🔖️1`, `📕️norm::📘️en1992::🔖️1`, `🕸️dag::🕸️dag::🔖️1` —
  the three plugins confirmed above to be mid-edit. Retried 3× (`scratch-w1b1-check-2/3/4.txt`,
  same result each time, ~40-90s apart) per the transient-failure protocol; churn did not settle
  within the retry window (SMO's fan-out is a large multi-hour job, not expected to finish in
  minutes).
- Proof these are not caused by my diff: `grep -n "🌿️vcs\|📡️spr/🎮️command\|📡️spr/📜️history\|
  MutationMeta\|CompositionPin\|group_id\|composition_pins"` against the full failing-test output
  (`scratch-w1b1-check-3.txt`) matches only (a) three pre-existing unrelated `unnecessary
  qualification` warnings in `📡️spr/🎮️command/🦀️component.rs:492,533` that predate this ticket
  (`crate::os_vcs::Identified`/`Patchable` used with a redundant path prefix — not code I touched)
  and (b) my own new test passing (`os_vcs::tests::content_addressed_checkpoint_id_composition_pins_are_deterministic_and_backward_compatible
  ... ok`). Zero occurrences of my changed types/fields inside the actual failure/panic text. My own
  files' scoped tests (`os_vcs::`, `os_spr::command::`, `os_spr::history::`) are 100% green.
- Per `📌️important.md`'s protocol: not fixing these (outside my boundary, another session's
  in-progress work), reporting as `blocked-mechanism` for these 2 tests specifically with the proof
  above, while `cargo check -p semio-framework-os-kernel` for the whole crate (which DOES include
  compiling `fem`/`norm`/`dag`'s Rust code, not just their DSL fixtures) stays fully clean at 0
  errors — the churn only manifests as `.semio` grammar-fixture mismatches, not a Rust compile
  break.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` (`MutationMeta.group_id` +
  round-trip test extension + 1 collateral literal)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️component.rs`
  (`HistoryOpMeta.group_id` + `write_op_meta`/`read_op_meta` + fixture)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs` (`CompositionPin`,
  `Checkpoint.composition_pins`, `content_addressed_checkpoint_id` extension, 2 new `VcsError`
  variants, updated + new unit tests)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` (bridge fns + 10 collateral
  struct-literal one-line fixes + 2 `content_addressed_checkpoint_id` call-site fixes)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs` (1 collateral
  `HistoryOpMeta` literal fix)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs` (1 collateral literal fix)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🔀️crdt/🦀️component.rs` (1 collateral literal fix)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🔗️causal/🦀️component.rs` (2 collateral literal fixes)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/scratch-w1b1-check-1.txt`
  through `-4.txt` (cargo check/test output)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave1-reports/b1-spr-vcs-report.md`
  (this report)

`📓️status.md` not touched. Ticket left open (not closed).
