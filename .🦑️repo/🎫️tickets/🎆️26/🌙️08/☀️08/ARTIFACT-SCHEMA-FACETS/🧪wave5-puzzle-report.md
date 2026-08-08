# Wave 5 Report — Puzzle (Artifact Schema Facets)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Wave W5 owns `✏️s/🔌️plugins/🧩️puzzle/**` plus this ticket folder.
Crate: `semio-s-plugin-puzzle`.

## 1. What changed

### Three artifacts × fifteen leaves

| Artifact dir | key | prefix | Snapshot type | Artifact type | Diff type |
| --- | --- | --- | --- | --- | --- |
| `🗿️artifacts/◻2d/` | `puzzle2d` | `Puzzle2d` | `Puzzle2dSnapshot` | `Puzzle2dArtifact` | `Puzzle2dDiff` |
| `🗿️artifacts/🧊️3d/` | `puzzle3d` | `Puzzle3d` | `Puzzle3dSnapshot` | `Puzzle3dArtifact` | `Puzzle3dDiff` |
| `🗿️artifacts/🖐️5d/` | `puzzle5d` | `Puzzle5d` | `Puzzle5dSnapshot` | `Puzzle5dArtifact` | `Puzzle5dDiff` |

Each has 5 formats under `🧬️schema/`, `📸️snapshot/🧬️schema/`, `🔺️diff/🧬️schema/` (json / ts / rs / graphql / proto).

### Pack move

`🎒️pack/` → `📸️snapshot/🎒️pack/` for all three artifacts. TS index + glue paths updated.

### Rename

- Typed document: `PuzzleXdProjection` → `PuzzleXdSnapshot` (no alias).
- Diff whole-replacement field: `document:` → `artifact:` (§7.3).
- Mutation: `SetDocument { snapshot }` (field was `document`).
- SPR golden print strings updated `setDocument document` → `setDocument snapshot` (wire hex unchanged).

### Diff as sparse field delta

Each `PuzzleXdDiff` is a sparse field delta:

- `artifact: Option<Box<PuzzleXdArtifact>>` for whole replacement
- optional entry per non-effect artifact field
- identified collections use `{ added, removed, patched, reordered }`
- optional list UI fields use `PuzzleXdStringList` wrappers where needed (5d `kindCompatibility` uses `Puzzle5dKindCompatibilityList`)
- `MutationDiff<PuzzleXdSnapshot>` applies persistent entries; `apply_to_artifact` applies all state classes
- insert-at-index for new collection items sets `reordered` (lowpoly pattern)
- Value/play bridges field-splice (do **not** round-trip play JSON through typed Snapshot, which used to inject Default empty collections and force `SetDocument` fallback)

### Engines

`type Artifact = PuzzleXdArtifact`, `type Snapshot = PuzzleXdSnapshot`. Engines own real artifacts + cached snapshots.

### Glue

`extern crate semio_framework_schema as artifact_schema`. Leaf-prefixed modules with grouping `#[path = "."]`; nested snapshot/diff keep relative `../../` where needed. Diff runtime `pub use super::schema::*;`.

---

## 2. Play vs persisted classification (critical)

Puzzle has **two parallel document surfaces** per dimension:

| Surface | Type | Role |
| --- | --- | --- |
| **Persisted VCS/DSL/pack document** | `PuzzleXdSnapshot` (was typed `PuzzleXdProjection`) | What engines, pack, DSL, SPR, and sparse diffs operate on. Exactly the `Persistent` fields of `PuzzleXdArtifact`. |
| **Play `DocumentApp::Snapshot`** | `PuzzleXdPlayProjection(pub Value)` | Ad-hoc JSON fixture the play apps still mutate with hundreds of Value helpers. Satisfies `DocumentDsl`/`DocumentPack` via JSON bridges. **Not** the schema snapshot type. |

### Mapping onto facets

- **`XSnapshot`** = persisted document only (schema + domain/meta/collections/camera-as-document-field).
- **`XArtifact`** = Snapshot persistent fields **plus** shared-ui / local-ui / preview (selection, cameras-as-session, grid/lod/locale, hover, json blobs).
- **`XDiff.artifact`** = whole-artifact replacement (replaces old `document:`).

### Per-dimension notes

**2d — camera duality**

- Typed `Puzzle2dSnapshot.camera` is **persistent** (fixture/DSL/pack).
- Play session camera lives in config / local-ui (`camera_x` / `camera_y` / `camera_zoom`). `setCamera` stays `ActionKind::View`.
- Diff includes both persistent `camera` and local-ui camera scalars.

**3d**

- §10 “current” was `Puzzle3dProjection` → `Puzzle3dSnapshot`.
- Play apps use `Puzzle3dPlayProjection` + parallel app-local `Puzzle3dFixture` (duplicate shape vs artifact types). Document delta normalizes replay equality through `Puzzle3dSnapshot` so Fixture↔artifact serde skew does not force `SetDocument`.

**5d — §10 listed `Puzzle5dPlayProjection` as “current”**

- That was the **engine/play confusion**: the 5d engine previously treated PlayProjection as the typed document.
- Corrected: engine/schema use typed `Puzzle5dSnapshot` + `Puzzle5dArtifact`.
- Apps keep `Puzzle5dPlayProjection` as `DocumentApp::Snapshot` only.
- Thin mutations may bridge PlayProjection ↔ Snapshot via serde where needed.

### Why later waves care

Do **not** treat `*PlayProjection` as `XSnapshot`. PlayProjection is a Value newtype for DocumentApp; Snapshot is the facet schema type. Collapsing them reintroduces Default-field pollution and whole-document replace on every edit.

---

## 3. Field inventories

### puzzle2d

**Persistent (= snapshot):** schema, camera, nodes, edges, meta.

**SharedUi:** selected_ids, active_utility_id.

**LocalUi:** camera_x/y/zoom, selection_method, grid_snap_enabled, grid_factor, suggestion_offset, fill_count, brush_candidate_index, brush_candidate_source_handle_id, locale, terminology, lod_mode_by_pane_json, engagement_input_by_pane_json, brush_candidates_json, node_kind_weights_json, handle_kind_weights_json, active_utility_by_window_id_json.

**Preview:** hovered_node_id, preview_seq.

**Diff collections:** `nodes` / `edges` → `{ added, removed, patched, reordered }`; whole replace via `artifact`.

### puzzle3d

**Persistent:** schema, domain, meta, objects, attractions, target_volumes, references.

**SharedUi:** selected_object_ids, selected_vortex_ids, selected_attraction_ids, selected_target_volume_ids, selected_reference_ids, active_utility_id.

**LocalUi:** camera_position_*/target_*/zoom, selection_method, selection_mode_default, engagement_input, grid_visible, grid_snap_enabled, grid_spacing, overlap_budget, fill_count, brush_candidate_index, lod_*, proximity_radius, locale, runtime_extras_json.

**Preview:** hovered_object_id, hovered_vortex_full_id, hovered_kind_id, preview_seq.

**Diff collections:** objects / attractions / target_volumes / references deltas; `artifact` whole replace.

### puzzle5d

**Persistent:** schema, domain, label?, meta, kind_catalogs?, kind_compatibility, parts, fasteners.

**SharedUi:** selected_part_ids, selected_grip_ids, selected_fastener_ids, active_utility_id.

**LocalUi:** camera2d_*, camera3d_*, selection_method, grid_*, suggestion_offset, overlap_budget, fill_count, brush_candidate_index, lod_mode, locale, runtime_extras_json.

**Preview:** hovered_part_id, preview_seq.

**Diff collections:** parts / fasteners deltas; kind_compatibility uses list wrapper; `artifact` whole replace.

---

## 4. Glue convention

Leaf-prefixed modules + grouping `#[path = "."]` (same as lowpoly pilot). Nested `snapshot { schema, pack }` and `diff { schema }`. Snapshot type defined only in `snapshot::schema`; artifact roots re-export `pub use …::snapshot::schema::XSnapshot`.

---

## 5. Gate tails (verbatim)

### cargo check -p semio-s-plugin-puzzle

```
warning: `semio-s-plugin-puzzle` (lib) generated 76 warnings (run `cargo fix --lib -p semio-s-plugin-puzzle` to apply 56 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 2.84s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

### cargo test -p semio-s-plugin-puzzle --lib

```
test result: ok. 390 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.32s
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'puzzle'

```
(empty — no lines matched)
```

Direct `policyArtifactSchemaBreaches()` filter: **puzzle artifact-schema breaches: 0**.

Note: CLI `bun ./📜️script.ts policy` prints only `[DEBUG] runPolicyScript…` lines when stdout is a pipe (shared infra, same as wave4). Breach scan confirmed clean via direct import.

---

## 6. Shared-surface / fixup items

1. **`ViewState` removed from `semio_framework_plugin`** — tests now use `ViewModel` for `render(...)`.
2. **Policy CLI silent when piped** — must call `policyArtifactSchemaBreaches()` directly to verify (framework/script issue, not puzzle-local).
3. **Duplicate play Fixture types vs artifact schema types (3d especially)** — app-local `Puzzle3dFixture` / `Puzzle3dObject` / … parallel artifact types; delta replay must normalize through `Puzzle3dSnapshot` or Fixture/artifact serde skew forces `SetDocument`. Fixup: collapse play Fixture onto artifact schema types.
4. **`new_node_id` was deterministic (`blake3(file!+line!)`)** — broke multi-instance convergence (same id). Fixed to `AtomicU64` counter (matches 3d’s `PUZZLE3D_ID_COUNTER` pattern).
5. **Repo MCP unavailable** this session (`mcp-unavailable.txt` already present); worked inside existing ticket folder only.

---

## 7. Files touched (high level)

**Created:** 15 schema leaves × 3 artifacts; moved packs under `snapshot/`; this report + gate logs in ticket folder.

**Updated:** artifact roots, engines, mutations, diffs, SPR goldens, glue, Cargo.toml (`semio-framework-schema`), TS index, play apps (Snapshot naming, ViewModel, dispatch/`from_action` Option, id generator, delta Value bridges).
