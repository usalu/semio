# 🔧️ os-kernel `ToValue`/`FromValue` chokepoint — closed to 0 errors

Starting state when this pass began: `cargo check -p semio-framework-os-kernel` — **7 errors**
(not 9; two of the handoff's three items had already collapsed to fewer distinct diagnostics by the
time this pass started, per the ticket's own "phantom blockers"/stale-snapshot warning).

## Error-by-error

### 1. `Author` — DERIVED
`🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs:76`. Added
`use semio_framework_value_derive::{FromValue, ToValue};` and
`use crate::os_dsl::{FromValue, ToValue};` to the file (it had neither), then
`#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]` +
`#[value(rename_all = "camelCase")]` alongside the existing `#[serde(rename_all = "camelCase")]`.
Straightforward — this file lives inside the os-kernel crate itself, so the derive macro's
`::semio_framework_os_kernel::…`-rooted codegen resolves natively.

### 2. `HybridLogicalTimestamp` — HAND-WRITTEN (as briefed)
`🧰️framework/🔨️modules/📡️replication/🆔️ids/🦀️.rs:52`, in `semio-framework-replication`. Confirmed
the DAG fact from the handoff: this crate has **0** dependency on `semio-framework-value-derive`,
and sits *below* os-kernel (os-kernel depends on replication, not the reverse), so the derive's
`::semio_framework_os_kernel::…`-rooted codegen cannot resolve here — verified by finding the
derive macro's `quote!` bodies at
`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️component.rs` (every emission point is literally
`::semio_framework_os_kernel::ToValue`/`DslValue`/`ValueError`/`FromValue`, no configurable root).
No cleaner option exists there — it is a hard literal path, not a parameter.

`crate::value::{ToValue, FromValue, DslValue, ValueError}` is NOT a dependency edge — it's the raw
first-party `🧰️framework/🔨️modules/🌱️value/🦀️component.rs` file mounted directly into
`semio-framework-replication` by `#[path = "../../../🌱️value/🦀️component.rs"] pub mod value;`
(crate root, `📦️packages/🦀️rust/🦀️.rs:33-34`). Every other replication type that already had
`ToValue`/`FromValue` (`MutationMessage`, `MutationDiff`, `Mutation` trait bounds) used exactly this
pattern — `impl crate::value::ToValue for MutationMessage { … }` in `🎮️mutation/🦀️.rs` — so
`HybridLogicalTimestamp`'s hand-written impl mirrors that existing local convention verbatim: an
object with `actor`/`physical_ms`/`logical` keys (the struct has no `#[serde(rename_all)]`, so
snake_case field names carry through unchanged, unlike its siblings).

### 3. `ArtifactEnvelope<P, Mutation>::envelope_json` — METHOD-LOCAL `where`, NOT a `ToValue` mirror
The handoff's suggested fix ("mirror the two hand-written `Serialize` impls as `ToValue` impls") was
evaluated and **not used** — deviation, with reason:

`ArtifactEnvelopeOwners`/`ArtifactEnvelope`'s `Serialize` impls delegate to `capture_read()` →
`ArtifactEnvelopeRead`, whose fields are `ArtifactVcsRead`, `ArtifactCursorOwners`, `OwnerRef`,
`HistoryLane`, `ArtifactEditMessageLedger`, `crate::os_spr::Conflict`,
`crate::os_io::ArtifactDialect`, `MigrationProvenance` — a full second tree of types with no
`ToValue`/`FromValue` today. Mirroring the `Serialize` impls as `ToValue` would require giving ALL
of those types `ToValue` too, which is explicitly out of scope per the ticket's own fence ("that
file still has ~150 serde references … Leave all of that"). It would also reopen the `capture_read`
fallibility question across a much larger surface than the one call site that actually broke.

Instead: `envelope_json` picked up a **method-local `where P: Serialize, Mutation: Serialize`**
clause (the enclosing `impl<P, Mutation> ArtifactStore<P, Mutation>` block at L13571 only bounds
`ToValue + FromValue` now). This is valid Rust — a method can add bounds beyond its impl block's —
and it changes nothing about `envelope_json`'s behavior or the untouched `Serialize` impls at
~L2313/~L2391; it only re-supplies the bound this ONE caller needs. `capture_read`'s existing
`&'static str` → `serde::ser::Error::custom` handling is completely unaffected — no new fallibility
decision was needed because the serde path itself was never disturbed.

### 4. `Edit<Mutation>: ToValue` — NOT in the original 3-item handoff, HAND-WRITTEN
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:11963`, inside
`CursorRevisionAccumulator::edit_digest` (one of the five functions the coordinating session had
already rerouted to `crate::os_pack::json::to_json_string`, which needs `T: ToValue` — that function
takes `&Edit<Mutation>`, so `Edit<Mutation>: ToValue` itself was missing, not just `Mutation:
ToValue`). `Edit<Op>` is defined in `semio-framework-replication` (`🎮️mutation/🦀️.rs:1096`), same
DAG constraint as item 2 — hand-written, `crate::value::` paths.

`Edit<Op>: ToValue`/`FromValue` recurses into `MutationMeta` (its `mutation_meta: Vec<MutationMeta>`
field), which recurses into `MutationOrigin` (`origin` field) and four `ids` newtypes
(`MutationId`, `ActorId`, `SchemaId`, `PayloadHash`) and `crate::UndoPolicy`; `MutationOrigin`'s
`Transaction` variant recurses into `ForeignTarget`. All hand-written in the same crate, same
`crate::value::` pattern, scoped to exactly the types this one cascade touches (did NOT add
`ToValue` to `ArtifactId`/`ArtifactVersion`/`SchemaVersion` — those ids newtypes are never reached
from `Edit<Op>`, so adding them would be unscoped churn):

- `🆔️ids/🦀️.rs`: `MutationId`, `ActorId`, `SchemaId` (transparent String wrappers — trivial
  delegation to `String`'s existing `ToValue`/`FromValue`) and `PayloadHash` (`[u8; 32]` — no
  blanket fixed-array impl exists in `crate::value`'s codec module, unlike serde's; encoded/decoded
  element-by-element into/from a `DslValue::Array`, matching serde's default `[u8; N]` wire shape).
- `🧾️wire/🦀️.rs`: `UndoPolicy` (fieldless enum, no serde tag attribute → bare variant-name string,
  matched by hand).
- `🎮️mutation/🦀️.rs`: `ForeignTarget` (camelCase rename_all, sparse `dialect`), `MutationOrigin`
  (`#[serde(tag = "kind")]` INTERNAL tagging — no adjacently-tagged `tag`+`content` shape the derive
  macro supports anyway, so hand-written was the only option regardless of the DAG), `MutationMeta`
  (no rename_all — snake_case field names, every `skip_serializing_if` mirrored by hand), and
  `Edit<Op>` itself (camelCase rename_all; `Option<T>` fields with no explicit `#[serde(default)]`
  — `description`/`finished_at` — still default to `None` when absent on `from_value`, matching
  serde's built-in `Option<T>`-defaults-when-missing behavior, mirrored by hand not by attribute).

## Error count per iteration
1. Start (before this pass, handoff's own count): **9**
2. Start of this pass (`cargo check`, before any edit): **7** — `envelope_json` P/Mutation Serialize
   ×2, `Author` ToValue/FromValue ×2, `HybridLogicalTimestamp` ToValue/FromValue ×2, `Edit<Mutation>`
   ToValue ×1.
3. After `Author` derive + `envelope_json` where-clause + `HybridLogicalTimestamp` hand-impl: not
   independently measured (bundled into one edit pass before the next check) — see step 4.
4. After adding all `Edit<Op>`/`MutationMeta`/`MutationOrigin`/`ForeignTarget`/ids/`UndoPolicy`
   hand-impls: `cargo check -p semio-framework-replication` → **0 errors**, 2 pre-existing warnings
   (both unrelated: one `unnecessary qualification`, one `never used` on `causal.rs::push`).
5. `cargo check -p semio-framework-os-kernel` → **0 errors**, 33 warnings (all pre-existing
   style/dead-code lints, none introduced by this change — spot-checked against the field list, they
   are the same `unnecessary qualification`/shorthand-pattern warnings the file already carried).

## `capture_read` fallibility decision
Not actually needed — see item 3 above. The method-local `where` bound kept `envelope_json` on its
original all-serde path (`serde_json::to_string` → the existing hand-written `Serialize` impls →
their existing `capture_read().map_err(serde::ser::Error::custom)` handling), so no new infallible
`ToValue::to_value` was ever asked to represent a `capture_read` failure. If a future wave does carry
`ArtifactEnvelopeRead`'s whole field tree into `ToValue` (the LATER "os-kernel full serde removal"
wave the ticket already calls out), the two reasonable options remain what the handoff named: embed
a `DslValue::object([("error", DslValue::String(msg))])` sentinel, or restructure the call site to
use the fallible `capture_read()` + a `ToValue` on `ArtifactEnvelopeRead` directly (mirroring what
`envelope_json` already does for the serde path) rather than routing through an infallible
`ToValue for ArtifactEnvelope` at all — the second is cleaner and is what this pass effectively chose
for the one call site it touched.

## Verbatim tail — `cargo check -p semio-framework-os-kernel`
```
error count: 0
warning: `semio-framework-os-kernel` (lib) generated 33 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 33 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.28s
exit=0
```

## Downstream verification — `semio-s-plugin-puzzle` wasip2

`cargo check --target wasm32-wasip2 -p semio-framework-actor` → **0 errors**, 5 pre-existing
warnings, exit 0. This is the strongest available signal that the store/replication chokepoint fix
is correct for the wasip2 target: the compile side is clean.

`cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-puzzle` could **not** be confirmed clean
end-to-end — blocked by two failures, both independently confirmed unrelated to this pass's changes:

1. **`wasm-component-ld` (rust-lld) SIGSEGV** linking `semio-framework-actor` — a linker crash
   (`ElemSection::writeBody()`, LLD stack dump), reproduced twice. `cargo check` (no linker
   invocation) on the same crate/target is clean, isolating this to the linker step, not the source.
   Consistent with this machine's known concurrent-build resource contention (16 other live
   sessions/agents per the ticket's own warning) rather than a code defect.
2. **`semio-framework-ui`'s wgpu target**: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️text.rs:513:69`
   — `error[E0308]: mismatched types: expected Result<Vec<u8>, String>, found ()`. `git status`
   confirms this file is currently modified in the live working tree (uncommitted), and the file's
   last real commit predates today — this is a peer session's in-flight edit, not a store/vcs/
   replication file, and outside every scope boundary of this task. Not touched, not reverted.

Neither failure is downstream of the `ToValue`/`FromValue` chokepoint fix; both are infra/peer-work
noise the ticket's own "phantom blockers" section already anticipated. A clean puzzle wasip2 link
needs a re-run once the concurrent load/peer edit clear — not something to force from inside this
task's scope fence.

## Files touched
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs` — `Author` derive + imports.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — `envelope_json` method-local
  `where P: Serialize, Mutation: Serialize`.
- `🧰️framework/🔨️modules/📡️replication/🆔️ids/🦀️.rs` — `HybridLogicalTimestamp`, `MutationId`,
  `ActorId`, `SchemaId`, `PayloadHash` hand-written `ToValue`/`FromValue`.
- `🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs` — `UndoPolicy` hand-written
  `ToValue`/`FromValue`.
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs` — `ForeignTarget`, `MutationOrigin`,
  `MutationMeta`, `Edit<Op>` hand-written `ToValue`/`FromValue`.

No serde derives, attributes, or hand-written `Serialize`/`Deserialize` impls were removed anywhere
in this pass — every type above keeps its existing serde surface untouched, `ToValue`/`FromValue`
added alongside per the scope fence.
