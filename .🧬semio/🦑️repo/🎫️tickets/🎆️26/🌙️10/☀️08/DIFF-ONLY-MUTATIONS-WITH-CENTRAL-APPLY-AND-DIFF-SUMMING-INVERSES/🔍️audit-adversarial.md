# 🔍️ Adversarial Audit: Diff-Only Mutation Laws Versus The Textual Gate

Read-only audit against `📋️design.md` (laws L1–L5, rulings waves 1–4). No source edited, no builds or gate runs, no git writes. Auditor: adversarial pass, single agent, foreground.

Path shorthand used below: `P/` = `✏️s/🔌️plugins/`, `F/` = `🧰️framework/🛍️products/💻️os/🔨️modules/`. Long artifact segments (`🗿️artifacts/…/🏅️standards/…/🪆️subsets/…`) are elided with `…` where the file is unambiguous from the leaf name; grep-exact paths are reproducible from the leaf name.

## 1. Summary

- The gate passes because it only sees fns named exactly `diff`/`inverse`, only inside a few file shapes, and only a handful of spellings. Every confirmed violation below sits outside that net.
- Confirmed L1/L4 violations: lowpoly selection motions (F-01), process3d step timeline (F-04), raster paint/fill whole-image diffs (F-14), equation `replace`/`replace-graph` and animate `replace-tiles` snapshot differencing (F-06, F-07, F-08).
- Confirmed L2 derived or restore inverses hidden in helpers: lowpoly selection inverse (F-02), lowpoly paint/patch inverse (F-03), remodel and shooting `negative` (F-11), flow VCS and run diff-type inverses (F-12, F-13).
- Renamed-between generic diff: `value_diff_between` in stdio json/semio/pdf leaves (F-09), a generic value patch that is exactly the L1 non-goal.
- Design gap, not only gate gap: `pub fn apply_diff` (`🧰️framework/…/🎮️mutation/🦀️.rs:126`) mints `ApplyCapability` for any caller, so L5 "impossible by design" is not met; only the textual gate stands between leaves and the central applier (D-01).
- Coverage gap in the gate itself: 924 leaf kind files (`🦀️.rs` next to `🔺️diff`/`↩️inverse`) carry non-delegating inline logic and sit partly outside the textual rules (section 2, G-9).
- Gate last log (`🗑️generated/coord/gate-run-3.log`): R8=9, R9=0, R10=7, R11=0, R12=0, R13=0, R14=23, R15=109, R16=1. Zero R9/R11/R12/R13 rows is consistent with the blind spots below, not with compliance (the log may also be stale).
- Verified clean (no finding): energy sampled leaves and inverse guards, gltf `rewire` remap family, wfc change-tile-weight, bim cascade, hub space config/presence, norm `inverse_rows`/`between_rows` (sync-only), cad between (trait-level only).
- Biggest open ruling questions: derived-handle re-minting in leaves (AMB-2), `order` whole-id lists (AMB-1), and simulated diff-type inverses (AMB-3). See section 6.

## 2. Gate Blind Spots (Why The Textual Policy Passes)

All references are to `📜️script.ts` (`policyDiffOnlyFileBreaches`, ≈ lines 21624–21690; regex constants 21365–21385).

| ID | Blind spot | Code evidence | What evades it |
|---|---|---|---|
| G-1 | R8/R11/R12/R14 only scan fns whose name is exactly `diff` or `inverse` | `diffFns = fns.filter(fn.name === "diff")`, `inverseFns = fns.filter(fn.name === "inverse")` (21665–21666) | Any helper with another name: `lowpoly_selection_motion_diff`, `paint_edits_inverse`, `negative`, `state_after`, `graph_replacing`, `points_replacing`, `process3d_step_timeline_diff`, `paint`, `painted`, `value_diff_between` |
| G-2 | Every fn inside `impl MutationDiff<…>` or `impl DiffAlgebra<…>` is excluded from R8–R12/R14 | `fns = policyRustFns(...).filter(fn => !within(diffTypeImpls, fn.start))` (21664) | Diff-type `DiffAlgebra::inverse` bodies: `RunDiff::inverse`, `FlowDiff::inverse`, `RemodelingRows::negative` callers, `ProgramDiff`, lowpoly `DiffAlgebra::inverse` |
| G-3 | Files outside a `🧬️mutations/` dir with no `impl … Mutation<` are returned early | `if (!leafCandidate) return breaches;` (21659), `leafCandidate = !testPath && (underMutations \|\| IMPL_PROBE)` (21631) | All `🧬️schema/🔺️diff/🦀️.rs` helper modules (0 `impl Mutation<` in each one checked: lowpoly, equation, draw, process3d, flow, run, remodel), `🔨️modules/🧬️mutation-support/` modules (gltf `top-level-collections`), `🏭️process/🧊️process3d/🦀️.rs` |
| G-4 | R9 `apply` rule fires only in `🔺️diff`/`↩️inverse`/`🦠️mutation` directories under `🧬️mutations`, or inside `impl Mutation<` | `applyDirectory = underMutations && APPLY_DIRS.has(…)` (21668), `APPLY_DIRS = {🔺️diff, ↩️inverse, 🦠️mutation}` (21365) | `protocol::apply_diff(&wire, &mut mesh)` in `🧬️mutations/🦀️.rs:109` (lowpoly); `protocol::apply_diff` in `🧬️schema/🔺️diff/🦀️.rs:272` (lowpoly `patch_fold`) |
| G-5 | R10 regex `(?<!\w)between\s*\(` misses renamed helpers | `POLICY_DIFF_ONLY_BETWEEN_RE` (21378) | `keyed_between::<T>(…)` (equation `graph_replacing` L529–530; animate `tiles_replacing` L371/386; process3d L999–1000; forms; lowpoly L541), `value_diff_between(` (F-09), `between_rows(`, `page_changes(`, `*_between(` |
| G-6 | R14 variant regex is a closed name list | `POLICY_DIFF_ONLY_RESTORE_RE` (21383): `SetSnapshot\|PatchSnapshot\|ReplaceDocument\|ReplaceSnapshot\|Restore\w*\|(Self\|X)::Snapshot` | `CreateMesh` restoring whole `mesh_content`/`mesh_state` (F-02, lowpoly `🕸️create-mesh` inverse); `FlowDelta::HostSnapshot` (F-12); `SetDocument`/`ReplaceConfig`/`ReplaceState` names (none caught) |
| G-7 | R12 only matches `let mut x = <baseParam>….clone()/to_owned()` in a `diff` fn | regex at 21684 | `let mut mesh = object.mesh_state.clone()…` (F-01), `let mut state = base.to_vec()` (F-03, F-11), `let mut image = …pixels: frame.rgba8.clone()` (F-14), `let mut check = shallow(base)` (draw, L-1), `let mut state = base.clone()` in `DiffAlgebra::inverse` (F-13) |
| G-8 | R11 matches `diff(` or helpers named `*inverse*` whose args mention diff/outcome | `POLICY_DIFF_ONLY_INVERSE_HELPER_RE` (21381), `POLICY_DIFF_ONLY_DIFF_ARGUMENT_RE` (21382) | `negative(base)`, `paint_edits_inverse(&base…, &paint.edits)`, `cascade::inverse`, `lowpoly_selection_motion_inverse` (name matches, args do not mention diff) |
| G-9 | 924 leaf kind files (`🧬️mutations/<kind>/🦀️.rs`) with non-delegating inline `fn diff`/`fn inverse` bodies are scanned by the gate's `mutationImpls` path only, and my scans did not cover them before this pass | 3,015 kind files contain `fn diff(&self`/`fn inverse(&self`; 924 have bodies that do not delegate to `super::diff::`/`super::inverse::` | Inline `let mut … = base…clone()` (29 files), `&mut` non-self (58), `between`-like (9), restore-like variants (22) — see section 4 rows marked "inline" |
| G-10 | Hand-written `impl Mutation<P>` blocks are the only kind of "leaf" that gets the `Mutation` probe | `POLICY_DIFF_ONLY_IMPL_PROBE_RE` (21369) | Derive-generated leaves are checked only through their `🦀️.rs`; any helper reached from them is not |

Design-level gap (D-01): `pub fn apply_diff` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:126`) constructs `ApplyCapability { _sealed: () }` and calls `diff.apply(base, …)` for any caller. Any leaf can call `protocol::apply_diff`. L5 ("impossible by design") is therefore not satisfied; only the gate's R9/R16 text rules prevent it. Fix: make the minting entry `pub(crate)` to the store/vcs crate, or require a store-issued token, so leaves cannot name it.

## 3. Per-Plugin Table

Metrics are file-level pattern hits over `🔺️diff/🦀️.rs` and `↩️inverse/🦀️.rs` plus `🧬️schema/🔺️diff` helper modules (inline kind-file logic is in G-9 and section 4 only). "Gate-blind modules" counts `🧬️schema/🔺️diff/🦀️.rs` files without any `impl … Mutation<`. Verdict key: FAIL = confirmed L1/L2/L4 breach in the plugin; WARN = breach in a diff-type or dormant path; PASS* = passes with an open ruling; PASS = sampled clean; NV = not verified in this pass.

| Plugin | Diff files | Inverse files | Renamed-between hits | Apply/simulate-on-copy hits | Clone→mutate hits | Gate-blind schema modules | Findings | Verdict |
|---|---:|---:|---:|---:|---:|---:|---|---|
| 🗄️stdio | 403 | 229 | 42 | 36 | 53 | 58 | F-09, F-15 | FAIL |
| 📕️norm | 601 | 555 | 8 | 2 | 23 | 16 | AMB-1 | PASS* |
| 🔋️energy | 300 | 297 | 0 | 1 | 1 | 1 | none (guards checked) | PASS |
| 🏛️architect | 269 | 266 | 0 | 0 | 67 | 1 | AMB-1 | PASS* |
| 🧩️puzzle | 122 | 119 | 0 | 16 | 16 | 3 | inverse `🔤️text` oddity | NV |
| 🧱️block | 113 | 104 | 3 | 0 | 3 | 3 | `block_rows_between`, `block_patch_between` reach unknown | NV |
| 🏙️bim | 91 | 88 | 0 | 0 | 1 | 1 | `cascade::inverse` restores deleted records (legit) | PASS |
| 🀄️wfc | 88 | 73 | 0 | 5 | 5 | 5 | change-tile-weight guard consistent | PASS |
| 🏗️fem | 66 | 60 | 2 | 2 | 2 | 2 | `applied`, `inverse_against` chains | NV |
| 🌀️procedural | 45 | 39 | 2 | 2 | 2 | 2 | `applied`; `generation3d_transform_inverse` | NV |
| 📸️remodel | 39 | 36 | 0 | 1 | 3 | 1 | F-11 (`negative` on rows and content) | WARN |
| 🗒️note | 38 | 33 | 0 | 0 | 3 | 2 | F-11 callers | WARN |
| 🎥️shooting | 36 | 31 | 1 | 0 | 3 | 3 | F-11 callers; `between_list` | WARN |
| 🌍️gis | 28 | 20 | 1 | 0 | 7 | 4 | `delta` closure over `Rows::between` (L303) | NV |
| 🖍️draw | 25 | 23 | 1 | 0 | 1 | 1 | L-1 (dormant) | WARN (latent) |
| 💠️lowpoly | 24 | 21 | 1 | 1 | 1 | 1 | F-01, F-02, F-03 | FAIL |
| 📐️cad | 22 | 19 | 1 | 1 | 3 | 1 | between only in trait impl | PASS |
| ➗️mathematical | 21 | 18 | 18 | 1 | 1 | 1 | F-05, F-06, F-07 | FAIL |
| 🏭️process | 18 | 15 | 1 | 0 | 1 | 1 | F-04 | FAIL |
| 📋️forms | 16 | 13 | 1 | 2 | 2 | 1 | F-10 | WARN |
| 🔱️trinity | 16 | 10 | 1 | 0 | 2 | 2 | `between` helpers (L138–227) | NV |
| 🖨️raster | 15 | 12 | 1 | 1 | 1 | 1 | F-14 (paint/fill whole image) | FAIL |
| 🎞️animate | 12 | 9 | 1 | 0 | 1 | 1 | F-08 | FAIL |
| 🪐️space | 11 | 5 | 1 | 0 | 3 | 2 | framework collection between (sync) | NV |
| 🌿️vcs | 9 | 6 | 0 | 0 | 1 | 1 | `VcsTagsDelta::between` (L194) | NV |
| ✒️writer | 8 | 5 | 0 | 0 | 1 | 1 | — | NV |
| 🪵️sourcing | 6 | 3 | 1 | 0 | 2 | 1 | — | NV |
| 📏️layout | 3 | 0 | 1 | 1 | 1 | 1 | `applied` L414–950, `inverse_delta` L831, `page_changes` L1187 | NV (candidate) |
| Small plugins: 📜️imperative, 💡️reasoning, 🎬️sequence, 🌊️flow, 🕸️dag, 📖️playbook, 🎪️demonstrator | 1–4 each | 0–1 | 0 | 0 | 0–1 | 1 each | — | NV |
| 🌎️hub (`🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/`) | 0 leaves | 0 | 0 | 0 | 0 | 0 | 2 hand-written `impl Mutation`: config L254, presence L129; sparse per-arm diffs, absolute inverses | PASS |
| 🧰️framework (diff modules) | 11 | 0 | 0 | 0 | 2 | 2 | F-12, F-13 | FAIL |

## 4. Findings

Confidence: HIGH = code read and law clearly breached; MED = breach depends on a ruling or reach not fully verified; LOW = naming/evasion only.

### F-01 — lowpoly selection motion diff builds a whole mesh image and compares it (HIGH)
- File: `P/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:88–120` (`lowpoly_selection_motion_diff`), L95 and L109.
- Law: L1 (diff is a snapshot difference) and L4 (leaf calls apply). Violation classes V1-SNAPSHOT-DIFF, V3-LEAF-APPLY.
- Evidence:
  ```
  let Ok(mut mesh) = object.mesh_state.clone().expect("present managed mesh").into_mesh() else { … };
  …
  if protocol::apply_diff(&wire, &mut mesh).is_err() { … }
  let state = crate::LowpolyMeshState::from_mesh(mesh);
  if Some(&state) == object.mesh_state.as_ref() { … no-op … }
  ```
- Used by: move-selection, scale-selection, rotate-selection diff leaves (3).
- Gate evasion: G-1 (name), G-3 (file has 0 `impl Mutation<`), G-4 (R9 not in apply dir), G-7 (`clone()` on `mesh_state`).
- Fix: the motion is declarative. Emit sparse per-vertex rows (`vertex id → new position`) computed from base vertex positions and the motion; no clone, no `apply_diff`, no whole-state equality (no-op = empty rows). Mesh handle minting moves to the derived-child rule (AMB-2).

### F-02 — lowpoly selection inverse restores the whole mesh (MED-HIGH)
- File: same as F-01, `lowpoly_selection_motion_inverse` L123–137.
- Law: L2, V2-RESTORE-INVERSE. Evidence: `vec![LowpolyMutation::CreateMesh(CreateMesh { id, child_id, target, mesh_workspace: object.mesh_content.clone(), mesh_state: object.mesh_state.clone() })]` for an inverse of a sparse vertex motion.
- Evasion: G-6 (`CreateMesh` not in R14 list), G-8 (name contains `inverse`, args do not mention diff).
- Ruling read: "replace kinds may invert to the same kind carrying base entity value" applies to replace-mesh only, not to move/scale/rotate kinds.
- Fix: inverse = absolute per-vertex setters restoring base positions of moved vertices (read from base mesh); `create-mesh` only for a replace-mesh kind.

### F-03 — lowpoly paint/patch inverse simulates the forward edit list on a copy (MED-HIGH, diff-type level)
- File: `P/💠️lowpoly/…/🧬️schema/🔺️diff/🦀️.rs`: `paint_edits_inverse` L448–470 (L449 `let mut state = base.to_vec()`, L462 `std::mem::replace(&mut state[…], …)`); `patch_inverse` ≈ L286–302 (restores whole `mesh`, `mesh_content`, `mesh_state` when `mesh_motions` non-empty); `patch_between` L306–311 (`base.diff_patch(other)` whole-object compare).
- Law: L2 (walking the forward list, V2-DIFF-DERIVED-INVERSE) and L3 concern. Gate: G-2 (`DiffAlgebra::inverse`), G-3 (helper module), G-1 (name).
- Note: the leaf inverse for `apply-paint-stroke` uses the per-run `edit-paint-layer` inverse (`🎨️edit-paint-layer/↩️inverse/🦀️.rs`), which reads base pixels per run; that path is compliant.
- Fix: per-edit inverse rows read from base by index, composed with `absorb`; restore only the fields a patch sets.

### F-04 — process3d step timeline diff applies, re-mints and differences (HIGH)
- File: `P/🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs:995–1002` (`process3d_step_timeline_diff`).
- Evidence:
  ```
  let after = keyed_apply(&base.step_payloads, &steps)…;          // apply on base copy
  let mut scene = process_working_scene_from_snapshot(base);
  scene.steps = after;
  let minted = process_working_scene_to_snapshot(&scene, …);      // re-mint whole snapshot
  let tool_solids = keyed_between::<Process3dToolSolidsDelta>(&base.tool_solids, &minted.tool_solids);
  Process3dDiff { steps: (minted.steps != base.steps).then_some(minted.steps), …, tool_solids: … }
  ```
- Law: L1 (V1-SNAPSHOT-DIFF). Gate: G-5 (`keyed_between::<`), G-3 (file at artifact root).
- Callers: 7 leaf diffs (step create/delete/rename/change-step-enabled/change-step-origin/replace-step-measure/reorder-steps).
- Fix: derive `tool_solids` rows from the step-row delta (added/removed step ids → solid rows keyed by step id); keep the minted handle only under AMB-2.

### F-05 — equation derived-state helper copies base, applies, and re-mints (MED)
- Files: `P/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs:262–270` (`equation_state_diff`), `…🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:545–` (`state_after`: `let mut graph = base.graph.clone()`, then keyed_apply on nodes/edges; the diff's sparse slots are applied to the copy).
- Callers: 17 leaf diffs (connect-nodes, create-node, delete-node(s), insert-point, move-points, move-node(s), set-node-positions, set-point-positions, update-graph, change-node, change-graph, replace, remove-point, insert-point, disconnect-nodes, …).
- Law: L1 (after-state computed on a copy), with derived children returned as whole handles. MED because minting content addresses from the after-state is a ruling question (AMB-2).
- Evasion: G-1 (`*_state_diff`), G-3.
- Fix: leaves emit the sparse graph/geometry rows only; derived children are minted by the central applier after `apply_diff`, or the ruling in AMB-2 is adopted explicitly.

### F-06 — equation `replace-graph` differences a whole-graph payload against base (HIGH)
- File: `P/➗️mathematical/…/🕸️graph/🧬️schema/🧬️mutations/🔁️replace-graph/🔺️diff/🦀️.rs:9` (`equation_state_diff(graph_replacing(&base.graph, &payload.graph), base)`), plus `if base.graph == payload.graph` at the top of the same fn.
- Helper: `…✳️any/🧬️schema/🔺️diff/🦀️.rs:528–537` (`graph_replacing`: `keyed_between::<EquationNodesDelta>(&base.nodes, &replacement.nodes)`, `keyed_between::<EquationEdgesDelta>(…)`, scalar `!=`).
- Law: Wave-2 ruling "every `replacement(base, target)` snapshot-differencing helper is deleted"; L1.
- Evasion: G-5 (`graph_replacing`, `keyed_between::<`), G-1.
- Fix: delete `replace-graph`; the user action decomposes into concrete kinds (directed, algorithm, seed, node/edge row kinds), or whole-document replacement becomes the load/genesis path (`Effect::LoadDocument`).

### F-07 — equation `replace` (points) differences a whole point list (HIGH)
- File: `P/➗️mathematical/…/📐️geometry/🧬️schema/🧬️mutations/🔄️replace/🔺️diff/🦀️.rs:9` → `points_replacing(&base.geometry.points, &payload.points)` (helper `…✳️any/🧬️schema/🔺️diff/🦀️.rs:516–526`: prefix/suffix whole-list diff).
- Law: Wave-2 ruling (replacement helper deleted), L1 (whole-list replacement).
- Fix: delete, or express as explicit point-row kinds (set/insert/remove at index) emitted by the user gesture.

### F-08 — animate `replace-tiles` differences a whole tile collection (HIGH)
- File: `P/🎞️animate/🎬️presentation/🧬️schema/🧬️mutations/🔁replace-tiles/🔺️diff/🦀️.rs:12–14`: `if base.tiles == payload.new_tiles { … }` then `diff_set_presentation(base, None, Some(crate::diff::tiles_replacing(&base.tiles, &payload.new_tiles)))`.
- Helper: presentation `🔺️diff/🦀️.rs:386`: `keyed_between::<PresentationTilesDelta>(base, replacement)`.
- Law: Wave-2 ruling; L1. Evasion: G-5, G-1.
- Fix: per-tile kinds (`add-tile`, `remove-tile`, `patch-tile`) emitted by the editor gesture; delete `replace-tiles` or make it a load path.

### F-09 — generic value-tree diff via renamed between (HIGH evasion, MED-HIGH law)
- Helpers: `P/🗄️stdio/…/🧿️semio/…/🔢️value/🧬️schema/🔺️diff/🦀️.rs:374` (`value_diff_between(a: &SemioValue, b: &SemioValue)`, recursive list/object diff via `list_diff_between`), `P/🗄️stdio/…/🧾️json/…/🧬️schema/🔺️diff/🦀️.rs:401` (`value_diff_between(a: &JsonValue, b: &JsonValue)`).
- Leaf callers (not in schema modules): `P/🗄️stdio/…/🧿️semio/…/🧬️mutations/🧷set-node/🔺️diff/🦀️.rs:13`, `…/🗝️set-map-entry/🔺️diff/🦀️.rs:15`, `…/🧾️json/…/🧬️mutations/✏️set-member/🦀️.rs:28` (inline kind file), pdf `🔺️diff/🦀️.rs:1000, 1142, 1571, 2438, 2466, 2483, 2509`, json `🔺️diff/🦀️.rs:165, 436, 455`, semio `🔺️diff/🦀️.rs:180, 412, 432, 456, 511`.
- Law: L1 "never a generic value/JSON patch" (V1-GENERIC-DIFF), and the "renamed between" family the brief asks about. `(?<!\w)between` does not match `value_diff_between(`, so G-5 applies.
- Fix: typed per-field `set-value` kinds per addressed field (one kind per schema field), or a per-artifact edit-rules table in the norm `EDIT_RULES` shape (wave-4 ruling). Generic `value_diff_between` is a seam to delete, not wrap.

### F-10 — forms `change-block-field` copies the question, applies, compares (MED)
- File: `P/📋️forms/…/🎛️change-block-field/🔺️diff/🦀️.rs:16–18`: `let next = payload.change.applied(existing); if &next == existing { no-op }; if let Some(..) = refusal(&next, &payload.change) { fatal }`.
- Law: L1 borderline (copy-apply-compare for no-op and refusal). The emitted diff is sparse (one field), so this is the no-op/refusal path, not the emitted diff.
- Fix: check the single field in the change's own variant (`existing.<field> == new`), and compute refusal from the payload and base reads.

### F-11 — remodel and shooting/note `negative` simulate forward rows on a copy (MED, diff-type level)
- Remodel: `P/📸️remodel/…/📸️remodeling/…/🔺️diff/🦀️.rs:373–406` (rows `negative`: `let mut current = base.to_vec()`, `current.insert/remove`, `current[position]` replace, `member.write_patch(patch)`), `538–570` (content `negative`: `let mut current = base.clone()`). Callers: `DiffAlgebra::inverse` L859–864 (`delta.negative(base)`).
- Shooting: `P/🎥️shooting/…/🔺️diff/🦀️.rs:259` (`negative`), `:361` (`negative_list`).
- Note: `P/🗒️note/…/🔺️diff/🦀️.rs:518, 665, 795, 804`.
- Law: L2 at the diff-type level (walks `self.rows`), V2-DIFF-DERIVED-INVERSE. Gate: G-2 (DiffAlgebra exempt), G-3.
- Fix: row-wise inverse from base reads keyed by id/index (norm `inverse_rows` is the reference shape: reads base by key, emits remove/insert/patch rows).

### F-12 — flow VCS diff-type inverse simulates forward deltas on a projection of base (MED-HIGH, framework)
- File: `F/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🔺️diff/🦀️.rs:59–70` (`FlowDiff::inverse`: `FlowProjection::new(base)`, per delta `projection.inverse_of(delta)` then `projection.apply(delta)`), `:73–75` (`between` = `FlowDelta::HostSnapshot(other.clone())`).
- Law: L2 (derived inverse), L4 spirit (apply inside inverse), and a whole-snapshot delta variant (`FlowDelta::HostSnapshot`) that R14 does not match (G-6). Leaf reach not verified; flagged as diff-type-level.
- Fix: per-delta inverse read from base; remove `HostSnapshot` from the mutation delta vocabulary (load path only).

### F-13 — workflow run diff-type inverse simulates steps on a copy (MED, framework)
- File: `F/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/🔺️diff/🦀️.rs:166–176` (`RunDiff::inverse`: `let mut state = base.clone()`, `negations.push(step.negation(&state))`, `step_into(&mut state, step)`), `between` at `:178–200` (whole header via `RunHeaderEdit::of(other)`, record clones).
- Leaves checked: `🪵️append-run-log` inverse is concrete (`RetractRunLog { count: 1 }`); impact is confined to the diff-type negation.
- Fix: typed per-step inverse rows read from base (the step enum already has a concrete negation per variant; use it without a state copy).

### F-14 — raster paint-stroke and fill-region emit the whole painted image (HIGH)
- File: `P/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-stroke/🦀️.rs`: `paint()` L255–277 (`let mut image = RasterImage { …, pixels: frame.rgba8.clone() }`, `paint_stroke_in_place(&mut image, …)`, `image.pixels == frame.rgba8`, `painted(canvas, image.pixels, …)`); `diff` L294–317 emits `entries.insert(painted.key, Some(painted.asset))` (the whole painted image asset) plus the layer pointer.
- Same pattern in `🪣️fill-region/🦀️.rs:77–86` (`fill_in_place(&mut image, …)`) and `🫗️fill-selection`.
- Law: L1 ("never carries a whole after-snapshot"; whole image is the after-snapshot of the pixel asset). Violation classes V1-GENERIC-DIFF/V1-SNAPSHOT-DIFF. The compliant reference is lowpoly `diff_paint_stroke` (pixel runs).
- Gate: G-1 (`paint`, `painted`, `fill_in_place` helpers), G-7 (clone outside diff fn), G-8.
- Fix: emit pixel runs per stroke dab or fill span (offset + bytes, as lowpoly's stroke edit does), inverse = runs with base bytes read from the base frame.

### F-15 — generic seams in dxf/obj still exist (LOW-MED, ruling)
- Files: `P/🗄️stdio/…/🖋️dxf/…/🔺️diff/🦀️.rs` (`generic_apply` L1454/1592, `generic_inverse` L1460/1598, `generic_between` L1470/1608, `named_inverse` L121/416/546/667/788, `diff_inverse` L353/477/604/725/1268); `P/🗄️stdio/…/🗽️obj/…/🔺️diff/🦀️.rs` (`generic_apply` L320, `generic_inverse` L326, `generic_between` L336, `diff_inverse` L255/380/497/613).
- Law: wave-1/2 ruling "generic seams are deleted, not wrapped". Callers are DiffAlgebra impls (diff-type level) and not confirmed in leaves.
- Fix: concrete per-entity inverse and between in each kind's own directory, or justify the generic as an entity-level helper under a ruling.

### L-1 — draw `compare_layers` copies, applies, compares, falls back to whole-layer replace (LATENT, LOW)
- File: `P/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:542–596` (L588–589: `let mut check = shallow(base); if apply_layer_patch(&mut check, &patch).is_err() || check != shallow(other) { return LayerChange::Replace; }`).
- Reached only from `DrawingLayersDelta::between` (≈ L720), which is sync-only. No leaf caller found. Becomes a violation if a leaf adopts `between`.

### D-01 — `apply_diff` is public and mints the capability for any caller (design, HIGH)
- File: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:119–127` (`ApplyCapability { _sealed: () }`; `pub fn apply_diff` calls `diff.apply(base, ApplyCapability { _sealed: () })`).
- Law L5 not met by design; only the R9/R16 text rules stop leaves. F-01 (`protocol::apply_diff` inside a leaf helper) is proof that the text rules are the only barrier.
- Fix: restrict `apply_diff` (and the struct constructor) to the store/vcs module via `pub(crate)` or a store-issued token.

### Hub (`🌎️hub`) — no violation (PASS)
- `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎚️config/🦀️.rs:254–521` and `…/👥️presence/🦀️.rs:129–181`: hand-written `impl protocol::Mutation<…>`; diffs are per-arm sparse (`(base.x != *v).then(|| v.clone())`, keyed camera rows), inverses are absolute setters from base. No apply, no clones. Note only: hand-written `impl Mutation` bypasses `#[derive(Mutations)]` (V3-HAND-MUTATION risk class, not a current law breach).

## 5. Verified Clean (Checked To Avoid False Positives)

- gltf `rewire` family (`P/🗄️stdio/🗿️artifacts/🧊️gltf/…/🔨️modules/🧬️mutation-support/🗂️top-level-collections/🦀️.rs:174–215`): `rewire(base, family, remap)` reads base references and emits only changed referrers; `delete-node` inverse uses `create_node` with the removed record (legit restore of a deleted entity). Not flagged.
- Workflow `remove-parameter` inverse (`F/🔁️workflow/…/🧬️mutations/🧹remove-parameter/🦀️.rs:21–27`): re-adds the parameter and its bindings (legit delete inverse; the `let mut ops` hit is a false positive).
- `P/🔋️energy/…/↗️change-air-loop-supply`: forward refuses `new_supply_node_id == 0` (fatal, 🔺️diff L~12) and the inverse's guard matches that refusal. Consistent. Energy sampled inverse guards (`change-ground-building`, `create-people-gain`, `change-fault-start`, `connect-surfaces`) are refusal-guarded, not forward-diff-non-empty guarded.
- `P/🀄️wfc/◻️2d/…/⚖️change-tile-weight`: inverse returns empty only when target missing from base; forward returns Error `target-missing` there. Consistent (not V2-EMPTY-INVERSE).
- `P/🗄️stdio/🧿️semio/…/🌐apply-graph`: composite delegation to the subset mutation's diff/inverse, with `_ => Vec::new()`/Error for subset mismatch in both directions. Consistent.
- `P/🏛️architect/…/🏛️program` `between`/`algebra::between` and `P/📐️cad/…` `Rows::between` (lines 593–604): inside `impl DiffAlgebra`, sync-only; no leaf caller found.
- `P/📕️norm/🌍️en1997` `inverse_rows` / `between_rows` / `absorb_rows`: inverse reads base by key; `between_rows` used only from `DiffAlgebra::between`; `absorb_rows` coalesces (removed∘added, patch∘patch) as the wave-2 reference shape.
- `P/🏙️bim/…/🏚️delete-building` inverse via `cascade::inverse` restores deleted records (insert back), not a whole-snapshot restore.
- `P/🏙️bim/…/🎚️set-column-type` inverse: `payload.patch().minimal(record).negate(record)` restores only changed fields from base (absolute setter).
- `P/🖨️raster/…/patch_layer_in_tree` (L299–306, L384, L876): used on the apply path; not a diff finding.
- `P/🗄️stdio/🧿️semio/…/📑️document` `apply_xml_part`, docx/pptx/xlsx `apply_xml_part` (`apply_diff` at docx L888, pptx L733): apply-side only; `part_with_diff_applied` is used by `absorb_opc_diff` (absorb algebra).
- `P/🔋️…` and `P/🏗️fem` `applied` / `inverse_delta`: not verified (see NV rows); not flagged.

## 6. Ambiguities Needing A Ruling (Not Counted As Violations)

- AMB-1 — Ordered-id `order`/`reordered` fields (`Option<Vec<String>>`): built by cloning the base id list and inserting the new id (architect `ℹ️information/🌱️create/🔺️diff/🦀️.rs` L~16–20; norm `insert-*` and `reorder-*` leaves; about 67 architect and 40+ norm files). The norm list-delta reference (wave 2) itself carries `order`, so this is accepted shape; the clone-then-insert construction is the pattern the brief lists. Ruling needed: state that `order` is the reference shape, or express as a typed move/insert index.
- AMB-2 — Derived child/handle re-minting inside leaf diffs (F-01 mesh handle, F-04 `steps`/`tool_solids`, F-05 `notation/results/computed`, `diff_set_presentation` minting). Not in the rulings; L1 forbids copy-based computation. Ruling needed: derived handles are minted only by the central applier from the after-state (`apply_diff` then derive), never inside the leaf.
- AMB-3 — Diff-type `DiffAlgebra::inverse` as the "negative diff" (F-11, F-12, F-13) may be computed by simulation under L3, but L2 names "walking the forward diff" as forbidden. Ruling needed: forbid simulation even at diff-type level; require per-row reads (norm reference).
- AMB-4 — Per-entity field-level `*_diff_between` helpers in stdio `set-*` leaves (obj vertex/normal/texcoord/face, dxf `linetype_diff_between`, ifc/gif/tiff per-entity compares). Sparse field-by-field, so compliant in substance; the evasion is only the name. Ruling: keep and rename into the gate's vocabulary, or make the gate allow per-entity compares.

## 7. Gate Fixes (Recommended Textual Rules)

1. Scan every Rust fn (not only fns named `diff`/`inverse`) in: every file under `🧬️mutations/`, every `🔺️diff/` and `↩️inverse/` dir, every `🧬️schema/🔺️diff/🦀️.rs` helper module, every `🔨️modules/🧬️mutation-support/` module. Drop the `diffTypeImpls` exemption for `inverse`/`between` bodies (keep only `apply` forwarding for `MutationDiff` impls).
2. R10: match `(?<!\bfn\s+)\b\w*between\w*\s*(?:::<[^>]*>)?\(`, `\w*_state_diff\(`, `\w*_replacing\(`, `\w*_delta\(`, `\w*diff_between\(`.
3. R12/R8: treat `\.to_vec\(\)`, `\.to_owned\(\)`, `\.clone\(\)`, `Clone::clone` and `\.iter_mut\(\)` on base-like bindings as a copy when the binding is later mutated (`=`, `.push(`, `.insert(`, `.remove(`, `&mut`), in any fn.
4. R9: `apply_diff(`/`\.apply(` allowed only in the framework apply module and inside `impl MutationDiff::apply` bodies; everywhere else, including `🧬️mutations/🦀️.rs` and schema helpers.
5. R14: flag any inverse-returned payload whose type carries a whole collection or whole record field (`mesh_content`, `mesh_state`, `HostSnapshot`, `Document`, `Config`), plus `CreateMesh`/`ReplaceX` kinds unless the forward kind is a replace kind.
6. R11: flag inverse bodies that call any fn taking the forward payload as an argument and returning a diff-like or whole-state value (`negative`, `paint_edits_inverse`, `inverse_of`, `step.negation`).
7. R13: flag `Diff` struct fields whose type is the snapshot or a whole collection of the same artifact.
8. Make `protocol::apply_diff` visible only to the store/vcs crate (D-01); then R9/R16 become compile-time, not textual.

## 8. Not Verified (Follow-ups)

- Leaf reach of framework diff-types (`FlowDiff::inverse`, `RunDiff::inverse`) and of layout `applied`/`inverse_delta`, fem `applied`/`inverse_against`, procedural `applied`, gis closure `delta`, trinity `between` helpers, block `block_rows_between`, puzzle `🔤️text` inverse, shooting `ShootingScenePatch::between` (L500), vcs `VcsTagsDelta::between` (L194).
- The 924 inline kind-file bodies were pattern-scanned (section 2 G-9, section 4), not read one by one; expect more F-14-class hits in `stdio` (`🎛️sampler`, `🌳️node`, `🕸️mesh`, `🪟️buffer-view` `delete`/`move` leaves) where inline `&mut` and base clones appear next to `fn diff`.
- Individual `🧬️schema/🔺️diff` builders for the ~1,900 non-listed leaves were not read; pattern counts only.
- Gate log is from `🗑️generated/coord/gate-run-3.log`; the gate was not re-run (no bun/cargo per brief).

## 9. Reproduction (Read-Only)

Scripts kept in `🗑️generated/audit-adv/` (`callgraph.py`, `cloneMutate.py`, `enclose.py`, `showfn.py`); outputs were deleted after this report was written. Key commands:

- File lists: `find "✏️s/🔌️plugins" "🧰️framework" -path '*/🔺️diff/🦀️.rs' -not -path '*/🧪️*'` (2,479 diff, 2,111 inverse).
- Gate scope: `rg -n 'const fns = policyRustFns|const diffFns|const inverseFns|if \(!leafCandidate\)|const applyDirectory' 📜️script.ts`.
- Helper-module blindness: `grep -c 'impl.*Mutation\(Kind\)\?\s*<' <file>` on any `🧬️schema/🔺️diff/🦀️.rs` returns 0.
- Inline kind logic: `find … -path '*/🧬️mutations/*' -name '*.rs' -not -path '*/🔺️diff/*' -not -path '*/↩️inverse/*' | xargs grep -l 'fn diff(&self\|fn inverse(&self'` (3,015 files).
