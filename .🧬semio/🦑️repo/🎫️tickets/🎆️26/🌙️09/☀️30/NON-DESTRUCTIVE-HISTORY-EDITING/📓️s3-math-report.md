# 📓️ S3-MATH — Mathematical plugin, design §20.15 conversion

Executor S3-MATH (coordinator `⚪b7db773a…`, fleet rules 1–33). Scratch: `🗑️generated/s3-math/`. Plugin root
`MA = ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation`, subsets `SUB = MA/🏅️standards/🔖️1/🪆️subsets`.

## Session 3 — 2026-10-03

### Status log (newest first)

- 12:10 design note written, choice (a) sent to `main` (no framework contract change); implementation started.

### 1. Design note — where is the equation graph's single source of truth?

**Facts (code, 12:00).** `EquationSnapshot` = three `#[child(kind = "s.stdio.semio")]` handles (`notation` text, `results`
table, `computed` value) + the plain `equation` expression. The graph (`directed`, nodes, edges, algorithm, seed) and the point
cloud live ONLY in an ephemeral `EquationWorkingScene` attached to the handles as `local_owner` (`MA/🦀️.rs` 🔖️WorkingScene); DSL,
pack, JSON and sqlite carry the handles only, so every decoded / reloaded / remote parent folds on an EMPTY scene (fixtures even
commit `equation-scene-unresolved-*` handles). The three children are a lossless SPLIT of that state (`equation_*_from_*`
converters, positionally zipped: text run i ↔ table row i; edges + points in the value map), never independently authored. All
16 graph/geometry leaves read the owner (`equation_graph` 12, `equation_geometry` 4 = gate `parentLeafReadsChild` 16).

**Choice: (a) the graph and the point cloud are parent-owned persisted state; the three children are derived outputs.**

- `EquationSnapshot` gains `#[state(artifact)] graph: EquationGraph` and `geometry: EquationGeometry` (schema-first: json /
  ts / graphql / proto twins, DSL grammar, pack, sqlite, fixtures). Every parent leaf reads and writes only `base.graph` /
  `base.geometry`; no `local_owner` read exists anywhere in the plugin (`EquationWorkingScene`, `equation_scene*`,
  `require_equation_scene` deleted).
- The children are **content-addressed derived handles**: `child_id = store::content_id(<slot>, canonical derived pack)`,
  re-minted by a leaf's diff together with the new `graph`/`geometry` (only the slots whose derived content changed get a new
  id). They are never edited: the existing runtime path `follow_derivable_children` → `genesis_child_pack` opens the newly
  addressed child from the parent's own state and retires the old one (the writer / lowpoly / cad precedent: "the handle alone is
  the change signal"). So a derived child is a store that receives the recomputed output, but never through an authored op —
  no framework contract change.
- **§20.15 compliance.** "Parent-lane leaves never read `local_owner`" holds by construction. "Every content edit is a child-lane
  leaf in the child's store" applies to content the child owns; here the child owns nothing (its content is a pure function of
  the parent), so the authoritative lane is the parent lane and the rule "edits target the authoritative lane; a derived child is
  never edited directly" is met. "Readers compose on read" → readers read `snapshot.graph` / `.geometry` (no materialization).
- **Replay / time-travel determinism.** The parent fold is pure over the parent snapshot (§20.9): every leaf decides from
  `base.graph` / `base.geometry`, and the handles it writes are content ids of that state, so the same history folds to the same
  head on every replica, after reload, and in Report-mode replay of any superseded op. Time travel `state_before` / preview render
  the parent snapshot directly; the children follow the replayed head by coordinate after accept/finalize. Save → fresh load
  needs no materialization step (`composed_reload_law!`).
- **Why not (b) (graph as child content, parent empty).** The graph would be split over three generic stdio stores with a
  positional zip invariant (text run i ↔ table row i) and cross-store references (value `edges` ↔ table `id`s) that no single
  fold guards; one gesture (create node) = three child ops in three histories, so editing one of them in history desyncs the
  other two; domain outcomes (`target-missing` for a node id, unique ids, `mutation.partial`) cannot be stated in the generic
  text/table/value vocabularies; the value child has no relative op for point drags. (b) only fits when one child IS the content
  model (flow / sequence / dag / wires compose a typed graph child), not for a lossy split into generic carriers.
- **Geometry drags.** The absolute `move-point {index, x, y}` is replaced by the relative `move-points {indices, dx, dy}` (the
  `move-nodes` twin; base-relative, `mutation.partial` / `target-missing` / `no-op` like it) whose inverse is ONE absolute row
  `set-point-positions {positions: [{index, x, y}]}` (point-invertible, the `set-node-positions` twin). `move-point` and its
  fixtures / oracle vectors / feature rows are deleted (rule 32).

### 2. Changes

(in progress)

### 3. Verification

(in progress)

### 4. Gate counts

(in progress)

### 5. Coordinator actions

(in progress)

## Session 4 — 2026-10-04

Executor S4-WIRES-MATH (coordinator `⚪487b04ad…`, fleet rules 1–38). Scratch `🗑️generated/s4-wires-math/`. Companion report for
wires: `📓️s3-wires-report.md` § Session 4. Implements the approved model (a) of §1 above.

### Status log (newest first)

- 21:50 math green: `--lib` 0 errors (`check-math-3`; fallout fixed: kernel no longer re-exports `DslValue`/`FromValue`/`ToValue`/
  `ValueError` → `semio_framework_value`, `protocol::native_decoding` → `semio_framework_value`, `IoError { message }` →
  `IoError::from_value_error` via new `io::{pack_decode_refusal, invalid_payload}`, duplicate `EquationGraph`/`EquationGeometry`
  import, `into_snapshot` → `TextError::from_value_error`), `--lib --tests` 0 errors (`check-math-tests-2`, then batched
  `check-both-tests-1`; test fixes: `CompleteDownload` arm, sqlite literals gain `graph`/`geometry`, txt serialize takes
  `ArchiveChildren`), hub `semio-hub-mathematical` wasip2 0 errors. COMPOSITION GREEN mathematical sent.
- 21:2x CARGO OPEN resumed. Math test tree converted (source): 13 scenario tests on inline state (codemod + manual seeds/create-node),
  new `🎯️move-points/🧪️tests/🧪️translates` + `📌️set-point-positions/🧪️tests/🧪️restores` with quintet fixtures, aggregate unit tests
  (move-points/set-point-positions laws), snapshot text/pack tests, crate-root tests rewritten (carrier, content addresses, genesis,
  state diff, fixture WRITER law `committed_fixtures_carry_their_derived_handles`), `👑️equation-scene-owner-law.json` deleted (only
  reference was the old root test). All 30 mutation fixture snapshots/diffs carry inline `graph`/`geometry`; handles still pre-writer.
  Oracles: geometry catalog/manifests (move-points, set-point-positions vectors), ✳️any catalog kinds, graph/equation rationales;
  case adapters + features for geometry (✳️any + 📐️geometry), stale graph adapter/feature text. Pack-error fallout converted per
  `📓️s4-packfix-report.md` mapping (envelope build/unwrap → `PackError::from(error.into_value_error())`, identity mismatch →
  InvalidValue ValueError, FromValue → `PackError::from(error)`), 3 sites. Lib/tests checks blocked: dependency
  `semio-s-artifact-stdio-semio` red (77 `PackError::Schema`), routed to S4-PACKFIX.
- 13:3x PARKED (coordinator usage limit). Production source of model (a) is complete; tests/fixtures are mid-conversion. Exact state and
  next steps: §6. No math cargo run happened this session (rules 43/44): everything below is source only, checks OWED.
- 13:2x audit-tools F1/F9 taken: stale editor test fixed + `composed_reload_law!`; equation tool-mismatch → `app.command.tool-mismatch`.
- 02:2x rule 34 repair-first: `git diff HEAD --stat -- ✏️s/🔌️plugins/➗️mathematical` = 127 files of earlier peer waves
  (ValueError, `warning`, value-crate paths, Cargo deps); no unstaged change, no S3-MATH source edit on disk. Nothing to repair.

### 2. Changes

Root `MA = ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation`, `SUB = MA/🏅️standards/🔖️1/🪆️subsets`.

- **State**: `EquationSnapshot`/`EquationArtifact`/`EquationDiff` gain parent-owned `graph`/`geometry` (Rust, ToValue/FromValue, pack
  record `EquationPackRecord` + controlled decode/encode, DSL grammar, JSON schemas with `$defs` graph/node/edge/point/geometry in
  `✳️any/🧬️schema/🔣️.json`, snapshot/diff JSON `$ref`s, TS (`parseEquationGraph`/`parseEquationGeometry`, diff TS), GraphQL and
  proto twins for artifact/snapshot/diff, sqlite: 4 new tables (`equation_graph`, `equation_graph_node`, `equation_graph_edge`,
  `equation_point`) in `🗄️.sql` + Rust projection/reconstruction/validation + TS twin).
- **Derived children** (`MA/🦀️.rs` 🔖️DerivedChildren): `equation_children`, `equation_state_diff`, `genesis_equation_child_pack`
  (verifies the content address), `equation_fixture`, `equation_snapshot_with_state`, `equation_snapshot_from_host_snapshot`.
  Deleted: `EquationWorkingScene`, `equation_scene*`, `require_equation_scene`, `equation_graph`/`equation_geometry`,
  `equation_children_from_state`, `equation_graph_geometry_from_children` (zero references).
- **Leaves**: all 15 graph/geometry diff/inverse files read `base.graph`/`base.geometry` and diff through `equation_state_diff`.
  New `SUB/📐️geometry/🧬️schema/🧬️mutations/🎯️move-points` (relative `{indices,dx,dy}`, partial/target-missing/no-op/target-mismatch,
  inverse ONE `set-point-positions`) and `📌️set-point-positions` (`{positions:[{index,x,y}]}`, point-invertible), both with
  descriptor, payload schema (en/de `x-semio-ui`, `x-semio-inverse-rows {fixed:1}`, `unique-point-indices`), Rust, diff, inverse.
  Deleted `🎯️move-point` (leaf, fixtures, flat `🧫️fixtures/🕹️move-point`). Aggregate enum/KINDS/module tree, mutation codecs (text +
  binary: MovePoints tag 13, SetPointPositions tag 17), the stale codec twins rewritten to the real wire (`📖️.grammar.semio`, `🅰️.g4`,
  `🔤️.ebnf`, text `🔗️.graphql`/`🛰️.proto`, `📡️.protocol.semio`), aggregate JSON/TS/GraphQL/proto twins (TS re-exports the graph types
  from the schema TS). Deleted the stale duplicate `✳️any/🧬️schema/🧬️mutations/📖️.grammar.semio` (old shape, zero references).
- **Readers**: editor extent/phases read `snapshot.graph/geometry` (`source_scene` deleted), csv/json/md serializers.
- **Audit-tools F1/F9**: `✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` stale owner test → value-decode law, `composed_reload_law!` added;
  `✏️editor/🦀️.rs:1129` → `app.command.tool-mismatch`. `composed_child_history_law!` is deliberately NOT added: under model (a) the
  three children are derived and never edited (§1, "a derived child is never edited directly"), so no seed gesture can land a child-lane
  edit — the law's precondition cannot exist. Parent leaves stay (they now edit parent-owned fields, model (a)); only `move-point`
  is deleted, as the audit's Fix says.
- **Fixtures**: sqlite fixture gains `graph`/`geometry`; Rust + TS sqlite tests construct them.

### 3. Verification

| Command | Result |
| --- | --- |
| `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-mathematical-equation --lib` (21:42, `check-math-3`) | exit 0, 0 errors, 33 warnings |
| same `--lib --tests --keep-going` (21:44, `check-math-tests-2`; 21:50 batched with wires, `check-both-tests-1`) | exit 0, 0 errors (lib-test 42 warnings) |
| `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-mathematical --lib --target wasm32-wasip2` (21:48-21:49) | exit 0, 0 errors |
| `SEMIO_EQUATION_WRITE_FIXTURES=1 cargo test … committed_fixtures_carry_their_derived_handles` (writer, FIRST) | OWED (rule 43) |
| `cargo test --lib` math, publication-authority law | OWED (rule 43), after the writer |

### 4. Gate counts

Not measured this session (no run allowed).

### 5. Coordinator actions

After CARGO OPEN: math lib check (expect first-compile fixes), then the fixture writer (§6 step 3) BEFORE the test run; `schema
generate`; taxonomy regeneration for `🎯️move-points`, `📌️set-point-positions`; describe mathematical; re-activation.

### 6a. State after the CARGO OPEN wave (21:50)

Steps 1–4 of the parked list are done (test tree, fixtures with inline state, writer law, case adapters/features/catalogs, `local_owner`
rationale). Remaining, all test runs (TESTS RESUMED): run the writer once with `SEMIO_EQUATION_WRITE_FIXTURES=1` (rewrites every
`🧫️fixtures/🧬️mutations/**` snapshot/diff handle — including the `*-unwritten` placeholders of `🧪️translates`/`🧪️restores` — and the
demo asset), then `cargo test --lib`. Step 5 (no editor point gesture) is unchanged.

### 6. Parked state and next steps (13:3x, historical)

Consistent on disk: all production source above. Test tree is mid-conversion and will not compile with `--tests` yet:
1. Per-scenario tests still reference removed APIs (19 files): `MA/🧪️tests/🔬️unit/🦀️.rs`, `✳️any/🚪️io/📸️snapshot/{💾️binary,📝️text}/
   🧪️tests/🔬️unit/🦀️.rs`, `✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` (move_point), `➗️equation/…/🎚️change/🧪️tests/🧪️raises`,
   `📐️geometry/…/{➕️insert-point/🧪️seeds,➖️remove-point/🧪️rejects,🔄️replace/🧪️replays}`, `🕸️graph/…/{✂️disconnect-nodes,❌️delete-node,
   ➕️create-node,🏷️change-node,🔗️connect-nodes,🕹️move-node,🗑️delete-nodes}/🧪️tests/🧪️rejects`, `🔁️replace-graph/🧪️replays`,
   `🧭️change-graph/🧪️keeps`, `🧮️update-graph/🧪️restates`. Plan: one shared `#[cfg(test)]` vector helper
   (`✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️vector/🦀️.rs`: decode the committed quintet, apply, after/diff/outcome/inverse/canonical
   laws) and each scenario file = its consts + leaf-specific facts, reading `base.graph`/`base.geometry`.
2. Committed mutation fixtures (`SUB/*/🧫️fixtures/🧬️mutations/**/📸️snapshot/{⬅️before,➡️after}/🔣️.json`, 🔺️diff of applied/no-op
   vectors) lack `graph`/`geometry` and carry placeholder handles: add the scenario's explicit state (empty graph
   `{directed:true,nodes:[],edges:[],algorithm:"",algorithmSeed:null}` / empty cloud, the colliding node for create-node) in ToValue
   order; new quintets + wire witnesses for `move-points` (`🧪️translates`) and `set-point-positions` (`🧪️restores`) — the crate-root
   module tree already mounts those two test files (they do not exist yet).
3. Writer/assert law in `MA/🧪️tests/🔬️unit/🦀️.rs` (env `SEMIO_EQUATION_WRITE_FIXTURES=1`): every committed snapshot/diff handle =
   `equation_children(graph, geometry)` (content ids need Rust: sha256 of the derived pack), the demo asset
   `✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio` = `print_dsl(&EquationSnapshot::default())`; the asset still has no graph/geometry.
4. Case adapters + features + catalogs: `✳️any/🧪️tests/📐️mutate-equation-1-any-geometry/{🦀️.rs,🥒️.feature}`,
   `📐️geometry/🧪️tests/📐️mutate-equation-1-geometry/{🦀️.rs,🥒️.feature}`, `🔮️oracles/🔣️.json` (✳️any, 📐️geometry, 🕸️graph, ➗️equation:
   move-point → move-points + set-point-positions vectors, and the `local_owner` rationale at `➗️equation/🔮️oracles/🔣️.json:39`);
   the vectors can now be forward evidence (graph inline) — UNOBSERVABLE lists shrink.
5. Editor point gesture: none exists (points are edited through `setPoints`/inspector); `move-points` is reachable from history,
   the inspector (x-semio-ui) and agents.

