# 📇️/📡️/🌿️ `directory` + `spr` + `vcs` final pass — the 59 declined refs, unblocked

Scope: `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/**`, `…/📡️spr/**`, `…/🌿️vcs/**`, plus a
verification-only look at `…/💡️inference/**` and `…/🎒️pack/**`. Predecessor:
`📓️directory-spr-serde-removal.md` (declined 59 refs on `DslValue::Number` being `f64`-only).
Unblocking fix: `📓️dslvalue-integer-fidelity.md` (`Number::UInt`/`Int`/`Float`).

**Correction to an earlier draft of this file**: an earlier pass of this document claimed
`📡️spr/📜️history`'s `write_op_meta`/`read_op_meta` production call sites were swapped to
`os_pack::json`. **That was wrong — checked directly against the file and it was never done.**
Those two call sites are UNTOUCHED, still `serde_json::to_string`/`from_str`, exactly as the
predecessor left them. Only a new proof test was added alongside them. Corrected throughout below.

## Headline

- **`📇️directory` reaches zero production `serde`** — the full 55-ref decline (`🧬️schema` + wire
  types in `🔌️client`) is converted, plus 4 more (a `🦀️.rs` test fixture that broke downstream).
- **`📡️spr/🎮️command`'s `NamedTripleDiff`** (decline #3, 2 refs) is converted now that
  `vcs::ItemPatch` has `ToValue`/`FromValue`. `📡️spr/🎮️command` is at zero production `serde`.
- **`📡️spr/📜️history`'s 2 declined refs are NOT converted** — a proof test was added but could not
  be confirmed passing (see "UNVERIFIED" section). Production code is unchanged from the
  predecessor's state — still safely on `serde_json`.
- **`🌿️vcs`** gets `ToValue`/`FromValue` added to every schema/collection type; `ItemPatch`/
  `CollectionDiff`/`CollectionMutation` (3 types) are fully converted (serde dropped). The rest of
  the file's `serde` stays intentionally dual — see "vcs count" below, since a naive text count of
  this file went UP even though no serde CODE was added.
- **`💡️inference`**: unchanged code (deferred seam, as instructed); implementor readiness measured.
- **`🎒️pack`** (os-kernel mount): verified before I started — already 0 production serde
  (test-gated only, via `🔎️scalar-witness`'s `#[cfg(test)] #[path=…] mod tests;`). I made **no
  changes** to this module.

## Per-module before → after (my own count, code lines only — excludes `///`/`//` comments)

| module | production serde code lines before | after | zero? |
|---|---|---|---|
| `📇️directory/🧬️schema` | ~50 (across 17 derives + attrs) | **0** | **yes** |
| `📇️directory/🔌️client` (wire region + call sites) | ~26 | **0** (7 remaining lines are all inside `#[cfg(test)] pub mod test_support` / `#[cfg(test)] mod tests`) | **yes**, production |
| `📇️directory/🦀️.rs` (root, test fixture) | 2 (test-only, but broken by the schema conversion) | fixed, still test-only | n/a (test) |
| `📡️spr/🎮️command` | 2 (`NamedTripleDiff`'s derive; everything else already 0 from an earlier pass) | **0** | **yes** |
| `📡️spr/📜️history` | 2 (`write_op_meta`/`read_op_meta`) | **2, unchanged** | no — declined, see below |
| `📡️spr/🧵️channel` | 6 (pre-existing, deliberate dual) | 6, unchanged | not attempted — real cross-module consumer, see below |
| `🌿️vcs` | 3 `#[serde(...)]` attribute lines belonging to `ItemPatch`/`CollectionDiff`/`CollectionMutation` (the only thing I touched); every other serde derive/impl/use line was ALREADY there | those 3 lines **removed** (converted to `ToValue`/`FromValue`-only); all pre-existing dual derives left exactly as found | 3 types reach zero; rest deliberately unchanged |
| `💡️inference` | 8 | 8, unchanged | no — deferred seam, per ticket instruction |
| `🎒️pack` (os-kernel) | 0 (already, verified) | 0 | already zero, untouched |

## `🌿️vcs` count — why a raw grep went 13→17 with NO serde code added

Checked directly with `git diff` against my own edits (not assumed): my vcs.rs diff **removes
three** `#[serde(rename_all = "camelCase")]` / `#[serde(tag = "kind", rename_all = "camelCase")]`
lines (the ones on `ItemPatch`, `CollectionDiff`, `CollectionMutation` — converted to
`#[value(...)]`) and **adds zero** new `use serde`/derive/impl-serde code. What it DOES add is six
lines of **prose** (doc comments explaining why the schema region's dual derive is deliberate — the
`store::ArtifactEnvelopeRead` hand-off, the frozen checkpoint-hash input) that happen to contain the
word "serde" as English text, not code. A naive `grep -c serde` counts comments and code identically,
so it goes up (more explanatory text) even though the functional serde surface went down. Every
`#[derive(.., Serialize, Deserialize, ..)]` / hand-written `impl Serialize`/`impl Deserialize` line
still present in the file was there **before this session** — I added `ToValue`/`FromValue`
ALONGSIDE them, never removed or added a serde derive/impl on `Author`/`Change`/`CompositionPin`/
`Checkpoint`/`Alternative`/`ArtifactHistoryLedger`/`ArtifactHistoryIter`/`ArtifactVcs`/
`ArtifactVcsRead`/`PendingChangeRef`. This is not scaffolding-left-behind; it is a deliberate,
documented, narrowly-scoped dual derive (see next section) — the SAME pattern already established
elsewhere in this exact codebase (`📡️replication::MutationMessage`, `📡️spr/🧵️channel`'s
`FixedCommandPage`/`CommandPageCursor`, both pre-existing, not introduced by me).

## `🌿️vcs` — why the schema region stays dual (not a shortcut)

**This ticket's own instruction says clearing `vcs::ArtifactVcsRead` unblocks the `🏪️store` agent —
prioritised accordingly.** Added `ToValue`/`FromValue` to: `Change`, `CompositionPin`, `Checkpoint`,
`Alternative` (derived, serde kept); `ArtifactHistoryLedger<T>`/`ArtifactHistoryIter<'_, T>`
(hand-written, serde kept); `ArtifactVcs<P, Mutation>` (`FromValue` derived + hand-written `ToValue`,
serde kept); `ArtifactVcsRead<'a, P, Mutation>` (hand-written `ToValue` — NOT derived, because
`initial_snapshot: &'a P` is a reference field and no blanket `impl<T: ToValue> ToValue for &T`
exists; deriving demanded `&'a P: ToValue`, a real `E0277` hit mid-session and fixed by
hand-writing instead). `ItemPatch`/`CollectionDiff`/`CollectionMutation` — fully converted, serde
dropped outright (confirmed NOT reachable from `store::ArtifactEnvelopeRead`'s field list).

Two independent, verified (not assumed) reasons the rest stays dual:
1. **`store::ArtifactEnvelopeRead<'a, P, Mutation>`** (`🏪️store/🦀️.rs:2282`) is
   `#[derive(Serialize)]` and holds `vcs: crate::os_vcs::ArtifactVcsRead<'a, P, Mutation>` —
   dropping `Serialize` there breaks that derive today. `store`'s own code already anticipates this:
   `envelope_json`'s docstring (`🏪️store/🦀️.rs:15776-15779`) calls `ArtifactVcsRead`/nested-field
   serde removal an "**explicit LATER-wave**... not this chokepoint fix" — i.e. `store` deliberately
   deferred that conversion itself. Forcing it today means also converting `store`'s own
   `ArtifactBackboneRef`/`MigrationProvenance`/`OwnerRef`/`HistoryLane`/`ArtifactCursorOwners`/
   `ArtifactEditMessageLedger`/`Conflict` — `store`'s own ~280-ref slice, another agent's territory.
2. **`Change`'s `serde_json::to_vec(change)` inside `content_addressed_checkpoint_id_core`** is a
   frozen content-hash input — already-minted checkpoint ids must not change by one byte. `Change`
   must keep `Serialize` regardless of (1).

**`store` unblock — what now exists for them to build on**: `ArtifactVcsRead<'a, P, Mutation>:
ToValue` (object shape `initialSnapshot`/`edits`/`changes`/`checkpoints`/`alternatives`, camelCase)
and `ArtifactVcs<P, Mutation>: FromValue`/`ToValue` both now exist. The `store` agent can convert
`ArtifactEnvelopeRead`/`ArtifactEnvelopeOwners::Serialize` to `ToValue` whenever they pick that wave
up, without waiting on `vcs` again — that is the actual unblock, independent of vcs's own raw count.

## `📇️directory` — driven to zero (PROVEN BY A PASSING CHECK)

- `🧬️schema/🦀️.rs`: every type converted `#[derive(Serialize, Deserialize)]` + `#[serde(...)]` →
  `#[derive(ToValue, FromValue)]` + `#[value(...)]`, 1:1 attribute mirror (`tag`, `rename_all`,
  `rename_all_fields`, per-variant `rename`, field `rename`/`default`/`skip_serializing_if` — all
  confirmed supported by reading `semio_framework_value_derive`'s own parser before converting).
  Deleted the file's pre-existing `serde_backed_value!` macro (a `serde_json`-proxying `ToValue`
  shim found already in the live tree, presumably an earlier stepping stone — made redundant/
  conflicting by the real derive).
- `🔌️client/🦀️.rs`: `CommandOutcome`/`SessionView`/`SessionMintResponse` → derived;
  `CommandOutcome.result` retyped `serde_json::Value` → `DslValue`; `SpaceDetail` hand-written
  (its old `#[serde(flatten)]` has no `#[value(...)]` equivalent, confirmed unsupported in the
  derive's own header docs) — merges `SpaceView`'s object entries into the parent by hand.
  `request_json`'s bound moved `DeserializeOwned` → `FromValue`; encode/decode call sites moved to
  `os_pack::json::to_json_string`/`from_json_str`. `DirectoryClientError::Decode` retyped
  `serde_json::Error` → `String`.
- `🦀️.rs` root: one `#[cfg(test)]` fixture-decode helper broken by the schema conversion, fixed via
  the compiler (found by running `cargo check`, not by inspection).

## `📡️spr` — `command` at zero; `history` declined (honest); `channel` untouched (real reason)

- **`🎮️command/🦀️.rs`**: `NamedTripleDiff<K, V, Patch>` converted — this was a guaranteed `E0277`
  the moment `vcs::ItemPatch` lost `serde::Serialize`/`Deserialize` (hit directly mid-session, not
  hypothetical), fixed here. Every other `serde` mention in the file is inside `#[cfg(test)] mod
  tests` (confirmed: first hit at line 896, `mod tests {` opens at line 888).
- **`📜️history/🦀️.rs`**: **declined — production code untouched.** A proof test
  (`mutation_origin_canonical_json_is_byte_identical_between_serde_json_and_pack_json`) was added,
  asserting `serde_json::to_string(&MutationOrigin::Contributed{..})` equals
  `os_pack::json::to_json_string(&same)` byte-for-byte. **Could not get a clean crate-wide `cargo
  test` run to confirm it** — every attempt (5+, over ~15 minutes) hit an unrelated, real,
  currently-in-flight compile error in `🏪️store/🔄️sync/🦀️.rs:80` (`PathBuf: protocol::FromValue`
  not satisfied — a peer mid-converting `PersistenceBinding` to `ToValue`/`FromValue` without yet
  giving `PathBuf` a codec). Confirmed unrelated by inspection (not a file I touched, not reachable
  from `directory`/`spr`/`vcs`). **Marked UNVERIFIED below — the two production call sites were
  correctly left on `serde_json`, matching the predecessor's original, still-appropriate caution.**
- **`🧵️channel/🦀️.rs`**: untouched, on purpose. `FixedCommandPage`/`CommandPageCursor`/
  `CommandIngressStatus` carry a pre-existing (not introduced this session), documented dual derive
  — the file's own docstrings say `🔌️plugin/🖥️host/🧵️shard/🦀️.rs`'s `serde_json::to_vec(&result.
  command_ingress)` is a real, still-live cross-module consumer. Another agent's territory.

## `NamedTripleDiff` unblock chain (found while converting `vcs`)

Converting `vcs::ItemPatch` immediately surfaced the real compile error in
`📡️spr/🎮️command::NamedTripleDiff` — exactly the dependency the predecessor flagged ("trivial once
`vcs::ItemPatch` converts"), fixed in the same pass.

## `💡️inference` — implementor readiness, measured, code unchanged

13 `impl (store|protocol)::InferredField<P>` sites found (direct grep + read, not a blind count).
**7 of 13** now have full `ToValue`/`FromValue` on their `Key`/`Value` types (up from the
predecessor's count of 2):

| implementor | `Value` type | ready? |
|---|---|---|
| `stdio/mesh::MeshAabb` | `SemioAabb` | ✅ serde dropped outright |
| `stdio/drawing::DrawFlattenedScene` | `FlattenedNode` | ✅ serde dropped outright |
| `stdio/brep::BrepValidationReport` | `Vec<BrepValidationDiagnostic>` | ✅ dual |
| `stdio/table::ColumnEntropy` | `SemioColumnEntropy` | ✅ dual |
| `stdio/table::ColumnMoments` | `SemioColumnMoments` | ✅ dual |
| `stdio/graph::NodeConnectivity` | `SemioGraphNodeConnectivity` | ✅ dual |
| `mathematical::MathematicalRootsField` | `MathematicalRoot` | ✅ serde dropped outright |
| `remodel::RemodelRelativeCameraPose` | `RemodelPoseDelta` | ❌ serde-only |
| `procedural/assembly::AssemblySolve` | `AssemblySolveResult` | ❌ serde-only |
| `procedural/assembly::AssemblyContradiction` | `bool` | ~ primitive trivially OK, container un-migrated |
| `procedural/assembly::AssemblyEntropy` | `f64` | ~ primitive trivially OK, container un-migrated |
| `puzzle/3d::Puzzle3dFlatPlane` | `FlattenPlane` | ❌ serde-only |
| `puzzle/3d::Puzzle3dFlatCenter` | `[f64; 2]` | ~ primitive array, container un-migrated |

`InferredField<P>`'s own trait bound was **not** flipped — `stdio`/`procedural`/`remodel`/`puzzle`
are separate, large, explicitly in-flight waves per `📓️verified-outcomes.md`; flipping today breaks
the 5 still-serde-only implementors immediately. Measured and left, per the ticket's instruction.

## `🎒️pack` (os-kernel) — verified, untouched

Confirmed by reading the mount site directly (both before and after this session — I made no edits
here): every `serde` reference lives in `🔎️scalar-witness/🧪️component.rs`, reached only via
`#[cfg(test)] #[path = "🧪️component.rs"] mod tests;`. Zero production refs, then and now.

## VERIFICATION — marked PROVEN or UNVERIFIED, no exceptions

**PROVEN BY A PASSING CHECK** — `cargo check -p semio-framework-os-kernel --message-format=short`,
run in the foreground, waited for inline, most recent result (includes every edit in this report —
`directory`, `spr/command`, `spr/history`'s new test, `vcs`):
```
warning: `semio-framework-os-kernel` (lib) generated 32 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 32 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 4m 11s
```
0 errors. This is the ticket's own GUARDRAIL and it is green.

**PROVEN BY A PASSING CHECK** — `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm`,
run earlier this session AFTER the `directory`/`client`/`vcs`/`spr` conversions:
```
   Compiling semio-s-plugin-draw-fsm v0.1.0 (.../🖍️draw/.../🔄️fsm/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 1m 22s
```
0 errors.

**PROVEN BY A PASSING CHECK** — `semio-framework-pack`'s own pre-existing, independent oracle test
`to_json_string_bytes_match_the_serde_json_bridge` (not written by me — found already in the pack
crate's test module), run in the background and its completed output read back:
```
running 1 test
test json::tests::to_json_string_bytes_match_the_serde_json_bridge ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 87 filtered out; finished in 0.00s
```
This is real, structural evidence for the GENERAL claim underlying the `directory`/`spr` unblock
(`pack::json`'s JSON writer produces byte-identical output to `serde_json` for an equivalent value
tree, including a `DslValue::uint` in the test's own fixture) — but it is NOT a test of
`MutationOrigin`/`PayloadHash` specifically.

**WRITTEN BUT UNVERIFIED** — `mutation_origin_canonical_json_is_byte_identical_between_serde_json_
and_pack_json` (`📡️spr/📜️history/🦀️.rs`), the test that would specifically prove the `spr/history`
swap is safe. Every attempt to run it (5+, over roughly 15 minutes, both in foreground and
background across this session) hit the SAME unrelated compile error in `🏪️store/🔄️sync/🦀️.rs:80`
(`PathBuf: protocol::FromValue` not satisfied) — confirmed, by re-running `cargo check` immediately
after, to be a real but TRANSIENT state (a peer's in-flight `PersistenceBinding` conversion): the
plain `cargo check` (no `--tests`) went green again minutes later, but every `cargo test` attempt
kept losing the race to a fresh red window. **Because production code was never changed at this
call site (still `serde_json::to_string`/`from_str`, unmodified from the predecessor's state), there
is no shipped risk** — only an added test that the next agent (or a retry once the tree settles)
should run before attempting the actual swap.

**WRITTEN BUT UNVERIFIED (as an individual test)** — `create_invite_ttl_secs_is_a_bare_integer_on_
the_wire` (`📇️directory/🧬️schema/🦀️.rs`) was never run in isolation with a captured pass/fail line;
its CODE PATH, however, is exercised by the crate-wide `cargo check` above (0 errors, so it at least
type-checks), and the same underlying `pack::json` integer-fidelity claim it tests is the one
`to_json_string_bytes_match_the_serde_json_bridge` (above) proves in the general case. Not the same
as a confirmed `cargo test` pass — flagged honestly rather than assumed.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs` — full serde → `ToValue`/`FromValue`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs` — full serde → `ToValue`/`FromValue`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs` — one test fixture helper
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` — `NamedTripleDiff`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs` — new proof test only, production code unchanged
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs` — `ToValue`/`FromValue` added throughout; `ItemPatch`/`CollectionDiff`/`CollectionMutation` fully converted

## What remains

- `📡️spr/📜️history`'s two declined refs — re-attempt once a clean crate-wide `cargo test` run is
  possible (blocked on a peer's `store/sync.rs` work, not on anything in this slice) and the new
  oracle test can actually be confirmed green.
- `📡️spr/🧵️channel`'s dual derive — depends on `🔌️plugin/🖥️host`'s own migration off `serde_json`.
- `🌿️vcs`'s schema-region dual derive — depends on `store::ArtifactEnvelopeRead`'s own migration
  (already flagged as a later wave in `store`'s own docstring) and, for `Change` specifically, a
  deliberate future re-derivation of the checkpoint-id content hash (a byte-format change, its own
  ticket).
- `💡️inference`'s `InferredField` trait bound — 5 of 13 implementors still serde-only.
