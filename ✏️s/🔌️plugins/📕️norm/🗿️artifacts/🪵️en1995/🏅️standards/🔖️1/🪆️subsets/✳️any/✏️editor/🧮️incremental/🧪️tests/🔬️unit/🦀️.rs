use crate::En1995Family;
use semio_s_artifact_norm_en1995_inferences::evaluate;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_schema::checks::timber::{catalogue_demo_document, entity_demo_document};
use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;

pub(super) fn documents() -> Vec<En1995Snapshot> {
    vec![En1995Snapshot::compliant_building_beam(), En1995Snapshot::noncompliant_building(), En1995Snapshot::compliant_bridge(), En1995Snapshot::noncompliant_bridge(), En1995Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), entity_demo_document(), catalogue_demo_document()]
}

semio_s_plugin_norm_compliance::incremental_laws!(source = En1995Family, full = evaluate, live = semio_s_artifact_norm_en1995_inferences::live::report, documents = documents());

#[test]
fn an_entity_unit_reads_the_rows_it_references_and_not_the_other_rows_of_their_collections() {
    let mut d = entity_demo_document();
    let mut other = d.members[0].clone();
    other.id = "beam-B2".into();
    d.members.push(other);
    let units = <En1995Family as UnitSource>::units(&d);
    let reads = |entity: &str| units.iter().find(|u| u.subjects.iter().any(|s| s.id == entity)).map(|u| u.reads.clone()).expect("unit of the entity");
    for entity in ["taper-1", "cutout-1", "cutout-2", "fastener-1", "floor-1"] {
        let own = reads(entity);
        assert!(own.covers("members.0.hM") && own.covers("members.1.id"), "{entity} reads its member and the member ids");
        assert!(!own.covers("members.1.hM"), "{entity} does not read the other member");
    }
    assert!(reads("fastener-1").covers("connections.0.number"));
    assert!(!reads("panel-1").covers("members.0.hM") && !reads("panel-1").covers("connections.0.number"), "a panel references nothing");
    let edges = <En1995Family as UnitSource>::references(&d);
    assert!(edges.iter().any(|e| e.from.id == "taper-1" && e.to.id == "beam-B1"));
    assert!(edges.iter().any(|e| e.from.id == "fastener-1" && e.to.id == "conn-C1"));
    assert!(edges.iter().all(|e| e.from.id != "panel-1"));
}

#[test]
fn a_row_field_edit_leaves_the_non_row_units_clean() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;
    let units = <En1995Family as UnitSource>::units(&documents().remove(0));
    let reads = |id: &str| units.iter().find(|unit| unit.id.0 == id).map(|unit| unit.reads.clone()).unwrap_or_else(|| panic!("the unit {id}"));
    let global = reads("global:document");
    assert!(global.covers("annex") && global.covers("members.0.id") && global.covers("members.0.actions.0.id") && global.covers("cases.0.numbers.0.id") && global.covers("fasteners.0.id"));
    assert!(!global.overlaps("members.0.hM") && !global.overlaps("connections.0.nailDiameter") && !global.overlaps("cases.0.entryId"), "a member or connection field edit leaves the global unit clean");
}
