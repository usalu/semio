# 📓️ Packet `contract-action` — report

## Done

Replaced the three scaffold files wholesale, kept the `//! @emoji` headers and `//#region` structure:

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️action.rs` — regions `🔖️Action` (`ActionId`,
  `Trigger`, `ActionBinding`, `MenuRef`, `UiIntent`) and `🔖️Value` (`UiValue`).
- `…/🦀️presence.rs` — region `🔖️Presence` (`Activity`, `PeerMark`, `OwnPresence`, `PresenceUpdate`).
- `…/🦀️limits.rs` — regions `🔖️Limits` (`UiDocumentLimits` + byte/finite/section helpers), `🔖️Validate`
  (`UiContractViolation`, `validate_snapshot`, the shared iterative `validate_core`), `🔖️Apply`
  (`PatchRejection`, `QuotaKind`, `apply_patch`, `apply_op`, `remove_subtree`).

Exactly the four required types are defined once each in the crate: `Activity` (presence.rs),
`ActionBinding` (action.rs), `MenuRef` (action.rs), `UiValue` (action.rs) — confirmed via a crate-wide
grep for duplicate `pub struct`/`pub enum` names (none found; see Acceptance commands).

Every non-test `fn` carries the `// 🚫️async: U1 …` tag (verified by script, not by eye — see Acceptance).
No `async fn`, no `.await`, no `dyn` anywhere in the three files.

`validate_core` is generic over `V: Borrow<UiNodeRecord>` so `validate_snapshot` (borrowed
`HashMap<UiNodeId, &UiNodeRecord>` built from a `UiSnapshot`'s `Vec`) and `apply_patch`'s post-op check
(owned `HashMap<UiNodeId, UiNodeRecord>` on `UiSnapshotState`) share one traversal — no logic duplicated
between the two entry points. The walk is iterative (explicit `WalkFrame::Enter`/`Exit` stack), not
recursive, matching the flat-table design `🦀️document.rs`'s own header calls out; cycle detection uses
an ancestor-path set (`on_path`) so a true back-edge is distinguished from a node merely reachable via
two parents (the latter is silently deduped, not flagged — no violation variant was specified for a
DAG-shaped share, only for a genuine cycle).

## Acceptance: UNRUN (ruling U4)

I did not run cargo. Commands for the coordinator (per U4: `CARGO_TARGET_DIR` in the session scratchpad,
never the ticket folder; both `--lib` and `--all-targets`; explicit 600000ms timeout):

```
cargo check -p semio-framework-ui-contract --lib --timeout 600000
cargo check -p semio-framework-ui-contract --all-targets --timeout 600000
cargo test  -p semio-framework-ui-contract --lib --timeout 600000
cargo test  -p semio-framework-ui-contract --lib --features typegen --timeout 600000
```

The `typegen` run matters specifically to settle the `UiValue` recursion question below — I could not
execute it myself.

What I *did* run (non-cargo, cheap, this session):

- Brace/paren/bracket balance and `//#region`/`//#endregion` pairing on all three files (`python3 -c`
  count + grep) — balanced, all six required regions present and closed.
- Crate-wide `grep` for `^pub struct \|^pub enum ` across every `*.rs` in the crate dir, deduped by name
  — zero collisions; the four required types appear exactly once each, nowhere redefined.
- A script asserting every non-test `fn`/`pub fn`/`pub const fn` in the three files has the U1 tag on
  its immediately preceding line — 21 of 21 pass (3 in action.rs, 1 in presence.rs, 17 in limits.rs).
- Grepped for `serde_json`/`pack::` usage outside `#[cfg(test)]` blocks in the three files — none; the
  crate's dependency-free-for-wasm32-wasip2 guarantee (Cargo.toml has no `pack`/`serde_json` runtime
  dep) is not violated by anything I wrote.
- Manually traced every `#[test]` in `limits.rs` against the actual struct/enum definitions in
  `component.rs`/`document.rs`/`layout.rs` (field names, types, variant names) to catch type-mismatch
  bugs a compiler would otherwise need to find — no discrepancies found on this pass.

## Decisions

**Real `ActionDescriptor` usage → `ActionId` mapping.** Grepped the fleet (`grep -n "ActionDescriptor *{"`
across `✏️s/🔌️plugins/**` and the os plugin module). The shape is uniform:
`ActionDescriptor { controller_id: String, action: String, args: Option<DslValue> }`, built almost
everywhere through `ActionFactory::new(CONTROLLER_ID).action(name, args)`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:5839`). Real values:
`controller_id` is kebab-case (`"cad-play"`, from `CAD_PLAY_CONTROLLER_ID` in the CAD plugin) or a bare
domain word (`"app"`); `action` is camelCase (`"objectMove"`, `"setValue"`, `"addWidget"`, `"submit"`,
`"edit"`). This maps directly onto `ActionId { scope: controller_id, name: action, version }` — `scope`
takes over `controller_id` verbatim, `name` takes over `action` verbatim, and `version` is the wholly new
axis (no old counterpart), defaulted to `1` via `ActionId::v1(scope, name)`. `Display` renders
`"{scope}.{name}@{version}"` per the brief's own example.

**`UiValue` recursion and ts-rs.** This crate has no runtime dependency I could use to actually run
`cargo test --features typegen` (U4 forbids it regardless). I read the vendored `ts-rs-10.1.0` source at
`~/.cargo/registry/src/*/ts-rs-10.1.0/src/export.rs` instead of guessing: `export_recursive::<T>` guards
itself with `if !seen.insert(TypeId::of::<T>()) { return Ok(()); }` before visiting `T`'s dependencies —
i.e. ts-rs already assumes a type can depend on itself (directly or through a container) and short-circuits
the walk rather than looping. `ts-rs-macros-10.1.0` also greps positive for `untagged` (in
`types/enum.rs`, `attr/variant.rs`, `attr/enum.rs`), confirming `#[serde(untagged)]` is a recognised
attribute, not silently ignored. Both facts together support that `UiValue`'s shape — `Vec<UiValue>` and
`BTreeMap<String, UiValue>`, both heap-indirected so Rust's own derives (`Clone`/`Debug`/`PartialEq`/
`Serialize`/`Deserialize`) already resolve it with no special handling — should export as a self-referential
TS type alias (`type UiValue = null | boolean | number | string | UiValue[] | { [k: string]: UiValue }`),
which TypeScript itself permits for array/index-signature indirection. I am confident in the Rust-side
compile (ordinary recursive-through-Vec/BTreeMap enum, a well-established pattern with no derive-macro
special case needed) but **UNCONFIRMED on the actual generated `.ts` output** — that needs the
`cargo test -p semio-framework-ui-contract --lib --features typegen` run above, since I did not execute it.

**`UiDocumentLimits` defaults and reasoning** (also inline as field docstrings in `🦀️limits.rs`):

| field | default | reasoning |
|---|---|---|
| `max_nodes` | 20 000 | covers the largest known tree (full product tree/timeline) with headroom, well under where a `HashMap<UiNodeId, UiNodeRecord>` becomes a memory concern for a malicious flood |
| `max_depth` | 128 | far beyond any legitimate nesting (deepest real shape, a Tree/TreeSection/TreeItem chain, rarely exceeds a few dozen); doubles as the traversal's own bounded-recursion-depth guarantee |
| `max_children` | 4 096 | covers the largest legitimate flat list (an unpaginated tree section) without one node alone approaching `max_nodes` |
| `max_text_bytes` | 65 536 (64 KiB) | generous for authored UI copy, far beyond a label/description, while refusing an arbitrarily large string through one component |
| `max_patch_ops` | 4 096 | mirrors `max_children`'s order of magnitude — no legitimate single reconciliation pass needs more ops than the crate's own largest single-node fan-out |
| `max_patch_bytes` | 1 048 576 (1 MiB) | conservative single-frame/actor-mailbox transport budget |

**`max_nodes`/`max_depth` vs. `max_children`/`max_text_bytes`/`max_patch_ops`/`max_patch_bytes`
enforcement split.** The first two are whole-*document*-shape properties, only knowable after a patch's
ops are applied to the shadow draft — they surface as `UiContractViolation::NodeQuota`/`DepthQuota` inside
`PatchRejection::InvariantViolated`. The other four are properties of the incoming *patch itself* (or of
one op's payload) and are checked by `apply_patch`/`apply_op` directly, before or instead of touching the
draft — surfaced as `PatchRejection::QuotaExceeded { quota: QuotaKind, .. }`. `QuotaKind` therefore only
has 4 variants (`Children`, `TextBytes`, `PatchOps`, `PatchBytes`), not 6 — `NodeQuota`/`DepthQuota` are
never duplicated into it.

**`patch_byte_estimate` is a proxy, not exact wire bytes.** The crate has no `pack`/`serde_json` runtime
dependency (by design — see `📦️glue.rs`'s wasm32-wasip2 guarantee), so `max_patch_bytes` can't be checked
against an actual encoding. It sums UTF-8 byte lengths of each op's own text-bearing payload
(`component_text_bytes`, itself reused by `max_text_bytes` enforcement) plus a flat 16-byte per-op
overhead. Documented as an estimate in its own docstring.

**`UiContractViolation`/`PatchRejection` variant → test-scenario mapping**, since the brief gave variant
names with elided fields (`{..}`) and two required `validate_snapshot` test scenarios needed a specific
home:
- *"a dangling child reference"* → `OrphanChild { parent, child }` — a parent's `children` names an id
  with no record.
- *"a node unreachable from the root"* → `DanglingRoot { node }` — generalized to mean "not reachable from
  a valid root by any path," which also subsumes the degenerate case where `root` itself names a missing
  id (then nothing is reachable, and every live node gets its own `DanglingRoot`).
- `SectionNested { node }` — a `Container` with `role: Section` directly or transitively inside another
  `Section` ancestor; sectioning is flat by design (one level), so nested `Section`s are rejected rather
  than silently allowed with ambiguous chrome.

**Not implemented / possible gap:** `apply_patch` does not check `patch.surface == state.surface`. The
brief's three named rejection reasons (revision mismatch / quota exceeded / invariant violated) don't
include a surface mismatch, so I kept `PatchRejection` to exactly those three categories rather than
adding a fourth unrequested one. Flagging in case the coordinator wants it — it would be a same-file,
backward-compatible addition (`PatchRejection::SurfaceMismatch { expected, actual }`) if wanted.

## Registrar-requests

None. All required work fit inside the three owned files; no change to any file outside my OWNS list was
needed.

## Deviations

None from the packet brief. One clarification: `ButtonProps::icon`/`RowAction::icon` etc. (plain `String`
icon keys) are pre-existing decisions from sibling packets (`contract-doc`), not something I touched or
relied on beyond using the already-defined `crate::Component`/prop-struct shapes as given.

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️action.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️presence.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️limits.rs`
