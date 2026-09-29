"""📦️ S20 window-3 prepared patch: CAD's current/in-play/modelspace exports write the pane's real solids.

`collect_pane_solids` was a documented stub (`Vec::new()`: "no live per-pane object list on CadSnapshot"), so
`saveCurrent format=step|obj|stl` always fell back to an empty spatial DSL (`cad.current.spatial.dsl`, `objects: []`,
io-matrix 27 19:5x) and the `brep` media port refused "no solids to export". The seam exists in-process: every pane's
composed model child carries its materialized `CadWorkingScene` (`cad_pane_local_scene` — the child's `local_owner`, or the
bundled catalogue for a wire-loaded example), which is exactly what the viewport renders (`edit::cad_pane_working_scene` +
`cad_pane_working_objects`). The patch builds each visible object's solid the way the viewport builds its mesh
(`object_mesh_data`: the host-snapshot primitive the object names, moved onto the primitives' centroid, else the typology
primitive — the size rule now shared with `typology_brep_mesh` through `typology_local_solid`) and places it with the node
transform of its instance (`world_instances_json`: scale, rotation quaternion `[x, y, z, w]`, translation to `origin`).
An empty pane's `saveCurrent` is refused (`cad.export.empty-pane`) instead of writing a spatial DSL under a STEP/OBJ/STL
request. Law: `current_pane_exports_its_real_solids` (forest Building pane → STEP/OBJ/STL files).

Usage: python3 s20-patch-cad-solids.py [--dry-run]   (idempotent)
"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
DRY = "--dry-run" in sys.argv
BASE = ROOT / "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any"
INFERENCES = BASE / "🧬️schema/💡️inferences/🦀️.rs"
EDITOR = BASE / "✏️editor/🦀️.rs"
IO_COMMANDS = BASE / "✏️editor/🎮️commands/📥️io/🦀️.rs"
TESTS = BASE / "✏️editor/🧪️tests/🔬️unit/🦀️.rs"

SCENE_USE_OLD = "        centroid_from_host_snapshot_primitives, objects_from_host_snapshot_model, parse_geometry, semio_model_snapshot_from_objects, tessellate_object_mesh, tessellate_object_mesh_from_host_snapshot, CadGeometry, CadObject, CadPrimitiveSlot,\n"
SCENE_USE_NEW = "        centroid_from_host_snapshot_primitives, import_geometry_handles, objects_from_host_snapshot_model, parse_geometry, resolve_primitive_handle, semio_model_snapshot_from_objects, tessellate_geometry_handle, tessellate_object_mesh, tessellate_object_mesh_from_host_snapshot, CadGeometry, CadObject,\n        CadPrimitiveSlot,\n"

MESH_OLD = """        let [ex, ey, ez] = extent.unwrap_or(CAD_DEFAULT_TYPOLOGY_EXTENT);
        let (width, depth, height) = (ex.max(0.05), ey.max(0.05), ez.max(0.05));
        let handle = match typology_mesh_kind(typology) {
            "cylinder" => kernel.cylinder_prim(width.max(depth) * 0.5, height),
            "sphere" => kernel.sphere_prim(width.max(depth).max(height) * 0.5),
            _ => kernel.box_prim(width, depth, height),
        };
        let Ok(handle) = handle else {
            return mesh_from_kind(typology_mesh_kind(typology));
        };
"""
MESH_NEW = """        let Some(handle) = typology_local_solid(&mut kernel, typology, extent) else {
            return mesh_from_kind(typology_mesh_kind(typology));
        };
"""

SOLIDS_ANCHOR = "    fn mesh_centroid(mesh: &MeshData) -> Option<[f32; 3]> {\n"
SOLIDS_NEW = """    /// @emoji 🧊️ A typology's primitive as a local kernel solid, sized from authored geometry (or the universal fallback
    /// extent) — the one size rule [`typology_brep_mesh`] tessellates and [`pane_world_solids`] exports.
    fn typology_local_solid(kernel: &mut Brep, typology: &str, extent: Option<[f64; 3]>) -> Option<GeometryHandle> {
        let [ex, ey, ez] = extent.unwrap_or(CAD_DEFAULT_TYPOLOGY_EXTENT);
        let (width, depth, height) = (ex.max(0.05), ey.max(0.05), ez.max(0.05));
        match typology_mesh_kind(typology) {
            "cylinder" => kernel.cylinder_prim(width.max(depth) * 0.5, height),
            "sphere" => kernel.sphere_prim(width.max(depth).max(height) * 0.5),
            _ => kernel.box_prim(width, depth, height),
        }
        .ok()
    }

    /// @emoji 🌍️ A pane's visible objects as WORLD-space kernel solids, built the way the viewport builds their meshes
    /// ([`object_mesh_data`]): the host-snapshot primitive an object names (moved onto the primitives' centroid, as
    /// [`align_mesh_to_host_snapshot_centroid`] moves the mesh), else its typology primitive — then placed by its
    /// instance's node transform (`world_instances_json`). What a STEP/OBJ/STL export writes is what the pane shows.
    pub(crate) fn pane_world_solids(kernel: &mut Brep, objects: &[CadObject], geometry: Option<&CadGeometry>) -> Vec<GeometryHandle> {
        let handles = geometry.map(|geometry| import_geometry_handles(kernel, geometry)).unwrap_or_default();
        objects
            .iter()
            .filter(|object| object.visible)
            .filter_map(|object| {
                let local = geometry.filter(|_| !object.primitives.is_empty()).and_then(|geometry| host_snapshot_local_solid(kernel, object, geometry, &handles)).or_else(|| typology_local_solid(kernel, &object.typology, object.extent))?;
                place_object_solid(kernel, local, object)
            })
            .collect()
    }

    /// @emoji 🧲️ The imported host-snapshot primitive an object names, moved onto the primitives' own centroid when it
    /// is more than 5 cm away — the kernel twin of [`align_mesh_to_host_snapshot_centroid`].
    fn host_snapshot_local_solid(kernel: &mut Brep, object: &CadObject, geometry: &CadGeometry, handles: &std::collections::HashMap<String, String>) -> Option<GeometryHandle> {
        let (handle_id, kind) = resolve_primitive_handle(&object.primitives, handles)?;
        let handle = GeometryHandle(handle_id.clone());
        let Some(target) = centroid_from_host_snapshot_primitives(geometry, &object.primitives) else {
            return Some(handle);
        };
        let current = tessellate_geometry_handle(kernel, &handle_id, &kind).as_ref().and_then(mesh_centroid)?;
        let delta = [target[0] - f64::from(current[0]), target[1] - f64::from(current[1]), target[2] - f64::from(current[2])];
        if delta[0].abs() + delta[1].abs() + delta[2].abs() > 0.05 {
            kernel.translate(&handle, delta).ok()
        } else {
            Some(handle)
        }
    }

    /// @emoji 📍️ An object's node transform applied to its local solid: scale about the local origin, rotation by its
    /// unit quaternion `[x, y, z, w]`, translation to `origin` — the order the viewport composes an instance.
    fn place_object_solid(kernel: &mut Brep, local: GeometryHandle, object: &CadObject) -> Option<GeometryHandle> {
        let scale = object.scale.unwrap_or([1.0, 1.0, 1.0]);
        let scaled = if scale == [1.0, 1.0, 1.0] { local } else { kernel.scale_axes(&local, scale, [0.0, 0.0, 0.0]).ok()? };
        let [x, y, z, w] = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let sine = (x * x + y * y + z * z).sqrt();
        let rotated = if sine <= f64::EPSILON { scaled } else { kernel.rotate(&scaled, [x / sine, y / sine, z / sine], 2.0 * sine.atan2(w)).ok()? };
        kernel.translate(&rotated, object.origin).ok()
    }

"""

COLLECT_OLD = """/// ⚠️ Same documented gap as `apply_transformation_mutations` — there is no live per-pane object
/// list on `CadSnapshot` to collect solids from anymore (only composed model-child HANDLES,
/// unresolved at this boundary).
pub fn collect_pane_solids(_kernel: &mut Brep, _envelope: &CadPlayView, _pane: CadPaneId) -> Vec<GeometryHandle> {
    Vec::new()
}
"""
COLLECT_NEW = """/// 📦️ The pane's visible objects as world-space kernel solids, read through the composed pane-model child's in-process
/// seam (`edit::cad_pane_working_scene`: the child's materialized working scene, or the bundled catalogue for a
/// wire-loaded example) and built as the viewport builds them
/// ([`crate::standards::v1::subsets::any::schema::inferences::pane_world_solids`]).
pub fn collect_pane_solids(kernel: &mut Brep, envelope: &CadPlayView, pane: CadPaneId) -> Vec<GeometryHandle> {
    let Some(scene) = crate::editor::cad::modes::edit::cad_pane_working_scene(&envelope.document, pane) else {
        return Vec::new();
    };
    let (objects, geometry) = crate::editor::cad::modes::edit::cad_pane_working_objects(&scene, pane);
    crate::standards::v1::subsets::any::schema::inferences::pane_world_solids(kernel, objects, geometry)
}
"""

SAVE_OLD = """        let effect = match export_solid_for_pane(&view, pane, format) {
            Some(export) => cad_solid_export_effect(export),
            None => cad_spatial_export_effect(&export_spatial_json(&view, "current", Some(pane))?, "cad.current.spatial.dsl"),
        };
        Ok(Emit::effect(effect))
"""
SAVE_NEW = """        let export = export_solid_for_pane(&view, pane, format).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("cad.export.empty-pane"), "The current pane has no solid to export."))?;
        Ok(Emit::effect(cad_solid_export_effect(export)))
"""

LAW = """
//#region 🔖️PaneSolidExport
/// ⚖️ A pane's current export writes the pane's real geometry: the forest's Building pane has world-space solids and
/// exports as STEP, OBJ and STL files — never the empty spatial fallback (io-matrix 27 19:5x measured
/// `cad.current.spatial.dsl` with `objects: []` for all three). The files' parsers (ISO 10303-21 structure, three.js
/// OBJ/STL loaders) judge them live in `verify io`.
#[semio_framework_async_macros::async_test]
async fn current_pane_exports_its_real_solids() {
    use crate::standards::v1::subsets::any::io::{CAD_SOLID_EXPORT_DIALECT_OBJ, CAD_SOLID_EXPORT_DIALECT_STL};
    let view = forest_view();
    let mut kernel = cad_brep_kernel();
    assert!(!collect_pane_solids(&mut kernel, &view, CadPaneId::Building).is_empty(), "the Building pane has solids");
    for (format, extension) in [(CAD_SOLID_EXPORT_DIALECT_STEP, ".step"), (CAD_SOLID_EXPORT_DIALECT_OBJ, ".obj"), (CAD_SOLID_EXPORT_DIALECT_STL, ".stl")] {
        let export = export_solid_for_pane(&view, CadPaneId::Building, format).expect("a solid export");
        assert!(export.filename.ends_with(extension), "{} names its format", export.filename);
    }
}
//#endregion 🔖️PaneSolidExport
"""



#: 🏁️ Set-level landing markers `(repo path, text)` — `None` = the set deletes that file. All present → the set is
#: landed and nothing is applied (per-hunk checks alone cannot see an insert whose text a later codemod reworded).
LANDED = [('✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs', 'fn current_pane_exports_its_real_solids')]


def landed_guard() -> bool:
    """🏁️ True when every landing marker is in the tree; a partial landing is a conflict, never a second write."""
    tree = Path("/Users/ueli/Documents/semio")
    present = [(not (tree / rel).exists()) if marker is None else ((tree / rel).exists() and marker in (tree / rel).read_text()) for rel, marker in LANDED]
    if all(present):
        print("landed: every set marker is in the tree — nothing to apply")
        return True
    if any(present):
        raise SystemExit(f"CONFLICT: set partially landed (markers {present}) — nothing written")
    return False


def main() -> None:
    if landed_guard():
        return
    edits = [
        (INFERENCES, "scene_compute imports", SCENE_USE_OLD, SCENE_USE_NEW),
        (INFERENCES, "typology_brep_mesh shares the size rule", MESH_OLD, MESH_NEW),
        (INFERENCES, "world solids", SOLIDS_ANCHOR, SOLIDS_NEW + SOLIDS_ANCHOR),
        (EDITOR, "collect_pane_solids", COLLECT_OLD, COLLECT_NEW),
        (IO_COMMANDS, "saveCurrent refuses an empty pane", SAVE_OLD, SAVE_NEW),
    ]
    texts = {path: path.read_text() for path in {INFERENCES, EDITOR, IO_COMMANDS, TESTS}}
    notes = []
    for path, name, old, new in edits:
        text = texts[path]
        if text.count(old) == 1 and new not in text:
            texts[path] = text.replace(old, new)
            notes.append(f"apply     {name}")
        elif new in text:
            notes.append(f"applied   {name}")
        else:
            notes.append(f"CONFLICT  {name}: anchor found {text.count(old)}×")
    if "fn current_pane_exports_its_real_solids" in texts[TESTS]:
        notes.append("applied   law")
    else:
        texts[TESTS] = texts[TESTS].rstrip("\n") + "\n" + LAW
        notes.append("apply     law")
    print("\n".join(notes))
    if any(line.startswith("CONFLICT") for line in notes):
        raise SystemExit("conflict: nothing written")
    if not DRY:
        for path, text in texts.items():
            path.write_text(text)
    print("dry-run" if DRY else "applied")


if __name__ == "__main__":
    main()
