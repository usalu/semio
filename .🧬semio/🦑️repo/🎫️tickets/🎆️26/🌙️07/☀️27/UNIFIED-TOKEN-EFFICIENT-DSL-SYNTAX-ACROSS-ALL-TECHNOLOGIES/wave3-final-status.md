# Wave 3 — Final convergence gate: status

## 1. Structural gate

Deleted `impl DocumentDsl for serde_json::Value` from `vcs/rs/lib.rs` (was L241-258, including its
doc comment). This is the one and only intentional deletion in this wave; everything else below is
fallout from that deletion (plus one piece of unrelated, live concurrent-session fallout — see §4).

## 2. Stragglers found and fixed

`cargo build --workspace` after the deletion pointed at exactly three straggler sites, all using the
raw-JSON `DocumentDsl` bridge instead of a typed derive:

- **`compose/client/lib/rs/lib.rs`** (`KitSnapshot`, in `pub mod kit_vcs`) — `KitSnapshot` is a
  *deliberately* schema-less bridge (doc comment already said so: no plain "whole kit" struct exists
  to derive `dsl::DslDocument` on; the real state is a live `Arc<RwLock<..>> Kit` entity graph).
  Fixed by inlining the JSON parse/print directly (`crate::external_adapters::serde_json::from_str` /
  `to_string_pretty`) instead of delegating to the deleted blanket impl — same behavior, now a local
  bridge instead of a repo-wide one. Also updated the stale doc comment that referenced the deleted
  impl.
- **`puzzle/plugin/rs/lib.rs`** — `Puzzle2dPlayApp`, `Puzzle3dPlayApp`, `Puzzle5dPlayApp` all declared
  `type Projection = Value` (a documented, deliberate design from the prior, separately-closed ticket
  `26/07/26/CONVERT-PUZZLE-2D-3D-5D-TO-TYPED-DSL-DERIVE-ENGINE`, which converted `puzzle_2d`/`_3d`/`_5d`
  themselves to typed `DocumentDsl` structs but explicitly left `puzzle-plugin`'s own ~12.6k lines of
  Value-manipulating scene-mutation/rendering code untouched as out-of-scope follow-up work). Since
  `serde_json::Value` no longer implements `DocumentDsl`, and `Value` is foreign to `puzzle-plugin` (no
  local impl possible — orphan rule), a full rewrite of that scene-mutation code onto the typed structs
  was not attempted (real, substantial, already-scoped-out work, not a small straggler). Instead, added
  a local newtype bridge per crate — `Puzzle2dPlayProjection(pub Value)` / `Puzzle3dPlayProjection` /
  `Puzzle5dPlayProjection`, one per crate in each crate's `🔖️ValueBridge` region (new `🔖️PlayProjection`
  subregion) — implementing `DocumentDsl` (JSON round-trip, mirrors the `KitSnapshot` fix above) and
  delegating `Operation`/`OperationDiff` straight through to the crate's existing `Operation<Value>`/
  `OperationDiff<Value>` impls via `.0`. `puzzle-plugin` then uses these newtypes as
  `DocumentApp::Projection`; every touch point (`type Projection`, `initial_projection`, the 14
  `DocumentView<'_, Value>` signatures across `handle_action`/`render`/`window_engagements`/
  `window_measures`/`tool_measures`, and the ~52 `app.projection().expect("projection")` call sites in
  the crate's own tests) now unwraps `.0` to keep working with plain `Value` exactly as before — zero
  behavior change, only the trait-satisfying wrapper type changed. This is a genuine local-bridge
  pattern already established elsewhere in this same codebase (`compose`'s `KitSnapshot`), not a new
  kind of escape hatch.
  - Mid-fix, a **second**, unrelated trait bound (`vcs::DocumentPack`) appeared on
    `DocumentApp::Projection` — added live by a different, concurrently-running session working ticket
    `26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS` (confirmed via file mtimes and a live
    system-reminder mid-session showing `puzzle/5d/rs/lib.rs` being concurrently edited by that other
    session). Since my three new newtypes also need to satisfy that bound to keep `puzzle-plugin`
    compiling at all, added `impl vcs::DocumentPack for Puzzle{2d,3d,5d}PlayProjection` right next to
    the `DocumentDsl` impls, delegating to `serde_json::Value`'s own `DocumentPack` impl (still
    standing — that ticket has not deleted its JSON bridge yet) via `.0`. This is necessary
    interoperability with the current state of the shared `DocumentApp` trait, not scope creep into the
    Pack ticket's own work.

## 3. Build / clippy / test status for everything this ticket touched

- `cargo build -p vcs -p compose -p puzzle_2d -p puzzle_3d -p puzzle_5d -p puzzle-plugin`: **green**,
  zero errors (re-confirmed after the concurrent `DocumentPack` churn settled).
- `cargo test -p vcs -p compose -p puzzle_2d -p puzzle_3d -p puzzle_5d -p puzzle-plugin --lib`:
  - `vcs`: green.
  - `compose`: `kit_snapshot_dsl_round_trips` / `kit_snapshot_store_document_text_round_trip` (the two
    tests directly exercising my fix) both pass. 12 unrelated pre-existing failures (GraphQL kit
    mutation round-trips: `create_design_on_kit_graphql_roundtrip` and 11 others) — confirmed
    byte-for-byte identical failure set (same 12 test names, same panic messages/line numbers, same
    "59 passed; 12 failed" count) against `wave0-test-workspace-3.txt`, captured *before* this wave's
    changes. Not caused by this wave.
  - `puzzle_2d`: 119 passed, 2 failed — `puzzle2d_default_manifest_satisfies_board_host_validation`
    (`InvalidHandleKindColor("var(--muted-foreground)")`) and
    `board_host_fill_base_core_rectangular_excludes_cylindric_tambour`. Both are the exact two
    pre-existing failures already documented in the closed
    `CONVERT-PUZZLE-2D-3D-5D-TO-TYPED-DSL-DERIVE-ENGINE` ticket summary (CSS-variable color parsing and
    a brush-fill placement count, neither related to the document/DSL layer). Not caused by this wave.
  - `puzzle_3d`, `puzzle_5d`, `puzzle-plugin`: all green (105/105 for `puzzle-plugin`, up from before
    since the new `PlayProjection` code paths are now exercised).
- `cargo clippy -p vcs -p compose -p puzzle_2d -p puzzle_3d -p puzzle_5d -p puzzle-plugin`: no new
  warning classes introduced by this wave's edits specifically. The 2 lines I added to `compose`
  (`crate::external_adapters::serde_json::...`) do trigger rustc's `unused_qualifications` lint, but
  that is the crate's own pre-existing, pervasive style (32 other call sites already trigger the exact
  same warning for the exact same `external_adapters::serde_json::` pattern before this wave) —
  required by this repo's "external libraries behind an interface" rule (`CLAUDE.md`), not something to
  "fix" by reaching around the interface wrapper.
  `cargo clippy --workspace -- -D warnings` cannot run clean workspace-wide regardless of this wave —
  the repo has large pre-existing clippy debt in many unrelated crates (`pack_async`,
  `mathematical_polynomial`, `animate_video`, …), all pre-existing and out of scope per the task's own
  instructions ("do not fix unrelated pre-existing warnings outside the scope of this work").

## 4. `cargo build --workspace` — full-workspace status (not this ticket's crates)

**Not green**, but every single remaining error belongs to one of two other, actively-in-progress
concurrent sessions, confirmed via live file mtimes and (for one) a live system-reminder mid-session:

1. **UI theming refactor** (`ChromePalette` fields renamed; `ui/styling/rs/generated.rs` not yet
   regenerated to match `ui/tui/rs/lib.rs`'s new field usage) — blocks `ui_tui` → `repo_cli` (its only
   dependent). Zero relation to `DocumentDsl`/`serde_json::Value`/DSL text syntax; confirmed by grep
   (`ui/tui/rs/lib.rs` has no `DocumentDsl`/`vcs::`/`serde_json::Value` references at all).
2. **`26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS`** (a separate, concurrently-open ticket
   adding `vcs::DocumentPack` as a second bound on `DocumentApp::Projection`, then rolling typed
   `DocumentPack` impls out crate-by-crate) — over the course of this session it progressively broke,
   then (for the crates I needed) got fixed again, `framework/sync`, `puzzle-plugin`'s dependency chain,
   and by the final check newly broke `dag-plugin`, `flow-plugin`, `imperative-plugin`,
   `mathematical-plugin`, `procedural-plugin`, `sequence-plugin`, `shooting-plugin`, `trinity-plugin`
   (their typed projections — `DagDocument`, `FlowFixture`, `GraphFixture`, `ImperativeDocument`,
   `MathProjection`, `Procedural2dDocument`, `Procedural3dDocument`, `SequenceFixture`,
   `ShootingFixture` — don't yet implement the new `DocumentPack` bound). Several of these
   (`flow-plugin`, `sequence-plugin`) are nominally this ticket's own Wave-2 P3 adopter crates, but the
   *specific* failure in each is 100% about the unrelated `DocumentPack` trait, not `DocumentDsl` — a
   `grep -c` for `DocumentDsl`-related trait-bound errors across a full fresh
   `cargo build --workspace --keep-going` run returns **zero** hits repo-wide.

Not attempted: fixing another live session's in-progress, still-changing trait rollout would mean
chasing a moving target and risks conflicting with that session's own in-flight edits (twice already
observed live: `puzzle/5d/rs/lib.rs` and `framework/sync/rs/lib.rs` were both concurrently modified by
that other session mid-way through this wave). Per this repo's own concurrent-dev rules, that ticket's
own stragglers are that ticket's to fix.

## 5. Fixture hand-review (5 fixtures read end to end)

1. **Wire-heavy** (`infinite/board/port/directed/dag/example/demo.dag`): `computation`/embedded
   sub-blocks read as clean SoA tables (`inputs [id:TEXT label:TEXT … shape:ENUM visible:BOOL] { in
   value "" "" "" _ _ _ _ _ "!" semicircle true }`, `edges [id:TEXT source:TEXT target:TEXT
   route-style:ENUM properties:MAP] { e1 "slider:out" "scale:in" bezier {} … }`) — kebab column names,
   UPPERCASE type tags, `_` placeholders, bare-preferred. **Finding**: the top-level `nodes=[...]` list
   stays AoS (each node repeats `id=`/`name=`/… — likely legitimate, since node payloads are
   heterogeneous per kind and not self-delimiting as a single record shape, matching the
   architect/program and remodel findings about nested/heterogeneous table-cell limits already recorded
   in `wave2-p10-sweep-status.md`). **Also confirmed still-outstanding**: edge `source`/`target` still
   use the old `"nodeId:portId"` string format (`"slider:out"`), not the unified `nodeId@portId` port
   syntax — `infinite/board/port/directed/dag/rs::split_dag_endpoint` (line ~1873) still
   `rsplit_once(':')`. This is the exact, already-flagged, deliberately-deferred 3-way gap from
   `wave2-p10-sweep-status.md` finding #1 (node-graph producer / `s/plugin` producer / dag consumer all
   need to change atomically) — confirmed still open as of this wave, not fixed here (would require
   touching P4's/other packages' files single-handedly, against that finding's own explicit warning).
2. **Table/SoA-heavy** (`sourcing/curate/example/demo-stock.curate`): `curated [object-id:TEXT
   count:UINT] { beam-glulam-gl24h 3  window-casement-100x120 2 }` — clean, compact SoA table exactly
   per spec. `stock=[...]` stays AoS (documented: `ObjectKind.geometry` is `#[dsl(statements)]`,
   non-self-delimiting, can't be a table column).
3. **`.ops`-format artifact**: no standalone `.ops` fixture file exists on disk (the format is
   exercised only through `DocumentVcsEnvelope`/generated test text, never `include_str!`'d). Reviewed
   by temporarily adding one `eprintln!` to the existing
   `document_text_round_trips_with_an_active_alternative_and_a_quoted_description` test in
   `vcs/rs/lib.rs`, running it once, capturing the output, then removing the `eprintln!` again (no net
   change to the file). Output:
   ```
   doc demo schema=demo/v1
   edit edit-2 started="…" actor=local finished="…" description="said \"hi\" and used a \\ backslash"
     set-n n=1
   change change-3 saved="…" edits=[ edit-2 ]
   checkpoint checkpoint-4 at="…" changes=[ change-3 ] by=[ ]
   alternative alternative-5 name="branch \"a\"" checkpoints=[ checkpoint-4 ]
   active alternative-5
   ```
   Sigil-free keywords, id positional-first, kebab keys (`set-n`), real lists (`edits=[ edit-2 ]`),
   proper quote-escaping, no space around `=`. Matches spec exactly.
4. **Draw (SVG-path-subset)** (`draw/example/semio.draw`): `segments { M 1.25,196.933  L
   36.25,161.125  L 36.25,43.75  L 1.25,43.75  Z }` — genuine SVG-path-data subset (M/L/Z), reads
   directly as compact path data. Matches spec.
5. **Jack/query** (`writer/example/jack.writer`, `writer/example/dag.jack.writer`): embedded Cypher-subset
   text, e.g. `MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = "core" RETURN a.name, b.name` —
   `->` connections, PascalCase kinds (`Piece`, `Connection`). **Finding**: `dag.jack.writer`'s embedded
   query uses lowercase kind labels instead (`MATCH (n:computation) RETURN n.name, n.kind`) — plausibly
   legitimate (DAG node `kind` values are themselves lowercase runtime strings, e.g. `slider`/`select`/
   `computation`/`screen` as literally stored in `demo.dag`, not Rust-enum-derived PascalCase tags, so
   Jack naturally matches that vocabulary as-is) rather than a casing bug, but flagged for visibility
   since it reads inconsistently against the `Piece`/`Connection` example sitting right next to it.

## 6. Residue grep (leftover temp tests / `[DEBUG]` logs, outside ticket folders)

- **Leftover regen/canonicalize test functions**: zero hits anywhere in the repo
  (`fn canonicalize_fixture`, `fn *_regen_fixture`, `fn *_migrate_fixture`, `fn regen_*`) outside ticket
  folders. Clean.
- **`[DEBUG] ` log lines**: none were introduced by this wave (verified — my only temporary one, added
  to `vcs/rs/lib.rs` for the `.ops` fixture review in §5.3, was removed again before this file was
  written). A targeted grep across every crate this whole DSL-syntax ticket touched (Waves 0-3: `dsl/*`,
  `vcs`, `mathematical/graph/dsl(+core/js)`, `compose`, `puzzle_2d`/`_3d`/`_5d`/`plugin`,
  `mathematical/plugin`, `sourcing/curate`, `protocol/module/procedural`, `vcs/plugin`, `protocol`,
  `cad`, `remodel`, `architect/program`, `flow/core`+`plugin`, `sequence/core`+`plugin`, `draw/plugin`,
  `raster`, `note`, `writer`, `infinite/board/port/directed/dag`, `s`+`plugin`, `trinity/ram`+`rewrite`,
  `framework/product/os/core`) found a handful of pre-existing `[DEBUG]`-prefixed logs, none in code
  this wave (or any DSL wave) added: `compose` test-fixture skip guards (`eprintln!("[DEBUG] skip
  kit_store_comprehensive_fixture_contract_is_valid: …")` and similar, ~10 sites), `puzzle_3d`
  perf-timing prints (`println!("[DEBUG] apply_fill_count(5): …")`, 2 sites),
  `infinite/board/port/directed/dag`'s `dag_debug_log` calls (~10 sites, all routed through a named
  helper, not raw prints), and `s/plugin`'s studio create/open logs (4 sites). All predate this ticket
  (none reference `DocumentDsl`/DSL text/fixtures) — not migration residue from this effort, just this
  repo's pre-existing, much larger `[DEBUG]`-log backlog (a full unscoped repo grep turns up hundreds
  more across completely unrelated modules like `trinity/rewrite`, `framework/renderer/wgpu`, etc.). Not
  cleaned up here as out of this ticket's scope (general debt, not this wave's residue) — flagged for
  the parent/owning sessions.
- Also noted in passing (not `[DEBUG]`, not residue, just an observation): `bun nx run
  @semio-tech/plugin-registry:check` fails ("plugin registry catalog is stale:
  generated/playgrounds.json, generated/playgrounds.ts"), blocking `bun ./📜️script.ts verify` past its
  first two gates (dependency-cruiser passed clean — 1430 modules, 1136 dependencies, zero violations).
  Confirmed not caused by this wave: this wave touched no `Cargo.toml`, no playground/app-metadata,
  no example-fixture *files* (only fixture-generating Rust source); `git status` shows the concurrent
  `PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS` and other concurrent sessions actively editing
  `framework/product/os/core/rs/lib.rs`, `framework/plugin/host/rs/lib.rs`, `framework/plugin/rs/lib.rs`
  and several `ui/*` files — any of which can feed the playground registry generator. The remaining
  `verify` gates (region/host-contract lints, no-px check, leveled-test-target coverage) were run
  individually and all pass: `framework-renderer-react:lint` ok, `framework-os-dev:plugin lint` ok,
  `ui-styling-tokens:check-no-px` ok.

## 7. New files outside the ticket folder

None. `git status --porcelain --untracked-files=all` (repo-wide) shows zero untracked (`??`) entries at
the time of this check. Every edit this wave made was to an existing file.

## Files touched by this wave (Wave 3 only)

- `vcs/rs/lib.rs` — deleted `impl DocumentDsl for serde_json::Value` (structural gate).
- `compose/client/lib/rs/lib.rs` — `KitSnapshot`'s `DocumentDsl` impl now round-trips JSON directly
  instead of delegating to the deleted blanket impl; doc comment updated.
- `puzzle/2d/rs/lib.rs` — new `🔖️PlayProjection` subregion: `Puzzle2dPlayProjection` newtype +
  `DocumentDsl`/`DocumentPack`/`Operation`/`OperationDiff` impls.
- `puzzle/3d/rs/lib.rs` — same, `Puzzle3dPlayProjection`.
- `puzzle/5d/rs/lib.rs` — same, `Puzzle5dPlayProjection`.
- `puzzle/plugin/rs/lib.rs` — `Puzzle2dPlayApp`/`Puzzle3dPlayApp`/`Puzzle5dPlayApp`'s `DocumentApp`
  impls switched from `Projection = Value` to the new `Puzzle{2d,3d,5d}PlayProjection` newtypes;
  updated the 3 `type Projection`/`initial_projection` pairs, all 14 `DocumentView<'_, Value>`
  signatures (`handle_action`/`render`/`window_engagements`/`window_measures`/`tool_measures` across
  the three apps) to the new projection types with `.0` unwraps at every `doc.projection` touch point,
  and all ~52 `app.projection().expect("projection")` call sites in the crate's own tests (mechanical
  `.0` insertion, zero test-logic changes).

No files created or deleted. Scratch/log files for this wave live under this ticket's `scratch/`
directory (`wave3-final-build-workspace.txt`, `wave3-test-puzzle-compose.txt`,
`wave3-clippy-touched-crates.txt`, `wave3-script-verify.txt`).
