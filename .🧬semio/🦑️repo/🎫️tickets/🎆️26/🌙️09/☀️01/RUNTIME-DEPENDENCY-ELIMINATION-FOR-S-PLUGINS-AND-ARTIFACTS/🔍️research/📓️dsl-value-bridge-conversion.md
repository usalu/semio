# 🌉️ The generic `to_dsl_value`/`from_dsl_value` bridge — converted to first-party

## Headline

`serde` is **still in `semio-s-plugin-draw-fsm`'s `wasm32-wasip2` `cargo tree`** — unchanged at 11
entries / 6 genuinely linked (`serde`, `serde_core`, `serde_json`, `itoa`, `memchr`, `zmij`). That
was expected and predicted by `📓️serde-off-guest-path.md`: the two remaining `serde` sources are
`semio-framework-os-kernel`'s own **direct** `Cargo.toml` entry (~150 hand-written usages, e.g.
`impl Serialize for ArtifactEnvelope`/`ArtifactCursor`) and `semio-framework-replication`'s own
**direct** entry (kept for `OrderedMap`/`Dictionary`, `⚠️diagnostic`, and the still-live
`impl From<&DslValue> for serde_json::Value` JSON-export bridge). Both were explicitly fenced off
in this ticket's brief as a **separate, later wave** — this wave's job was narrower: stop the
*generic* bridge itself from forcing serde onto every type that crosses it. That job is done, and
proven: **zero source call-site edits were needed anywhere in `os-kernel`, `replication`, or
`draw-fsm`** for the bound change to compile clean, because a peer had already landed unconditional
`ToValue`/`FromValue` for `ArtifactChild<S>`/`ArtifactRef` — the type shape the overwhelming
majority of the bridge's call sites cross.

## The bridge

`🧰️framework/🔨️modules/🌱️value/🦀️component.rs:170` (mounted into `semio-framework-replication` as
`protocol::value`, and re-exported from there through `os-kernel`'s `os_dsl`/`dsl` facade, hence
`dsl::to_dsl_value`/`semio_framework_os_kernel::to_dsl_value`/`os_dsl::to_dsl_value` all name the
same function):

```rust
// before
pub fn to_dsl_value<T: serde::Serialize>(value: &T) -> Result<DslValue, String>
pub fn from_dsl_value<T: serde::de::DeserializeOwned>(value: DslValue) -> Result<T, String>

// after
pub fn to_dsl_value<T: ToValue>(value: &T) -> Result<DslValue, String> { Ok(value.to_value()) }
pub fn from_dsl_value<T: FromValue>(value: DslValue) -> Result<T, String> {
    T::from_value(value).map_err(|error| error.to_string())
}
```

The `Result<_, String>` return shape was kept deliberately (even though `ToValue::to_value` itself
is infallible) so every existing `?`/`.map_err(...)`/`.unwrap_or(...)`/`.expect(...)` call site
needed **zero** source changes as long as its `T` already implemented `ToValue`/`FromValue`.

The old serde-visitor bridge module (`🧰️framework/🔨️modules/🌱️value/🔀️serde/🦀️component.rs`, 717
lines implementing a full `serde::Serializer`/`Deserializer` pair over `DslValue`) had exactly one
caller — these two functions — confirmed by a repo-wide grep before deleting it. It is now gone,
not deprecated, per this ticket's "no shims, no compat layers" rule.

## Measurement — distinct types vs raw call-site count

| method | count |
|---|---|
| raw substring grep (`to_dsl_value\|from_dsl_value` anywhere, the ticket brief's own method) | **932** |
| real generic-bridge call sites (word-boundary match, excluding `pack::json::to_dsl_value`/`from_dsl_value` and the flattened `json_to_dsl_value`/`json_from_dsl_value` re-export — a **different, non-generic, `pack::json::Value`-only** bridge with the same function names) | **365**, across **115 files** |

The gap between 932 and 365 is almost entirely the `pack::json_to_dsl_value`/`json_from_dsl_value`
flat re-exports, whose names contain `to_dsl_value`/`from_dsl_value` as a **substring** and inflated
every earlier raw grep in this ticket (including the brief's own "925"). They are a sibling bridge
(`pack::json::Value ↔ DslValue`, non-generic, never serde-bound) and were never in scope here.

**Distinct types actually crossing the generic bridge**: the 365 call sites resolve to two very
different populations —

1. **The composed-child/link pattern** — the large majority. Every hand-written `ToValue`/
   `FromValue` impl for a plugin's `*Diff`/`*Snapshot`/`*Artifact` struct that has a
   `store::ArtifactChild<S>` or `Option<Option<store::ArtifactLink>>`-shaped field routes that one
   field through `to_dsl_value(&self.field)`/`from_dsl_value(field(...))` — this is
   `📓️serde-fanout-playbook.md`'s documented trap #3/#6 pattern, and it is why the call count is so
   high (every composed-artifact plugin repeats it once per composed field). **All of these already
   satisfy the new bound with zero edits**: a peer had already landed
   `impl<S> ToValue for ArtifactChild<S>` / `impl<S> FromValue for ArtifactChild<S>`
   (`🏪️store/🦀️component.rs:2658,2664`, unconditional on `S` — the impl only touches `child_id`/
   `target`, never the phantom snapshot) and `ArtifactRef` already carries
   `#[derive(..., ToValue, FromValue)]` (`🚪️io/🧬️schema/🦀️component.rs:157`). Trap #6's own
   "follow-up work" section, written earlier in this ticket, asked for exactly this impl — it had
   already landed by the time this wave started. Sample verified by inspection:
   `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/{🦀️component.rs,🔺️diff/🦀️component.rs,📸️snapshot/🦀️component.rs}`.
2. **Named domain types via turbofish or explicit `let x: T =`** — a small set, 22 turbofish sites,
   ~7 distinct type names: `Value`/`serde_json::Value` (ambiguous per-file alias, the most common),
   `protocol::DispatchReport`, `semio_framework::kernel::PresenceUpdate`, `ViewModel`,
   `ui_contract::UiIntent`, `ui_contract::UiPatchOps`, `OsWorkflowCamera`.

## Converted by derive vs by hand — and why

**Nothing needed converting in this wave's own edit** for `os-kernel`/`replication`/`draw-fsm` to
compile: population 1 above was already covered by the pre-landed `ArtifactChild<S>`/`ArtifactRef`
impls, and population 2's `DispatchReport`/`MergeReport`/`Conflict` already had hand-written
`impl crate::value::ToValue`/`FromValue` in `📡️replication/⚔️conflict/🦀️.rs:195,207,289,298,340,352`
(landed by the `serde-off-guest-path.md` pass, for the unrelated reason of unblocking
`plugin`/`plugin-host`'s own decode calls). Converting the bridge's bound just let this pre-existing
coverage finally pay off end to end.

**Found missing, NOT converted this wave** (population 2's remaining names) — every one lives in a
crate (`semio-framework-plugin`, `semio-framework-plugin-host`, the `semio-framework` facade that
mounts `🛂️manifest`) that was **already unable to compile before this wave's edit**, for a
confirmed-unrelated reason (next section), so there was no way to verify a hand-written impl
actually compiles:

| type | file | derives today | needs |
|---|---|---|---|
| `semio_framework::kernel::PresenceUpdate` | `🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️presence.rs:77` | `Serialize, Deserialize` only | `ToValue, FromValue` derive — straightforward (`rename_all = "camelCase"`, `default`+`skip_serializing_if` fields, all macro-supported), but transitively needs `Activity`, `PeerMark`, `OwnPresence` (same file, same gap) and `crate::SurfaceId` (`🦀️document.rs:19`, a tuple struct wrapping `UiText`, a hand-rolled fixed-capacity string type with its own hand-written `Serialize`/`Deserialize` — needs a hand-written `ToValue`/`FromValue` too, tuple structs aren't derive-supported) |
| `ViewModel` | `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:4133` | `Serialize, Deserialize` only | `ToValue, FromValue` derive, straightforward shape |
| `ui_contract::UiIntent` | `🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️action.rs:1336` | `Serialize, Deserialize` only | derive, plus its `SurfaceId`/`UiRevision`/`UiNodeId` field types need the same treatment |
| `ui_contract::UiPatchOps` | `🖱️ui/🧬️contract/♻️retirement/📋️patch/🦀️component.rs:27` | hand-written `impl Serialize`/`Deserialize` only (sequence-shaped, no derive at all — private fields, custom bounded-capacity storage) | hand-written `ToValue`/`FromValue` mirroring the existing sequence-shaped serde impl (`DslValue::Array` of each `UiPatchOp::to_value()`), which itself needs `UiPatchOp` (an enum, `🦀️document.rs:194`) converted first |
| `Value` / `serde_json::Value` (raster, gis, remodel, renderer engine elements, `🌉️mcp/workspace`, `🏃️run`, `🖥️host`) | ~15+ sites | N/A — raw `serde_json::Value` | **not** a derive target — per this ticket's own design, `serde_json::Value` should never get `ToValue`/`FromValue`. Each site should route through the direct `DslValue::from(&serde_json::Value)` / `serde_json::Value::from(&DslValue)` conversions already in this same file (`🌱️value/🦀️component.rs:113,140`, infallible, no `Result`) instead of the generic bridge — a mechanical rewrite, not a derive/impl addition |

None of the above were edited. `DispatchReport`/`MergeReport`/`Conflict` needed **no** change (already covered, hand-written, pre-existing). `OsWorkflowCamera` appears only in a doc-comment
(`✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/⚙️engine/🦀️component.rs:226`) describing an OLD path, not a
live call site — not counted as real.

## A separate, pre-existing, confirmed-unrelated blocker discovered while verifying

`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs` (`ActionDescriptor`
and siblings) derive `Serialize, Deserialize` on structs with an `Option<DslValue>` field.
`DslValue` has **never** implemented `serde::Serialize`/`Deserialize` — confirmed by a repo-wide
grep (`impl.*Serialize for DslValue` / `impl.*Deserialize.*for DslValue`, zero hits) — true both
before and after this wave's edit, and the old `🔀️serde` module never provided such an impl either
(it implemented a `Serializer`/`Deserializer` that serialize *other* types *through* `DslValue`, not
`DslValue` itself). This produces 14 `E0277`s compiling `semio-framework-ui` (the `wgpu` feature),
which is an **unconditional** dependency of `semio-framework-plugin`, `semio-framework-plugin-host`,
and the `semio-framework` facade (hence blocking `cargo check -p semio-framework` and everything
above it, for both native and `wasm32-wasip2` targets — this is a source-level trait-bound error,
not a link-time/target-gating issue, so `--target wasm32-wasip2` does not route around it). The
file's last commit is `a03e259755` (2026-08-26), working tree clean at session start — not a live
peer edit, a real latent bug. Flagged as a background task
(`task_55899c56`, "Fix DslValue missing serde impl in ui/wgpu component.rs") rather than fixed here,
since it is unrelated to this ticket's serde-fanout scope and touching it risked reintroducing serde
onto `DslValue` itself — exactly the debt this ticket removes — without being the right fix.

**Consequence for this wave's scope**: `semio-framework-ui-contract` (home of `PresenceUpdate`/
`UiIntent`/`UiPatchOps`/`Activity`/`PeerMark`/`OwnPresence`/`SurfaceId`) compiles **clean on its
own** (`cargo check -p semio-framework-ui-contract` → 0 errors, confirmed after this wave's edit) —
it doesn't call the bridge on its own types, only *consumers* of it do, and every one of those
consumer crates (`plugin`, `plugin-host`, `semio-framework` facade) was already unable to compile
before this wave touched anything. This wave did not turn any previously-compiling crate red — the
missing-derive gaps in the table above are pre-existing latent risk, surfaced by the bound change
but not yet provably converted, because there is currently no way to compile-verify a fix to them
until the unrelated `wgpu`/`DslValue` bug is resolved.

## Verification — verbatim tails

```
$ cargo check -p semio-framework-replication --message-format=short
    Checking semio-framework-replication v0.1.0 (...)
warning: `semio-framework-replication` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized] target(s) in 52.27s
```

```
$ cargo check -p semio-framework-os-kernel --message-format=short
    Checking semio-framework-os-kernel v0.1.0 (...)
warning: `semio-framework-os-kernel` (lib) generated 33 warnings
    Finished `dev` profile [unoptimized] target(s) in 4.90s
```
(33 warnings — the exact same count as this ticket's own previously-recorded baseline; 0 new
warnings, 0 errors.)

```
$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm --message-format=short
   Compiling semio-s-plugin-draw-fsm v0.1.0 (...)
    Finished `dev` profile [unoptimized] target(s) in 9.36s
```

```
$ cargo check -p semio-framework-ui-contract --message-format=short
    Checking semio-framework-ui-contract v0.1.0 (...)
warning: `semio-framework-ui-contract` (lib) generated 47 warnings (23 duplicates)
    Finished `dev` profile [unoptimized] target(s) in 9.99s
```

### `cargo tree -i serde` / `-i serde_json` for draw-fsm, wasm32-wasip2 — before and after

Unchanged from the ticket's own prior measurement (both trees read in full, not truncated):

```
$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i serde --edges normal
serde v1.0.228
├── semio-framework-os-kernel v0.1.0 (...)
│   └── semio-s-plugin-draw-fsm v0.1.0 (...)
└── semio-framework-replication v0.1.0 (...)
    ├── semio-framework-os-kernel v0.1.0 (...) (*)
    └── semio-framework-pack v0.1.0 (...)
        └── semio-framework-os-kernel v0.1.0 (...) (*)

$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i serde_json --edges normal
serde_json v1.0.149
└── semio-framework-os-kernel-dsl-derive v0.1.0 (proc-macro) (...)   # host-only, not linked
    └── semio-framework-os-kernel v0.1.0 (...)
        └── semio-s-plugin-draw-fsm v0.1.0 (...)

serde_json v1.0.149
├── semio-framework-os-kernel v0.1.0 (...) (*)
└── semio-framework-replication v0.1.0 (...)
    ├── semio-framework-os-kernel v0.1.0 (...) (*)
    └── semio-framework-pack v0.1.0 (...)
        └── semio-framework-os-kernel v0.1.0 (...) (*)
```

**Both edges into `serde`/`serde_json` are `os-kernel`'s and `replication`'s own direct `Cargo.toml`
entries** — neither routes through the generic bridge any more (the bridge itself is now
`ToValue`/`FromValue`-bound and carries no serde edge). This is exactly the "remaining path to zero"
this ticket's own `serde-off-guest-path.md` predicted: `os-kernel`'s ~150 direct usages and
`replication`'s `OrderedMap`/`Dictionary`/`diagnostic`/JSON-export reasons, both explicitly fenced
off as later waves.

## Files touched

- `🧰️framework/🔨️modules/🌱️value/🦀️component.rs` — the bridge signature + doc comments (only file
  edited for the core conversion).
- `🧰️framework/🔨️modules/🌱️value/🔀️serde/🦀️component.rs` — deleted (717 lines, dead code, single
  caller was the two functions above).

No other file needed a source change — every reachable call site in `os-kernel`, `replication`, and
`draw-fsm` already satisfied the new bound.

## What remains (counts, for whoever picks this up)

1. **Separate wave, already fenced off by this ticket's own brief**: `os-kernel`'s ~150 direct
   serde usages and `replication`'s `OrderedMap`/`Dictionary`/`diagnostic`/JSON-export reasons. Not
   touched here; this is what keeps serde in draw-fsm's tree.
2. **4 named types + their transitive field types**, found but not converted, blocked from
   compile-verification by the unrelated `wgpu`/`DslValue` bug (task `task_55899c56`): `PresenceUpdate`
   (+ `Activity`, `PeerMark`, `OwnPresence`, `SurfaceId`/`UiText`), `ViewModel`, `UiIntent`,
   `UiPatchOps` (+ the `UiPatchOp` enum). Once the blocker crate compiles again, add
   `#[derive(ToValue, FromValue)]` (mirroring each struct's existing `#[serde(...)]` attributes with
   `#[value(...)]` twins, same recipe as `📓️serde-fanout-playbook.md`) to the derive-eligible ones,
   and hand-write `UiText`/`SurfaceId`/`UiPatchOps`/`UiPatchOp` (tuple struct, custom sequence shape,
   enum — none derive-eligible).
3. **~15+ `serde_json::Value`/`Value` pass-through call sites** (raster, gis, remodel, renderer
   engine elements, `🌉️mcp/workspace`, `🏃️run`, `🖥️host`) — mechanical rewrite to the direct
   `DslValue::from(&serde_json::Value)`/`serde_json::Value::from(&DslValue)` conversions instead of
   the generic bridge, not a derive/impl addition. Also blocked from verification by the same bug
   for most of these files.
4. **The unrelated `wgpu`/`DslValue` serde bug itself** (flagged as `task_55899c56`) — blocks
   `cargo check` on `semio-framework-plugin`, `semio-framework-plugin-host`, and the `semio-framework`
   facade entirely, for every target including `wasm32-wasip2`, independent of this ticket.
