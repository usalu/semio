# Hardening result: manifest, plugin instance, plugin host, hub bootstrap

Executor: Sonnet 5.5, foreground, no sub-agents, 2026-10-15. Scope: full-resolution differential (task A), `//#region` restoration (task B), `ChildRestoreProjection` rewrite (task C) for the splits in `RESULT-MANIFEST-SPLIT.md`, `RESULT-PLUGIN-INSTANCE-HOST-SPLIT.md` and `RESULT-HUB-BOOTSTRAP-SPLIT.md`. The same text is appended as a "Hardening" section to those three files.

## 1. Method

Oracle: `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib|bin --crate-name <crate> --emit=metadata -o /dev/null --error-format=json --cap-lints allow [--test] [--target wasm32-wasip2] [--cfg 'feature="x"'] <crate root>`. Only analysis-only rustc runs that write to `/dev/null`. No cargo, no scratch workspace, no private target dir, no `RUSTC`. (Empty-crate `.rmeta` stubs I built for an early iteration lived under the scratchpad and were deleted once `extern crate` stripping replaced them.)

Compare: error multiset keyed by (code, message, primary-span text, label, file normalised relative to the crate root with `x/../` collapsed). All files of the split family (parent plus children) fold into one file key, so errors that moved between files still compare. Script: scratchpad `hard/oracle.py`, `diff.py`, `diff2.py`.

Baseline: sandbox copy of the repo (directories on the path of every replaced file are real, everything else symlinks, so `#[path]` and `include!` resolve physically in the sandbox) with only the monolith replaced by its pre-split text:
- manifest: `git show HEAD:` (sha256 `34a16a80...`, identical to the executor's saved original) plus the peer's 3-line `space_index` mount, so only the split differs.
- plugin instance / host: `git show HEAD:` (byte-identical to `🗑️generated/plugin-instance-host/*-original.rs.txt`), instance with the peer's 7-line `ChildRestoreProjection` substitution applied (task C).
- hub: `hubsplit-tools/orig-bootstrap.rs.txt` (sha256 `746beedb...`, equals HEAD).
Losslessness of each baseline against the current split tree (normalised line multiset, `pub(...)` and `../` stripped): 0 missing lines in all four; extras are only mounts, docstrings, `use super::*`, impl re-wraps, the peer's space-index text and, for the hub, the rewritten `use` preamble.

Coordinator patches (applied identically to baseline and split copies, never to the repo; `neutral.py`): (1) remove `extern crate` lines (and the `#[cfg]` attribute lines above them) for crates that are missing, because rustc aborts on them; (2) replace every glob import from an external crate (`use ext::*;`, `pub use ext::a::*;`) with `const _: () = ();`, because a failed glob suppresses all unresolved-name errors in that scope and everything that glob-imports it. Effect per tree: manifest facade 0 externs / 0 globs (75 modules; patch is a no-op, results below are the unpatched run); plugin 5 externs / 26 globs in 20 of 448 modules (including `use semio_framework_ui_contract::*;` in the instance parent, which blinded all six instance children before); host 3 externs / 0 globs; hub 1 extern / 0 globs. Pitfall found on the way: stripping `extern crate` must also drop the attribute lines above it, otherwise a dangling `#[cfg(test)]` gates the next item (it showed up as 25 bogus `Bytes` errors in the hub baseline and proves the oracle sees a cfg-gated import used in non-test code).

Typeck limit: any unresolved trait path in a bound or impl (`impl ToValue for`, `T: Serialize`) makes rustc abort before typeck, so E0616 (field), E0624 (method) and E0451 (struct literal field) are invisible in all four crates (verified on a minimal file, and seeded: making `ActionArgDef::with_schema` private is not reported, making `struct ActionArgDef` private is). Those are covered by the definition scan in section 4.

## 2. Task A: differential results

Counts are total errors, baseline / split. "Non-extern" removes errors about unresolved external crates (`cannot find module or crate`, `unresolved import <extern>`), which repeat once per child by construction.

| Crate | Config | Before fixes (split vs baseline) | Final |
|---|---|---|---|
| manifest (`semio_framework`) | native | 1790 / 1790 | 1790 / 1790, 0 differing keys |
| | `--test` | 2691 / 2691 | 2691 / 2691 |
| | wasm32-wasip2 | 1790 / 1790 | 1790 / 1790 |
| | `--test` wasm32-wasip2 | 2691 / 2691 | 2691 / 2691 |
| plugin instance (`semio_framework_plugin`, patched) | native | 9236 / 9248: 12 new E0433 | 9236 / 9236 (non-extern 1333 / 1333) |
| | `--test` | not reached (killed, superseded) | 20822 / 20822 (2436 / 2436) |
| | wasm32-wasip2 | | 9527 / 9527 (1405 / 1405) |
| | `--test` wasm32-wasip2 | | 21107 / 21107 (2507 / 2507) |
| | features (all four) native | | 10084 / 10084 (1382 / 1382) |
| | features + `--test` | | 20845 / 20845 (2436 / 2436) |
| plugin host (`semio_framework_plugin_host`, patched) | native | | 1334 / 1334 (non-extern 56 / 56) |
| | `--test` | | 2796 / 2796 (68 / 68) |
| hub bin (`os_hub`) | native | 4 / 4 non-extern | 4 / 4 (raw 768 / 1045 extern repetition) |
| | `--test` | 16 / 24: 8 new | 10 / 10 |
| | `integration-fixtures` | 4 / 4 | 4 / 4 |
| | `--test` + `integration-fixtures` | 22 / 30: 8 new | 11 / 11 |

The unpatched plugin run (native) printed 1361 / 1361 "identical" with 675 distinct keys; the patched run exposes 9236 errors with 1369 keys, i.e. the earlier equality was blind.

Hub and plugin host were not compiled for wasm (native only, as briefed).

### 2.1 Regressions found and widenings

Plugin instance (12 errors, 3 distinct: `cannot find child_emit_preparation / window_mutation / window_transient in super`): the code moved one module level deeper, so `super::X` pointed at the instance parent instead of the plugin root. Fix (path depth, as with the `../` prefixes on `#[path]`): `super::` to `super::super::` for those three module names in `🧵️instance/🧾️typed-operations/🦀️.rs` at lines 657, 659, 660, 891, 897, 1063, 1069, 1241, 1247 (12 occurrences).

Hub (test configs; 8 errors, 5 distinct):
- `🔑️access-grants/🦀️.rs:30` added `#[cfg(test)] use crate::presence::{db_artifact_id, document_scope_key_v1};` (the `include!`d `🧪️tests/🔬️standalone` fixtures call them; 3 E0425).
- `🧭️open-plan/🦀️.rs:87` `static DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVING` inside the `thread_local!` to `pub(crate)` (2 E0425 in `🔬️bin-unit`).
- `🛑️lifecycle/🦀️.rs:230` `const HUB_SHUTDOWN_CLOSE_CODE` to `pub(crate)` (1 E0425 in `🤝️two-client-document`, path `super::super::`).
- `🦀️.rs:115` added `#[cfg(test)] use std::collections::BTreeMap;` (2 E0433 in `🔬️bin-unit`; the original preamble imported it).
- From the definition scan (section 4): `🧠️state/🦀️.rs:298` `HubState.admin_dir` to `pub(crate)` (read in `🧩️assets`, in the root's `HubState { .. }` literal and in `🔬️bin-unit`; E0616/E0451, invisible to the oracle).

Manifest and plugin host: no regression, no edit.

## 3. Task B: hub region markers

The original has 36 marker lines (18 regions; `Rest` contains `ExecutionTargetLease`, `Blobs`, `CheckpointPublication`, `CheckIn`). Placement is item-accurate: every original top-level item (685 via the splitter's own lexer, `hubregions.py`) was matched to its exact new location (all non-`use` items matched exactly once; the `include!` item matched by hand in `🔑️access-grants`), each item got its region path from the original, and markers were generated by walking each child's items in order and opening/closing regions where the path changes (so a region spanning several children gets matching markers in each, nesting is preserved, an interleaved region reopens). Marker text is the original text, `//#region` directly before the first line of the first item (including its doc comments), `//#endregion` directly after the last line.

Evidence (`hubverify.py`): tree has 80 marker lines = 40 balanced pairs in 23 files (root + 22 children; `🎭️actors` and `🧾️provenance` are TS-only). Per region the multiset of enclosed non-blank lines (after `pub(crate)` stripping) is identical to the original for all 18 regions, with one expected exception (`include!("../🧪️tests/...")` path depth, `Rest`):

| Region | original pairs | tree pairs | enclosed lines (orig = tree) |
|---|---|---|---|
| Errors | 1 | 1 | 78 |
| State | 1 | 7 | 1706 |
| Auth | 1 | 1 | 83 |
| AdminAuth | 1 | 3 | 613 |
| Rest | 1 | 3 | 1502 |
| ExecutionTargetLease | 1 | 1 | 205 |
| Blobs | 1 | 1 | 60 |
| CheckpointPublication | 1 | 1 | 161 |
| CheckIn | 1 | 1 | 436 |
| WebSocket | 1 | 1 | 983 |
| Directory | 1 | 8 | 3259 |
| Admin | 1 | 2 | 1159 |
| Extensions | 1 | 1 | 57 |
| PluginModules | 1 | 1 | 29 |
| AdminPage | 1 | 2 | 153 |
| Inference | 1 | 2 | 170 |
| Main | 1 | 3 | 738 |
| Tests | 1 | 1 | 14 |

The parent keeps `Main` (4 items) and `Tests` (3 mounts and the two-client test).

Gated root imports: the root preamble has the `#[cfg(test)]` gated imports the splitter created. A non-test build resolves everything: the native and `integration-fixtures` (non-test) differentials show 0 name differences against the original (4 / 4 non-extern, identical keys), and the oracle demonstrably flags a gated import used by non-test code (the `Bytes` artifact above, 25 errors). The one import the tests needed and the root lacked (`BTreeMap`) was a test-only gap, fixed. rustfmt parses all 23 hub files (root + 22 children) and the edited instance child.

## 4. Definition-level scan (fields, methods, items, traits)

Tool: scratchpad `hard/scan.py`, `scan2.py`, `scan3.py` (token lexer, module tree following `#[path]` and `include!`, visibility scopes `private`, `pub(super)`, `pub(crate)`, `pub(in ..)`). For every private or narrowed struct field, inherent method/associated const, trait, and top-level item defined in a family file, it reports uses in files outside the visibility scope: `.name`, `.name(`, `Type::name`, `Self::name`, struct literal fields, trait-method calls, bare item identifiers not declared locally. Sensitivity checked by seeding: stripping `pub(super)` from all 17 widened methods in instance `📨️dispatch` reproduces all 17; privatising `with_schema`, `struct ActionArgDef`, `struct HubState` is each reported.

| Tree | Candidates | Real | Notes |
|---|---|---|---|
| manifest | 36 | 0 | all name collisions on private types (`InputVariant`, `ResolvedInput`, `InputSchemaReader`) and `.first`, `.0`, `new`; receiver refinement: 0 type-matching uses |
| plugin instance | 0 | 0 | |
| plugin host | 433 field groups, 10 `Self::new` | 0 | collisions only; the only family-typed access (`ActorHostState` fields in `📥️imports`) is `AsyncActorHostState`, a different struct |
| hub | 142 defs, 1154 hits | 1 | `HubState.admin_dir` (fixed). The others are extern fields (`.code`, `.refusal`, `.envelopes`, ...) or locals; receiver check left 328 hits with unprovable receivers in 22 groups, each read: all foreign types |
| top-level item scan (the "free struct stays private" blind spot) | 0 in all four | | |
| trait method scope | 0 in all four; hub: no module calls `PayloadStorage`, `SessionStore`, `StreamExt`, `SinkExt`, `IntoResponse` methods without importing the trait | | |

## 5. Task C: `store::ChildRestoreProjection` rewrite

Conclusion: a peer's deliberate concurrent change, not a split artifact. Evidence: `semio-framework-composition-admission` (`🧰️framework/🔨️modules/🧬️schema/🧩️composition/📥️admission`) was created 02:51 to 02:56 (`project.json` 02:53, `script.ts` 02:56, crate files re-touched up to 03:34), untracked; it defines `ChildRestoreProjection`, `ChildRestoreProjectionError` and tests; `🔌️plugin/📦️packages/🦀️rust/Cargo.toml` gains the dependency (uncommitted, +1 line, mtime 02:53); the `store` crate's own definition was removed (`🏪️store/**` diff: 7 insertions, 136 deletions; `store` is only an alias of `extern crate semio_framework_os_kernel`, so the old path no longer exists); non-split files were switched the same way (`📜️declaration/🧩️app`, `📜️declaration/🪟️surface`, `🪆️child`, `🧪️testing`, `🧪️tests/...`). The ingress and dispatch children carry mtime 02:53, after the 02:50 split. Left untouched. The baseline got the same 7-line substitution so the differential isolates the split. After the substitution baseline and tree have an identical line multiset except structure (0 missing).

## 6. Residual risks

- Typeck-phase privacy (E0616, E0624, E0451), trait-in-scope for method calls and type/trait coherence stay unproven by rustc; the evidence is the name- and receiver-based scan (it found one real case, `admin_dir`, which the oracle could not). Receiver-unprovable hits (hub 22 groups, host 59 groups) were checked by reading, not by a compiler. Run the real `check` first thing when the compiler-ingress gate admits the crates.
- All externals are unresolved, so everything that hinges on an external item's shape (extern trait methods, derive output, macro expansion from external proc macros) is out of reach. Globs from `super`/local modules were left intact and are exercised.
- Hub lib/bin: the bin is a separate crate from `semio_hub`; widenings are `pub(crate)` inside the bin only and no lib item was touched. Paths into `semio_hub::...` are all unresolved in the oracle.
- `pub(super)` widenings in a child are visible to the parent subtree only. Consumers outside it must use items the child makes `pub`/`pub(crate)`; module-path uses from other crates modules were covered by resolution, field/method uses by the scan (parent subtree scan, which is where the monolith's privacy boundary was).
- Peers keep editing the same trees; results hold for the tree state at ~04:50.
