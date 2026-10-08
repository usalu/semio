# 📓️ Coordination — Diff-Only Mutations

## Wave 1 (2026-10-08) — audits (Haiku), all complete
Reports: `🔍️audit-*.md` (framework-spine, framework-os, stdio-semio, stdio-pdf-gltf, stdio-mid, stdio-small, norm-a/b/c,
energy, architect, puzzle-trinity, block-wfc, fem-procedural-layout, seven-mid, small-plugins).
Repo-wide heuristic: 2,887 hand `MutationKind` impls in 96 artifacts; ~1,700 carry a V1–V3 code; 0 leaves had an
inverse-sum (L3) test; `ApplyCapability` absent; 57 whole-state `MutationDiff` impls; 91 hand `impl Mutation<`.

## Wave 2 — executors (Sonnet), launched 2026-10-08
| Label | Scope | Report |
|---|---|---|
| fw-spine | protocol trait, central `apply_diff`, store/plugin/tool folds, derive, law helper | `📓️exec-fw-spine.md` |
| fw-gate | `verify mutation-outcome-law` rules R8–R16 | `📓️exec-fw-gate.md` |
| fw-os-leaves | framework kinds + hand impls (config/print/run/window/presence) | `📓️exec-fw-os-leaves.md` |
| energy | 🔋️energy | `📓️exec-energy.md` |
| architect | 🏛️architect | `📓️exec-architect.md` |
| norm-a / norm-b / norm-c | 📕️norm split (+ `commit_value_tree_edit` owned by norm-a) | `📓️exec-norm-*.md` |
| stdio-semio / stdio-pdf / stdio-gltf / stdio-mid / stdio-small | 🗄️stdio split (`snapshot_patch_leaf!` owned by stdio-small) | `📓️exec-stdio-*.md` |
| puzzle-trinity | 🧩️puzzle 🔱️trinity | `📓️exec-puzzle-trinity.md` |
| block-wfc | 🧱️block 🀄️wfc | `📓️exec-block-wfc.md` |
| draw-cad-gis-raster | 🖍️draw (reference violation) 📐️cad 🌍️gis 🖨️raster | `📓️exec-draw-cad-gis-raster.md` |
| shooting-remodel-note | 🎥️shooting 📸️remodel 🗒️note | `📓️exec-shooting-remodel-note.md` |
| fem-procedural-layout | 🏗️fem 🌀️procedural (kinds) 📏️layout | `📓️exec-fem-procedural-layout.md` |
| small-plugins | 14 small plugins + procedural hand config | `📓️exec-small-plugins.md` |

Build gate: `🚦️gate.sh` (4 slots, < 10 rustc, `CARGO_BUILD_JOBS=3`).

## Events
- 01:45 SPINE-API-LANDED (replication green; `ApplyCapability`, `apply_diff`, `apply_to` deleted).
- ~02:10 fw-gate done: R8–R16 live, baseline 4152 breaches (stdio 1241, norm 938, energy 592, architect 277).
- ~02:15 all 17 other executors cut off by API session limit (reset 04:40); 11:23 all 17 resumed via SendMessage,
  stale gate slots cleared, one hung norm-a heredoc shell killed.
- 12:00–16:20 fleet idle/limited; disk fell to 1.2 GB at worst (peer ticket 26/08/11 CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT
  `🗑️generated` = 540 GB — not touched, dev informed). Builds OOM/disk-killed; nothing compiled.
- 16:30 framework replication + os-kernel GREEN (566 warnings, 0 errors). Shared build-dir stale-unit prune (keep 2, idle > 12 h,
  flock) freed 60.3 GiB + nx cache > 12 h → 75 GB free. Gate reduced to 2 slots. Wave-3 rulings (inverse replay order, no
  feature loss). All 18 executors resumed for verification rounds.
- 17:32 fine-grain lock cycle (stdio-semio + norm-b wasm checks, `prebuild_lock_exclusive`, 20 min, 0 rustc) — killed both;
  gate → 1 slot. 17:35 `🔁️foundation-watch.sh` started (10-min framework check → `🗑️generated/coord/foundation.status`);
  first status RED (peer's 🌱️value retirement refactor breaks `replication/🔗️causal/🔀️transition/🔁️fold`).
- ~17:50 gate run 2: 999 breaches (from 4152): R15 433, rule-2 codes 268, rule-1 outcome 170, R16 46, R14 37, R10 15,
  R8 14, R13 11, R12 3, R9 2. New executor `bim-sequence-flow-hub` for unowned 🏙️bim 🎬️sequence 🌊️flow 🌎️hub space.
  Per-owner burn-down lists sent; frozen-outcome-code ruling added.
- 18:40 pending relays at GREEN resume: stdio-mid ← svg R8/R14; stdio-small ← png/tiff/bmp R8, png/jpg/wav/bmp R10, xml base
  test R16 (list in 📓️exec-stdio-semio.md); stdio-semio ← kit document-contract diffCases JSON, model proto, document
  insert-block nested guard; raster ← switch to framework `assert_mutation_inverse_sum_law_cold` once fw-spine adds it;
  brep/model editors use `bounded_config_store_one_item_preparation_factory_birth_bytes` root export (plugin API moved).
- ~19:00 gate run 3: 149 breaches (framework now 0 after fw-os-leaves 3b). fw-os-leaves found its RestoreN rename reverted
  on disk (reflog: auto-sync fast-forward merge + reset) — re-verify rule added. Rulings: ephemeral roots, translators
  deleted, host-scene reconciliation = translator. New executors puzzle2d-editor, puzzle-3d5d-host; Haiku translator audit.
- ~19:20 translator audit (`🔍️audit-translators.md`): 15 translator families (stdio snapshot-edit lane 88 editors + 38
  net_mutations; raster, wfc, remodel, block ×6, procedural, flow, vcs, gis, playbook), 13 whole-doc kinds, framework
  `whole_document_operation`. Wave-4 rulings added; each owner sent its rows. stdio-small owns the shared edit-rules
  resolver (others implement per-artifact tables after its "Edit-rules API" lands).
- W6 withdrawn: architect `📃️document` is the program's artifact-record register (entity replace by id), not the whole artifact; exchange/example already use the load effect.
- Trade-off for dev review: block 2d/3d/5d whole-document JSON text `edit` is now a load effect (no addressed change exists in its payload), so it is not undoable as a history row; a field-addressed text gesture would restore undo.
- ~19:50 whole fleet cut by API session limit (reset 21:20); session restarted. 22:05 disk 3 GB free → stale-unit prune
  (keep 2, idle > 6 h, flock) + nx > 6 h + idle incremental → 18 GB. Shared cache ≈ 90 GB total; the rest of the disk is
  other sessions (peer ticket 26/08/11 `🗑️generated` 540 GB, untouched per dev). Foundation watcher restarted.
- Dev cap: ≤ 10 agents at once. Wave A (10): fw-spine, stdio-small (edit-rules API first), puzzle2d-editor,
  puzzle-3d5d-host, bim-sequence-flow-hub, small-plugins, draw-cad-gis-raster, fem-procedural-layout,
  shooting-remodel-note, norm-b. Queue: stdio-semio, stdio-pdf, stdio-gltf, stdio-mid (need stdio-small's API),
  puzzle-trinity (wire guard + verify), energy/architect/norm-a/norm-c/block-wfc/fw-os-leaves verification at GREEN.
- 22:30 adversarial audit (`🔍️audit-adversarial.md`): gate blind spots G-1…G-10, FAIL plugins stdio (F-09/F-15),
  lowpoly (F-01…03), mathematical (F-05…07), process3d (F-04), raster (F-14), animate (F-08), framework (F-12/13);
  WARN remodel/shooting/note (F-11), forms (F-10), draw latent (L-1); rulings wave 5 (AMB-1 positional rows, AMB-2 derived
  data in central apply, AMB-3 negative diffs read base, D-01 gate-enforced). Disk: peer ticket 26/08/11 `🗑️generated`
  grew 540 → 584 GB (still writing); 9.3 GB free at 22:22.
- QUEUE (start as slots free, ≤ 10): stdio-pdf (edit-rules + F-09), stdio-gltf (edit-rules), draw-cad-gis-raster (F-14 raster
  paint whole image, L-1 draw compare_layers, gis `Rows::between` delta, patchPositions), norm-a/norm-c (AMB-1 list-delta
  positional rows), architect (AMB-1), block-wfc (`block_rows_between`/`block_patch_between` reach), fem-procedural-layout
  (`applied`, `inverse_against`, layout `inverse_delta`/`page_changes`), puzzle-trinity (wire guard, `between` helpers in
  trinity L138–227), then verification rounds once foundation GREEN.
- ~23:10 gate run 4 (blind spots G-1…G-10 closed, 41 self-tests): 449 breaches (437 diff-only: R8 142, R9 88, R10 68,
  R11 5, R12 119, R14 15; R15/R16 0). stdio 293 (semio 74, pdf 33, dxf 20, docx 20, …), framework 25, norm 16.
  Each running owner sent its slice; framework → fw-spine wave 9. Still queued: energy (6), architect (4), block-wfc
  (bitmap 5), layout (5), puzzle (2d/3d/5d 3 each), norm-c, puzzle-trinity (wire guard), verification rounds at GREEN.
- ~23:30 stdio-semio ran one forbidden `git rm --cached` on an already-deleted net file (index-only); dev informed. DiffAlgebra::between deletion ruled; executor between-removal launched.
- PENDING (next free slot): fw-gate — extend R13/R14 ephemeral exemption to window-transient lanes by type (`*TransientMutation`/`*PresenceMutation`), self-test, gate-run-5. (Stopped once at 11 running to respect the ≤10 cap.)
- ~23:50 framework `protocol::list_delta` landed (API: 📓️exec-fw-spine.md 'List-delta API'); norm + puzzle migrated, copies deleted. Remaining copies to migrate: stdio contract (stdio-small), gis + cad (draw-cad-gis-raster), architect algebra, stdio-mid docx/pptx/xlsx, gltf, energy splice (verify), any others found by rg. Gate: 27 breaches.
- ~00:20 gate 11 breaches (pdf 10, layout 1). Trade-off for dev review: stdio html `textEdit` (window shows the DSL envelope = whole-snapshot dump) is now a load, not undoable — same class as block's whole-JSON `edit`. stdio-small edited framework `list_delta` (`Keyed` for (K,V), `key = keyed` macro arm) — recheck duplicates at compile.
- ~00:40 gate: 1 breach (layout R12). Deferred to compile round: pdf → protocol::list_delta (keyed-lane removals need base index; pages index lanes already positional); wfc keyed rows migrating; block/grid2d/energy/raster/lowpoly fixtures need regeneration by Rust test runs.
