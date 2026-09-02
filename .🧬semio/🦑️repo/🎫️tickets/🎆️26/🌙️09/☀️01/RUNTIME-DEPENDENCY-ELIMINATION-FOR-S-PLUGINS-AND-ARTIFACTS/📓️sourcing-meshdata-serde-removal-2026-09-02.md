# Sourcing: Removed Last `MeshData: serde::Serialize` Requirement

## What was wrong

The previous attempt at this fix did not persist to disk (disk-full incident). On re-inspection,
`🧬️schema/🦀️.rs` was still on the `serde_json::json!`/`Value` version:

- `kind_mesh_json` (line 239) called `json!({ "id": kind.id, "data": mesh })` where `mesh: MeshData`
  — `json!` requires `MeshData: Serialize`, which `MeshData` (in
  `🧰️framework/🔨️modules/🔺️mesh-engine/🦀️.rs`) does not carry in production (only
  `#[cfg_attr(test, derive(Serialize, Deserialize))]`, plus hand-written first-party
  `impl pack::value::ToValue for MeshData` / `impl pack::value::FromValue for MeshData`, already
  landed by other work on this same ticket).
- `instance_json` (line 247) used `json!`/`Value` too, for call-site type consistency with
  `kind_mesh_json`, though it never touched `MeshData`.

## Fix

Converted both functions in `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
to return `dsl::DslValue` (built with `dsl::DslValue::object([...])`), taking the mesh via
`dsl::ToValue::to_value(&mesh)`. Removed the now-unused `use serde_json::{json, Value};` import.
`dsl` here is `extern crate semio_framework_os_kernel as dsl;` (crate root alias), whose
`DslValue`/`ToValue`/`FromValue` are re-exports of `protocol::value::{...}` — the identical trait
`MeshData`'s hand-written impls target (`pack::value` in mesh-engine is `pub use protocol::value;`
in the `🎒️pack` crate), so no bridging or new dependency was introduced.

Positions/rotation/scale arrays go through `dsl::ToValue::to_value` on `[f64; N]` (via the generic
`impl<T: ToValue, const N: usize> ToValue for [T; N]`, backed by `impl_float_codec!(f64, f32)`),
which encodes as `DslValue::Number(Number::Float(..))` — never touching the integer arms, so no
positions/scale value risked being emitted as an int. Mesh indices stay integers because they flow
through `MeshData`'s own hand-written `ToValue` (`ints` helper → `Number::UInt`), untouched by this
change.

Updated the two call sites that serialized these functions' return values to strings:
- `…/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs`: `meshes_json`/`instances_json` now
  built with `dsl::json::to_json_string(&dsl::DslValue::Array(vec![...]))`; removed
  `use serde_json::json;`.
- `…/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/🦀️.rs`: same pattern over the accumulated
  `meshes`/`instances` vectors; removed `use serde_json::json;`.

## Verification (default `target/`, warm, foreground `cargo check -p semio-s-plugin-sourcing`)

To get a real before/after count without a destructive git operation, the three edited files were
temporarily reverted to their pre-fix (serde) content, `cargo check` was run to capture the
baseline, then restored to the fixed content from an in-scratchpad backup and re-checked.

- **Before** (serde version, confirmed present on disk at task start):
  12 previous errors (rustc summary) / 10 distinct `: error[` lines via
  `grep -cE ': error(\[|:)'`. Included:
  `🧬️schema/🦀️.rs:242:5: error[E0277]: the trait bound `MeshData: serde::Serialize` is not
  satisfied ... required by a bound introduced by this call`.
- **After** (this fix): 11 previous errors (rustc summary) / 9 distinct `: error[` lines. The
  `MeshData` error is gone. Diffing the two error line lists confirms the after-set is exactly the
  before-set **minus** the `MeshData` line — no new errors introduced by this change.

### Attribution of the remaining 9 errors (none touch `MeshData`, `kind_mesh_json`, or
`instance_json`; all pre-exist this fix and match the three concurrent peer migrations flagged in
the task):

- **Peer churn** (9/9): 6× `ArtifactChild<SemioKitSnapshot>: serde::Serialize`/`Deserialize` not
  satisfied (`🧬️schema/🦀️.rs`, `📸️snapshot/🦀️.rs`), 1× `SemioKitType: serde::Serialize` not
  satisfied (`🗂️curate/🦀️.rs:203`), 2× `command_from_action`/`host_configuration_mutation`
  incompatible type `expected dsl::DslValue, found serde_json::Value` (`✏️editor/🦀️.rs:963,967`).
  These are all `store`/mutation-descriptor-shaped errors consistent with the UiNode/const-eval and
  `Mutation` trait `DESCRIPTORS` migrations, not the mesh/JSON path touched here.
- **Attributable to me**: 0.

Success criterion met: no `MeshData`/serde-family error remains in `semio-s-plugin-sourcing`.

## Files touched

- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
  — `kind_mesh_json`/`instance_json` converted to `dsl::DslValue`; removed
  `use serde_json::{json, Value};`.
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs`
  — call sites now use `dsl::json::to_json_string`; removed `use serde_json::json;`.
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/🦀️.rs`
  — same; removed `use serde_json::json;`.

No changes were made to `🧰️framework/🔨️modules/🔺️mesh-engine` (verified untouched, per task
instruction — it already carries `MeshData`'s first-party `ToValue`/`FromValue` plus its
`#[cfg(test)]` serde oracle). No git commands used; no worktrees; ticket not opened/closed/reopened
by this session.
