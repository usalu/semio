# Packet: puzzle + trinity `⚙️engine` elimination

Targets: `✏️s/🔌️plugins/🧩️puzzle` (3 engine dirs, ~9,553 LOC), `✏️s/🔌️plugins/🔱️trinity` (2 engine dirs, ~2,712 LOC).

**Status: DONE.** All 5 `⚙️engine` directories under `🗿️artifacts/*` are gone. Trinity was done by hand; puzzle was delegated to 3 parallel sub-agents (one per artifact: ◻2d, 🧊️3d, 🖐️5d), each scoped to its own artifact+app subtree. Shared files (`📦️glue.rs` ×2, plugin-root `✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs`) were consolidated centrally by me afterward, along with a cross-crate fix in the framework OS renderer that no per-plugin `cargo check` could have caught.

## Trinity — completed by hand

### `♻️rewrite/…/⚙️engine` (526 lines, DELETED)

| Region | Destination | Rule |
|---|---|---|
| `RewriteRuleEngine` struct + impl | DELETED outright | Rule 1 — zero construction sites repo-wide |
| `Lhs`/`Rhs`/`Rule`/`ParameterKind`/`ParameterSpec`/`PatternJson`/`AssignmentJson`/`ApplyRuleResult`/`RuleQueryResult` + `apply_rule`/`apply_rule_json`/`build_rule_query`/`rule_query_json`/`parse_bindings_json`/helpers | `🧬️schema/🦀️component.rs` region `🔖️RuleApplication` | Rule 3 — pure helpers/types over the `jack::Graph` document type, no app/AppIo dependency |
| `#[cfg(test)] mod tests` (10 tests) | `🧬️schema/🦀️component.rs` region `🧪️RuleApplicationTests` | Rule 8 — moved verbatim beside the code it tests |
| `pub mod io_registry { … }` | `🚪️io/🦀️component.rs` region `🚪️DerivedIoRegistry` | Rule 5 |

Call sites fixed: `🗿️artifacts/♻️rewrite/🦀️component.rs` (`declaration()`'s `.composers(...)` + shim `use ... as v1;`), 4 app files under `🎛️apps/♻️rewrite/` — 21 occurrences of `crate::artifacts::rewrite::engine::` → `crate::artifacts::rewrite::schema::` (verified none collided with the unrelated `TrinityBoardEngine`/`self.engine` canvas-engine field also present in `🌍️world/🦀️component.rs`). `📦️glue.rs`: removed `pub mod engine;` mapping + its shim.

Assertion arithmetic: 10 tests / 35 assertions in the original file → 10/35 in the new location. Exact match.

### `🔌️jack/…/⚙️engine` (root file 134 lines + 4 kernel submodules 1,887 lines = 2,021 lines, DELETED)

| Region | Destination | Rule |
|---|---|---|
| `TrinityGraphEngine` struct + impl | DELETED outright | Rule 1 — zero construction sites |
| `empty_jack_document()` | `🧬️schema/🦀️component.rs` region `🔖️EmptyDocument` | Rule 3 |
| `#[cfg(test)] mod tests` (1 test) | `🧬️schema/🦀️component.rs` region `🧪️EmptyDocumentTests` | Rule 8 |
| `pub mod io_registry { … }` | `🚪️io/🦀️component.rs` region `🚪️DerivedIoRegistry` | Rule 5 |
| `🌳️ast/`, `🔤️lexer/`, `🧮️executor/`, `🗣️language-service/` (shared jack-query-language kernel, consumed at crate root as `crate::{ast,lexer,executor,language_service}` by both the jack app and rewrite's `apply_rule`) | physically `mv`'d to `🧬️schema/🌳️ast/`, `🔤️lexer/`, `🧮️executor/`, `🗣️language-service/` | Rule 3 — pure compute over `jack::Graph`, no `super::`/relative refs so the move needed zero content edits |

Call sites fixed: `🗿️artifacts/🔌️jack/🦀️component.rs` (`declaration()` + shim), one call site in `🧬️mutations/💾️binary/🦀️component.rs`. `📦️glue.rs`: repointed `ast`/`lexer`/`executor`/`language_service` `#[path]`s at `🧬️schema/...`; removed jack's `pub mod engine;` grouping and shim.

Assertion arithmetic: 1 test / 2 assertions → 1/2 in the new location. Exact match. Content-identity check (`diff` vs `git show HEAD:<old path>`) on the 4 moved kernel files: **all 4 byte-identical**.

## Puzzle — 3 parallel sub-agents + central integration

### `◻2d/…/⚙️engine` (297+972+85+1308+930+711 = 4,303 lines, DELETED)

| Region | Destination | Rule |
|---|---|---|
| `Puzzle2dEngine` struct | DELETED outright | Rule 1 — zero construction sites |
| `empty_puzzle2d_snapshot()`, doc-schema helpers | `🧬️schema/🦀️component.rs` | Rule 3 |
| `register_media_io()` | `🎛️apps/◻2d/🦀️component.rs` (own region) | Rule 6 |
| `io_registry` | `🚪️io/🦀️component.rs` | Rule 5 |
| `🎲️board-host/`, `📐️layout/`, `🔗️linking/`, `🔣️icons/`, `🖌️brush/` (stateful canvas host + its satellite modules) | `🎛️apps/◻2d/⚙️engine/` (new app-side dir, same 5 submodules) | Rule 4/7 — `BoardHost` is a per-session, `RefCell`-held mutable host; `taxonomy.json`/`discovery.ts` explicitly forbid `⚙️engine` under `🗿️artifacts`/subsets but *require* it as a legal `taxonomyLeafParentDirs` name and name it as the canonical exemplar for "behaviour belongs to the app (`🎛️apps/<app>/⚙️engine`)" — confirmed by reading `🧰️framework/…/📚️library/🔍️discovery/🟦️component.ts:426` and `🔣️taxonomy.json:3` directly before finalizing this placement (I initially renamed it to `🖥️host` out of an overcautious reading of the ticket's literal verification grep, then reverted after finding the taxonomy source of truth — see Deviations) |

Plugin-root `setup()`: `crate::apps::puzzle2d::register_media_io();`.

### `🧊️3d/…/⚙️engine` (718+635+1245+972+73+570 = 4,213 lines, DELETED)

| Region | Destination | Rule |
|---|---|---|
| `Puzzle3dEngine` struct | DELETED outright | Rule 1 |
| `BrushPlacePayload`, `Puzzle3dEngineCommand`, `Puzzle3dEngineOutcome` and other pure catalog/kind/scene types, `empty_puzzle3d_snapshot()` | `🧬️schema/🦀️component.rs` | Rule 3 |
| `📐️geometry/🎛flatten/` (pure snapshot→projection fn, consumed cross-artifact by puzzle2d's and puzzle5d's own inference files — the "mirror side" of `puzzle2d_manifest_fragment` the ticket flagged) | `🧬️schema/💡️inferences/🎛flatten/` | Rule 2 — derived compute from a snapshot |
| `register_mesh_io()` | `🎛️apps/🧊️3d/🦀️component.rs` | Rule 6 |
| `io_registry` | `🚪️io/🦀️component.rs` | Rule 5 |
| `⏳️session/`, `📐️geometry/` (minus flatten), `🖌️brush/`, `🪣️fill/` (stateful precompute/collision solver, `Puzzle3dPrecomputeSession`) | `🎛️apps/🧊️3d/⏳️precompute/` | Rule 4/7 |

Plugin-root `setup()`: `crate::apps::puzzle3d::register_mesh_io();`.

### `🖐️5d/…/⚙️engine` (334+262+236 = 832 lines, DELETED)

| Region | Destination | Rule |
|---|---|---|
| `Puzzle5dEngine` struct | DELETED outright | Rule 1 |
| `Puzzle5dPrecomputeSession` | `🎛️apps/🖐️5d/🧠️precompute/🦀️component.rs` (thin facade delegating to puzzle3d's own precompute session) | Rule 4/7 |
| `empty_puzzle5d_snapshot()`, `puzzle5d_grip_kinds_compatible()`, `next_id()` | `🧬️schema/🦀️component.rs` | Rule 3 |
| `puzzle5d_parse_dsl_json()` (wasm-bindgen boundary) | `🎛️apps/🖐️5d/🧠️precompute/🦀️component.rs` | Rule 4 |
| `✂️transfer/` | `🧬️schema/✂️transfer/` (renamed, content unchanged) | Rule 3 |
| `📐️flatten/` | folded into `🧬️schema/💡️inferences/` | Rule 2 |
| `register_mesh_io()` | `🎛️apps/🖐️5d/🦀️component.rs` | Rule 6 |
| `io_registry` | `🚪️io/🦀️component.rs` | Rule 5 |

Plugin-root `setup()`: `crate::apps::puzzle5d::register_mesh_io();`.

## Central integration (done by me after all 3 sub-agents)

- **`📦️glue.rs` (puzzle):** removed 3 stale artifact-side `pub mod engine {…}` groupings + 3 shims (◻2d, 🧊️3d, 🖐️5d); added `apps::puzzle2d::engine {…}` (5 submodules, mirroring the physical `🎛️apps/◻2d/⚙️engine/` move); added `apps::puzzle5d::precompute`, `schema::transfer` (puzzle5d), `schema::inferences::flatten` (puzzle3d — this one was still unwired even after the sub-agent's own work landed). Verified 0 dangling `#[path]` targets and 0 `standards::v1::engine`/`subsets::any::engine` residue.
- **Plugin root `✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs`:** `setup()`'s 3 calls repointed from `artifacts::puzzleXd::standards::v1::engine::register_*` to `apps::puzzleXd::register_*`.
- **Cross-artifact breakage found and fixed:** puzzle2d's and puzzle5d's own `🧬️schema/💡️inferences/…` files still imported `crate::artifacts::puzzle3d::engine::geometry::flatten::…` (5 call sites across 3 files) — fixed to `crate::artifacts::puzzle3d::schema::inferences::flatten::…`.
- **My own bug, caught by the actual compile, fixed:** a python substring replace I ran on `🎛️apps/🖐️5d/🧠️precompute/🦀️component.rs` produced `crate::artifacts::apps::puzzle3d::precompute::…` (wrong) instead of `crate::apps::puzzle3d::precompute::…` — fixed (2 sites).
- **Pre-existing bug in the sub-agent's move, caught by the actual compile, fixed:** `🎛️apps/◻2d/⚙️engine/🎲️board-host/🦀️component.rs`'s test-only `include_str!("../../../../../../🛂️manifest.jsondefault.manifest.json")` kept its pre-move relative depth (6 levels, correct for the old artifact-side location) after moving to a shallower app-side location — fixed to 4 levels (`../../../../🗿️artifacts/◻2d/🛂️manifest.jsondefault.manifest.json`), verified the resolved path exists.
- **Pre-existing bug in the sub-agent's move, caught by the actual compile, fixed:** `🧬️schema/💡️inferences/🎛flat-position/🦀️component.rs`'s `use super::fastened_layout_snapshot;` was one nesting level too shallow for glue.rs's `pub mod flat_position { mod component; … }` wrapping — fixed to `use super::super::fastened_layout_snapshot;`.
- **Pre-existing bug in the sub-agent's move, caught by the actual compile, fixed:** `🧬️mutations/🦀️component.rs` (puzzle2d) still imported `engine::empty_puzzle2d_snapshot` in 6 test-module `use` statements — fixed to `schema::empty_puzzle2d_snapshot`.

## Framework cross-crate consumer: `BoardHost`

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs` — outside both target plugins, a framework crate — held 6 references to `puzzle::artifacts::puzzle2d::engine::BoardHost`/`board_host::puzzle_board_host()` (field type, a doc-comment, `sync_board_host`, `with_board_host_mut`, `with_board_host`, and the constructor call). `cargo check -p semio-s-plugin-puzzle` cannot see this crate at all. Verified independently (own `grep`, not taken on faith) before and after: all 6 now read `puzzle::apps::puzzle2d::engine::BoardHost`/`board_host::puzzle_board_host()`, matching where `BoardHost` actually landed. This is an explicit, scoped exception to "work only inside the two target plugins" — exactly this one file, these exact references.

## Compiler verification

- **`semio-s-plugin-puzzle` (both lib and lib-test targets, `--all-targets`, `RUSTC_WRAPPER=""`):** `Finished `dev` profile [unoptimized] target(s) in 3m 42s` — **zero errors**, 67 + 79 warnings (all pre-existing style lints: unused imports/variables, unnecessary qualification). Verified directly by me, not relayed.
- **`semio-s-plugin-trinity`:** structurally verified clean (0 `⚙️engine` dirs, 0 stale `standards::v1::engine`/`subsets::any::engine`/`artifacts::{rewrite,jack}::engine` references, 4/4 moved kernel files byte-identical, test-assertion counts matched exactly). A full `cargo check -p semio-s-plugin-trinity --all-targets` could not complete: the shared build directory had ~25 concurrent cargo processes from other sessions in this ticket wave contending for one file lock; two attempts stalled on `stdio` mid-refactor by another session (unrelated, evidenced by `error: couldn't read ...🗿️artifacts/🧿️semio/…/📄set-snapshot/↩️inverse/🦀️component.rs: No such file or directory` — a file physically absent mid another session's edit) and a third attempt was killed on the coordinator's request once `semio-s-plugin-puzzle` (sharing the identical dependency graph, including the now-stable `stdio`) had already compiled clean, to stop adding pressure to a shared lock ahead of one central `--workspace --keep-going` pass. **Not claimed green — reported as structurally verified, compiler-unconfirmed, blocked by build-directory contention.**
- **`semio-framework-os-renderer-wgpu`** (the `BoardHost` consumer's own crate): `--all-targets` run completed and surfaced real pre-existing errors (`GraphHost`, `store_sync::{ArtifactActorMsg,ArtifactEvent,...}`, `MapHost`, `os_dsl`, `OrbitController`, `Camera3d`/`Vec3` in `semio_framework_3d`, `TutorialBase.document_dsl`, `InvocationResult.operations`, `UndoGroup.{operations,inverse_operations}`) — **zero of them reference `puzzle`, `BoardHost`, or `board_host`** (checked directly: the only `puzzle`-adjacent hits in the whole log are benign unused-import *warnings* inside puzzle's own crate, not renderer errors). Attribution: these are pre-existing, unrelated failures in the renderer's tutorial/map-host/graph-host/3d-camera subsystems — not caused by this packet.

## Deviations

- I initially (wrongly) renamed `🎛️apps/◻2d/⚙️engine/` to `🎛️apps/◻2d/🖥️host/` out of concern that the ticket's own literal verification grep (`grep -rn "::engine::" <plugin> → 0`) would flag the new app-side module. Reading `🔣️taxonomy.json`'s own comment and `🔍️discovery/🟦️component.ts:420-427` directly showed `⚙️engine` is explicitly legal — even canonical — one level up from artifacts, under `🎛️apps/<app>/`; the ticket's literal grep is a heuristic that doesn't distinguish artifact-side from app-side `engine`, and the real gate is "no `🗿️artifacts/*/⚙️engine` directories, no `standards::v1::engine`/`subsets::any::engine` module paths." Reverted the rename (42 call sites both ways) before finishing.
- Sub-agents were instructed not to touch `📦️glue.rs` or the plugin root, to avoid 3-way concurrent writes to shared files; I applied all necessary changes there centrally, re-reading fresh before each edit since another concurrent session/coordinator was also observed touching `📦️glue.rs` mid-session (confirmed: puzzle3d's artifact-side `engine` shim and its `apps::puzzle3d::precompute` mount appeared already wired between two of my own reads, without action on my part).

## Unverified

- `semio-s-plugin-trinity`'s actual `rustc` pass — see Compiler verification above. Structural correctness is as solid as trinity's evidence gets without a compiler run; a central workspace pass was reported as pending by the coordinator.
