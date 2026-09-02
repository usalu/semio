# 🌉️mcp additive ToValue/FromValue — done; 🎭️actor — architecturally blocked without Cargo.toml

## Scope worked
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/` — **NOTE**: the ticket brief said this module lives
  at `🧰️framework/🔨️modules/🌉️mcp/`; that path does not exist. The real, only `🌉️mcp` module in the repo
  is under `🛍️products/💻️os/🔨️modules/🌉️mcp/`, mounted by its own crate `semio-framework-os-mcp`
  (`🌉️mcp/📦️packages/🦀️rust/🦀️.rs`). Worked that one.
- `🧰️framework/🔨️modules/🎭️actor/` — path as given, mounted by crate `semio-framework-actor`.

## 🌉️mcp — complete, all 64 real serde-only derives now dual

Command from the brief (`find ... | xargs grep -n '#\[derive('  | grep -E 'Serialize|Deserialize' |
grep -v cfg_attr | grep -v ToValue`) returns **67 raw hits before**, **8 after** — but the 8 remaining
are ALL confirmed false positives (grep can't see a hand-written impl or a derive placed on a
*different* line than the struct it decorates):
- 3× in `🏠️workspace/🦀️.rs` (`ProbeSnapshot`/`ProbeDiff`/`ProbeMutation`) — already had hand-written
  `store::ToValue`/`store::FromValue` impls immediately below each, **pre-existing**, untouched.
- 1× in `🗂️catalog/🦀️.rs` (`CapabilityOwner`) — hand-written by me (see below), so no `ToValue`/
  `FromValue` token sits on the `#[derive(...)]` line itself.
- 4× in `🧭️protocol/🦀️.rs` (`JsonRpcId`, `JsonRpcIncoming`, `JsonRpcOutcome`, `ContentBlock`) — same
  reason, hand-written by me.

**Real count: 67 − 3 pre-existing = 64 needing work → 64/64 done.** Per-facet: errors(2) audit(3)
policy(1) ui(1) inference(2) handles(4) catalog(10, 1 hand-written) schema(8) protocol(17, 4
hand-written) dispatch(9) bridge(7, 1 nested inside `#[cfg(test)] mod server`).

### Two places the derive genuinely cannot mirror serde — hand-written instead
1. **`#[serde(rename_all = "SCREAMING_SNAKE_CASE")]`** — the derive only supports `camelCase` /
   `kebab-case` / `lowercase` / `snake_case` (checked its own source,
   `🌱️value/✨️derive/🦀️.rs` line ~150). Every occurrence (`GatewayErrorCode`, `AuditDecision`,
   `JobStatus`×2, `HandleKind`, `InvocationStatus`, `JobState`) got `#[value(rename = "...")]` spelled
   out per-variant instead — same wire names, still uses the derive.
2. **`#[serde(skip_serializing_if = "...")]` on an enum VARIANT's own named field** — the derive's
   internally-tagged codegen (`expand_to_value`/`expand_from_value`, `Fields::Named` arms) never reads
   `skip_serializing_if` at that position; only struct-field position honors it. A derive there would
   silently start emitting `null` for an omitted field instead of dropping the key —
   a real wire-shape change. Two types hit this: `CapabilityOwner::Plugin{app_id,window_kind_id,
   mode_id}` (`🗂️catalog/🦀️.rs`) and `ContentBlock::ResourceLink{name,mime_type}` /
   `ContentBlock::Resource{mime_type,text,blob}` (`🧭️protocol/🦀️.rs`). Both got fully hand-written
   `ToValue`/`FromValue` impls mirroring serde's tag/rename_all/omit-if-`None` behavior exactly.

### `#[serde(untagged)]` — hand-written (3, all in `🧭️protocol/🦀️.rs`)
`JsonRpcId` (Number/String/Null), `JsonRpcIncoming` (Batch/Single, dispatched on array-vs-not shape),
`JsonRpcOutcome` (Result/Error, dispatched on which of the two keys is present). Each impl mirrors
serde's own untagged variant-order probing.

### The `deserialize_double_option` pattern
`JsonRpcRequest.id: Option<JsonRpcId>` uses `#[serde(default, deserialize_with = "deserialize_some",
skip_serializing_if = "Option::is_none")]` to keep an explicit `"id":null` distinct from an absent
`id` key. The derive's own doc names this exact shape ("deserialize_double_option") as supported via
`default` + `deserialize_with`; wrote a `value_deserialize_some_id` twin and combined it the same way.

### `serde_json::Value` fields
No `ToValue`/`FromValue` impl exists for foreign `serde_json::Value` (checked — none in the repo).
Bridged every occurrence via `#[value(serialize_with = ..., deserialize_with = ...)]` (and an
`Option<serde_json::Value>` variant) built on `🌱️value/🦀️.rs`'s own pre-existing, infallible
`impl From<&DslValue> for serde_json::Value` / `impl From<&serde_json::Value> for DslValue` — a
handful of tiny private free functions per file, not a new dependency.

### Everything else
Plain `#[value(...)]` twins: `transparent` (2 tuple structs), `tag`/`tag+content`/`rename_all_fields`
combos (all supported directly), `flatten` (`JsonRpcResponse.outcome`, works because my hand-written
`JsonRpcOutcome` already returns/accepts a `DslValue::Object`), per-field `rename`/`default`/
`skip_serializing_if` mirrored 1:1 where the position (struct field) actually supports it.

No `Cargo.toml` touched: `semio-framework-os-mcp` already depended on `semio-framework-os-kernel`
(for `store::sync::ArtifactHost`), which re-exports both the `ToValue`/`FromValue` **traits** and the
`#[derive(ToValue, FromValue)]` **proc-macros** at its crate root (`🛍️products/💻️os/📦️packages/
🦀️rust/🦀️.rs` lines ~332-347) — every edited file just added
`use semio_framework_os_kernel::{DslValue, FromValue, ToValue, ValueError};` (subset as needed).

## 🎭️actor — NOT started, hard architectural blocker

`semio-framework-actor` has **zero** path to `ToValue`/`FromValue`/`DslValue` and cannot get one
without editing `Cargo.toml`, which this ticket's rules forbid:

1. `semio_framework_value_derive`'s generated code hardcodes `::semio_framework_os_kernel::{DslValue,
   ToValue, FromValue, ValueError}` (verified in the derive's own source, `🌱️value/✨️derive/🦀️.rs`
   — every codegen path, no `#[value(crate = "...")]` escape hatch exists). Any crate using the
   derive MUST have `semio_framework_os_kernel` reachable under that literal name.
2. `semio-framework-os-kernel`'s own `Cargo.toml` already lists `semio-framework-actor` as a
   dependency (`features = ["ureq"]`). Actor depending back on os-kernel would be a cyclic package
   dependency — Cargo rejects this outright, feature-gating does not change that (the manifest graph
   itself must be acyclic).
3. `semio-framework-actor`'s Cargo.toml (confirmed by reading it in full) has no other dependency that
   re-exports `DslValue`/`ToValue`/`FromValue` under any name — not `semio-framework-job`, not the
   wasm-only `semio-framework-async`.
4. A workaround of `#[path]`-mounting `🌱️value/🦀️.rs` directly into `actor` (no Cargo.toml edit
   needed for a source-level `#[path]` mount) was considered and rejected: it would give `actor` its
   own **nominally distinct** `DslValue` type, incompatible with every other crate's `DslValue` —
   values would not actually interoperate across the crate boundary, which defeats the whole point of
   this migration and would be silently wrong rather than merely incomplete.

**Recommendation** (not mine to execute — outside this ticket's "no Cargo.toml" rule): either (a) add
`semio-framework-value-derive` + a non-cyclic `DslValue`-owning crate as a real dependency of `actor`
once one exists that `os-kernel` itself doesn't depend on `actor` through, or (b) change the derive
macro to support a `#[value(crate = "...")]` path override so a foundational crate below `os-kernel`
in the graph can point it at whatever local/aliased path actually has `DslValue`. Zero files under
`🎭️actor/` were touched.

## Verification
- Crate that actually mounts these files: `semio-framework-os-mcp` (confirmed via `#[path = ...]` in
  its own `📦️packages/🦀️rust/🦀️.rs`), NOT the top-level `semio-framework` crate.
- `cargo check -p semio-framework-os-mcp` cannot reach type-checking of `🌉️mcp`'s own files at all,
  before OR after my edits: `semio-framework-plugin-host` (an unconditional, non-wasm dependency of
  `semio-framework-os-mcp`) fails to compile due to **pre-existing, unrelated** peer churn
  (`AppRef`/`dsl::io_schema::IoPayload` missing `Serialize`/`Deserialize`, in `🎚️config`/`🔌️plugin`
  — not my modules, not touched by me). Cargo aborts the whole dependency graph at the first failing
  crate, so `semio-framework-os-mcp`'s own compilation never starts.
- Used the `git diff` → `git apply -R` → check baseline → `git apply` → check again pattern specified
  in the ticket brief. Baseline error count/signature was NOT stable across repeated checks — one run
  showed the `AppRef`/`IoPayload` signature (8 errors), a later run (same reverted state) showed a
  DIFFERENT 10-error signature entirely in `🖱️ui`'s `UiNode: ToValue`/`FromValue` — live concurrent
  peer edits mid-session, exactly as flagged ("three peer migrations are live"). With my edits
  re-applied, the error count returned to the 8-error `AppRef`/`IoPayload` signature, **zero of which
  are in any `🌉️mcp` file** (grepped `🌉️mcp` against every check's error list, always empty).
  Also tried `--target wasm32-wasip2` to route around `plugin-host` (excluded there via
  `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`) — blocked by a different pre-existing
  issue instead (`tokio`'s `full` feature set is not wasm-buildable).
- `cargo check -p semio-framework` — **0 errors**, confirmed before and after (this crate doesn't
  depend on `semio-framework-os-mcp`, so it was never going to be affected either way, but the ticket
  brief said to check it explicitly).
- Given the above, correctness for the ~64 mcp types rests on manual verification against the value
  derive's own source (read in full: container-attribute parsing, both struct/enum `ToValue`/
  `FromValue` codegen paths, the `transparent`/`flatten`/`skip_serializing_if`/`deserialize_with`
  branches specifically) rather than a green compile — flagged here so it gets a real compile check
  once the peer churn on `plugin-host`/`config`/`ui` settles.

## Confirmation
**No `Serialize`/`Deserialize` derive or import was removed anywhere.** Every change in `🌉️mcp/` is
additive: a new `ToValue, FromValue` in an existing `#[derive(...)]` list, a new `#[value(...)]`
attribute alongside an unchanged `#[serde(...)]` one, a new hand-written `impl ToValue`/`impl
FromValue` block, or a new tiny private bridge function. No file under `🎭️actor/` was edited. No
`Cargo.toml` was edited anywhere.
