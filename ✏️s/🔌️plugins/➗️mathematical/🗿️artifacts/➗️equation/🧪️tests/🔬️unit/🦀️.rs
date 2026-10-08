use super::*;

/// 🪪️ `artifact_kind().schema` IS `MATH_DOCUMENT_SCHEMA`: a document kind has ONE schema identity — the hub's codec rows, document-open targets and genesis, the MCP workspace
/// store and host-media contributions all key on it (ticket 26/09/23 W4: a distinct "media schema" left the package without a
/// codec owner, so the trusted catalog refused it). The former media string was the kind id
/// itself (`computation.equation`), which stays.
#[semio_framework_async_macros::async_test]
async fn artifact_kind_names_the_store_schema() {
    assert_eq!(artifact_kind().schema, MATH_DOCUMENT_SCHEMA);
    assert_eq!(artifact_kind().source_format, "semio.equation/v1");
}

#[semio_framework_async_macros::async_test]
async fn default_graph_has_nodes_and_edges() {
    let graph = EquationGraph::default();
    assert!(!graph.nodes.is_empty());
    assert!(!graph.edges.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn default_geometry_has_points() {
    assert!(!EquationGeometry::default().points.is_empty());
}

fn moved_geometry() -> EquationGeometry {
    let mut geometry = EquationGeometry::default();
    geometry.points[0].x += 1.0;
    geometry
}

/// 📦️ LAW (model (a), design §20.15): the foreign carrier is exactly the parent-owned state — `graph`, `geometry` and
/// `equation` — and the value wire carries the whole snapshot, so a decoded document is the authored one.
#[semio_framework_async_macros::async_test]
async fn the_carrier_is_the_parent_state_and_the_value_wire_keeps_it() {
    let snapshot = EquationSnapshot::default();
    let fixture = equation_carrier_snapshot(&snapshot);
    assert_eq!((fixture.graph.clone(), fixture.geometry.clone(), fixture.equation.clone()), (snapshot.graph.clone(), snapshot.geometry.clone(), snapshot.equation.clone()));
    assert_eq!(equation_snapshot_from_host_snapshot(fixture), snapshot, "the carrier rebuilds the snapshot with its derived handles");
    assert_eq!(EquationSnapshot::from_value(snapshot.to_value()).expect("the value wire decodes"), snapshot);
}

/// 🧮️ LAW: every derived handle is content-addressed — deterministic, its id equal to its target artifact id, and moved only
/// by the state it derives from: a point drag re-addresses `computed` alone, a label edit `notation` and `computed`.
#[semio_framework_async_macros::async_test]
async fn derived_children_are_content_addressed_from_the_parent_state() {
    let (graph, geometry) = (EquationGraph::default(), EquationGeometry::default());
    let (notation, results, computed) = equation_children(&graph, &geometry);
    assert_eq!(equation_children(&graph, &geometry), (notation.clone(), results.clone(), computed.clone()), "equal state addresses equal children");
    for (child_id, artifact_id) in [(&notation.child_id, &notation.target.artifact_id), (&results.child_id, &results.target.artifact_id), (&computed.child_id, &computed.target.artifact_id)] {
        assert_eq!(child_id, artifact_id, "a derived child's id is its target artifact id");
    }
    let (dragged_notation, dragged_results, dragged_computed) = equation_children(&graph, &moved_geometry());
    assert_eq!((dragged_notation, dragged_results), (notation.clone(), results.clone()), "a point drag leaves the graph-derived children at their address");
    assert_ne!(dragged_computed, computed, "a point drag re-addresses the computed child");
    let mut relabelled = graph.clone();
    relabelled.nodes[0].label = "Alpha".into();
    let (relabelled_notation, relabelled_results, _) = equation_children(&relabelled, &geometry);
    assert_ne!(relabelled_notation, notation, "a label edit re-addresses the notation child");
    assert_eq!(relabelled_results, results, "a label edit leaves the node table at its address");
}

/// 🌱️ LAW: genesis answers exactly the derived coordinate a document names — the derived pack of its own state — and nothing
/// else (a stale id, an unknown slot).
#[semio_framework_async_macros::async_test]
async fn genesis_answers_only_the_derived_coordinate() {
    let snapshot = EquationSnapshot::default();
    let notation = genesis_equation_child_pack(&snapshot, "notation", &snapshot.notation.child_id).expect("the named notation child derives");
    assert_eq!(<SemioTextSnapshot as store::ArtifactPack>::decode_pack(&notation).expect("text pack"), equation_notation_from_graph(&snapshot.graph));
    let computed = genesis_equation_child_pack(&snapshot, "computed", &snapshot.computed.child_id).expect("the named computed child derives");
    assert_eq!(<SemioValueSnapshot as store::ArtifactPack>::decode_pack(&computed).expect("value pack"), equation_computed_from_state(&snapshot.graph, &snapshot.geometry));
    let stale = equation_children(&snapshot.graph, &moved_geometry()).2.child_id;
    assert!(genesis_equation_child_pack(&snapshot, "computed", &stale).is_none(), "a coordinate the document does not name derives nothing");
    assert!(genesis_equation_child_pack(&snapshot, "absent", &snapshot.notation.child_id).is_none(), "an unknown slot derives nothing");
}

/// 🔺️ LAW: a state diff carries the new state together with its re-minted handles, and applying it yields exactly the snapshot
/// built from that state.
#[semio_framework_async_macros::async_test]
async fn a_state_diff_carries_the_state_and_its_handles() {
    let base = EquationSnapshot::default();
    let diff = equation_state_diff(EquationDiff { points: Some(crate::diff::points_replacing(&base.geometry.points, &moved_geometry().points)), ..Default::default() }, &base);
    let applied = protocol::apply_diff(&diff, &base).expect("a state diff applies");
    let mut expected = equation_snapshot_with_state(&base.graph, &moved_geometry());
    expected.equation = base.equation.clone();
    assert_eq!(applied, expected);
}

/// 🖋️ LAW (fixture writer): every committed mutation fixture snapshot and applied diff of this artifact carries the content
/// addresses of its own `graph`/`geometry`, and the committed demo document is the default snapshot. With
/// `SEMIO_EQUATION_WRITE_FIXTURES=1` the law rewrites them in place (canonical two-space JSON / DSL text) instead of asserting.
#[test]
fn committed_fixtures_carry_their_derived_handles() {
    let write = std::env::var_os("SEMIO_EQUATION_WRITE_FIXTURES").is_some();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets");
    let mut files = Vec::new();
    let mut pending = vec![root.clone()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory).expect("fixture directory reads") {
            let path = entry.expect("fixture entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.file_name().is_some_and(|name| name == "🔣️.json") && path.components().any(|part| part.as_os_str() == "🧫️fixtures") && path.components().any(|part| part.as_os_str() == "🧬️mutations") {
                files.push(path);
            }
        }
    }
    files.sort();
    let mut stale = Vec::new();
    for path in &files {
        let text = std::fs::read_to_string(path).expect("fixture reads");
        let snapshot_side = path.components().any(|part| part.as_os_str() == "📸️snapshot");
        let diff_side = path.parent().and_then(|parent| parent.file_name()).is_some_and(|name| name == "🔺️diff");
        let expected = if snapshot_side {
            let mut snapshot: EquationSnapshot = semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed snapshot decodes");
            (snapshot.notation, snapshot.results, snapshot.computed) = equation_children(&snapshot.graph, &snapshot.geometry);
            semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&snapshot.to_value()))
        } else if diff_side {
            let mut diff: EquationDiff = semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
            let before_path = path.parent().and_then(std::path::Path::parent).expect("fixture directory").join("📸️snapshot/⬅️before/🔣️.json");
            if diff.notation.is_some() || diff.results.is_some() || diff.computed.is_some() {
                let before_text = std::fs::read_to_string(&before_path).expect("the before snapshot beside a handle-carrying diff reads");
                let before: EquationSnapshot = semio_framework_pack_json::from_json_str(&before_text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed before snapshot decodes");
                let (graph, geometry) = diff.state_after(&before).expect("committed diff applies to its before snapshot");
                let (notation, results, computed) = equation_children(&graph, &geometry);
                (diff.notation, diff.results, diff.computed) = (Some(notation), Some(results), Some(computed));
            }
            semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&diff.to_value()))
        } else {
            continue;
        };
        let expected = format!("{expected}\n");
        if text != expected {
            if write {
                std::fs::write(path, &expected).expect("fixture writes");
            }
            stale.push(path.display().to_string());
        }
    }
    let demo = root.join("✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio");
    let printed = <EquationSnapshot as store::ArtifactDsl>::print_dsl(&EquationSnapshot::default());
    if std::fs::read_to_string(&demo).expect("demo asset reads") != printed {
        if write {
            std::fs::write(&demo, &printed).expect("demo asset writes");
        }
        stale.push(demo.display().to_string());
    }
    assert!(write || stale.is_empty(), "committed equation fixtures are stale (run with SEMIO_EQUATION_WRITE_FIXTURES=1): {stale:#?}");
}

/// 🧬️ The projection names exactly the snapshot's declared child slots — what the live envelope load checks
/// before a decoded document may replace the store.
#[test]
fn the_child_restore_projection_names_every_declared_child_slot() {
    let snapshot = crate::EquationSnapshot::default();
    let projection = crate::equation_child_restore_projection(&snapshot).expect("the loaded-parent child projection");
    assert_eq!(projection.len(), <crate::EquationSnapshot as semio_framework_schema_composition::ArtifactCompositionFields>::child_slots().len());
}


/// 🏗 DSL txt carrier: serialize then deserialize must restore the snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn txt_dsl_carrier_round_trips_exactly() {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use semio_framework_os_kernel::io::io_mechanism::{Deserializer, Serializer};
    use semio_framework::io_schema::IoPayload;
    let snapshot = crate::EquationSnapshot::default();
    let exported = export::txt::v_utf_8::any::EquationIntoTxt::serialize(&snapshot, &semio_framework_os_kernel::io::io_mechanism::ArchiveChildren::default()).await.expect("dsl txt export");
    let IoPayload::Text(text) = exported.value else { panic!("txt is a text payload") };
    let back = import::txt::v_utf_8::any::TxtIntoEquation::deserialize(&IoPayload::Text(text)).await.expect("dsl txt import");
    assert_eq!(back.value, snapshot);
}
