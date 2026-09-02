# procedural + cad: serde_json::Value production elimination (2026-09-02)

Scope: `✏️s/🔌️plugins/🌀️procedural` and `✏️s/🔌️plugins/📐️cad`. Goal: replace production
`serde_json::Value`/`Map`/`json!` usage (left behind by an earlier derive-conversion pass that
bridged through `serde_json::Value` rather than eliminating it) with the first-party `DslValue`,
using `pack::json` (reachable as `dsl::json::…`/`protocol::json::…` through the os-kernel crate
alias) for JSON *text* encode/decode. Test-domain dirs (`🧪️oracle/`, `🧪️test/`, `🔬️probes/`,
`🏭️generator/`, `🧫️fixtures/`) and `#[cfg(test)]`/`mod tests` code were left untouched everywhere
— serde_json is a deliberate differential oracle there.

⚠️ **Verification was NOT attempted, by instruction.** The coordinator for this ticket explicitly
forbade running any `cargo` command in this session (workspace has 40-60 concurrent
rustc/peer-session processes; a foreground build reliably exceeds the tool timeout and gets killed
at turn end). Every edit below was made by static reading only — cross-checking field/function
signatures across files, but never compiled. Treat this as a strong first pass, not a green build.

## Key discoveries that shaped the whole pass

1. **The previous pass's JSON-text bridge is itself incomplete.** It routed text encode/decode
   through `serde_json::to_string(&serde_json::Value::from(protocol::ToValue::to_value(x)))` /
   `<T as FromValue>::from_value(DslValue::from(serde_json_parsed_value))` — this still calls
   `serde_json::to_string`/`from_str`, so serde_json stays linked into the wasm component even
   though the type flowing through it is DslValue-shaped. The real fix is `pack::json`'s own
   `to_json_string<T: ToValue>`/`from_json_str<T: FromValue>` (defined in
   `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs`, re-exported at the os-kernel crate root as
   `json`, reachable in both plugins via their existing `extern crate semio_framework_os_kernel as
   protocol/dsl/…` aliases as `protocol::json::to_json_string`/`dsl::json::from_json_str`) — these
   never touch serde_json at all. Every agent on this pass was told to replace the old bridge
   shape with a direct `pack::json` call wherever found.
2. **`dsl::to_dsl_value`/`dsl::from_dsl_value` (free functions) ARE reachable** — verified via
   `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs:18,26`
   (`pub use crate::os_dsl::schema::*;` + explicit `{from_dsl_value, to_dsl_value}`), globbed all
   the way to the os-kernel crate root. They require `T: ToValue`/`T: FromValue`. Several call
   sites in procedural2d/3d pass a literal `serde_json::Value` to `dsl::to_dsl_value` — **this does
   not compile**, since `serde_json::Value` has no `ToValue` impl. These are genuine pre-existing
   compile errors (not introduced by us), found and fixed by routing through the DslValue `From`
   impls instead (see below).
3. **The `DslValue <-> serde_json::Value` bridge already exists, framework-provided, infallible**:
   `impl From<&DslValue> for serde_json::Value` / `impl From<&serde_json::Value> for DslValue` (and
   owned variants) in `🧰️framework/🔨️modules/🌱️value/🦀️.rs:218-272`. No new bridge code was
   written anywhere in this pass — every remaining serde_json boundary conversion just calls
   `DslValue::from(x)` / `serde_json::Value::from(x)`.
4. **A genuine, framework-owned boundary exists and cannot be fully eliminated from inside these
   two plugins**: `flow::playbook::generation_forms::FormGeneration` (defined in
   `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️.rs:396-400`) has
   `pub values: serde_json::Map<String, serde_json::Value>`. `is_block_visible`
   (`📖️playbook/🦀️.rs:254`) also takes `&serde_json::Map<String, serde_json::Value>` directly, and
   `dsl_value_to_json` (`📖️playbook/🦀️.rs:229`) returns `serde_json::Value`. Any procedural2d/3d
   code that constructs/reads a `FormGeneration` (directly, or via its local `FormGenerationDsl`/
   `ChangeGenerationValue` twins) is structurally required to still produce/consume that exact
   shape. Per this ticket's rules we did not touch framework code, so this boundary remains —
   flagged per-file below. **A follow-up wave should retype `FormGeneration.values` in framework
   to `Vec<(String, DslValue)>` (or reuse `DslValue::Object` directly) to fully close this out.**

## Work distribution

This was split across parallel sub-agents (each briefed with the same conversion guide) plus two
groups done directly in this session. Status as of this writing:

- **cad `✏️editor/🦀️.rs`** (~3767 lines, the single largest file in either plugin) — delegated,
  in progress/pending report.
- **cad, all other files** (~26 files: `🎬️interaction-spec`, `🚪️io/*`, `🧬️schema/*`,
  `✏️editor/*` minus the main file, `🧩️extensions/*`) — delegated, in progress/pending report.
- **procedural3d `🧬️mutations/💾️binary/🦀️.rs`** (~3737 lines) — delegated, **completed**. Two
  broken `dsl::to_dsl_value`/`dsl::from_dsl_value` misuse sites fixed (lines 140, 161) via the
  `DslValue::from`/`serde_json::Value::from` bridge. The entire generation-values JSON-stack
  subsystem (`Procedural3dMutationJsonFrame`/`assign_json`/`json_stack`, `procedural3d_copy_json`,
  `procedural3d_copy_generation`, `Procedural3dReplayDisplaced::Json`) was left as `serde_json`
  (option b from the guide) — it exists solely to feed `ChangeGenerationValue.new_value`/
  `FormGeneration.values`, both boundary-constrained. See the agent's full site list in this
  ticket's transcript; not reproduced in full here for space, ask to re-run that agent's report if
  needed.
- **procedural2d `🧬️mutations/💾️binary/🦀️.rs`** (~3525 lines) — delegated, in progress/pending
  report (same boundary shape expected as its procedural3d sibling above).
- **procedural3d, all other files** (~29 files under the artifact, minus the mutations/binary
  file) — delegated, in progress/pending report.
- **procedural2d, all other files** (~19 files under the artifact, minus the mutations/binary
  file) — delegated, in progress/pending report.
- **procedural `🧩️assembly` artifact** (7 files) — done directly in this session, see below.
- **procedural root `✏️s/🔌️plugins/🌀️procedural/🦀️.rs`** — checked directly, see below (no edit
  needed).

## Completed directly in this session

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`
(test module starts at line 796 — everything at/after that line was left untouched)
- `encode_one` (streaming JSON writer for the WFC assignments page): `serde_json::to_string(&slot)`
  / `&module` (both plain `String`, `String: ToValue` confirmed at
  `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:147`) → `protocol::json::to_json_string(&slot)` /
  `&module`. Dropped the now-pointless `.map_err(|error| error.to_string())?` (the new call is
  infallible).
- `create_job_from_wire`: `serde_json::from_slice::<AssemblyInferenceRequest>(payload)` →
  `std::str::from_utf8(payload)` then `protocol::json::from_json_str::<AssemblyInferenceRequest>(…)`
  (both `AssemblyInferenceRequest`/`AssemblyInferenceCommit` already derive `ToValue, FromValue`).
- `solve_with_job`'s terminal-outcome match arm: same pattern, decoding
  `AssemblyInferenceCommit` from `StepOutcome::Complete`'s payload bytes.
- `AssemblySolve::dep_input` / `AssemblyContradiction::dep_input` (×2, identical bodies): the old
  bridge `serde_json::to_vec(&serde_json::Value::from(protocol::ToValue::to_value(snapshot)))` →
  `protocol::json::to_json_string(snapshot).into_bytes()`.
- Verified zero remaining production `serde_json` references (everything left is at/after line
  796, inside `mod tests`).

### `.../🧩️assembly/.../🧩️wfc-engine/🧵️job/🦀️.rs` (~2243 lines; function-level `#[cfg(test)]` at
lines 619/629/634/639, the real `mod tests` module starts at 1866 — production code runs from 1
through 1865 minus those four small test-only helpers)
- Two `emit_preview`/restore-preview-step sites (lines ~1183, ~1613): old bridge
  `serde_json::to_vec(&serde_json::Value::from(protocol::ToValue::to_value(&preview)))` →
  `protocol::json::to_json_string(&preview).into_bytes()`.
- Confirmed the two `serde_json::from_slice::<WfcPreview>` reads (lines 1994, 2064) are inside
  `mod tests` (>1866) — left untouched.

### `.../🧩️assembly/.../🧩️wfc-engine/🔍️search/🦀️.rs` (test module starts at line 718)
- One production site (line ~521): `serde_json::from_slice::<crate::wfc_engine::job::WfcCommit>(…)`
  (`WfcCommit` already derives `ToValue, FromValue`) → `std::str::from_utf8(&candidate.output)`
  then `protocol::json::from_json_str::<crate::wfc_engine::job::WfcCommit>(…)`.

### `.../🧩️assembly/.../🧩️wfc-engine/{💾️serial,🎛️bitset,🆔️ids}/🦀️.rs` and
### `.../🧩️assembly/🧬️schema/📸️snapshot/🦀️.rs`
No production `serde_json` usage — every match in these four files is inside `mod tests` (test
module starts at line 181/220/79/125 respectively; all matches are at higher line numbers), or (in
`💾️serial/🦀️.rs`) a single doc-comment mention (line 8, prose only, not code). No edits made.

### `✏️s/🔌️plugins/🌀️procedural/🦀️.rs` (crate root, 369 lines) — checked, no edit
Two production sites, both genuinely boundary-constrained, **left as-is**:
- `generation_form(spec, values: &serde_json::Map<String, serde_json::Value>, …)` (line 165) — the
  parameter is passed straight into `flow::playbook::is_block_visible(question, values)` (framework
  fn, signature `fn is_block_visible(block: &PlaybookBlock, values: &serde_json::Map<String,
  serde_json::Value>) -> bool`, confirmed at `📖️playbook/🦀️.rs:254`) and reads via
  `flow::playbook::dsl_value_to_json(…)` (framework fn returning `serde_json::Value`, confirmed at
  `📖️playbook/🦀️.rs:229`). Cannot be retyped without changing those framework signatures.
- Line 211: `serde_json::json!(field.value.unwrap_or(0.0))` builds one element of a
  `Vec<serde_json::Value>` fallback inside the same boundary-constrained function — same reasoning,
  left as-is (the surrounding array is itself already `serde_json::Value`-typed by construction).

## Uncertain / needs follow-up (compiled so far)

- **`FormGeneration.values: serde_json::Map<String, serde_json::Value>`** (framework,
  `📖️playbook/🦀️.rs:399`) is the root cause of every remaining boundary-constrained site across
  both plugins (procedural2d/3d's `🧬️mutations/💾️binary/🦀️.rs` JSON-stack subsystems, the
  `ChangeGenerationValue.value`/`.new_value` payload fields, `generation_form` above, and likely
  more in the still-pending delegated files). A dedicated follow-up wave to retype this framework
  field (and `is_block_visible`/`dsl_value_to_json`'s signatures) to a `DslValue`-native shape
  would let the remaining ~2 hand-rolled JSON-stack parsers per artifact fully drop serde_json.
- Sub-agent reports for cad (`✏️editor/🦀️.rs` + the rest of cad), procedural2d's
  `🧬️mutations/💾️binary/🦀️.rs`, and the "rest of procedural2d"/"rest of procedural3d" groups were
  still in flight when this file was first written — this document should be re-read and appended
  to (or superseded) once those land; do not treat the "Work distribution" section's "in
  progress/pending" entries as done.
