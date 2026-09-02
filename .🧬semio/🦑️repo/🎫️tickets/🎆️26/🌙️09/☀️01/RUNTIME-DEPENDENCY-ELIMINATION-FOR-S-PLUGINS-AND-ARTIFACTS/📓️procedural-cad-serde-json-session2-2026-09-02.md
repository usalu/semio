# procedural + cad: serde_json:: elimination — session 2 (2026-09-02)

Scope: `✏️s/🔌️plugins/🌀️procedural` and `✏️s/🔌️plugins/📐️cad` only. Derives were already converted
to `ToValue`/`FromValue` before this session started (per brief; not redone). Task: remove remaining
production `serde_json::` **usage** (calls/types), never the `#[cfg(test)]`/test-domain-dir kind
(deliberate differential oracle, left alone everywhere).

⚠️ **Heavy concurrent churn.** Several other sessions were actively editing these same two plugins
throughout (consistent with the earlier `📓️procedural-cad-serde-json-value-elimination-2026-09-02.md`
in this folder, whose sub-agent wave was still landing). Counts below were re-measured repeatedly
because files kept changing under me; the "before" figure is therefore approximate — treat the
"after" figure and my specific fixes as the reliable part of this report.

## Counting method
`grep -rn "serde_json::" <crate> --include="*.rs"`, then excluding: test-domain dirs
(`🧪️oracle/`, `🧪️test/`, `🧪️tests/`, `🔬️probes/`, `🏭️generator/`, `🧫️fixtures/`), comment-only
lines, and — critically — anything inside a `#[cfg(test)]`-gated item, found with a brace-matching
Python scanner (`/private/tmp/.../scratchpad/find_prod_serde.py`, not committed, scratch only) since
several files have a small production head and a multi-thousand-line `#[cfg(test)] mod tests { … }`
tail that a naive line-range guess misses. A raw `grep -c` on these two crates way overcounts
(300+/100+) purely from those test tails and doc-comment mentions of the string "serde_json" — the
brace-matched count is the trustworthy one.

## Real fixes made this session (not boundary-blocked, genuinely converted)

1. **`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`** —
   `cad_solid_export_effect` (~line 506) and `CadPlayApp::export_media` (~line 1873) both matched
   `export.data` (type `CadSolidExport.data`, which is `protocol::DslValue` — confirmed at
   `…/🚪️io/🦀️.rs:313`) against `Value::String(..)` where `Value` was the `serde_json::Value` import.
   This is a genuine type mismatch (the match arms couldn't have compiled against a `DslValue`
   scrutinee), plus a `other.to_string()` fallback that also can't compile (`DslValue` has no
   `Display`). Fixed both to match `protocol::DslValue::String(text) => text` and use
   `protocol::json::to_json_string(&other)` for the non-string fallback (infallible, no
   `.unwrap_or_default()` needed). This looks like fallout from a concurrent, incomplete edit to
   `CadSolidExport.data`'s type (io.rs) that hadn't propagated to this file yet — fixed, not just
   worked around.
2. **`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`**
   — `parse_flow_camera_json` (~line 105) called `serde_json::from_value::<flow::CameraJson>(serde_json::Value::from(camera.clone()))`
   with an in-place comment claiming `CameraJson` "still derives an unconditional Serialize/
   Deserialize" as the reason to keep the serde_json bridge. That's stale: `CameraJson`
   (`🌊️flow/📄️artifact/🦀️.rs:177`) derives `ToValue, FromValue` **alongside** `Serialize,
   Deserialize` — confirmed by reading the current derive line. Replaced with
   `dsl::from_dsl_value::<flow::CameraJson>(camera.clone())`, no `serde_json` involved, and updated
   the comment to say why.

## Genuine framework-boundary sites — left as `serde_json`, confirmed not bridgeable from here

Every remaining production `serde_json::` reference in both crates (113 in procedural, 13 in cad,
final counts below) traces to one of these **framework-owned** signatures, all outside
`🧰️framework` which this ticket forbids editing:

- **`flow::playbook::FormGeneration.values: serde_json::Map<String, serde_json::Value>`**
  (`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️.rs:396-400`), plus sibling framework fns
  `is_block_visible`, `dsl_value_to_json`, `generation_operations` (`…playbook/🦀️.rs:536`) that all
  take/return that same shape. Drives: procedural2d/3d's `🧬️mutations/💾️binary/🦀️.rs` JSON-stack
  decoder subsystems (`*MountedJsonFrame`/`*MutationJsonFrame`, `assign_json`, `procedural3d_copy_json`,
  `procedural3d_observe_json`) — by far the largest chunk (16+16+39+6 = 77 of procedural's 113 sites
  — this exact subsystem was already identified and deliberately left in an earlier pass on this
  ticket, re-verified here, still correct); `ChangeGenerationValue.new_value`/`FormGenerationDsl`
  twins; every `*-generation` command file (`add`/`remove`/`rename`/`select`/`update-generation-
  values`, both 2d and 3d); `evaluate_generation_preview`/`generation_values_to_pack_object` in both
  artifacts' `🧬️schema/🦀️.rs`; `generation_form` in the procedural crate root `🦀️.rs`.
- **`semio_framework_plugin::MeshData`** — only derives `Serialize, Deserialize`, no `ToValue`/
  `FromValue` (confirmed at `🧰️framework/🔨️modules/🔺️mesh-engine/🦀️.rs:17-19`). Drives every
  mesh-decode site in both plugins' editor files (`mesh_data_to_dsl` helpers, `MeshData` `from_str`/
  `from_value` sites) — already correctly bridged via the framework's own `impl From<&DslValue> for
  serde_json::Value` (no new bridge code written, per this ticket's existing rule).
- **`MeshDwgDocumentImporter = fn(&MeshData) -> Result<serde_json::Value, String>`**
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`) — the literal return type a registered
  importer function pointer must match. Drives `cad_document_from_mesh` (`…/🚪️io/🦀️.rs:718`) and
  `procedural3d_document_from_mesh` (`…/✏️editor/🦀️.rs:1336`). Both already correctly documented
  in-place from an earlier pass.
- **`semio_framework_plugin::optional_json_to_dsl(Option<serde_json::Value>) -> Option<DslValue>`**
  (`🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs:803`) and the sibling `apply_world3d_sun_action`/
  `apply_world3d_projection_action`/`world3d_projection_action_moves_pose`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`) all take `Option<&serde_json::Value>`.
  Drives every `cad_action`/`cad_window_action`/`command_from_action`-style helper and every
  `procedural_action: impl Fn(&str, Option<serde_json::Value>) -> ActionDescriptor` window-measure
  closure parameter in both plugins. Several sites (cad's `🌞️sun/🦀️.rs`, `🎥️camera/🦀️.rs`) already
  build the payload as a `DslValue` and bridge once with `serde_json::Value::from(&dsl_value)` right
  at the call — this is the correct pattern per this ticket's stated non-trap boundary conversion;
  applied consistently by whichever session touched them last.

None of the above can be closed from inside these two plugins without editing `🧰️framework`
(forbidden by this ticket). A dedicated framework-side wave retyping `FormGeneration.values` (and
the sibling playbook fns) to a `DslValue`-native shape, plus adding `ToValue`/`FromValue` to
`MeshData` and widening the `optional_json_to_dsl`/`apply_world3d_*`/`MeshDwgDocumentImporter`
signatures, would let essentially all of the remaining 126 sites (113 + 13) drop `serde_json`
without any further plugin-side work — this matches the earlier session's same conclusion.

## ⚠️ Found but NOT touched: framework mid-flight and currently broken

`cargo check -p semio-s-plugin-procedural` (isolated target dir, see recipe below) failed with real
compiler errors in `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️.rs` — `serde_json::Value:
ToValue`/`FromValue` not satisfied (lines 587, 973, 230, 236) and `BlockKindPayload: FromValue` not
satisfied (line 996) — i.e. some concurrent session is mid-flight converting exactly the
`FormGeneration`/playbook boundary described above, and it is **currently red**. This is
`🧰️framework`, out of this ticket's write scope, and per this repo's rules I did not touch it or
wait on it — noting it here because it means `procedural`'s build could not be verified end-to-end
this session (see Verification below), and because whoever owns that wave should know it's broken
right now.

## Verification

- **cad**: `cargo check -p semio-s-plugin-cad` does not depend on `flow`/`playbook` at all (checked
  its `Cargo.toml` — no `flow` dependency), so it is unaffected by the framework breakage above.
  Kicked off in the isolated target dir; still running (cold isolated cache, large dep graph) when
  this session ended — **could not confirm a green build, only that it hadn't errored yet**. Do not
  treat cad as proven-green from this session.
- **procedural**: `cargo check -p semio-s-plugin-procedural` genuinely cannot pass right now because
  of the unrelated framework breakage above (confirmed by real compiler output, not assumed) — any
  attempt will show `playbook.rs` errors that are not this session's fault. Real before/after error
  counts for procedural specifically are therefore **not obtainable** until that framework wave
  lands or is reverted.
- Isolated-target recipe used (matches the brief):
  ```
  cd /Users/ueli/Documents/semio
  export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/isolated-target
  export RUSTC_WRAPPER=""
  cargo check -p semio-s-plugin-procedural --message-format short   # got real E0277s from playbook.rs, not from this session's edits
  cargo check -p semio-s-plugin-cad --message-format short          # still running at session end
  ```

## Final measured counts (brace-matched, non-test, non-comment)

- procedural: 113 production `serde_json::` references remain (all boundary-blocked per above; down
  from ~145 measured at the start of this session, mostly via concurrent peer convergence plus one
  real fix here).
- cad: 13 production `serde_json::` references remain (all boundary-blocked per above; stable across
  this session aside from the 2-site fix above, with several other sites converged by peers in the
  interim).

These are **not** the 204/47 figures quoted in this session's brief — that discrepancy is explained
by the brief's counter apparently not excluding `#[cfg(test)] mod tests { … }` tails (several files
in both crates have a short production head followed by a multi-thousand-line test module; a raw
`grep -c "serde_json::"` on those files way overcounts). The brace-matched count above is the one to
trust for "how much boundary-blocked serde_json is left."

## Files touched this session

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

No `Cargo.toml` edited. No test-domain files edited. No `#[cfg(test)]` code edited. No git commands
run.

## Update — cad's isolated `cargo check` finished, found + fixed a third real site

`cargo check -p semio-s-plugin-cad` (isolated target, warm cache after the first cold run) finally
completed: **6 errors**, three of them mine to fix, three unrelated:

- **Fixed** (3rd genuine site in `…/📐️cad/…/✏️editor/🦀️.rs`, same family as the two already fixed):
  line 1838, `geometry:in`'s media-import handler built `payload` as
  `Value::String(json.clone())` (`Value` = `serde_json::Value` import) then passed `&payload` into
  `import_cad_object_by_extension(name: &str, payload: &DslValue)` (`…/🚪️io/🦀️.rs:560`, our own
  function, no framework boundary here) — a real `E0308` mismatched-types error (`expected &DslValue,
  found &Value`), confirmed by the compiler. Fixed to `protocol::DslValue::String(json.clone())`.
  Re-swept the whole cad crate afterward for the same `Value::String(`/`Value::Object(` etc. pattern
  feeding a `DslValue`-typed parameter — none left; this file is back down to 1 site (the legitimate
  `use serde_json::{json, Value};` import for `cad_window_action`'s `optional_json_to_dsl` boundary).
- **NOT fixed, unrelated to serde_json, out of this task's scope** — two `E0046` "not all trait items
  implemented: missing `DESCRIPTORS`, `descriptor`" errors, in
  `…/✏️editor/🎚️config/🦀️.rs:320` (`impl Mutation<CadConfig> for CadConfigMutation`) and
  `…/✏️editor/👥️presence/🦀️.rs:103`. The `Mutation` trait
  (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:145`) requires `const DESCRIPTORS` and
  `fn descriptor(&self)` — nothing to do with `serde_json`/`DslValue` conversion, looks like a
  concurrent, unrelated in-flight change to the `Mutation` trait's required associated items that
  hasn't propagated to these two impls yet. Left alone per this ticket's scope (serde_json only) and
  the "ignore unrelated concurrent churn" rule — flagging here so whoever owns that wave sees it.

Did not re-run the full check again after this third fix (isolated target build takes several
minutes per attempt and kept exceeding the tool's foreground window); the 3 serde_json/DslValue
mismatches found by the one completed run are fixed, the 3 unrelated `Mutation` trait errors are
not, and cad has **not** been confirmed to build clean end-to-end this session.

## Update — re-run after the third fix: confirms the fix, surfaces more unrelated churn

Re-ran `cargo check -p semio-s-plugin-cad` (same isolated target, warm cache). Result: **5 errors,
none of them serde_json/DslValue** — the three mismatched-type errors from the previous run
(including the one fixed above) are gone from the error list, i.e. **the DslValue fix is confirmed
by the compiler, not just by reading**. What's left, all unrelated to this ticket's scope and not
touched:

- 3× `E0432` unresolved import `crate::artifacts::cad::mutations::{change_active_model_definition,
  create_node, rename_node}::mutation` — "could not find `mutation` in" each module. Looks like a
  concurrent session mid-restructure of those mutation modules (a `mutation` submodule that existed
  moments ago no longer resolves). Not present in the prior run's error list — confirms the file set
  is actively changing under this session, not stale output.
- 2× `E0046` (same `Mutation::DESCRIPTORS`/`descriptor` gap already noted above, unchanged).

Net effect: cad's serde_json/DslValue surface is now clean by direct compiler confirmation; the
crate as a whole still doesn't build, entirely for reasons outside this ticket (a different
concurrent mutation-module refactor plus the pre-existing `Mutation` trait gap). Stopping here per
this ticket's scope — not chasing unrelated, actively-moving files.
