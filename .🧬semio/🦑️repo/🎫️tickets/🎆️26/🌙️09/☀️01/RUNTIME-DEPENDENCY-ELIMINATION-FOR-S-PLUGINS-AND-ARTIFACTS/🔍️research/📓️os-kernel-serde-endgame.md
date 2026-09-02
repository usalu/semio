# 🏁️ os-kernel serde endgame — `🏪️store`/`📡️spr`/`🌿️vcs`/`💡️inference`, this wave

Continues `📓️store-serde-final.md` (79→39, five deferrals) and `📓️directory-spr-vcs-final.md`
(`📇️directory`→0). This wave re-verified all five of the predecessor's deferrals against a live,
concurrently-changing tree, cleared two in full (one unexpectedly larger than scoped), converted a
sixth item (`📡️spr/📜️history`), and fixed a real, previously-undiagnosed macro-scoping bug blocking
`cargo test --lib --no-run` crate-wide.

## Headline — code-only production serde counts (this wave's own classifier, `🗑️generated/
classify_serde5.py`, brace-matching `#[cfg(test)]` + `#[cfg_attr(test,…)]` + doc-comment exclusion)

| module | before | after | delta |
|---|---|---|---|
| `🏪️store` | 51 | **41** | **-10** |
| `📡️spr` | 11 | **9** | **-2** |
| `🌿️vcs` | 39 | 39 | 0 (re-verified, still correctly blocked) |
| `💡️inference` | 7 | 7 | 0 (deferred seam, per instruction — unchanged) |
| **total** | **108** | **96** | **-12 (-11%)** |

No module reached zero this wave. `🏪️store` and `📡️spr` both still carry real, out-of-fence
consumers (`stdio`'s remaining ~15-file wave is now genuinely empty — see below — but `📡️spr/
🧵️channel` and `🏪️store`'s pack_rt/InteractionState cluster remain).

**Why the "before" numbers differ from the ticket's own briefing (96/16/11/3)**: this is a live,
concurrently-edited tree — the classifier was run fresh at this wave's start, after other sessions'
intervening edits. Consistent with every prior wave's own note on this same discrepancy.

## Deferral 1 — `ArtifactChild<S>`/`ArtifactLink`/`LinkPin`/`BlobRef` — CLEARED IN FULL

The predecessor reverted this after the compiler named `stdio`'s `SemioKit` snapshot structs as the
blocker and correctly fenced off "stdio's own ~563-file wave." Re-attempted this wave with the same
compiler-enumeration method, gating all four types' serde derives to `#[cfg_attr(test, derive(…))]`
in `🏪️store/🦀️.rs`, then running `cargo check -p semio-s-plugin-stdio` repeatedly and fixing exactly
what it named — not the 563-file wave, six specific struct definitions and two call sites:

- `✳️kit/🧬️schema/📸️snapshot/🦀️.rs` — `SemioKitSnapshot` (the file the predecessor already
  diagnosed, but had not yet edited)
- `✳️kit/🧬️schema/🦀️.rs` — `SemioKitArtifact` (a second, sibling "full artifact state" mirror of the
  same fields the predecessor's diagnosis didn't separately name)
- `✳️object/🧬️schema/📸️snapshot/🦀️.rs` — `SemioObjectSnapshot`
- `✳️object/🧬️schema/🦀️.rs` — `SemioObjectArtifact`

All four already had hand-written `dsl::ToValue`/`dsl::FromValue` impls that never touched serde
(confirmed by reading each before editing) — the derive gate was pure subtraction, matching the
established `#[cfg_attr(test, derive(Serialize, Deserialize))]` pattern used ~10 other places in
this repo (`🏪️store/🧬️schema/🧬️mutations/**`, etc.).

Gating those four surfaced two more real, compiler-named consumers the predecessor's earlier attempt
never reached (it reverted before getting this far): `entity_count()` in the `object` and `kit`
editor/viewer window files (4 files total —
`✳️object/✏️editor/…/🪟️main/🦀️.rs`, `✳️object/👁️viewer/…/🪟️main/🦀️.rs`,
`✳️kit/✏️editor/…/🪟️main/🦀️.rs`, `✳️kit/👁️viewer/…/🪟️main/🦀️.rs`) each called
`serde_json::to_value(document)` directly on the now-serde-free snapshot type. Converted all four,
identically, to `dsl::ToValue::to_value(document)` + a `DslValue::Object`/`DslValue::Array` walk —
same behavior (max array-length-under-any-field, clamped 1..=6), zero serde.

**Result: `semio-s-plugin-stdio` compiles clean — 0 errors** (confirmed by a completed
`cargo check -p semio-s-plugin-stdio`, 4m40s, 1452 warnings none of them errors). This is the first
time this specific deferral has been cleared end-to-end; the predecessor's "own ~563-file wave"
framing turned out to describe `stdio`'s *general* serde surface, not this specific seam — the seam
itself needed exactly 6 files.

Store-side `git diff` scope: `🏪️store/🦀️.rs` only (`ArtifactChild`, `ArtifactLink`, `LinkPin`,
`BlobRef` derive lines). stdio-side scope: the 6 files named above, struct-derive-line and
one-function-body edits only — no call-site sweep, no touching the other ~15 `stdio` files the
predecessor already confirmed don't need `Serialize` themselves.

## Deferral 2 — `ArtifactRepositoryHistoryEntryAuthority<T>` — STILL BLOCKED, unchanged

Re-checked `✏️s/🔌️plugins/🎞️animate/…/🧬️mutations/🦀️.rs`: `PresentMutation` still derives only
`Serialize, Deserialize, dsl::DslEnum, dsl::Mutations` — no `ToValue`/`FromValue`. The
`SEMANTIC-MUTATIONS-OVERHAUL` ticket's churn has not landed this. Outside my fence (animate plugin);
left untouched, matching the predecessor's own decision.

## Deferral 3 — `🧵️canonical-edit/🦀️.rs`'s `ScalarBytes` — LEFT ON SERDE (per explicit instruction)

Not attempted. Verified the file's own docstring reasoning still holds (`pack::json` still doesn't
emit ryu-identical scientific notation). Correct, permanent exception — not touched.

## Deferral 4 — `🔄️sync/🦀️.rs` `PathBuf: ToValue` gap — FIXED, plus a second bug found underneath it

Added `impl ToValue`/`impl FromValue for std::path::PathBuf` to `🌱️value/🔁️codec/🦀️.rs` (lossy
UTF-8 string round-trip — this is a local-only config path, never a content-hash input, so lossy is
correct and matches the file's existing precedent for non-byte-exact leaves). This alone took
`cargo test -p semio-framework-os-kernel --lib --no-run` from 13 errors (predecessor's count) to 4.

The remaining 4 were **not** the same root cause the predecessor assumed. Full-format diagnostics
showed the *real* first error: `cannot find derive macro 'ToValue' in this scope` inside
`🔄️sync/🦀️.rs`'s `pub mod backbone_worker_wire { … }` — a nested module that imports
`ArtifactActorConfig`/`ArtifactActorMsg`/`ArtifactEvent`/`PersistenceBinding` and
`to_dsl_value`/`from_dsl_value`, but never imports the `ToValue`/`FromValue` derive macros
themselves, even though it uses `#[derive(…, ToValue, FromValue)]` twice
(`BackboneWorkerRequest`/`BackboneWorkerResponse`). This is why `PersistenceBinding: ToValue` etc.
looked like the blocker — the derive silently failed to expand, so every downstream use looked like
a missing trait impl. Fixed with one import line:
`use semio_framework_value_derive::{FromValue, ToValue};` inside the submodule.

**Verified**: `cargo check -p semio-framework-os-kernel --tests` — **0 errors originating in
os-kernel's own files.** The 2 `--tests` failures remaining are both in *downstream, out-of-fence*
crates that happen to get pulled into the same check:
- `semio-framework-os-infinite`: `#[value(...)] does not support field attribute 'flatten'` at
  `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:829` — an in-flight peer conversion hitting a
  derive-macro gap, unrelated to serde removal.
- `semio-framework-plugin-host`: `PresenceUpdate: FromValue` — byte-for-byte the same pre-existing
  issue `📓️store-serde-final.md` already documented as "a different concurrent agent's in-flight
  `PresenceUpdate` migration."

Both recorded, neither chased, per the ticket's own rule. **Consequence**: `cargo test -p
semio-framework-os-kernel --lib` still cannot fully LINK today — not because of anything in my four
modules, but because two unrelated dependency crates don't currently compile. This is a materially
different (and smaller) blocker than the predecessor's framing ("PathBuf gap, needs its own
follow-up") — the PathBuf gap is fixed; what's left is two other tickets' in-flight breakage.

## Deferral 5 — `📡️spr/🧵️channel` (6 refs, dual) — RE-VERIFIED, still correct as-is

`FixedCommandPage`/`CommandPageCursor`/`CommandIngressStatus`'s dual derive: re-confirmed the real
consumer is still live —
`🔌️plugin/🖥️host/🧵️shard/🦀️.rs:188`: `serde_json::to_vec(&result.command_ingress)`. Unrelated
crate, unrelated wave. Not attempted, matching the predecessor.

## `📡️spr/📜️history` — the 2 declined refs — CONVERTED, and the byte-parity test now type-checks

`write_op_meta`/`read_op_meta`'s `MutationOrigin` call sites swapped `serde_json::to_string`/
`from_str` → `crate::os_pack::json::to_json_string`/`from_json_str`. Safe because
`crate::os_spr::MutationOrigin` (defined in `📡️replication/🎮️mutation/🦀️.rs:1524`) already has
hand-written `ToValue`/`FromValue` (confirmed by direct read, not assumed) — the predecessor's own
earlier-drafted, later-corrected claim about this file is now actually true.

**`mutation_origin_canonical_json_is_byte_identical_between_serde_json_and_pack_json`**: previously
"written but never confirmed passing" (blocked, per the predecessor, on a transient
`🏪️store/🔄️sync/🦀️.rs` `PathBuf` error). That error is now fixed (see Deferral 4). Status this
wave: **type-checks cleanly** (`cargo check -p semio-framework-os-kernel --tests` — 0 errors from
os-kernel's own files, this test file included) but **still not confirmed by an actual passing
`cargo test` run** — the crate's test *binary* cannot link while
`semio-framework-os-infinite`/`semio-framework-plugin-host` (two unrelated dependency crates) don't
compile. Once either of those two unrelated fixes lands, this test can be run and should pass — the
byte-identity claim itself was never in question (`pack::json`'s own oracle test,
`to_json_string_bytes_match_the_serde_json_bridge`, already proves the general case, and
`MutationOrigin` has no float fields to trigger the one known divergence).

## `💡️inference`'s `InferredField` bound — re-measured, unchanged at 7 of 13

Re-checked all 13 `impl InferredField<P>` sites directly (not from memory):

| implementor | ready? |
|---|---|
| `stdio/mesh::MeshAabb` | ✅ |
| `stdio/drawing::DrawFlattenedScene` | ✅ |
| `stdio/brep::BrepValidationReport` | ✅ dual |
| `stdio/table::ColumnEntropy` | ✅ dual |
| `stdio/table::ColumnMoments` | ✅ dual |
| `stdio/graph::NodeConnectivity` | ✅ dual |
| `mathematical::MathematicalRootsField` | ✅ |
| `remodel::RemodelRelativeCameraPose` (`RemodelPoseDelta`) | ❌ still serde-only |
| `procedural/assembly::AssemblySolve` (`AssemblySolveResult`) | ❌ still serde-only |
| `procedural/assembly::AssemblyContradiction` (`bool`) | ~ primitive OK, container un-migrated |
| `procedural/assembly::AssemblyEntropy` (`f64`) | ~ primitive OK, container un-migrated |
| `puzzle/3d::Puzzle3dFlatPlane` (`FlattenPlane`) | ❌ still serde-only |
| `puzzle/3d::Puzzle3dFlatCenter` (`[f64; 2]`) | ~ primitive array OK, container un-migrated |

Unchanged from the predecessor's count. `RemodelPoseDelta`/`AssemblySolveResult`/`FlattenPlane` are
each in a different plugin (remodel, procedural, puzzle) — real, small, additive conversions each,
but three separate out-of-fence waves; not attempted. The trait bound itself
(`💡️inference/🦀️.rs:84-85`, `Serialize + DeserializeOwned`) was not flipped.

## `🌿️vcs` — attempted a real reduction, found a genuine (not stale) blocker, reverted cleanly

Hypothesis going in: the predecessor's stated reason for `Author`/`CompositionPin`/`Checkpoint`/
`Alternative`/`ArtifactHistoryLedger`/`ArtifactHistoryIter`/`ArtifactVcs` staying dual —
`store::ArtifactEnvelopeRead`'s own `#[derive(Serialize)]` needing them reachable — is now STALE,
since `store-serde-final.md` already converted `ArtifactEnvelopeRead`/`ArtifactEnvelopeOwners`/
`envelope_json` off `Serialize` onto `ToValue` outright (confirmed directly: `ArtifactEnvelopeOwners`
now derives only `Debug, PartialEq`, no `Serialize` impl exists on it at all).

Gated all seven types' serde to test-only and grepped for a live reason before compiling (this
ticket's "grep first to sanity-check, then let the compiler enumerate" order) — found one
immediately: `💻️os/🖥️host/🦀️.rs`'s `BackboneDocument<P, Op>` still unconditionally
`#[derive(Serialize, Deserialize)]`s over `vcs: ArtifactVcs<P, Op>`. This is independently confirmed
by `🏪️store/🦀️.rs`'s own `ArtifactCursor::Serialize` docstring, which names the exact same
consumer as the reason `store::ArtifactCursor` itself can't drop serde yet either. Two independent
textual confirmations, same real, live, out-of-fence (`💻️os/🖥️host`, not one of my four modules)
consumer.

**Reverted the gating in full** (confirmed net-zero: classifier count 39 before and after) —
kept only the corrected docstrings explaining the real (not stale) reason, so the next agent doesn't
re-attempt the now-disproven "stale hand-off" theory. `Change` itself was never touched (its
`Serialize` is independently load-bearing for the frozen `content_addressed_checkpoint_id_core`
hash — unrelated to `BackboneDocument`, a second, permanent reason).

## Verification — verbatim tails, all fresh, all after every edit in this report

```
$ cargo check -p semio-framework-os-kernel --message-format=short
    Finished `dev` profile [unoptimized] target(s) in 1.55s
```
0 errors, 35 warnings. GUARDRAIL — green.

```
$ cargo check -p semio-framework-os-kernel --tests --message-format=short
```
0 errors originating in any `semio-framework-os-kernel` file. 2 `could not compile` lines for
`semio-framework-os-infinite` (1 error, `#[value(flatten)]` gap, unrelated peer work) and
`semio-framework-plugin-host` (3 errors, pre-existing `PresenceUpdate: FromValue`, documented by the
predecessor) — both out-of-fence, recorded, not chased.

```
$ cargo check -p semio-s-plugin-stdio --message-format=short
    Finished `dev` profile [unoptimized] target(s) in 4m 40s
```
0 errors, 1452 warnings.

```
$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm --message-format=short
    Finished `dev` profile [unoptimized] target(s) in 2.48s
```
0 errors.

## Can os-kernel's `Cargo.toml` drop `serde`/`serde_json`?

**No — not yet.** `🏪️store` (41) and `📡️spr` (9) are not at zero; both still carry real production
`serde` (the `stdio`-facing `ArtifactChild`/`ArtifactLink` seam is now clear, but
`ArtifactCursor`/`pack_rt`/`InteractionState` in `store` and `🧵️channel` in `spr` are still real,
out-of-fence-blocked). Per the ticket's own rule, did not touch the `Cargo.toml` line.

## Files touched this wave

- `🏪️store/🦀️.rs` — `ArtifactChild<S>`, `ArtifactLink`, `LinkPin`, `BlobRef` derive lines gated
  `#[cfg_attr(test, …)]`
- `🌱️value/🔁️codec/🦀️.rs` — added `PathBuf: ToValue`/`FromValue`
- `🏪️store/🔄️sync/🦀️.rs` — added the missing `use semio_framework_value_derive::{FromValue,
  ToValue};` inside `backbone_worker_wire`
- `📡️spr/📜️history/🦀️.rs` — `write_op_meta`/`read_op_meta`'s two `MutationOrigin` call sites moved
  to `os_pack::json`
- `🌿️vcs/🦀️.rs` — docstrings only (corrected the stale `store::ArtifactEnvelopeRead` reasoning to
  the real `💻️os/🖥️host::BackboneDocument` one); zero functional diff, confirmed by an unchanged
  classifier count (39→39)
- stdio (6 files): `✳️kit/🧬️schema/📸️snapshot/🦀️.rs`, `✳️kit/🧬️schema/🦀️.rs`,
  `✳️object/🧬️schema/📸️snapshot/🦀️.rs`, `✳️object/🧬️schema/🦀️.rs`,
  `✳️object/✏️editor/…/🪟️main/🦀️.rs`, `✳️object/👁️viewer/…/🪟️main/🦀️.rs`,
  `✳️kit/✏️editor/…/🪟️main/🦀️.rs`, `✳️kit/👁️viewer/…/🪟️main/🦀️.rs` (8 files total — derive gates
  plus the `entity_count` conversions)

## What remains (counts, for whoever picks this up next)

1. **`🏪️store`, 41 lines** — `ArtifactCursor` (blocked on `💻️os/🖥️host::BackboneDocument`,
   confirmed live this wave), `pack_rt`/`InteractionState` (blocked on `📡️replication`, unchanged),
   `🧵️canonical-edit::ScalarBytes` (8 lines, permanent exception).
2. **`📡️spr`, 9 lines** — `🧵️channel`'s dual derive (blocked on `🔌️plugin/🖥️host`, re-confirmed
   live this wave).
3. **`🌿️vcs`, 39 lines** — genuinely blocked on `💻️os/🖥️host::BackboneDocument` (new, sharper
   finding this wave, replacing the stale "store hand-off" reasoning) + `Change`'s permanent
   content-hash exception. Unblocks the moment `BackboneDocument` either drops its own
   `#[derive(Serialize, Deserialize)]` or gains `ToValue`/`FromValue` alongside it — a
   `💻️os/🖥️host` wave, not a `vcs` one.
4. **`💡️inference`, 7 lines** — trait bound unflippable until `RemodelPoseDelta`/
   `AssemblySolveResult`/`FlattenPlane` (3 separate plugins) gain `ToValue`/`FromValue`.
5. **Two unrelated, currently-live compile breaks** block `cargo test -p semio-framework-os-kernel
   --lib` from running at all: `semio-framework-os-infinite`'s `#[value(flatten)]` gap and
   `semio-framework-plugin-host`'s `PresenceUpdate: FromValue` gap. Neither is mine to fix; both
   are the only things stopping `mutation_origin_canonical_json_is_byte_identical_…` from getting
   its first confirmed passing run.

`os-kernel`'s own `Cargo.toml` still needs `serde`/`serde_json` — not close to droppable this wave
(store/spr both non-zero), consistent with every prior wave's own conclusion.
