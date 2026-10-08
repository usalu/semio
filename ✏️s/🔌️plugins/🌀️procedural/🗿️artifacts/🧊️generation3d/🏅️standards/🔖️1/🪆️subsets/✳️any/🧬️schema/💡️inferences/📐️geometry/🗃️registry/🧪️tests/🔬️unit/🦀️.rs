//! 🧪️ Registry laws: one compute per catalogue kind, no unknown or duplicate ids, every registered compute honours the output contract.

use crate::standards::v1::subsets::any::io::text::snapshot::catalogue::catalogue;
use super::*;
use crate::standards::v1::subsets::any::schema::inferences::geometry::compute::{WidgetStep,GeometryComputeContext};
use crate::standards::v1::subsets::any::io::geometry::NativeGeometryComputeContext;
use crate::standards::v1::subsets::any::schema::inferences::geometry::contract_checked;
use crate::standards::v1::subsets::any::schema::inferences::geometry::inputs::resolve_inputs;
use crate::standards::v1::subsets::any::schema::inferences::geometry::value::WidgetEvaluation;
use semio_framework_artifact_flow_flow::neural::Dictionary;
use std::collections::BTreeSet;

fn localized(fault: &WidgetFault) -> bool {
    !fault.message.en.trim().is_empty() && !fault.message.de.trim().is_empty() && fault.message.en != fault.message.de && fault.code.starts_with(FAULT_PREFIX)
}

#[test]
fn every_registered_id_is_a_catalogue_kind_and_registered_once() {
    let mut seen = BTreeSet::new();
    for entry in TABLES.iter().chain(crate::standards::v1::subsets::any::io::geometry::TABLES).flat_map(|table| table.iter()) {
        assert!(catalogue().kind(entry.id).is_some(), "{} is not a catalogue kind", entry.id);
        assert!(seen.insert(entry.id), "{} is registered twice", entry.id);
    }
    assert_eq!(registered().len()+crate::standards::v1::subsets::any::io::geometry::index().len(),seen.len());
    assert!(["math.number", "math.plane", "brep.primitive.box", "brep.primitive.sphere"].iter().all(|id| lookup(id).is_some()));
}

#[test]
fn every_table_belongs_to_one_catalogue_category() {
    for table in TABLES {
        let categories: BTreeSet<&str> = table.iter().filter_map(|entry| catalogue().kind(entry.id)).map(|kind| kind.category.as_str()).collect();
        assert!(categories.len() <= 1, "a table mixes categories: {categories:?}");
    }
}

/// 🧩️ The native capability composition covers the admitted catalogue.
#[test]
fn geometry_compute_context_admits_every_catalogue_kind() {
    let missing:Vec<_>=catalogue().kinds().filter(|kind|NativeGeometryComputeContext.lookup(&kind.id).is_none()).map(|kind|kind.id.as_str()).collect();
    assert!(missing.is_empty(), "{} of {} catalogue kinds have no compute yet: {}", missing.len(), catalogue().kinds().count(), missing.join(", "));
}

#[test]
fn a_kind_without_a_compute_answers_with_a_localized_compute_missing_fault() {
    let Some(id) = missing_kinds(catalogue()).first().copied() else { return };
    let kind = catalogue().kind(id).expect("a missing kind is catalogued");
    let mut job = start(kind, WidgetInputs::new("w", kind, Default::default()));
    let WidgetStep::Done(evaluation) = job.step(1) else { panic!("a missing compute is already finished") };
    let fault = evaluation.fault.expect("no compute is a fault");
    assert_eq!(fault.code, format!("{FAULT_PREFIX}compute-missing"));
    assert!(localized(&fault) && evaluation.outputs.is_empty());
}

#[test]
fn every_registered_compute_with_default_inputs_honours_the_output_contract_or_faults_in_both_languages() {
    for id in registered() {
        let kind = catalogue().kind(id).expect("registered ids are catalogued");
        let evaluation = match resolve_inputs(kind, "w", &Dictionary::new(), &[], &|_| None) {
            Err(fault) => WidgetEvaluation::faulted(fault, kind.quality),
            Ok(inputs) => {
                let mut job = lookup(id).expect("registered")(kind, inputs);
                loop {
                    if let WidgetStep::Done(evaluation) = job.step(1_000) {
                        break evaluation;
                    }
                }
            }
        };
        let checked = contract_checked(kind, evaluation);
        match &checked.fault {
            Some(fault) => assert!(localized(fault), "{id}: {fault}"),
            None => assert_eq!(checked.outputs.keys().map(String::as_str).collect::<BTreeSet<_>>(), kind.outputs.iter().map(|port| port.name.as_str()).collect::<BTreeSet<_>>(), "{id}"),
        }
    }
}
