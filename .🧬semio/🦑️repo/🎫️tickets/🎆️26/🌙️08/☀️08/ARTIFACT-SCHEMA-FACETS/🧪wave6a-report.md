# Wave 6a Report — Rust Projection→Snapshot Residue

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Wave **6a** owns **Rust only** (sibling 6b owns TS/TSX).
Repo MCP was **unavailable** this session (`repo://goals` / ticket tools not registered); worked inside the existing ticket folder only.

## 1. Inventory

| | Lines matching `[Pp]rojection` in `*.rs` (excluding `🛢️db`) |
| --- | --- |
| **Before** | **3270** (`🧪wave6a-inventory.txt`) |
| **After** | **2963** (`🧪wave6a-inventory-after.txt`) |
| **Delta** | **−307** |

Key identifier counts (all `*.rs`, excluding ticket/target):

| Identifier | Before → After |
| --- | --- |
| `document_projection_schema` | 99 → **0** |
| `document_snapshot_schema` | (SDK-only) → **102** |
| `PlayProjection` | 154 → **0** |
| `PlaySnapshot` (Value newtype) | 0 → **154** |
| `WorkflowDocument` | 170 → **0** |
| `WorkflowSnapshot` | 0 → **170** |
| `CadProjectionDsl` (camera — kept) | 8 → 8 |

## 2. What changed

### Item 1 — kill the compatibility shim
- Renamed kernel/manifest/host/plugin field `document_projection_schema` → `document_snapshot_schema` across **88** `.rs` files (99 call sites).
- Plugin SDK ↔ kernel mapping in `🔌️plugin/🦀️component.rs` is now a same-name field copy (no spelling translation). Adapter deleted by unification.

### Item 2 — finish `semio-s-plugin-puzzle`
- Folders `🧬️mutations/📄set-document/` → `📄set-snapshot/` (×3 artifacts).
- Mutation variant `SetDocument` → `SetSnapshot` (dsl key `setSnapshot`).
- **Play Value newtypes**: `Puzzle{2,3,5}dPlayProjection` → `Puzzle{2,3,5}dPlaySnapshot` (see judgement #1 — **not** collapsed into schema `PuzzleXdSnapshot`).
- Glue `#[path]` entries updated; 390 lib tests green.

### Item 3 — remaining Rust residue
- `WorkflowDocument` → `WorkflowSnapshot` (+ helpers `empty_workflow_snapshot`, `validate_workflow_snapshot`, …) across workflow kernel, os, host, space plugin.
- Kernel `document_projection: DslValue` → `document_snapshot`.
- `PatchProjection` → `PatchSnapshot` (vcs demo command; document-state patch).
- Fixture-sweep stale `*Projection` imports → correct `*Snapshot` names (`MathematicalSnapshot`, `VcsSnapshot`, …).
- `initial_projection` → `initial_snapshot` in s/plugins (compose kit APIs **reverted** — out of technology scope).
- Infinite-board DAG kernel `DagDocument`/`SetDocument` → `DagSnapshot`/`SetSnapshot`; plugin bridge updated.
- Writer + raster app commands `SetDocument` → `SetSnapshot` (wire keywords updated; tests green).

## 3. Judgement calls

1. **`PuzzleXdPlayProjection` → `PuzzleXdPlaySnapshot`, not `PuzzleXdSnapshot`.**  
   Schema leaves already declare typed `PuzzleXdSnapshot`. Wave-5 report forbids collapsing the play Value newtype into the schema snapshot (Default-field pollution → whole-document `SetSnapshot` fallback). Parent brief asked for `PuzzleXdSnapshot`; following the leaves + wave-5, the Value newtype keeps a distinct `PlaySnapshot` name that removes “Projection”.

2. **`WorkflowDocument` → `WorkflowSnapshot`.**  
   It is the persisted document-state noun for the workflow artifact (same role as every other `XSnapshot`). Renamed for consistency.

3. **`RunDocument` left as-is.**  
   Execution-run state in the run module, not the artifact snapshot facet. Different noun; not renamed.

4. **`CadProjectionDsl` / `setProjection` / `WorldProjection*` left.**  
   3D camera/world projection (category 2). Untouched.

5. **`SetDocumentMutation<D>` in `📕️norm/📄️document` left.**  
   Shared generic bridge; named in `📜️script.ts` policy allowlist (TS = wave-6b). Renaming requires coordinated TS policy edit.

6. **`DagDiff.document` field left** (whole-replace slot).  
   §7.3 prefers `artifact:`; serde/wire change deferred rather than break board pack silently in this wave. Type is already `Option<DagSnapshot>`.

7. **Local parameter names `projection: &mut XSnapshot` and `expect("projection")` left in many plugins.**  
   Type/API names cleaned; thousands of locals are residual wording, not type residue. Not mass-renamed to avoid churn risk.

8. **Compose `hydrate_kit_from_initial_projection_*` reverted.**  
   Accidental substring rename from `initial_projection` → `initial_snapshot`; compose is out of wave scope / already broken at HEAD for other reasons.

## 4. Deliberately left as “projection”

| Kind | Examples | Why |
| --- | --- | --- |
| CQRS read-model | `🛢️db/📽️projection`, `installProjection`, … | Category 1 — disambiguation target of the rename |
| 3D camera/world | `WorldProjectionConfig`, `CadProjectionDsl`, `setProjection`, matrices | Category 2 |
| GIS | `reprojection` | Category 3 |
| Compose kit bootstrap | `*_initial_projection_*` | Unrelated technology; reverted |
| Norm generic | `SetDocumentMutation` | Needs wave-6b policy allowlist update |
| Locals/comments | `let projection = …`, region names like `DefaultProjection` | Wording residue; types already `*Snapshot` |

## 5. Files edited (grouped)

- **Field rename `document_*_schema`:** ~88 `.rs` under `🧰️framework/**`, `✏️s/🔌️plugins/**` window kinds (plus some historical ticket copies touched by the first bulk pass).
- **Puzzle plugin:** `✏️s/🔌️plugins/🧩️puzzle/**` (mutations, apps, engines, spr, glue; folders renamed).
- **Workflow / space:** `🔁️workflow/🦀️component.rs`, os + host re-exports, `🪐️space/**`.
- **VCS patch command:** `🌿️vcs/.../🎮️commands/🧹️patch`, app routing.
- **Infinite DAG port + dag plugin bridge:** `♾️infinite/.../🕸️dag`, `🕸️dag` plugin.
- **Writer + raster app commands:** `✒️writer/**`, `🖨️raster/**` (Rust only; `.ts` stubs in renamed puzzle folders untouched).
- **Fixture-sweep:** `🗣️dsl/✪️fixture-sweep/🦀️component.rs`.
- **Norm:** comment-only fixes under `📄set-snapshot`; `SetDocumentMutation` API kept.
- **Ticket artifacts:** this report + inventories + gate logs.

## 6. Gate tails (verbatim)

### `cargo check -p semio-framework-os-kernel`
```
warning: `semio-framework-os-kernel` (lib) generated 45 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 22 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 1.53s
```

### `cargo check -p semio-framework-plugin`
```
warning: `semio-framework-plugin` (lib) generated 15 warnings (run `cargo fix --lib -p semio-framework-plugin` to apply 15 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 2.75s
```

### `cargo check -p semio-framework-os --features os-host-full`
```
warning: `semio-framework-os` (lib) generated 36 warnings (run `cargo fix --lib -p semio-framework-os` to apply 4 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.34s
```

### `cargo test -p semio-s-plugin-puzzle --lib`
```
test result: ok. 390 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.37s
```

### `bun ./📜️script.ts policy 2>&1 | rg -i 'puzzle'`
```
(empty — no lines matched)
```

### Plugin lib sweep (`🧪wave6a-plugin-sweep.txt`)
```
semio-s-plugin-animate                        test result: ok. 206 passed; 0 failed
semio-s-plugin-architect                      test result: ok. 248 passed; 0 failed
semio-s-plugin-block                          test result: ok. 100 passed; 0 failed
semio-s-plugin-cad                            test result: ok. 124 passed; 0 failed
semio-s-plugin-cad-aec-building               test result: ok. 1 passed; 0 failed
semio-s-plugin-cad-aec-building-energy        test result: ok. 1 passed; 0 failed
semio-s-plugin-cad-aec-building-structure     test result: ok. 1 passed; 0 failed
semio-s-plugin-cad-spatial-shape              test result: ok. 1 passed; 0 failed
semio-s-plugin-dag                            test result: ok. 69 passed; 0 failed
semio-s-plugin-demonstrator                   test result: ok. 16 passed; 0 failed
semio-s-plugin-draw                           test result: ok. 83 passed; 0 failed
semio-s-plugin-draw-fsm                       test result: ok. 26 passed; 0 failed
semio-s-plugin-draw-fsm-macros                test result: ok. 10 passed; 0 failed
semio-s-plugin-energy                         test result: ok. 244 passed; 0 failed
semio-s-plugin-fem                            test result: ok. 332 passed; 0 failed
semio-s-plugin-flow                           test result: ok. 84 passed; 0 failed
semio-s-plugin-flow-extension-bim             test result: ok. 8 passed; 0 failed
semio-s-plugin-flow-extension-brep            test result: ok. 18 passed; 0 failed
semio-s-plugin-flow-extension-dictionary      test result: ok. 5 passed; 0 failed
semio-s-plugin-flow-extension-draw            test result: ok. 42 passed; 0 failed
semio-s-plugin-flow-extension-list            test result: ok. 8 passed; 0 failed
semio-s-plugin-flow-extension-logic           test result: ok. 3 passed; 0 failed
semio-s-plugin-flow-extension-math            test result: ok. 8 passed; 0 failed
semio-s-plugin-flow-extension-primitive       test result: ok. 4 passed; 0 failed
semio-s-plugin-flow-extension-text            test result: ok. 3 passed; 0 failed
semio-s-plugin-forms                          test result: ok. 87 passed; 0 failed
semio-s-plugin-gis                            test result: ok. 144 passed; 0 failed
semio-s-plugin-imperative                     test result: ok. 75 passed; 0 failed
semio-s-plugin-imperative-control             test result: ok. 1 passed; 0 failed
semio-s-plugin-imperative-effect              test result: ok. 3 passed; 0 failed
semio-s-plugin-imperative-logic               test result: ok. 1 passed; 0 failed
semio-s-plugin-imperative-math                test result: ok. 1 passed; 0 failed
semio-s-plugin-imperative-text                test result: ok. 4 passed; 0 failed
semio-s-plugin-layout                         test result: ok. 116 passed; 0 failed
semio-s-plugin-lowpoly                        test result: ok. 139 passed; 0 failed
semio-s-plugin-mathematical                   test result: ok. 52 passed; 0 failed
semio-s-plugin-norm                           test result: ok. 834 passed; 0 failed
semio-s-plugin-note                           test result: ok. 65 passed; 0 failed
semio-s-plugin-playbook                       test result: ok. 49 passed; 0 failed
semio-s-plugin-playbook-procedural            test result: ok. 15 passed; 0 failed
semio-s-plugin-procedural                     test result: ok. 193 passed; 0 failed
semio-s-plugin-process                        test result: ok. 128 passed; 0 failed
semio-s-plugin-process-concrete               test result: ok. 4 passed; 0 failed
semio-s-plugin-process-metal                  test result: ok. 4 passed; 0 failed
semio-s-plugin-process-robotic                test result: ok. 4 passed; 0 failed
semio-s-plugin-process-wood                   test result: ok. 5 passed; 0 failed
semio-s-plugin-puzzle                         test result: ok. 390 passed; 0 failed
semio-s-plugin-raster                         test result: ok. 55 passed; 0 failed
semio-s-plugin-reasoning-mindmap              test result: ok. 58 passed; 0 failed
semio-s-plugin-remodel                        test result: ok. 376 passed; 0 failed
semio-s-plugin-sequence                       test result: ok. 121 passed; 0 failed
semio-s-plugin-shooting                       test result: ok. 95 passed; 0 failed
semio-s-plugin-sourcing                       test result: ok. 64 passed; 0 failed
semio-s-plugin-sourcing-beams                 test result: ok. 1 passed; 0 failed
semio-s-plugin-sourcing-slabs                 test result: ok. 1 passed; 0 failed
semio-s-plugin-sourcing-windows               test result: ok. 1 passed; 0 failed
semio-s-plugin-space                          test result: ok. 87 passed; 0 failed
semio-s-plugin-trinity                        test result: ok. 174 passed; 0 failed
semio-s-plugin-trinity-jack-lsp               test result: ok. 0 passed; 0 failed
semio-s-plugin-trinity-jack-shell             (no lib target — binary-only)
semio-s-plugin-vcs                            test result: ok. 41 passed; 0 failed
semio-s-plugin-writer                         test result: ok. 91 passed; 0 failed
```

**ALL GREEN** — 61 crates with `test result: ok` (≈5124 lib tests in the recorded sweep; writer/raster re-verified after late fixes). `semio-s-plugin-trinity-jack-shell` is binary-only (no lib). `semio-s-plugin-trinity-jack-lsp` reports `0 passed` (empty lib).

## 7. Could not validate / deferred

- Repo MCP ticket open/close/goals — server not available.
- `semio-framework-os-kernel-db` / `semio-compose-rs` — out of scope (already broken at HEAD).
- Wave-6b must update `.ts`/`.tsx` (including puzzle `📄set-snapshot` TS stubs still mentioning set-document in comments, and any `documentProjectionSchema` JS fields).
- Norm `SetDocumentMutation` rename blocked on policy allowlist in `📜️script.ts`.
- Mass rename of local `projection` bindings / `expect("projection")` strings not done.
- `DagDiff.document` → `artifact` serde rename deferred.
