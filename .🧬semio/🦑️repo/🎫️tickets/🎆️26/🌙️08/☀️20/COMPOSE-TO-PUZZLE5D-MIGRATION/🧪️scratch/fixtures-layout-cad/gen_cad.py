#!/usr/bin/env python3
"""Emit the 20 handcrafted cad mutation fixtures."""
import copy, json, os, pathlib, re

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
MUT = ROOT / "✏️s/\U0001f50c️plugins/\U0001f4d0️cad/\U0001f5ff️artifacts/\U0001f4d0️cad/\U0001f3c5️standards/\U0001f516️1/\U0001fa86️subsets/✳️any/\U0001f9ec️schema/\U0001f9ec️mutations"

def child(child_id, artifact_id, subset):
    return {"childId": child_id,
            "target": {"artifactId": artifact_id,
                       "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": subset}}}

REF_1 = {"id": "ref-1", "sourceUrl": "https://example.test/plan.png", "mediaKind": "image",
         "origin": [0.0, 0.0, 0.0], "orientation": None, "scale": 1.5, "widthWorld": 8.0,
         "hidden": False, "locked": True, "opacity": 0.5}
REF_2 = {"id": "ref-2", "sourceUrl": "https://example.test/site.png", "mediaKind": "image",
         "origin": [5.0, 0.0, 0.0], "orientation": None, "scale": None, "widthWorld": 16.0,
         "hidden": False, "locked": False, "opacity": None}

NODE_3 = {"id": "node-3", "label": "Column", "kind": "solid"}

BASE = {
    "schema": "cad.document",
    "id": "cad-fixture",
    "shapeModel": child("shape-model-1", "cad-shape-1", "model"),
    "buildingModel": child("building-model-1", "cad-building-1", "model"),
    "energyModel": child("energy-model-1", "cad-energy-1", "model"),
    "structureClassicModel": child("structure-classic-model-1", "cad-structure-1", "model"),
    "drawings": [child("drawing-1", "cad-drawing-1", "drawing")],
    "referencesByModelDefinitionId": {"spatial.shape": [REF_1]},
    "nodes": [
        {"id": "node-1", "label": "Root", "kind": "group"},
        {"id": "node-2", "label": "Base Plate", "kind": "solid"},
    ],
    "activeModelDefinitionId": "spatial.shape",
}

APPLIED = {"status": "applied"}

def base():
    return copy.deepcopy(BASE)

#region transforms — hand-applied, one per mutation, mirroring exactly what its diff builder emits
def t_create_shape_model(s):
    s["shapeModel"] = child("shape-model-2", "cad-shape-2", "model")

def t_delete_shape_model(s):
    del s["shapeModel"]

def t_create_building_model(s):
    s["buildingModel"] = child("building-model-2", "cad-building-2", "model")

def t_delete_building_model(s):
    del s["buildingModel"]

def t_create_energy_model(s):
    s["energyModel"] = child("energy-model-2", "cad-energy-2", "model")

def t_delete_energy_model(s):
    del s["energyModel"]

def t_create_structure_classic_model(s):
    s["structureClassicModel"] = child("structure-classic-model-2", "cad-structure-2", "model")

def t_delete_structure_classic_model(s):
    del s["structureClassicModel"]

def t_create_drawing(s):
    s["drawings"].append(child("drawing-2", "cad-drawing-2", "drawing"))

def t_delete_drawing(s):
    s["drawings"] = [c for c in s["drawings"] if c["childId"] != "drawing-1"]

def t_create_node(s):
    s["nodes"].append(copy.deepcopy(NODE_3))

def t_delete_node(s):
    s["nodes"] = [n for n in s["nodes"] if n["id"] != "node-2"]

def t_rename_node(s):
    s["nodes"][0]["label"] = "Assembly Root"

def t_change_reference_hidden(s):
    s["referencesByModelDefinitionId"]["spatial.shape"][0]["hidden"] = True

def t_change_reference_locked(s):
    s["referencesByModelDefinitionId"]["spatial.shape"][0]["locked"] = False

def t_change_reference_width(s):
    s["referencesByModelDefinitionId"]["spatial.shape"][0]["widthWorld"] = 12.0

def t_move_reference(s):
    s["referencesByModelDefinitionId"]["spatial.shape"][0]["origin"] = [1.0, 2.0, 3.0]

def t_replace_reference_media(s):
    ref = s["referencesByModelDefinitionId"]["spatial.shape"][0]
    ref["sourceUrl"] = "https://example.test/plan-v2.png"
    ref["mediaKind"] = "drawing"
    ref["scale"] = 2.0
    ref["opacity"] = 0.25

def t_replace_references(s):
    s["referencesByModelDefinitionId"]["spatial.shape"] = [copy.deepcopy(REF_2)]

def t_change_active_model_definition(s):
    s["activeModelDefinitionId"] = "aec.building"
#endregion

def slot_case(slug, verb, field, snake, human, new_child, new_artifact, old_child, old_artifact, others, transform, mutation, case, mod, fn1, fn1doc, fn2, fn2doc, blurb, change, inverse, probe):
    return dict(leaf=slug, kind=slug, case=case, mod=mod, transform=transform, mutation=mutation,
                blurb=blurb, fn1=fn1, fn1doc=fn1doc, change=change, fn2=fn2, fn2doc=fn2doc,
                inverse=inverse, probe=probe)

CASES = []

#region the four fixed model slots — create OVERWRITES an occupied slot, delete VACATES it
SLOTS = [
    ("shape-model", "shape_model", "ShapeModel", "shape", "cad-shape-2", "shape-model-2", "cad-shape-1", "shape-model-1",
     ["building_model", "energy_model", "structure_classic_model"], t_create_shape_model, t_delete_shape_model),
    ("building-model", "building_model", "BuildingModel", "building", "cad-building-2", "building-model-2", "cad-building-1", "building-model-1",
     ["shape_model", "energy_model", "structure_classic_model"], t_create_building_model, t_delete_building_model),
    ("energy-model", "energy_model", "EnergyModel", "energy", "cad-energy-2", "energy-model-2", "cad-energy-1", "energy-model-1",
     ["shape_model", "building_model", "structure_classic_model"], t_create_energy_model, t_delete_energy_model),
    ("structure-classic-model", "structure_classic_model", "StructureClassicModel", "structure-classic", "cad-structure-2", "structure-classic-model-2", "cad-structure-1", "structure-classic-model-1",
     ["shape_model", "building_model", "energy_model"], t_create_structure_classic_model, t_delete_structure_classic_model),
]

for (slug, field, variant, pane, new_artifact, new_child, old_artifact, old_child, others, t_create, t_delete) in SLOTS:
    new_uri = f"{new_artifact}!s.stdio.semio@v1/model"
    old_uri = f"{old_artifact}!s.stdio.semio@v1/model"
    other_asserts = " && ".join(f"after.{o}.as_ref().map(|c| c.child_id.as_str()) == Some(\"{o.replace('_', '-')}-1\")" for o in others)
    CASES.append(dict(
        leaf=f"create-{slug}", kind=f"create-{slug}", case=f"rehandles-the-occupied-{pane}-slot",
        mod=f"tests_rehandles_the_occupied_{pane.replace('-', '_')}_slot", transform=t_create,
        mutation={"mutation": "create" + variant, "childId": new_child, "target": new_uri},
        blurb=f"Proves `create-{slug}` OVERWRITES an already-occupied fixed slot and that undo restores the displaced handle — it is not an insert-if-absent.",
        fn1=f"replaces_the_{pane.replace('-', '_')}_handle_in_place",
        fn1doc=f"▶️ `create-{slug}` writes the fixed `{field}` slot even when it is already occupied; the other three slots never move.",
        change=f"""    let handle = after.{field}.as_ref().expect("create-{slug} leaves the slot occupied");
    assert_eq!(handle.child_id, "{new_child}", "create-{slug} must install the payload's child id");
    assert_eq!(handle.target.to_uri(), "{new_uri}", "create-{slug} must parse the payload target URI back into a real ArtifactRef");
    assert!({other_asserts}, "create-{slug} must leave the other three fixed model slots untouched");
    assert_eq!(after.drawings.len(), 1, "create-{slug} must not touch the drawings child collection");""",
        fn2=f"inverse_reinstalls_the_displaced_{pane.replace('-', '_')}_handle",
        fn2doc=f"↩️ Because BASE's slot was occupied, the inverse is another `create-{slug}` carrying the DISPLACED handle — never a bare delete.",
        inverse=f"""    assert_eq!(inverse.len(), 1, "create-{slug} inverts to exactly one step");
    match &inverse[0] {{
        CadMutation::Create{variant}(step) => {{
            assert_eq!(step.child_id, "{old_child}", "the inverse must reinstall the handle create-{slug} displaced");
            assert_eq!(step.target, "{old_uri}", "the inverse must carry the displaced handle's target URI");
        }}
        other => panic!("create-{slug} over an OCCUPIED slot must invert to create-{slug}, got {{other:?}}"),
    }}""",
        probe=f"""    let slot = produced.diff().{field}.as_ref().expect("create-{slug} fills the `{field}` slot diff");
    let handle = slot.as_ref().expect("create-{slug}'s diff sets the slot to the occupied arm");
    assert_eq!(handle.child_id, "{new_child}", "the slot diff carries the payload child id");
    assert!(produced.diff().drawings.is_none() && produced.diff().nodes.is_none(), "create-{slug} emits nothing but its own slot field");""",
    ))
    CASES.append(dict(
        leaf=f"delete-{slug}", kind=f"delete-{slug}", case=f"vacates-the-{pane}-slot",
        mod=f"tests_vacates_the_{pane.replace('-', '_')}_slot", transform=t_delete,
        mutation={"mutation": "delete" + variant},
        blurb=f"Proves `delete-{slug}` clears the fixed slot to the vacated arm while the sibling slots and the drawings collection stay put.",
        fn1=f"clears_the_{pane.replace('-', '_')}_slot_only",
        fn1doc=f"▶️ `delete-{slug}` empties the fixed `{field}` slot; the composed child document itself is not this parent's business.",
        change=f"""    assert!(after.{field}.is_none(), "delete-{slug} must empty the fixed slot");
    assert!({other_asserts}, "delete-{slug} must leave the other three fixed model slots occupied");
    assert_eq!(after.drawings.len(), 1, "delete-{slug} must not touch the drawings child collection");
    assert_eq!(after.nodes.len(), 2, "delete-{slug} must not cascade into the node tree");""",
        fn2=f"inverse_reinstalls_the_escrowed_{pane.replace('-', '_')}_handle",
        fn2doc=f"↩️ The inverse is a `create-{slug}` carrying the handle escrowed from BASE.",
        inverse=f"""    assert_eq!(inverse.len(), 1, "delete-{slug} on an occupied slot inverts to exactly one step");
    match &inverse[0] {{
        CadMutation::Create{variant}(step) => {{
            assert_eq!(step.child_id, "{old_child}", "the inverse must reinstall the escrowed child id");
            assert_eq!(step.target, "{old_uri}", "the inverse must carry the escrowed handle's target URI");
        }}
        other => panic!("delete-{slug} must invert to create-{slug}, got {{other:?}}"),
    }}""",
        probe=f"""    let slot = produced.diff().{field}.as_ref().expect("delete-{slug} fills the `{field}` slot diff");
    assert!(slot.is_none(), "delete-{slug}'s diff sets the slot to the vacated arm (outer Some = changed, inner None = now empty)");
    assert!(produced.diff().drawings.is_none() && produced.diff().nodes.is_none(), "delete-{slug} emits nothing but its own slot field");""",
    ))
#endregion

CASES += [
    dict(
        leaf="create-drawing", kind="create-drawing", case="appends-drawing-2",
        mod="tests_appends_drawing_2", transform=t_create_drawing,
        mutation={"mutation": "createDrawing", "childId": "drawing-2", "target": "cad-drawing-2!s.stdio.semio@v1/drawing"},
        blurb="Proves the Vec-cardinality drawings composition grows by one handle, emitted as a WHOLE post-state list.",
        fn1="appends_a_second_drawing_handle", fn1doc="▶️ `create-drawing` appends one `s.stdio.semio.drawing` handle to the forward composition slot.",
        change="""    assert_eq!(after.drawings.iter().map(|handle| handle.child_id.as_str()).collect::<Vec<_>>(), vec!["drawing-1", "drawing-2"], "create-drawing appends the new handle after the existing ones");
    assert_eq!(after.drawings[1].target.to_uri(), "cad-drawing-2!s.stdio.semio@v1/drawing", "create-drawing must parse the payload target URI into a real drawing-subset ArtifactRef");
    assert!(after.shape_model.is_some(), "create-drawing must not touch the fixed model slots");""",
        fn2="inverse_deletes_the_drawing_it_created", fn2doc="↩️ `create-drawing` always inverts to `delete-drawing` of the id it minted — it never inspects BASE.",
        inverse="""    assert_eq!(inverse.len(), 1, "create-drawing inverts to exactly one step");
    match &inverse[0] {
        CadMutation::DeleteDrawing(step) => assert_eq!(step.child_id, "drawing-2", "the inverse must delete the drawing id create-drawing minted"),
        other => panic!("create-drawing must invert to delete-drawing, got {other:?}"),
    }""",
        probe="""    let list = produced.diff().drawings.as_ref().expect("create-drawing fills the drawings child list");
    assert_eq!(list.values.iter().map(|handle| handle.child_id.as_str()).collect::<Vec<_>>(), vec!["drawing-1", "drawing-2"], "create-drawing emits the WHOLE post-state list, not an added/removed delta");
    assert!(produced.diff().nodes.is_none(), "create-drawing must not emit a nodes delta");""",
    ),
    dict(
        leaf="delete-drawing", kind="delete-drawing", case="removes-drawing-1",
        mod="tests_removes_drawing_1", transform=t_delete_drawing,
        mutation={"mutation": "deleteDrawing", "childId": "drawing-1"},
        blurb="Proves the addressed drawing handle is filtered out of the whole-list diff, leaving an empty collection.",
        fn1="filters_the_addressed_drawing_out_of_the_list", fn1doc="▶️ `delete-drawing` drops one handle from the Vec-cardinality composition slot.",
        change="""    assert!(after.drawings.is_empty(), "delete-drawing must remove the addressed handle, leaving the collection empty");
    assert!(after.shape_model.is_some() && after.building_model.is_some(), "delete-drawing must not touch the fixed model slots");
    assert_eq!(after.nodes.len(), 2, "delete-drawing must not cascade into the node tree");""",
        fn2="inverse_recreates_the_drawing_with_its_target", fn2doc="↩️ The inverse is a `create-drawing` carrying the escrowed handle's id AND target URI.",
        inverse="""    assert_eq!(inverse.len(), 1, "delete-drawing inverts to exactly one step");
    match &inverse[0] {
        CadMutation::CreateDrawing(step) => {
            assert_eq!(step.child_id, "drawing-1", "the inverse must recreate the removed drawing id");
            assert_eq!(step.target, "cad-drawing-1!s.stdio.semio@v1/drawing", "the inverse must carry the removed handle's target URI, not a stub");
        }
        other => panic!("delete-drawing must invert to create-drawing, got {other:?}"),
    }""",
        probe="""    let list = produced.diff().drawings.as_ref().expect("delete-drawing fills the drawings child list");
    assert!(list.values.is_empty(), "delete-drawing emits the WHOLE post-state list — empty here — rather than a removed-id delta");
    assert!(produced.diff().shape_model.is_none(), "delete-drawing must not emit a model-slot diff");""",
    ),
    dict(
        leaf="create-node", kind="create-node", case="appends-node-3",
        mod="tests_appends_node_3", transform=t_create_node,
        mutation={"mutation": "createNode", "node": NODE_3},
        blurb="Proves a whole `CadNode` record (label and kind) enters the id-keyed node collection.",
        fn1="brings_a_whole_node_record_into_the_tree", fn1doc="▶️ `create-node` appends the payload's `CadNode`, label and kind carried verbatim.",
        change="""    assert_eq!(after.nodes.iter().map(|node| node.id.as_str()).collect::<Vec<_>>(), vec!["node-1", "node-2", "node-3"], "create-node appends the new node (the nodes delta's `added` always pushes at the end)");
    let created = after.nodes.iter().find(|node| node.id == "node-3").expect("create-node inserts node-3");
    assert_eq!(created.label, "Column", "create-node must carry the payload node's label");
    assert_eq!(created.kind, "solid", "create-node must carry the payload node's kind");
    assert_eq!(after.references_by_model_definition_id["spatial.shape"].len(), 1, "create-node must not touch the reference lists");""",
        fn2="inverse_deletes_the_node_it_created", fn2doc="↩️ `create-node` always inverts to `delete-node` of the id it minted — it never inspects BASE.",
        inverse="""    assert_eq!(inverse.len(), 1, "create-node inverts to exactly one step");
    match &inverse[0] {
        CadMutation::DeleteNode(step) => assert_eq!(step.node_id, "node-3", "the inverse must delete the node id create-node minted"),
        other => panic!("create-node must invert to delete-node, got {other:?}"),
    }""",
        probe="""    let delta = produced.diff().nodes.as_ref().expect("create-node fills the nodes delta");
    assert_eq!(delta.added.len(), 1, "create-node adds exactly one node");
    assert_eq!(delta.added[0].id, "node-3", "create-node's `added` entry is the payload node");
    assert!(delta.removed.is_empty() && delta.patched.is_empty() && delta.reordered.is_none(), "create-node touches only the `added` arm of the nodes delta");""",
    ),
    dict(
        leaf="delete-node", kind="delete-node", case="removes-node-2",
        mod="tests_removes_node_2", transform=t_delete_node,
        mutation={"mutation": "deleteNode", "nodeId": "node-2"},
        blurb="Proves a node id leaves the collection and that undo re-materializes the full record.",
        fn1="drops_node_2_and_keeps_the_root_node", fn1doc="▶️ `delete-node` removes only the addressed node — sibling nodes and every other lane are untouched.",
        change="""    assert_eq!(after.nodes.iter().map(|node| node.id.as_str()).collect::<Vec<_>>(), vec!["node-1"], "delete-node must remove node-2 and only node-2");
    assert_eq!(after.nodes[0].label, "Root", "delete-node must not relabel the surviving node");
    assert_eq!(after.drawings.len(), 1, "delete-node must not cascade into the drawings collection");""",
        fn2="inverse_recreates_node_2_with_its_label_and_kind", fn2doc="↩️ The inverse is a `create-node` carrying the ENTIRE removed record, not just its id.",
        inverse="""    assert_eq!(inverse.len(), 1, "delete-node inverts to exactly one step");
    match &inverse[0] {
        CadMutation::CreateNode(step) => {
            assert_eq!(step.node.id, "node-2", "the inverse must recreate the removed node");
            assert_eq!(step.node.label, "Base Plate", "the inverse must carry the removed node's label, not a stub");
            assert_eq!(step.node.kind, "solid", "the inverse must carry the removed node's kind");
        }
        other => panic!("delete-node must invert to create-node, got {other:?}"),
    }""",
        probe="""    let delta = produced.diff().nodes.as_ref().expect("delete-node fills the nodes delta");
    assert_eq!(delta.removed, vec!["node-2".to_string()], "delete-node's diff carries the id in `removed`");
    assert!(delta.added.is_empty() && delta.patched.is_empty(), "delete-node touches only the `removed` arm of the nodes delta");""",
    ),
    dict(
        leaf="rename-node", kind="rename-node", case="relabels-the-root-node",
        mod="tests_relabels_the_root_node", transform=t_rename_node,
        mutation={"mutation": "renameNode", "nodeId": "node-1", "newLabel": "Assembly Root"},
        blurb="Proves the node `label` is patched while `kind` — the only other field `CadNodePatch` could carry — stays put.",
        fn1="relabels_the_node_without_retyping_it", fn1doc="▶️ `rename-node` patches `label` only; `CadNodePatch` has no `kind` field, so the node's type can never drift here.",
        change="""    let node = after.nodes.iter().find(|node| node.id == "node-1").expect("node-1 survives");
    assert_eq!(node.label, "Assembly Root", "rename-node must set the addressed node's label");
    assert_eq!(node.kind, "group", "rename-node must leave the node kind untouched");
    assert_eq!(after.nodes[1].label, "Base Plate", "rename-node must not relabel sibling nodes");""",
        fn2="inverse_restores_the_root_label", fn2doc="↩️ The inverse is a `rename-node` carrying the label captured from BASE.",
        inverse="""    assert_eq!(inverse.len(), 1, "rename-node inverts to exactly one step");
    match &inverse[0] {
        CadMutation::RenameNode(step) => {
            assert_eq!(step.node_id, "node-1", "the inverse must address the same node");
            assert_eq!(step.new_label, "Root", "the inverse must carry the pre-edit label");
        }
        other => panic!("rename-node must invert to rename-node, got {other:?}"),
    }""",
        probe="""    let delta = produced.diff().nodes.as_ref().expect("rename-node fills the nodes delta");
    assert_eq!(delta.patched.len(), 1, "rename-node patches exactly one node");
    assert_eq!(delta.patched[0].id, "node-1", "rename-node's patch entry addresses node-1");
    assert_eq!(delta.patched[0].patch.label.as_deref(), Some("Assembly Root"), "rename-node fills the patch's `label` field — the only field CadNodePatch has");
    assert!(delta.added.is_empty() && delta.removed.is_empty(), "rename-node touches only the `patched` arm of the nodes delta");""",
    ),
    dict(
        leaf="change-reference-hidden", kind="change-reference-hidden", case="hides-the-shape-reference",
        mod="tests_hides_the_shape_reference", transform=t_change_reference_hidden,
        mutation={"mutation": "changeReferenceHidden", "modelDefinitionId": "spatial.shape", "referenceId": "ref-1", "newHidden": True},
        blurb="Proves the `hidden` flag flips without disturbing `locked`, which is a separate mutation's field.",
        fn1="hides_the_reference_without_unlocking_it", fn1doc="▶️ `change-reference-hidden` rewrites one boolean of one reference inside one model-definition bucket.",
        change="""    let rows = &after.references_by_model_definition_id["spatial.shape"];
    let reference = rows.iter().find(|reference| reference.id == "ref-1").expect("ref-1 survives");
    assert!(reference.hidden, "change-reference-hidden must set the addressed reference's hidden flag");
    assert!(reference.locked, "change-reference-hidden must leave the locked flag exactly as BASE had it");
    assert_eq!(reference.width_world, 8.0, "change-reference-hidden must not resize the reference plane");
    assert_eq!(after.references_by_model_definition_id.len(), 1, "change-reference-hidden must not create other model-definition buckets");""",
        fn2="inverse_reveals_the_reference_again", fn2doc="↩️ The inverse is a `change-reference-hidden` carrying BASE's flag.",
        inverse="""    assert_eq!(inverse.len(), 1, "change-reference-hidden inverts to exactly one step");
    match &inverse[0] {
        CadMutation::ChangeReferenceHidden(step) => {
            assert_eq!((step.model_definition_id.as_str(), step.reference_id.as_str()), ("spatial.shape", "ref-1"), "the inverse must address the same reference in the same bucket");
            assert!(!step.new_hidden, "the inverse must carry the pre-edit hidden flag");
        }
        other => panic!("change-reference-hidden must invert to change-reference-hidden, got {other:?}"),
    }""",
        probe="""    let map = produced.diff().references_by_model_definition_id.as_ref().expect("change-reference-hidden fills the references map");
    assert_eq!(map.len(), 1, "change-reference-hidden emits only the addressed model-definition bucket");
    let rows = map.get("spatial.shape").expect("the addressed bucket is keyed by its model definition id");
    assert!(rows[0].hidden, "the emitted bucket carries the whole post-patch reference row");
    assert!(rows[0].locked, "the emitted row keeps every field the mutation did not address");""",
    ),
    dict(
        leaf="change-reference-locked", kind="change-reference-locked", case="unlocks-the-shape-reference",
        mod="tests_unlocks_the_shape_reference", transform=t_change_reference_locked,
        mutation={"mutation": "changeReferenceLocked", "modelDefinitionId": "spatial.shape", "referenceId": "ref-1", "newLocked": False},
        blurb="Proves the `locked` flag flips without disturbing `hidden`, which is a separate mutation's field.",
        fn1="unlocks_the_reference_without_revealing_it", fn1doc="▶️ `change-reference-locked` rewrites the lock boolean only — visibility is `change-reference-hidden`'s job.",
        change="""    let rows = &after.references_by_model_definition_id["spatial.shape"];
    let reference = rows.iter().find(|reference| reference.id == "ref-1").expect("ref-1 survives");
    assert!(!reference.locked, "change-reference-locked must clear the addressed reference's lock");
    assert!(!reference.hidden, "change-reference-locked must leave the hidden flag exactly as BASE had it");
    assert_eq!(reference.source_url, "https://example.test/plan.png", "change-reference-locked must not repoint the media");""",
        fn2="inverse_relocks_the_reference", fn2doc="↩️ The inverse is a `change-reference-locked` carrying BASE's flag.",
        inverse="""    assert_eq!(inverse.len(), 1, "change-reference-locked inverts to exactly one step");
    match &inverse[0] {
        CadMutation::ChangeReferenceLocked(step) => {
            assert_eq!((step.model_definition_id.as_str(), step.reference_id.as_str()), ("spatial.shape", "ref-1"), "the inverse must address the same reference in the same bucket");
            assert!(step.new_locked, "the inverse must carry the pre-edit locked flag");
        }
        other => panic!("change-reference-locked must invert to change-reference-locked, got {other:?}"),
    }""",
        probe="""    let map = produced.diff().references_by_model_definition_id.as_ref().expect("change-reference-locked fills the references map");
    let rows = map.get("spatial.shape").expect("the addressed bucket is keyed by its model definition id");
    assert!(!rows[0].locked, "the emitted bucket carries the whole post-patch reference row");
    assert!(produced.diff().nodes.is_none(), "change-reference-locked must not emit a nodes delta");""",
    ),
    dict(
        leaf="change-reference-width", kind="change-reference-width", case="widens-the-shape-reference-plane",
        mod="tests_widens_the_shape_reference_plane", transform=t_change_reference_width,
        mutation={"mutation": "changeReferenceWidth", "modelDefinitionId": "spatial.shape", "referenceId": "ref-1", "newWidthWorld": 12.0},
        blurb="Proves the world-width scalar is rewritten while the uniform `scale` factor is left alone.",
        fn1="rewrites_the_world_width_without_touching_the_scale", fn1doc="▶️ `change-reference-width` writes `width_world`; the uniform `scale` factor is a different field entirely.",
        change="""    let rows = &after.references_by_model_definition_id["spatial.shape"];
    let reference = rows.iter().find(|reference| reference.id == "ref-1").expect("ref-1 survives");
    assert_eq!(reference.width_world, 12.0, "change-reference-width must set the addressed reference's world width");
    assert_eq!(reference.scale, Some(1.5), "change-reference-width must leave the uniform scale factor untouched");
    assert_eq!(reference.origin, [0.0, 0.0, 0.0], "change-reference-width must not move the reference");""",
        fn2="inverse_restores_the_original_world_width", fn2doc="↩️ The inverse is a `change-reference-width` carrying BASE's width.",
        inverse="""    assert_eq!(inverse.len(), 1, "change-reference-width inverts to exactly one step");
    match &inverse[0] {
        CadMutation::ChangeReferenceWidth(step) => {
            assert_eq!((step.model_definition_id.as_str(), step.reference_id.as_str()), ("spatial.shape", "ref-1"), "the inverse must address the same reference in the same bucket");
            assert_eq!(step.new_width_world, 8.0, "the inverse must carry the pre-edit world width");
        }
        other => panic!("change-reference-width must invert to change-reference-width, got {other:?}"),
    }""",
        probe="""    let map = produced.diff().references_by_model_definition_id.as_ref().expect("change-reference-width fills the references map");
    let rows = map.get("spatial.shape").expect("the addressed bucket is keyed by its model definition id");
    assert_eq!(rows[0].width_world, 12.0, "the emitted bucket carries the whole post-patch reference row");
    assert_eq!(rows[0].scale, Some(1.5), "the emitted row keeps the scale factor the mutation did not address");""",
    ),
    dict(
        leaf="move-reference", kind="move-reference", case="moves-the-shape-reference-off-origin",
        mod="tests_moves_the_shape_reference_off_origin", transform=t_move_reference,
        mutation={"mutation": "moveReference", "modelDefinitionId": "spatial.shape", "referenceId": "ref-1", "newOrigin": [1.0, 2.0, 3.0]},
        blurb="Proves the 3-component origin is replaced as a unit while orientation and scale stay fixed.",
        fn1="translates_the_reference_origin_only", fn1doc="▶️ `move-reference` writes all three origin components at once; orientation and scale are untouched.",
        change="""    let rows = &after.references_by_model_definition_id["spatial.shape"];
    let reference = rows.iter().find(|reference| reference.id == "ref-1").expect("ref-1 survives");
    assert_eq!(reference.origin, [1.0, 2.0, 3.0], "move-reference must write all three origin components from the payload");
    assert!(reference.orientation.is_none(), "move-reference must not invent an orientation");
    assert_eq!(reference.scale, Some(1.5), "move-reference must leave the uniform scale factor untouched");""",
        fn2="inverse_moves_the_reference_back_to_the_origin", fn2doc="↩️ The inverse is a `move-reference` carrying BASE's origin triple.",
        inverse="""    assert_eq!(inverse.len(), 1, "move-reference inverts to exactly one step");
    match &inverse[0] {
        CadMutation::MoveReference(step) => {
            assert_eq!((step.model_definition_id.as_str(), step.reference_id.as_str()), ("spatial.shape", "ref-1"), "the inverse must address the same reference in the same bucket");
            assert_eq!(step.new_origin, [0.0, 0.0, 0.0], "the inverse must carry the pre-move origin triple");
        }
        other => panic!("move-reference must invert to move-reference, got {other:?}"),
    }""",
        probe="""    let map = produced.diff().references_by_model_definition_id.as_ref().expect("move-reference fills the references map");
    let rows = map.get("spatial.shape").expect("the addressed bucket is keyed by its model definition id");
    assert_eq!(rows[0].origin, [1.0, 2.0, 3.0], "the emitted bucket carries the whole post-patch reference row");
    assert_eq!(rows[0].width_world, 8.0, "the emitted row keeps the world width the mutation did not address");""",
    ),
    dict(
        leaf="replace-reference-media", kind="replace-reference-media", case="reattaches-the-shape-reference-to-a-new-plan",
        mod="tests_reattaches_the_shape_reference_to_a_new_plan", transform=t_replace_reference_media,
        mutation={"mutation": "replaceReferenceMedia", "modelDefinitionId": "spatial.shape", "referenceId": "ref-1",
                  "newSourceUrl": "https://example.test/plan-v2.png", "newMediaKind": "drawing",
                  "newOrientation": None, "newScale": 2.0, "newOpacity": 0.25},
        blurb="Proves the five-field media bundle is replaced atomically — and that a `null` orientation LEAVES the existing one alone rather than clearing it.",
        fn1="swaps_the_media_bundle_and_leaves_placement_alone", fn1doc="▶️ `replace-reference-media` rewrites url/kind/scale/opacity together; `origin`, `width_world`, `hidden` and `locked` are placement, not media.",
        change="""    let rows = &after.references_by_model_definition_id["spatial.shape"];
    let reference = rows.iter().find(|reference| reference.id == "ref-1").expect("ref-1 survives");
    assert_eq!(reference.source_url, "https://example.test/plan-v2.png", "replace-reference-media must repoint the source url");
    assert_eq!(reference.media_kind, "drawing", "replace-reference-media must rewrite the media kind");
    assert_eq!(reference.scale, Some(2.0), "replace-reference-media must rewrite the uniform scale factor");
    assert_eq!(reference.opacity, Some(0.25), "replace-reference-media must rewrite the opacity");
    assert!(reference.orientation.is_none(), "a null new_orientation leaves the reference's orientation as BASE had it — the patch never clears it");
    assert_eq!((reference.origin, reference.width_world, reference.locked), ([0.0, 0.0, 0.0], 8.0, true), "replace-reference-media must not touch placement fields");""",
        fn2="inverse_reattaches_the_original_plan_bundle", fn2doc="↩️ The inverse is a `replace-reference-media` carrying all five BASE media fields.",
        inverse="""    assert_eq!(inverse.len(), 1, "replace-reference-media inverts to exactly one step");
    match &inverse[0] {
        CadMutation::ReplaceReferenceMedia(step) => {
            assert_eq!((step.model_definition_id.as_str(), step.reference_id.as_str()), ("spatial.shape", "ref-1"), "the inverse must address the same reference in the same bucket");
            assert_eq!(step.new_source_url, "https://example.test/plan.png", "the inverse must carry the pre-edit source url");
            assert_eq!(step.new_media_kind, "image", "the inverse must carry the pre-edit media kind");
            assert_eq!((step.new_scale, step.new_opacity), (Some(1.5), Some(0.5)), "the inverse must carry the pre-edit scale and opacity");
            assert!(step.new_orientation.is_none(), "the inverse must carry BASE's absent orientation");
        }
        other => panic!("replace-reference-media must invert to replace-reference-media, got {other:?}"),
    }""",
        probe="""    let map = produced.diff().references_by_model_definition_id.as_ref().expect("replace-reference-media fills the references map");
    let rows = map.get("spatial.shape").expect("the addressed bucket is keyed by its model definition id");
    assert_eq!(rows[0].media_kind, "drawing", "the emitted bucket carries the whole post-patch reference row");
    assert_eq!(rows[0].width_world, 8.0, "the emitted row keeps the placement fields the media bundle does not own");""",
    ),
    dict(
        leaf="replace-references", kind="replace-references", case="swaps-the-shape-reference-list",
        mod="tests_swaps_the_shape_reference_list", transform=t_replace_references,
        mutation={"mutation": "replaceReferences", "modelDefinitionId": "spatial.shape", "references": [REF_2]},
        blurb="Proves the whole per-model-definition list is substituted — ids present before but absent from the payload simply vanish.",
        fn1="substitutes_the_whole_bucket_rather_than_merging", fn1doc="▶️ `replace-references` is a wholesale list substitution, not an id-keyed merge: ref-1 disappears because the payload omits it.",
        change="""    let rows = &after.references_by_model_definition_id["spatial.shape"];
    assert_eq!(rows.iter().map(|reference| reference.id.as_str()).collect::<Vec<_>>(), vec!["ref-2"], "replace-references must substitute the whole bucket — ref-1 is dropped because the payload omits it");
    assert_eq!(rows[0].width_world, 16.0, "replace-references must store the payload rows verbatim");
    assert!(rows[0].scale.is_none(), "replace-references must store the payload rows verbatim, absent optionals included");
    assert_eq!(after.references_by_model_definition_id.len(), 1, "replace-references must not create other model-definition buckets");""",
        fn2="inverse_restores_the_original_reference_list", fn2doc="↩️ The inverse is a `replace-references` carrying BASE's entire list back.",
        inverse="""    assert_eq!(inverse.len(), 1, "replace-references inverts to exactly one step");
    match &inverse[0] {
        CadMutation::ReplaceReferences(step) => {
            assert_eq!(step.model_definition_id, "spatial.shape", "the inverse must address the same bucket");
            assert_eq!(step.references.iter().map(|reference| reference.id.as_str()).collect::<Vec<_>>(), vec!["ref-1"], "the inverse must carry BASE's entire list, not a diff of it");
            assert_eq!(step.references[0].width_world, 8.0, "the inverse must carry each restored row in full");
        }
        other => panic!("replace-references must invert to replace-references, got {other:?}"),
    }""",
        probe="""    let map = produced.diff().references_by_model_definition_id.as_ref().expect("replace-references fills the references map");
    assert_eq!(map.len(), 1, "replace-references emits only the addressed model-definition bucket");
    let rows = map.get("spatial.shape").expect("the addressed bucket is keyed by its model definition id");
    assert_eq!(rows.len(), 1, "replace-references emits the payload list verbatim as the bucket's new value");
    assert_eq!(rows[0].id, "ref-2", "the emitted bucket is the payload list, not a merge with BASE");""",
    ),
    dict(
        leaf="change-active-model-definition", kind="change-active-model-definition", case="switches-the-active-pane-to-the-building-model",
        mod="tests_switches_the_active_pane_to_the_building_model", transform=t_change_active_model_definition,
        mutation={"mutation": "changeActiveModelDefinition", "newModelDefinitionId": "aec.building"},
        blurb="Proves the active-pane selector is a plain persisted string that carries no content with it.",
        fn1="repoints_the_selector_without_moving_any_content", fn1doc="▶️ `change-active-model-definition` writes one root string; no model slot, reference bucket or node moves with it.",
        change="""    assert_eq!(after.active_model_definition_id, "aec.building", "change-active-model-definition must repoint the selector");
    assert!(after.references_by_model_definition_id.contains_key("spatial.shape"), "change-active-model-definition must not migrate reference buckets between model definitions");
    assert!(!after.references_by_model_definition_id.contains_key("aec.building"), "change-active-model-definition must not conjure a bucket for the newly selected model definition");
    assert!(after.shape_model.is_some() && after.building_model.is_some(), "change-active-model-definition must not touch the fixed model slots");""",
        fn2="inverse_reselects_the_shape_model_definition", fn2doc="↩️ The inverse is a `change-active-model-definition` carrying BASE's selector.",
        inverse="""    assert_eq!(inverse.len(), 1, "change-active-model-definition inverts to exactly one step");
    match &inverse[0] {
        CadMutation::ChangeActiveModelDefinition(step) => assert_eq!(step.new_model_definition_id, "spatial.shape", "the inverse must carry the pre-edit selector"),
        other => panic!("change-active-model-definition must invert to change-active-model-definition, got {other:?}"),
    }""",
        probe="""    assert_eq!(produced.diff().active_model_definition_id.as_deref(), Some("aec.building"), "change-active-model-definition fills the root `active_model_definition_id` diff field");
    assert!(produced.diff().references_by_model_definition_id.is_none() && produced.diff().nodes.is_none(), "change-active-model-definition emits nothing but the selector");""",
    ),
]

TEMPLATE = '''//! \U0001f9ea️ `{kind}` fixture — `{case}`.
//!
//! {blurb}
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{{Mutation, MutationDiff}};

const BEFORE: &str = include_str!("\U0001f4f8️snapshot/⬅️before/\U0001f523️component.json");
const AFTER: &str = include_str!("\U0001f4f8️snapshot/➡️after/\U0001f523️component.json");
const MUTATION: &str = include_str!("\U0001f9a0️mutation/\U0001f523️component.json");
const OUTCOME: &str = include_str!("\U0001f3af️outcome/\U0001f523️component.json");

fn before() -> CadSnapshot {{
    serde_json::from_str(BEFORE).expect("{kind}/{case}: before snapshot decodes")
}}
fn expected_after() -> CadSnapshot {{
    serde_json::from_str(AFTER).expect("{kind}/{case}: after snapshot decodes")
}}
fn mutation() -> CadMutation {{
    serde_json::from_str(MUTATION).expect("{kind}/{case}: mutation decodes")
}}
fn applied() -> CadSnapshot {{
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("{kind} applies to its committed before-snapshot")
}}

/// {fn1doc}
#[semio_framework_async_macros::async_test]
async fn {fn1}() {{
    let after = applied();
{change}
    assert_eq!(after, expected_after(), "{kind}/{case}: applied state differs from the committed after-snapshot");
}}

/// {fn2doc}
#[semio_framework_async_macros::async_test]
async fn {fn2}() {{
    let base = before();
    let inverse = mutation().inverse(&base);
{inverse}
    let mut snapshot = applied();
    for step in &inverse {{
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("{kind}/{case}: inverse step applies");
    }}
    assert_eq!(snapshot, base, "{kind}/{case}: inverse did not restore the before-snapshot");
}}

/// \U0001f523️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: CadSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{kind}/{case}: committed {{label}} JSON is not canonical");
    }}
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{kind}/{case}: committed mutation JSON is not canonical");
}}

/// \U0001f3af️ The declared outcome matches what `{kind}`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {{
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "{kind}/{case}: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "{kind}/{case}: declared clean-applied but the diff builder reported {{:?}}", produced.messages());
{probe}
}}
'''

def dump(path, obj):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(obj, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

LEAVES = {}
for entry in sorted(os.listdir(MUT)):
    if not (MUT / entry).is_dir():
        continue
    LEAVES[re.sub(r"^[^a-z]*", "", entry)] = entry

written = []
for spec in CASES:
    leaf = MUT / LEAVES[spec["leaf"]]
    assert leaf.is_dir(), f"missing leaf {spec['leaf']}"
    case_dir = leaf / "\U0001f9ea️tests" / spec["case"]
    before_doc = base()
    after_doc = base()
    spec["transform"](after_doc)
    dump(case_dir / "\U0001f4f8️snapshot/⬅️before/\U0001f523️component.json", before_doc)
    dump(case_dir / "\U0001f4f8️snapshot/➡️after/\U0001f523️component.json", after_doc)
    dump(case_dir / "\U0001f9a0️mutation/\U0001f523️component.json", spec["mutation"])
    dump(case_dir / "\U0001f3af️outcome/\U0001f523️component.json", APPLIED)
    (case_dir / "\U0001f980️component.rs").write_text(TEMPLATE.format(**spec), encoding="utf-8")
    written.append((LEAVES[spec["leaf"]], spec["case"], spec["mod"]))

for row in written:
    print(row[0], row[1], row[2], sep="\t")
print(len(written), "cad cases")
