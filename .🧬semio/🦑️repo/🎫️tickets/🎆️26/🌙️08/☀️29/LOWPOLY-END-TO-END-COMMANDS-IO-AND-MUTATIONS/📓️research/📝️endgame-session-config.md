## Endgame: session/config/presence — CLEAN

Scope: `✏️editor/🖌️session/`, `✏️editor/🎚️config/`, `✏️editor/👥️presence/` only.

### 1. `DESCRIPTORS`/`descriptor`
`protocol::Mutation<P>` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:145`) gained two
required items: `const DESCRIPTORS: &'static [MutationLeafDescriptor]` (every direct mutation
leaf's static metadata, in aggregate-variant order) and `fn descriptor(&self) -> &'static
MutationLeafDescriptor` (this value's own leaf entry). Mirrored the already-landed
`procedural2d` config/presence precedent exactly: one `MutationLeafDescriptor` literal per enum
variant (`ExplicitMutation`/`Detect`/`Atomic`, `owner` a provisional not-yet-authored leaf-dir
path under this aggregate's own directory), `descriptor()` a plain match indexing `DESCRIPTORS`.
Added to all three `impl Mutation<..> for ..Mutation` blocks: `LowpolyConfigMutation` (12
variants), `LowpolyPresenceMutation` (1), `LowpolyTransientMutation` (1, in `🖌️session/`).

### 2. `&PaintStrokeState`/`&TransformState`/`&BTreeMap<..>: ToValue` (🖌️session/:610)
Root cause: `#[derive(value_derive::ToValue)]` on `LowpolyTransientStateRef<'a>` (a ref-mirror of
`LowpolyTransientState` used to avoid cloning on serialize) expands to
`ToValue::to_value(&self.field)`, which needs the *field's own type* to impl `ToValue` — for a
`Option<&'a T>` field that means `&'a T: ToValue`, which the codec deliberately never provides
(owned-only). Fix: dropped the derive from `LowpolyTransientStateRef`, kept `#[derive(Serialize)]`
(serde has blanket ref impls), and hand-wrote `impl<'a> dsl::ToValue for
LowpolyTransientStateRef<'a>` building `DslValue::Object` directly — each field converted through
the *owned* type's existing `ToValue` via ordinary method-call auto-deref (`self.stroke.map_or(Null,
ToValue::to_value)`, `dsl::ToValue::to_value(self.mesh_workspace)`), reproducing the same
camelCase object shape the derive would emit. (A blanket `&BTreeMap<..>: ToValue` impl isn't even
orphan-legal here — `BTreeMap` isn't local — so the hand-written struct-level impl was the only
option, not just the tidiest one.)

### 3. Signature mismatches (🖌️session/, was :913/:975, two call sites)
`store::TextSpan::at(line: u32, column: u32)` — call sites passed `error.line()`/`error.column()`
(`serde_json::Error`, both `usize`) with no cast. Fixed both `LowpolyTransient::parse_dsl` and
`LowpolyTransientMutation::OpText::parse_op` to `error.line() as u32, error.column() as u32`.

### Final state
`cargo check -p semio-s-plugin-lowpoly --all-targets` (full run, completed): **0 errors** in
`🖌️session/`, `🎚️config/`, `👥️presence/` — lib and lib-test both clean for these three dirs. The
6 remaining crate errors (`E0433`/`E0277`/`E0308`, all in `✏️editor/🦀️component.rs:281,2086,2091,
2099,2110`) are outside this scope — owned by other agents per the handoff.

Files touched: `$A/✏️editor/🎚️config/🦀️component.rs`, `$A/✏️editor/👥️presence/🦀️component.rs`,
`$A/✏️editor/🖌️session/🦀️component.rs` (`$A` = the lowpoly `✳️any` subset root). No command
behavior, emitted mutations, or interactive-job classification changed — additive trait-conformance
plumbing only.
