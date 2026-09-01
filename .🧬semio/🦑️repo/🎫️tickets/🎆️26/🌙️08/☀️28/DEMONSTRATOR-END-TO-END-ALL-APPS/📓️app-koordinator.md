# 🧭️ Koordinator (`s.cad.cad@1/*#editor`) — fixture / window / interactivity diagnosis

Plugin: `✏️s/🔌️plugins/📐️cad`. Default example: `hexagonal-cut-concrete-forest-left` (`CAD_EXAMPLE_FOREST_LEFT`).

## 1. Editor and default windows

Editor: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
(`CadPlayApp`, `impl ArtifactEditor` at :1667). Mode `edit` is a quad layout
(`.../✏️editor/🎭️modes/✏️edit/🦀️component.rs:23-49`), registered in `create_cad_app()` (:2031-2035).

| window id | body key | title | surface |
|---|---|---|---|
| `cad-play-shape` | `cad.play.shape` | Shape/Form | `SurfaceKind::World3d` |
| `cad-play-building` | `cad.play.building` | Building | `World3d` |
| `cad-play-energy` | `cad.play.energy` | Energy | `World3d` |
| `cad-play-structure-classic` | `cad.play.structure-classic` | Structure Classic | `World3d` |

## 2. `setActiveExample`

`.../✏️editor/🎮️commands/🗺️model-definition/🦀️component.rs:30-58` has a real branch for
`CAD_EXAMPLE_FOREST_LEFT` (alias `"forest-left"`); any unknown id silently no-ops (`Ok(Emit::default())`).

**But the branch builds a geometry-empty document.** `.../🧬️schema/💡️inferences/🦀️component.rs:975-1021`:

```rust
const FOREST_LEFT_MODEL_JSON: &str = include_str!(".../🔣️hexagonal-cut-concrete-forest-left.model.json");
fn forest_play_document(source_json: &str, id: &str) -> CadSnapshot {
    let _ = source_json;   // fixture JSON is loaded, then explicitly discarded
    CadSnapshot { shape_model: None, building_model: None, energy_model: None,
                  structure_classic_model: None, drawings: Vec::new(),
                  nodes: vec![CadNode { id: "node-root", label: "Concrete Forest Left", kind: "group" }],
                  references_by_model_definition_id: forest_references_for_model_definitions(...), .. }
}
```

A working importer for that asset DOES exist — `cad_document_pane_bundle` / `forest_pane_bundle`
(:911-936) parses `models[].model.objects`/`geometry` into real `CadObject`s with BREP solid handles —
but it is only reached from `forest_working_scene()` (`✏️editor/🦀️component.rs:2173-2190`), which is
**test-only**. `initial_snapshot()` (:1812-1814) also returns `forest_play_scene()`.

## 3. Document → surface: the structural blocker

All four windows render through one function,
`.../✏️editor/🎭️modes/✏️edit/🦀️component.rs:181-190`:

```rust
pub fn build_world_scene_for_pane(envelope: &CadPlayView, pane: CadPaneId, ...) -> ... {
    let objects: &[CadObject] = &[];   // ALWAYS empty, regardless of document content
    MeshWindowKit::render(&MeshView {
        camera_json: ...,
        meshes_json: world_meshes_json(objects, None),
        instances_json: world_instances_json(objects, &envelope.runtime),
        selection_json: ...,
    })
}
```

`world_instances_json(&[], ..)` returns `"[]"` → zero placed instances. `world_references_json`
(:161-179) — which would surface the fixture's background reference image — is real but **never called
from the render path** (only from a unit test at `✏️editor/🦀️component.rs:2886` and the separate viewer
copy). `MeshView` has no field for reference overlays at all.

**Net effect: all four Koordinator windows render an empty 3D scene (camera only) for every example.**
This is one shared render boundary, not four separate per-window bugs.

## 4. Interactivity

**Real and observable:** camera (`setCamera`/`setProjection`/`setProjectionParam`), sun
(`toggleSun`/`setSunAzimuth`/`setSunElevation`/`setSunIntensity`), `renameNode`, `setNodeSelection`,
reference patch/select/hover, `setLocale`/`setTerminology`, `setActiveUtility`/`setDislocateOption`,
and the engagement REPL statechart (`engagementInput`/`Submit`/`PossibleSelect`/`RepeatLast`/`Abort`,
`worldPointerDown`/`worldPointerMove`, `⚙️engine/🕹️interaction/🦀️component.rs`).

**Silent no-ops (`Ok(Emit::default())` / `Vec::new()`, no panic):**
- `🎮️commands/🧱️object/🦀️component.rs:1-101` — `addObject`, `patchObject`, `patchSelection`,
  `deleteObject`, `duplicateObject`: every handler returns `Emit::default()`.
- `🎮️commands/🔄️transform/🦀️component.rs:24-84` — `translateSelection`/`rotateSelection`/`scaleSelection`.
- `✏️editor/🦀️component.rs:457-459` `apply_transformation_mutations` → `Vec::new()`;
  :464-466 `collect_pane_solids`; :586-588 `object_field_mutation`; :638-640 `patch_objects_mutations`.
- `✏️editor/🦀️component.rs:671-701` `try_commit_session_mutations` — interactive draw builds a real
  ephemeral `CadObject` then drops it; only session state is cleared.
- `instance_is_component_hovered` / `gumball_active`
  (`🎭️modes/✏️edit/🦀️component.rs:58-72`) hardcoded `false` → selection highlight and gumball never show.

Save/export (`saveSelected`/`saveInPlay`/`saveCurrent`) dispatch real effects but reuse the empty solid
collectors, so exports contain no geometry.

## 5. Panels

- **Document/Artifact** (`📌️panels/📄️artifact/🦀️component.rs:191-202`): nodes and per-pane references are
  real; every pane's object section is `document_pane_section(..., &[], labels)` — hardcoded empty.
- **Catalogue** (`📌️panels/🛍️catalogue/🦀️component.rs`): static `TYPOLOGY_CATALOG`, not document-derived;
  its `addObject` rows dispatch into the stub handler above.
- **Inspection** (`📌️panels/🔍️inspection/🦀️component.rs:44-57`): fixed schema summary with a hardcoded
  `"Objects: 0"` row; never reflects real selection.

## 6. Root cause

All of the above is self-documented in-repo as a **ticket 26/08/12 UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM
wave-3** gap: object/geometry data was moved out into composed `s.stdio.semio.model` CHILD documents, but
the child-dispatch / child-resolution seam that render, commands and export need was never built. No
`todo!()`/`unimplemented!()` anywhere — every gap is a silent no-op.

## 7. Extensions are not the blocker

`cad-extension-{aec-building,aec-building-energy,aec-building-structure,spatial-shape}` have stale builds,
but the render path never resolves composed child model documents at all (production render bypasses
`cad_working_scene_from_models`, which is used only by tests and DWG import). The windows behave
identically with or without those extensions.

## 8. Fix options (ordered)

1. **Shortest path to a correct fixture:** make `forest_play_document` actually populate
   `shape_model`/`building_model`/`energy_model`/`structure_classic_model` from `FOREST_LEFT_MODEL_JSON`
   via the already-working `cad_document_pane_bundle`/`forest_pane_bundle` importer, and change
   `build_world_scene_for_pane` to read objects from `envelope.document` instead of `&[]`.
2. **Clean long-term solution (repo rules favour this):** land the missing child-dispatch seam so
   `cad_working_scene_from_models` output reaches render, commands and export.
3. Re-enable the object/transform command handlers to write into that resolved content instead of
   `Emit::default()`, and unhardcode `instance_is_component_hovered` / `gumball_active`.
4. Populate the Document panel's per-pane object sections and the Inspection panel from real selection.
