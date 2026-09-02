# 🏪️ `🏪️store` — final serde removal pass, this wave

Continues `📓️store-serde-removal.md` (predecessor: 397 → 92, five traced deferrals). This wave
re-checked every deferral against a fully live, concurrently-changing framework, converted the two
that had genuinely become unblocked, and re-verified the other three with more precise reasons than
before — one of which (`ArtifactChild`/`ArtifactLink`/`LinkPin`/`BlobRef`) is a NEW finding not in
the predecessor's cluster list.

## Headline — production serde count

| | count | method |
|---|---|---|
| before this wave | **79** | this wave's own classifier (see below), run before any edit |
| after this wave | **39** | same classifier, run after, both directions re-verified by `cargo check` |

**Methodology note on why 79 ≠ predecessor's stated 92**: the predecessor's own count and this
wave's count are NOT directly comparable — this wave's classifier additionally excludes
`#[cfg_attr(test, ...)]`-gated attribute lines (test-only surface that never ships) and naming
false-positives where "serde" is a substring of an unrelated identifier (`operation_envelope_serde`,
`envelope_serde` — modules that do NOT depend on the serde crate, confirmed by reading their bodies).
Both exclusions are consistent with the predecessor's own stated intent ("moved to TEST-ONLY", "the
module literally being called `operation_envelope_serde`") — this wave just applied them
mechanically instead of by inspection, which is why the honest starting number differs from the
briefing's headline 92. The reduction (79 → 39, **-40 lines, -51%**) is the real, compiler-verified
result of this wave, independent of which starting number is used.

Classifier: brace-matching `#[cfg(test)]`-item exclusion (any item, not just `mod`) +
`#[cfg_attr(test, …)]`-line exclusion + doc-comment/prose exclusion + `VcsError::Serialize`/
`VcsError::Deserialize` false-positive exclusion + naming-substring false-positive exclusion
(`serde`/`serde_json` only counted as a whole token, never as part of a longer identifier). Script
kept at `🗑️generated/classify_serde4.py` for this ticket's own record.

## What this wave converted (compiler-verified, `cargo check -p semio-framework-os-kernel` green
throughout, checked after every batch)

### Deferral 1 — the `ArtifactEnvelopeRead` tree — UNBLOCKED, converted in full

The predecessor's blocker was `🌿️vcs::ArtifactVcsRead` lacking `ToValue`. Re-checked this wave:
`🌿️vcs/🦀️.rs` now carries a hand-written `impl<P: ToValue, Mutation: ToValue> ToValue for
ArtifactVcsRead<'_, P, Mutation>` (landed by the concurrent agent owning that module — read, not
authored, by this wave). The predecessor's own fallibility analysis ("not a real blocker") already
covered the rest. Converted:

- **`ArtifactCursor`**: hand `impl Serialize`/`impl Deserialize` → hand `impl ToValue`/`impl
  FromValue`, delegating to `ArtifactCursorOwners` (already dual-derived) exactly as the old
  bridge delegated.
- **`ArtifactEditMessageLedger`**: hand `impl serde::Serialize` (a `SerializeSeq` loop) → hand
  `impl ToValue` (`DslValue::Array` from `self.iter().map(ToValue::to_value)`), using
  `crate::os_spr::EditMessages`'s own hand `ToValue` (already existed, in
  `📡️replication/⚔️conflict/🦀️.rs` — a second thing the predecessor could not have seen at the
  time).
- **`ArtifactEnvelopeRead<'a, P, Mutation>`**: `#[derive(Serialize)]` + eleven `#[serde(...)]`
  lines → a hand-written `impl<P: ToValue, Mutation: ToValue> ToValue`, mirroring the old
  `skip_serializing_if` shape by omitting the object key when the option is `None`/map is empty.
  Exactly the mechanical work the predecessor's "what remains" section named in advance.
- **`ArtifactEnvelopeOwners<P, Mutation>`** / **`ArtifactEnvelope<P, Mutation>`**: their hand
  `impl Serialize` blocks **deleted outright**, not replaced — confirmed by grep (repo-wide, both
  plugin and framework trees) that nothing requires either type to implement `Serialize` or
  `ToValue` as a trait; both were only ever reached through `capture_read()` + the now-`ToValue`
  `ArtifactEnvelopeRead`.
- **`envelope_json()`**: bound `P: Serialize, Mutation: Serialize` → `P: ToValue, Mutation:
  ToValue`; body `serde_json::to_string(&*self.envelope)` → `self.envelope.capture_read()?` then
  `crate::os_pack::json::to_json_string(&read)`.
- **`ArtifactBackboneRef`, `ArtifactCursorOwners`, `HistoryLane`, `MigrationProvenance`,
  `OwnerRef`**: all five dropped back to `ToValue`/`FromValue`-only, exactly as the predecessor's
  own "what remains" section predicted once the tree unblocks.

### Deferral 4 — VCS ops-log leaf metadata — UNBLOCKED, converted in full

The predecessor traced this to `crate::os_spr::{MutationMeta, MutationMessage, Conflict}` needing
`ToValue`/`FromValue`, blocked on `📡️replication` (out of scope). Re-checked this wave: all three
already carry hand-written `ToValue`/`FromValue` in `📡️replication/🎮️mutation/🦀️.rs` and
`📡️replication/⚔️conflict/🦀️.rs` — landed by the concurrent replication agent, not by this wave.
Converted the three call sites in `parse_ops_text`'s `OpsHeaderLine::{Metadata,Message,Conflict}`
arms: `serde_json::from_str(&data)` → `crate::os_pack::json::from_json_str(&data)`, same
`.map_err(...)` shape, same `TextError` wrapping.

## Deferrals re-verified, still blocked — with sharper reasons than before

### Deferral 2 — `ArtifactChild<S>` / `ArtifactLink` / `LinkPin` / `BlobRef` — NOT the predecessor's
### shape at all; a real external fan-out, found by the compiler

The predecessor's note said `ArtifactChild<S>` "needs a hand-written generic impl" — on inspection
that hand impl **already exists** (`impl<S> ToValue for ArtifactChild<S>` / `impl<S> FromValue`,
present before this wave started, landed by an intervening session). The actual remaining blocker is
different and was never named by the predecessor: the redundant `#[derive(Serialize, Deserialize)]`
on `ArtifactChild<S>`, and the same dual-derive on `ArtifactLink`/`LinkPin`/`BlobRef`, are
**load-bearing for other plugin crates**, not redundant.

Compiler-enumerated (the ticket's mandated method: gate the derive, let `cargo check` name every
consumer, restore, then decide): gating all four and running `cargo check -p semio-s-plugin-stdio`
named exactly this and nothing else —

```
error[E0277]: the trait bound `ArtifactChild<object::schema::snapshot::component::SemioObjectSnapshot>: serde::Deserialize<'de>` is not satisfied
error[E0277]: the trait bound `ArtifactChild<model::schema::snapshot::component::SemioModelSnapshot>: serde::Deserialize<'de>` is not satisfied
error[E0277]: the trait bound `ArtifactChild<SemioValueSnapshot>: serde::Deserialize<'de>` is not satisfied
error[E0277]: the trait bound `ArtifactLink: serde::Deserialize<'de>` is not satisfied
```
— all four inside `✏️s/🔌️plugins/🗄️stdio/…/🧿️semio/…/📸️snapshot/🦀️.rs`'s own `SemioKit` snapshot
structs, which still dual-derive `serde` unconditionally and embed these types directly. `stdio` is
the predecessor's own documented "own wave" (~563 production call-site files) — out of this wave's
fence. `grep` alone found the same four call sites but not the fact that they were the ONLY
consumers forcing the dual-derive — that required the compiler-enumeration pass, not just reading
the call sites, since `stdio`'s OTHER ~15 files reaching `ArtifactChild`/`ArtifactLink` (raster,
remodel, trinity/jack, flow, note — all grepped) turned out NOT to require `Serialize` themselves
(they go through the hand `ToValue`/`DslField` bridge already). Reverted immediately once the
compiler named the real consumer (matches this ticket's own "reverted mid-wave, caught by the
guardrail" precedent) — the edit never reached a committed state; `git diff` on `🏪️store/🦀️.rs`
confirmed byte-identical to before the attempt once reverted.

`LaneItemReceipt` was NOT touched — its own docstring already states the load-bearing reason
(`🔌️plugin/🦀️.rs`'s `TypedOperationResultPage::try_serialize` bound), so this wave didn't even
attempt it; correctly still dual-derived.

### Deferral 3 — `ArtifactRepositoryHistoryEntryAuthority<T>` — `Edit<Op>` confirmed unblocked, but
### a sharper, still-real blocker sits one level deeper

Verified `Edit<Op>` (`📡️replication/🎮️mutation/🦀️.rs:1422`) has a hand-written
`impl<Op: ToValue> ToValue for Edit<Op>` / `impl<Op: FromValue> FromValue for Edit<Op>` — the
predecessor's own "verify and unblock" instruction is now confirmed TRUE at the generic level.

But the one production call site (`✏️s/🔌️plugins/🎞️animate/…/🎬️present/…/💾️binary/🦀️.rs:320`,
`store::artifact_bounded_history_entry_decoder::<protocol::Edit<PresentMutation>>()`) instantiates
`Op = PresentMutation`, and `PresentMutation`
(`✏️s/🔌️plugins/🎞️animate/…/🧬️schema/🧬️mutations/🦀️.rs`) itself only derives `Serialize,
Deserialize, dsl::DslEnum, dsl::Mutations` — traced both derive macros
(`expand_dsl_enum`/`expand_derive_mutations` in `🗣️dsl/✨️derive/🦀️.rs`) directly and confirmed
neither emits `ToValue`/`FromValue`. So `Edit<PresentMutation>: FromValue` does not hold yet, and
changing `artifact_bounded_history_entry_decoder`'s bound from `DeserializeOwned` to `FromValue`
would break `semio-s-plugin-animate`'s build. Converting `PresentMutation` and its nine mutation-leaf
payload structs (`ResizeSourceFrame`, `ReplaceSource`, `CreateTile`, `DeleteTile`, `DeleteTiles`,
`RenameTile`, `ResizeTileCrop`, `ReorderTiles`, `ReplaceTiles`) to also carry `ToValue`/`FromValue`
would be additive and low-risk in isolation, but every one of those files lives under
`✏️s/🔌️plugins/🎞️animate/…` — a different plugin, and git status at this wave's start showed active,
in-flight deletions under the `SEMANTIC-MUTATIONS-OVERHAUL` ticket touching this exact mutation
family. Left untouched per the ticket's own fencing rule; not attempted.

### Deferral 5 — `pack_rt` compose bridge — unchanged, permanent, plus `InteractionState`
### (tenth-seam session's "Blocker 3") reconfirmed still live in `🏪️store`'s own code

`pack_rt::{encode_json_value, decode_json_value, json_value_to_dsl, dsl_value_to_json,
json_values_equal, renormalize_json_wire_value}` + `impl ArtifactPack for serde_json::Value`:
unchanged, external `semio_compose_rs` API, same precedent as every prior wave.

Also present in this same cluster, not previously itemized for `🏪️store` specifically:
`impl ArtifactPack for protocol::InteractionState` (`🏪️store/🦀️.rs:19775-19788`) — round-trips
through `serde_json::to_value`/`from_value` because `InteractionState` itself has no `ToValue`/
`FromValue` twin yet. This is exactly the tenth-seam session's documented "Blocker 3"
(`InteractionState` direct `serde_json` hits, "a different shape… larger, separate scope"),
reconfirmed present in `🏪️store`'s own impl, not just the `🕹️interaction/**` sites that session
named. Left alone — same cross-module boundary (`📡️replication`), same reason.

## NEW finding this wave — `🧵️canonical-edit/🦀️.rs`'s `ScalarBytes`, not in any prior wave's list

`ScalarBytes::from_node` (`🏪️store/🧵️canonical-edit/🦀️.rs:321-328`) formats one canonical-JSON
scalar (`null`/`bool`/`i64`/`u64`/`i128`/`u128`/`f32`/`f64`) via `serde_json::to_writer(&mut scalar,
&value)`, feeding this crate's own "byte accounting, and exact one-item sealing" (its own module
docstring) — i.e. a content-addressed/canonical hash input, where byte-exactness is the entire
point. Checked whether `pack::json` (this ticket's own first-party JSON writer) could replace it:
**no** — `🎒️pack/🔤️json/🦀️.rs`'s own test module states outright "this writer never emits scientific
notation the way `serde_json`'s ryu-based writer does" (line ~1607). Swapping the writer here would
silently change the canonical byte output — and therefore the hash — for any document containing a
float `serde_json` would render in scientific notation. Not attempted; would need either a first-party
float formatter proven byte-identical to `serde_json`'s ryu output (a differential-oracle project of
its own, matching this ticket's BLAKE3/DEFLATE precedent) or a proof that this codepath never sees
such floats. Neither exists yet. **8 lines, genuinely new, correctly deferred.**

## A pre-existing gap surfaced (not caused by this wave, not fixed by this wave)

`cargo test -p semio-framework-os-kernel --lib --no-run` — **not run by any prior wave's own
verification** (both this wave's predecessor and the tenth-seam session verified only `cargo check`
and `cargo build --target wasm32-wasip2`) — currently fails with 13 errors, all in
`🏪️store/🔄️sync/🦀️.rs` (a file this wave never touched — confirmed via `git diff`, zero changes),
e.g.:
```
error[E0277]: the trait bound `PathBuf: protocol::FromValue` is not satisfied
error[E0277]: the trait bound `BackboneWorkerRequest: protocol::ToValue` is not satisfied
```
`std::path::PathBuf` has no `ToValue`/`FromValue` impl anywhere in `🌱️value/🔁️codec/🦀️.rs`'s blanket
list (`bool`, `String`, `&str`, `()`, `Option<T>`, `Vec<T>`, `[T; N]`, `Box<T>`,
`BTreeSet`/`BTreeMap<String, T>`, `DslValue`, `PhantomData`, 2/3-tuples, `HashMap` — no `PathBuf`),
and `PersistenceBinding::Folder { path: PathBuf }` derives `ToValue`/`FromValue` unconditionally.
This does not appear in `cargo check`'s non-test build (production compiles clean — confirmed,
0 errors) so it is specifically a **test-target-only** gap, pre-dating this wave. Recorded here,
not fixed — outside this wave's own edits' blast radius and a real, separate `PathBuf: ToValue`
follow-up.

## Verification — verbatim tails, all fresh, all after this wave's final edit

```
$ cargo check -p semio-framework-os-kernel --message-format=short
    Finished `dev` profile [unoptimized] target(s) in 0.19s
```
0 errors, 32 warnings (down from the 33-warning baseline — one fewer "unnecessary qualification",
incidental to this wave's edits, not chased).

```
$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm --message-format=short
    Finished `dev` profile [unoptimized] target(s) in 1m 49s
```
0 errors.

Compiler-enumerated dependent crates (per the ticket's own method — never trust a single crate's
green):

```
$ cargo check -p semio-framework-os --message-format=short   # (plugin-host)
error: could not compile `semio-framework-plugin-host` (lib) due to 3 previous errors
```
All 3 name `semio_framework::kernel::PresenceUpdate: FromValue` / generic `T: FromValue`/`ToValue`
inside `🔌️plugin/🖥️host/🦀️.rs` — a file this wave never touched, no `envelope_json` call anywhere in
that crate (grepped), unrelated to anything converted this wave. Recorded, not chased, per the
ticket's own "if an error names a file outside your module, record it and move on" rule — this file
is a different concurrent agent's in-flight `PresenceUpdate` migration.

```
$ cargo check -p semio-s-plugin-flow --message-format=short
error: could not compile `semio-framework-os-flow` (lib) due to 18 previous errors
```
All 18 are in `📖️playbook/🦀️.rs` (5, `serde_json::Value: FromValue`/`ToValue` not satisfied — a
different, already-completed migration's own fallout, per the tenth-seam session's own doc) and
`🌿️vcs/🦀️.rs:2771` (1, `E0502` borrow-checker conflict, zero `Serialize`/`ToValue` involvement) —
**byte-for-byte the same two pre-existing issues the tenth-seam session already documented**, none
new, none caused by this wave.

```
$ cargo check -p semio-framework-os-infinite --message-format=short
    Finished `dev` profile [unoptimized] target(s) in 11.24s
```
0 errors — confirms `envelope_json()`'s new `ToValue`/`FromValue` bound (its only outside caller,
`♾️infinite/…/🕸️dag/🦀️.rs:9338`) still resolves.

## Files touched this wave

`🏪️store/🦀️.rs` only (`ArtifactCursor`, `ArtifactEditMessageLedger`, `ArtifactEnvelopeRead`,
`ArtifactEnvelopeOwners`, `ArtifactEnvelope`, `ArtifactBackboneRef`, `ArtifactCursorOwners`,
`HistoryLane`, `MigrationProvenance`, `OwnerRef`, `envelope_json`, and the three `parse_ops_text`
call sites). No other file in `🏪️store/**` was edited; `ArtifactChild`/`ArtifactLink`/`LinkPin`/
`BlobRef` were edited and then reverted in the same session (net zero diff, confirmed by `git diff`).

## What remains (counts, for whoever picks this up next)

1. **`ArtifactChild<S>`/`ArtifactLink`/`LinkPin`/`BlobRef`** (~10 lines) — blocked on `stdio`'s own
   serde removal (its documented ~563-file wave), specifically the `SemioKit` snapshot structs at
   `✏️s/🔌️plugins/🗄️stdio/…/🧿️semio/…/📸️snapshot/🦀️.rs`. Once those move to
   `#[cfg_attr(test, derive(Serialize, Deserialize))]` (matching this ticket's own established
   pattern), these four types can drop back to `ToValue`/`FromValue`-only.
2. **`ArtifactRepositoryHistoryEntryAuthority<T>`** (~5 lines) — blocked on `PresentMutation` (and
   its nine mutation-leaf payloads) gaining `ToValue`/`FromValue` in the animate plugin — real,
   small, but outside this module's fence and inside another ticket's active churn.
3. **VCS `InteractionState` bridge + pack_rt** (~17 lines) — `InteractionState` blocked on
   `📡️replication` gaining its own `ToValue`/`FromValue` (tenth-seam Blocker 3); `pack_rt` itself
   permanent.
4. **`🧵️canonical-edit/🦀️.rs`'s `ScalarBytes`** (8 lines) — needs a first-party float formatter
   proven byte-identical to `serde_json`'s ryu output before it can move; `pack::json`'s own test
   suite documents that its writer is NOT that today.
5. **`🔄️sync/🦀️.rs`'s test-target break** (13 `cargo test --no-run` errors, `PathBuf: ToValue` +
   `BackboneWorkerRequest`/`Response: ToValue`) — pre-existing, not touched, needs its own
   `PathBuf: ToValue`/`FromValue` follow-up.

None of the above blocks `serde`/`serde_json` from being removed from
`semio-framework-os-kernel`'s `Cargo.toml` Cargo-wide (unchanged fact from every prior wave) —
`🏪️store` is down to 39 production lines from 79, all five individually traced and either converted
or deferred with a checkable, compiler-verified reason.
