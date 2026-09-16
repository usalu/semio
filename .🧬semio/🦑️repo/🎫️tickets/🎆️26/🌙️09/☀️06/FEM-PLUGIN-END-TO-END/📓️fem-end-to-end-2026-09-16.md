# Fem End To End (2026-09-16)

Goal: `fem2d` (🛠️dev🏗️fem◻️2d⚛️react, 6086) and `fem3d` (🛠️dev🏗️fem🧊️3d⚛️react, 6087) build, test, activate,
serve, boot, render both windows, switch examples and dispatch actions on the React dev lane. Session
a002a5ee (Opus 5) resumed ticket 26/09/06/FEM-PLUGIN-END-TO-END; repo MCP down, bookkeeping on disk.
Running log: [📓️status.md](./📓️status.md). Peers were editing `🔌️plugin/🦀️.rs`, `🏪️store`, stdio brep and
sourcing concurrently; two transient peer breakages (E0609 in brep boolean, an `.await` outside async in the
plugin crate) delayed builds but resolved on their own.

## Outcome

| Gate | Result |
|---|---|
| `@semio-tech/fem-plugin:component-dev` (wasm32-wasip2) | green, 0 fem warnings |
| `fem-plugin:test` / `fem-2d-rs:test` / `fem-3d-rs:test` | 5/5, 900/900 (+1 `long`), 772/772 |
| `@semio-tech/fem-js:test` (vitest) | 6/6 |
| `fem-plugin:describe` | `🔣️.json` + `🛂️.descriptor.semio` re-emitted (wasm 615b29fc…) |
| `activate-fem2d-react-dev` / `activate-fem3d-react-dev` | green (activation 13) |
| fem2d boot (probe 11) | `data-semio-os-ready`, Model = structure layers, Results = static solve + legend |
| fem3d boot (probe 3) | ready, Model = members + meshed solid, Results = static view with caption |
| example switch | `setActiveExample demo` round-trips (LoadDocument → archive load → replacement) |
| action dispatch | `addNode` → history `create-node node=id=n12 x=9m y=1.5m`, both windows update (2d and 3d) |
| result display | `setResultDisplay mode=modal` renders the first mode shape (2d) |
| mounted analysis job (fem2d) | completes: 1112 host steps ≈ 11 s for the demo, visuals published |

Probes and scripts kept in this folder: `🐍️fem-console-dump-probe.mjs`, `🐍️fem-interact-probe.mjs`,
`🐍️fem-result-display-probe.mjs`, `🐍️fem-actions-recon.mjs`, `📜️component-dev-fem.sh`,
`📜️activate-fem-react.sh`, `📜️serve-fem-react.sh`, `📜️test-fem-native.sh`, `📜️nextest-fem-crate.sh`,
`📜️nextest-fem-both.sh`, `🔨️strip-diff-view-state-keys.py`, `🔨️contract-conjunct-harness.py`.
Launch entries added: `fem2d-react`, `fem2d-react-attach`, `fem3d-react`, `fem3d-react-attach`.

## Faults found and fixed (in boot order)

1. **Engine `assemble_system` dropped job payloads** — the synchronous driver matched
   `StepOutcome::Complete/Preview/Checkpoint(_)` and let `RetainedJobPayload` drop with pages
   (`debug_assert` → `unreachable` trap on the first results render). Every outcome is now closed
   page-by-page before drop; `fem2d session::take_retained_payload` no longer `?`-returns past an open
   payload. Same class in the analyses unit tests → `engine_test_vectors::close_outcome`.
2. **fem-3d did not compile without `component-app-assembly`** — `live_visual` mounted unconditionally
   while `semio-framework-ui-scene` is optional → gated.
3. **109 committed `🔺️diff/🔣️.json` fixtures** carried the seven view-state keys the diff structs dropped on
   09-09 (`resultSourceId`…`meshPreviewJson`) → stripped (all null), two "seventeen slots" asserts → ten.
4. **Store round-trip tests** built `ArtifactStore::new` without owners → `install_document_store_owners_exact`
   + exact close loop (space/home pattern).
5. **`SUBSPACE_MAXIMUM_ORDER = 40`** refused any eigenproblem past 40 DOFs — the 2d modal test (52) and the
   fem3d demo buckling (132). The real bound is one 16 KiB page per dense `n × m` owner, already enforced by
   `reserve_matrix_owner`; the cap is now derived (`PAGE / 8 / 9 = 227`).
6. **`AssemblyPreview.assembled_element_ids`** listed every id on every per-element preview (O(n²), a 42 ms
   step at 512 elements) → bounded delta since the previous published preview (`preview_cursor`).
7. **Mesh job test** with 1024 exactly cocircular points faulted (`mesh-edge-index-capacity`, a non-manifold
   Bowyer–Watson cavity from incircle roundoff) and would need >10 M single-unit steps anyway → 256-point
   circle. Robustness on dense cocircular inputs remains an engine gap (see debt).
8. **`add-load` inverse** emitted a `remove-load` for a warned duplicate-id no-op → empty inverse (2d + 3d).
9. **`build_document_store_initialization_job`** was never implemented by either fem editor (trait default
   refuses) → every `Effect::LoadDocument` (example switch) faulted; added the bounded initializer.
10. **Framework: childless archive loads raced the replacement lane** — `drive_store_replacement_jobs` seals
    zero members itself, so `advance_document_archive_load` never saw `AwaitingMembers` and reported
    "initialization failed before retained member admission". The load now proceeds to `AwaitReplacement`
    when the replacement ran ahead with nothing archived to admit (affects every childless artifact:
    writer, wires, rewriting, fem).
11. **fem3d had no `command_from_action` bridge** (default rejects every shell action) → added, 18 actions.
12. **The mounted sessions never started**: their census admits one owner per `prepare_snapshot_read`
    opportunity and the host offers ~2 opportunities per boot → census drains within one opportunity
    (`SESSION_PREFLIGHT_UNITS_PER_OPPORTUNITY = 4096`, both artifacts); fem2d caps (8 nodes / 2 elements /
    4 KiB page) raised to demo scale (48/24 nodes/elements, 80/40 mesh points/triangles, 16 KiB page).
13. **Session step budgets were 8 µs**: `now_us + deadline_ms.min(8)` mixed units, and the job bridges drove
    ONE unit per host `step-job` → `× 1_000` and back-to-back units until the 8 ms ceiling
    (`SESSION_UNITS_PER_STEP`). Demo analysis: minutes → 11 s.
14. **Host never refreshed on job progress** (`requestIsolatedJobUiPoll` had no consumer) →
    `subscribeSpawnedJobProgress` on the plugin handle, coalesced 120 ms, ShellHost full refresh.
15. **Model windows were lease-only and unrenderable on React**: `canvas2d_snapshot_with_page` has zero
    production callers (World3d pages are read only by the wgpu world), and the lease's u64 revision made
    `decodeScenePackValue` refuse the whole doc. Both model windows now render the document structure
    (`fem2d_structure_layers`, `fem3d_scene_parts`) and keep the lease on `scene.snapshot`; wide scene
    integers decode as `bigint`; fem3d's real static/modal/buckling results views are back from `#[cfg(test)]`.
16. **fem's scene helpers bypassed lanes** — the fem3d results scene (meshed solid + stress) exceeded the
    32 KiB surface doc → `scene_surface` (paged meshes/instances lanes).
17. **Artifact one-item footprints declared `work_items: 1`** → every durable mutation failed
    "batched item candidate failed its exact fixed fold contract" → `for_one_invertible_item` (2d + 3d).
18. fem3d initial camera sat inside a column → frames the demo hall; 40 s subspace cancellation test moved to
    the `long` level; stale session-test needles dropped; four fem warnings cleaned.

## Debt left (documented, not chased)

- **fem3d mounted live-visual job** refuses the demo: the mounted assembly's 4 KiB `MOUNTED_OWNER_PAGE_BYTES`
  cannot hold `dof_map.order` for 143 analysis nodes and `MOUNTED_ANALYSIS_NODE_SLOTS`/`MAXIMUM_FIELDS` = 128
  < 143; every owner refusal is reported as `FemError::Singular`. A 64 KiB/256/512 attempt broke four
  page-law tests pinned to 4096 → reverted. Both fem3d windows render synchronously, so the lane is only a
  console `job done status=failed`.
- **`verify interactivity tool-jobs --p6h-only/--p6i-only`** (`toolJobFem*` contracts in `📜️script.ts`, no nx
  target runs them) read fem through `🪆️subsets/✳️any` (repointed to fem's sanctioned `🌐️any`) and pin 60+
  identifiers/test names that no longer exist; `🔨️contract-conjunct-harness.py` enumerates them.
- **Mesher robustness** on dense exactly-cocircular boundaries (1024-gon, radius 100) — non-manifold cavity.
- **`@semio-tech/plugin-registry:check`** fails on a peer collision (playground ports 6019/6119 shared by
  `assembly` and `shooting`), unrelated to fem; fem's descriptor gate has no warning.
- The results windows still solve synchronously inside `render` (v0 design, both artifacts).
