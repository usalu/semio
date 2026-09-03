# 🧵 serde_json removal — commands + schema (4 owned files)

## Per-file conversion
- **✏️patch-object/🦀️.rs**: dropped `use serde_json::Value`. `serde_json::from_str::<Value>` →
  `dsl::json::from_json_str::<dsl::DslValue>`. `.as_str()`/`.as_bool()` unchanged (same API on
  `DslValue`). Payload shape/route-oracle-pinned struct untouched.
- **🌞sun/🦀️.rs**: dropped `use serde_json::json`. `json!({"value": value})` →
  `dsl::DslValue::object([...]).into()`. Framework's `apply_world3d_sun_action`
  (`🧰️framework/…/🔌️plugin/🦀️.rs`, `world3d_host`, untouched) still requires
  `Option<&serde_json::Value>` — genuinely framework-forced; only the type name appears (via
  `Into` inference), never a `serde_json::` call.
- **🧰utility/🦀️.rs**: dropped `use serde_json::{Map, Value}`. `SetUtilityParam::handle` now:
  `serde_json::Value → DslValue::from(&params)` (peer's `utility_params_value`, `🧭️view/🦀️.rs`,
  still returns `serde_json::Value` — verified against its current post-edit code) → merge as
  `Vec<(String, DslValue)>` → `dsl::json::to_json_string`.
- **🧬schema/🦀️.rs**: dropped `use serde_json::Value` (prod) and a second `use serde_json::Value`
  inside the `#[cfg(all(test, feature="cad-fixtures"))]` mod (was still tripping the file-level
  import-regex despite being test-only). `mesh_data_from_transfer` body now bridges through
  `dsl::DslValue::from`/`dsl::FromValue` instead of `serde_json::from_value`; `mesh_document_from_mesh`
  now builds via `dsl::DslValue::object(...).into()` instead of `serde_json::json!`.
  `mesh_data_from_transfer`/`mesh_document_from_mesh`/`mesh_from_mesh_document`/
  `lowpoly_document_from_mesh` all KEEP `serde_json::Value` at their signature boundary (fully
  qualified, no `use`) because the peer's `⚙️engine`/`✏️editor` call sites still hand these fns a
  real `serde_json::Value` — verified against the peer's current code, not my earlier read.

## Breach gate
0 `testing/dependency` breaches remain for ALL of lowpoly (both this slice's 4 files and the
peer's 7) as of the latest `bun ./📜️script.ts test` run. Before: 11/11. After: 0/11.

## Compilation — root-caused, nothing to revert in these 4 files
`cargo check -p semio-s-plugin-lowpoly --all-targets` (confirmed via `Compiling semio-s-plugin-lowpoly`
in the log, not a framework-only run) shows 34 `error[E0277]`, ALL the identical shape: some type
(`LowpolyObject`, `LowpolyObjectPatch`, `CreateMesh`, `Severity`) does not implement
`serde::Serialize`/`Deserialize`. Traced to ONE root: `LowpolyObject` itself
(`✏️s/…/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️.rs:105`) — outside BOTH agents' 11-file slice —
currently derives only `Clone, Debug, PartialEq, ToValue, FromValue`, with NO Serialize/Deserialize
at all, not even `#[cfg_attr(test, ...)]`. `git status` shows this file `M` (uncommitted), distinct
from either agent's edits — a third, concurrent session's in-flight change (consistent with
CLAUDE.md's framework-churn warning). Every downstream type that transitively contains
`LowpolyObject` and still carries a test-only `#[cfg_attr(test, derive(Serialize, Deserialize))]`
now fails in test builds.

Only 3 of the 34 errors cite one of my 4 files (`🧬schema/🦀️.rs:12` and `:20`) — both are the
**pre-existing, untouched** `#[cfg_attr(test, derive(Serialize, Deserialize))]` line on
`LowpolyArtifact` and its `objects: Vec<LowpolyObject>` field, not anything from this pass's edits.
Zero errors reference `dsl::json`, `DslValue`, or any function I touched — no type mismatch, no
unresolved path, nothing from my conversions. There is nothing in these 4 files to revert to
`serde_json`; the break is upstream and out of scope for this ticket slice.
