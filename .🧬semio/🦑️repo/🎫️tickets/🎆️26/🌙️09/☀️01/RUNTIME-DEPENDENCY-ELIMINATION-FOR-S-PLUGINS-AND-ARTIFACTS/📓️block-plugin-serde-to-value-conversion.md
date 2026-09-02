# 🧱️ block plugin: serde → first-party value conversion

## Scope
`✏️s/🔌️plugins/🧱️block` (crate `semio-s-plugin-block`), the only slice I own this wave.

## What was converted
- **138 production files** with `#[derive(… Serialize, Deserialize …)]` → `#[derive(… dsl::ToValue, dsl::FromValue …)]`,
  and every `#[serde(…)]` → `#[value(…)]` (bodies unchanged: `rename_all`, `default`,
  `skip_serializing_if`, `tag`, `rename` only — no `flatten`/`with`/`skip` present in this crate).
  Existing first-party derives (`dsl::DslRecord`, `dsl::DslEnum`, `dsl::DslOps`, `dsl::DslArtifact`,
  `ArtifactSchema`) left exactly as-is.
- Every converted type ALSO keeps `#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]`
  + `#[cfg_attr(test, serde(…))]` twins, because block2d/3d/5d's ~100 per-mutation fixture tests
  under `🧪️tests/<scenario>/🦀️.rs` round-trip `Block*Snapshot`/`Block*Mutation`/`Block*Diff` through
  `serde_json` as a differential oracle against committed JSON fixtures (`committed_json_is_canonical`,
  `produces_committed_diff`, etc.) — exactly the sanctioned exception in the brief. Confirmed by
  `.🧬semio/…/📓️corrected-scope-plugin-serde.md`: "retaining `#[cfg_attr(test, …)]` wherever a
  `serde_json` differential oracle test reads that type." Did not touch anything under `🧪️tests/`.
- `dsl::ToValue`/`dsl::FromValue` resolve via block's existing
  `extern crate semio_framework_os_kernel as dsl;` alias (`📦️packages/🦀️rust/🦀️.rs`) — `dsl` (=
  `protocol`=`store`=`vcs`) re-exports both the traits and the derive macros at its crate root.
  **No Cargo.toml change was needed or made** (no `semio-framework-value-derive` dependency added).
  `serde`/`serde_json` lines in Cargo.toml were left untouched (still load-bearing: `#[cfg_attr(test,
  …)]`, and a handful of production sites below).

## Production call sites also converted (not just derives/attrs)
Removing `Serialize`/`Deserialize` broke 9 real production call sites that depended on it
(everything else using `serde_json` in this crate — UI action-payload `json!{}` literals, framework
`UiNode`/`WindowLayout` rendering, a foreign stdio type's own field — is unaffected and was left
alone; verified file-by-file):
- `🗿️artifacts/{◻2d,🧊️3d,🖐️5d}/…/🚪️io/📤️export/…/🦀️.rs` (3 files): `serde_json::to_value` →
  `dsl::ToValue::to_value` + `dsl::json::from_dsl_value` bridge into stdio's `JsonSnapshot`, matching
  the exact proven pattern in `🔱️trinity`'s equivalent leaf (`pack::json_from_dsl_value`, same
  function via a different re-export path). `serialize_bytes` now uses stdio's own `write_json_text`.
- `🗿️artifacts/{◻2d,🧊️3d,🖐️5d}/…/🚪️io/📥️import/…/🦀️.rs` (3 files): `serde_json::from_value` →
  `dsl::FromValue::from_value`; `serde_json::from_str` → stdio's own `parse_json_text`.
- `🗿️artifacts/{◻2d,🧊️3d,🖐️5d}/…/✏️editor/🎮️commands/🎨️edit/🦀️.rs` (3 files): the whole-document
  text-edit handler's `serde_json::from_str::<Block*Snapshot>` → `dsl::json::from_json_str`.
- `🗿️artifacts/◻2d/…/🧬️schema/🧬️mutations/🦀️.rs`: `block2d_mutation_report_json` (a non-test, always
  compiled `pub fn` "TestBridge" used by the `mutate-block-2d-1` integration test) rewritten off
  `serde_json::{from_str,to_value,json!}` onto `dsl::json::from_json_str` / `dsl::DslValue::object` /
  `dsl::ToValue::to_value` / `dsl::json::to_json_string`, mirroring the identical bridge already
  shipped in `🏗️fem`/`🌍️gis` gismap for this same ticket.

All of the above patterns were cross-checked against already-converted sibling plugins (`🏗️fem`,
`🔱️trinity`, `📸️remodel`, `🎪️demonstrator`) before writing, not invented from scratch.

## Verify
`cd /Users/ueli/Documents/semio && cargo check -p semio-s-plugin-block`

- **Before**: crate fails with **~5,843 errors** (confirmed cause per the assignment: mutation
  types still `Serialize`/`Deserialize` while `protocol::MutationKind` requires `ToValue + FromValue`).
- **After**: NOT YET CONFIRMED BY A REAL COMPILER RUN — do not treat this as a pass. Two
  `cargo check -p semio-s-plugin-block` invocations were attempted this session; both stalled
  (first was killed by the harness after ~50min wall-clock, its rustc children were orphaned but
  kept running standalone for over an hour; a second invocation launched afterward sat blocked on
  the build lock for 10+ min with ~0s CPU). System state at last check: load average 34-48
  (climbing, started ~22-27), 18 concurrent `rustc --crate-name` processes, a single local
  `sccache` server serializing nearly all of it. This is confirmed host-wide contention (many
  concurrent agent sessions), not a defect in this crate — but it means the mandatory verify step
  is still outstanding. NEXT AGENT/SESSION: re-run `cargo check -p semio-s-plugin-block` (likely
  fast now — most of the dependency graph and much of this crate itself were already compiled by
  the orphaned run) and fill in the real error count here. Do not report success without doing so.
  All 138 production files plus the 10 call-site rewrites were independently verified BY HAND (see
  below) — cross-checked line-for-line against already-converted sibling plugins (`🏗️fem`,
  `🔱️trinity`, `📸️remodel`, `🎪️demonstrator`) for every non-trivial signature
  (`dsl::json::from_dsl_value`, `dsl::json::{from_json_str,to_json_string}`, `dsl::DslValue::object`,
  stdio's `write_json_text`/`parse_json_text`/`JsonSnapshot::from_value`/`to_serde_value`), and
  programmatically for balanced derive/attr syntax and paired `cfg_attr(test, …)` twins across all
  138 files (0 issues found). This gives high confidence but is NOT a substitute for the compiler.

## Update — three attempts, all blocked by build-lock contention, not by this crate
Three separate `cargo check -p semio-s-plugin-block` attempts this session, none produced real
compiler output:
1. First run compiled for ~50min wall-clock, was killed by the harness; its rustc children were
   orphaned and kept running standalone for over an hour before finally disappearing (never
   produced output either — likely also killed once orphaned/stalled).
2. Second run: `Blocking waiting for file lock on build directory`, killed after being stuck.
3. Third run: same `Blocking waiting for file lock on build directory`, still stuck after 13+ min
   at last check. Root cause identified via `ps`: at least two OTHER sessions are each running a
   full `cargo check --quiet --workspace --message-format=json --manifest-path
   .../Cargo.toml --keep-going --compile-time-deps --all-targets -Zunstable-options` (both PIDs
   started simultaneously at 9:40AM, both sitting at ~0s CPU with no growth over 13+ minutes —
   plausibly deadlocked against EACH OTHER's lock, not merely slow). Every single-package `cargo
   check -p X` from any session — mine included — queues behind whichever of these holds the lock.
   This matches this ticket's own `📓️central-verification.md`, which independently found the
   workspace build lock saturated (~77 concurrent rustc at one point) and worked around it for two
   crates by verifying in a standalone crate OUTSIDE the workspace — not available to me here
   without either a forbidden custom `CARGO_TARGET_DIR`/worktree, or copying the whole dependency
   graph, neither of which this assignment permits.

**I have not obtained a real `cargo check -p semio-s-plugin-block` exit code or error list this
session.** Do not treat the absence of errors above as a pass — it is an absence of data. The code
change itself is complete and was independently re-verified by hand multiple times (paired
`#[value(…)]`/`#[cfg_attr(test, serde(…))]` attributes, balanced derive syntax, every non-trivial
bridge call cross-checked against 4 already-converted sibling plugins' actual source). Next
session: check `ps aux | grep 'cargo check --quiet --workspace'` first — if it's still there and
not progressing, that is the blocker to resolve/wait out, not this crate's code.

## Files touched
138 production `.rs` files under `✏️s/🔌️plugins/🧱️block` (derive/attr conversion), plus the 9
production call-site rewrites and 1 TestBridge rewrite listed above (all included in the same 138,
except the TestBridge function body which lives in the block2d mutations aggregate file already in
that set). No files under `🧪️tests/`, `🔬️probes/`, `🏭️generator/`, `🧫️fixtures/` were touched.
Cargo.toml untouched.
