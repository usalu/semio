# 1028 result: policyfix

STATUS (22:55 checkpoint): edits complete and compile-atomic, still UNVERIFIED. A fresh run of the verifying script is queued/running detached (it started after the store layer went green and waited >2800 s on the cargo capacity queue). No new edits or checks started.
NEXT STEP: wait for `C/🗑️generated/1028/policyfix-all.rc` (marker per check: policyfix-{energy,fw,stdio,hub}.rc, logs policyfix-*.txt); if absent, `cd /Users/ueli/Documents/semio && nohup C/1028-tools/policyfix/run.sh &`. Then grep the logs for errors in files named in the Fixed table (📥️pack, 🏭️factory, 🧺️sources, 🧾️services, checkpoint-order, 📊️outputs, ✂️deletion/🧱️surface, energy mut-* and ifc composition) and mark rows verified. Bounds errors belong to puzzleplugin.

Previous status: PARTIAL, edits applied but NOT compiler-verified. Every native check (`C/🗑️generated/1028/policyfix-{energy,fw,stdio,hub}.txt`, script `C/1028-tools/policyfix/run.sh`) stopped on foreign upstream errors before reaching the touched crates:
- fw / energy: store history command/replay/metadata trait-bound errors (`P: Clone`, `M: ColdMutation<P>`) in `🏪️store/🕰️history/🎮️command` and `🏁️replay` (peer frontier); store-document `P: Send/Sync` errors (hub).
- stdio: source-boundary gate refused `semio-s-artifact-stdio-ifc-2x3-base` (files limit), a pre-existing over-limit crate, not touched here.
Re-run `1028-tools/policyfix/run.sh` once those crates compile; it covers every touched crate.

## Fixed
| Cat | File | Change |
|---|---|---|
| 2 | `🏪️store/🦀️.rs` | deleted alias `original_history_transitions` (zero consumers) and alias `original_history_branch`; its only consumer `⚙️engine/🌿️branch/commit-tests.rs` now uses `crate::` (file is not referenced by any `mod`) |
| 3 | 11 energy `🧬️mutations/*/🦀️.rs` wiring modules (window-layer-optics visible, air-loop nodes, plant-loop design, people-gain fractions, lighting-gain emission, ideal-loads limits, year-schedule holidays, site ground, model simulation, surface properties, fenestration geometry) | removed `#![allow(async_fn_in_trait)]` and `#![allow(clippy::too_many_arguments)]` (22 allows); files are pure `#[path]` wiring, so any warning now surfaces from components and must be fixed there |
| 3 | `🏗️ifc/🧩️composition/🦀️.rs`, hub `📇️catalog/🏗️projection` and `📜️definition` | removed `async_fn_in_trait` and `long_running_const_eval` allows (6) |
| 4 | `🏗️ifc/🧩️composition/🦀️.rs` | deleted the caller-less `io_registry::register()` and its 7 `expect`s (registration is declarative via `.composers(...)`); dropped the unused import |
| 4 | plugin `📥️pack` (8), `🏭️factory` (8), `🧺️sources` (2), `🧾️services` (1); store `checkpoint-order.rs` (2); energy `📊️outputs` (12), `✂️deletion/🧱️surface` (9) | `.as_mut()/.as_ref()/.take().unwrap()` replaced by `.ok_or_else(invalid)?` using each file's existing `invalid` helper (the crate's `ValueError`). `advance_demands` in checkpoint-order was concurrently changed by a peer to return `Result` |

## Left alone (with diagnosis)
| Cat | Item | Why |
|---|---|---|
| 2 | `os-store` `original_replay_publication` alias (store `🦀️.rs:45`) | about 25 consumers across store tests and the plugin crate (including `crate::original_replay_publication` aliases inside plugin); repointing needs new Cargo deps per crate in a churning area |
| 2 | `🔁️replay/🦀️.rs:8` `pub use ... as replay_preparation` | consumed by plugin native publication and store lifted modules through the alias; same dependency ripple |
| 2 | `wp2-declaration/🦀️.rs:23` `as staged_form` | public `app::staged_form` facade path, 16 external stdio/plugin consumer files; repointing needs a new dep on the decl crate in each |
| 2 | the 8 plain `Original*` name re-exports (`OriginalVcsApp`, `OriginalCheckpointEditOrder`, `OriginalToolOverlayFold`, ...) | domain type names re-exported by name, not aliases; not shims |
| 4 | energy `✨️derive/*` (`delete/📆️annual`, `🧱️surface`, `🪟️fenestration`, `🌱️birth`, `delete/🦀️.rs:27`) | `expect`s are inside `quote!` generated code in trait methods that return a bare `Row`/`Self` (`finish`, `fenestration`), invariants already enforced by `validate_*`; propagating needs trait signature changes |
| 4 | energy `🫧️network/↩️inverse` (2), `deletion/🧱️surface` 2 projection `expect`s | inside `RetainedCloneRef::project` closures that must return a reference |
| 4 | value `✨️derive/🛬️receive:58`, `🛬️decode/🧳️owned/🌳️value:129,136`, `🗂️map:20` | same: closures in `project(...)`, shared value crate |
| 4 | plugin `📦️original` delegate `expect`s (Deref), `media-transfer:127`, store `checkout.rs:48` | `Deref` cannot return `Result`; the other two were not converted (no local error helper; not compile-verifiable now) |
| 4 | `staged-source-tests.rs` | test code (scanner false positive) |

## SHARED CHANGE
none (store alias removal touched `🏪️store/🦀️.rs` only).
