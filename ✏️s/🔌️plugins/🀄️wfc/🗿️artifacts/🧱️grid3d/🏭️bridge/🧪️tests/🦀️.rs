use super::*;
use serde_json::{json, Value};

#[test]
fn portable_coordinates_match_original_production_descriptor_oracle() {
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let coordinates = fixture["coordinates"].as_array().unwrap();
    assert_eq!(coordinates.len(), CONTRIBUTIONS.len());
    let descriptors: &[&[protocol::MutationLeafDescriptor]] = &[<semio_s_artifact_wfc_grid3d::standards::v1::subsets::any::schema::mutations::Grid3dMutation as Mutation<semio_s_artifact_wfc_grid3d::standards::v1::subsets::any::schema::snapshot::Grid3dSnapshot>>::DESCRIPTORS];
    for (row, contribution) in coordinates.iter().zip(CONTRIBUTIONS) {
        let coordinate = &contribution.coordinate;
        assert_eq!(coordinate.artifact, row["artifact"].as_str().unwrap());
        assert_eq!(coordinate.standard, row["standard"].as_str().unwrap());
        assert_eq!(coordinate.subset, row["subset"].as_str().unwrap());
        assert_eq!(coordinate.surface.unwrap_or(""), row["surface"].as_str().unwrap());
        assert_eq!(coordinate.owner, row["owner"].as_str().unwrap());
        let prefix = format!("{}/", coordinate.owner);
        let mut seen = std::collections::HashSet::new();
        let mut rows = Vec::new();
        for descriptor in descriptors.iter().flat_map(|source| source.iter()) {
            if !descriptor.owner.starts_with(&prefix) || coordinate.surface.is_none() && descriptor.owner[prefix.len()..].split('/').any(|segment| fixture["excludedOwnerSegments"].as_array().unwrap().iter().any(|value| value.as_str() == Some(segment))) || !seen.insert(descriptor.semantic_kind) { continue; }
            rows.push(json!({"id": descriptor.semantic_kind,"variant":descriptor.aggregate_variant,"outcomes":descriptor.outcome_classes.iter().map(|class| class.as_str()).collect::<Vec<_>>()}));
        }
        let mut expected = json!({"schema":"semio.repository-test.runtime-inventory/v2","artifact":coordinate.artifact,"standard":coordinate.standard,"subset":coordinate.subset,"bridgeVersion":1,"producedBy":fixture["producedBy"],"mutations":rows});
        if let Some(surface) = coordinate.surface { expected["surface"] = json!(surface); }
        let mut arguments = vec!["list-mutations".into(),coordinate.artifact.into(),coordinate.standard.into(),coordinate.subset.into()];
        if let Some(surface) = coordinate.surface { arguments.push(surface.into()); }
        arguments.extend(["--maximum-units".into(),"10000000".into(),"--maximum-owned-bytes".into(),"1048576".into(),"--budget-ms".into(),"60000".into()]);
        let mut observe = |_| Ok(());
        let output = inventory_for_command(PRODUCED_BY, CONTRIBUTIONS, &arguments, &mut observe).unwrap();
        assert_eq!(serde_json::from_slice::<Value>(&output).unwrap(), expected);
        println!("[DEBUG] child={} surface={} mutations={} bytes={}", coordinate.artifact,coordinate.surface.unwrap_or("document"),expected["mutations"].as_array().unwrap().len(),output.len());
    }
}

#[test]
fn child_refuses_sibling_coordinate_and_retains_progress_cancellation() {
    let mut args = vec!["list-mutations".into(),"s.unowned.sibling".into(),"1".into(),"any".into(),"--maximum-units".into(),"10000000".into(),"--maximum-owned-bytes".into(),"1048576".into(),"--budget-ms".into(),"60000".into()];
    let mut observe = |_| Ok(());
    assert_eq!(inventory_for_command(PRODUCED_BY, CONTRIBUTIONS, &args, &mut observe).unwrap_err(), MutationInventoryError::Coordinate);
    args[1] = CONTRIBUTIONS[0].coordinate.artifact.into();
    let mut callbacks = 0;
    let mut observe = |_| { callbacks += 1; Err(MutationInventoryError::Cancelled) };
    assert_eq!(inventory_for_command(PRODUCED_BY, CONTRIBUTIONS, &args, &mut observe).unwrap_err(), MutationInventoryError::Cancelled);
    assert_eq!(callbacks, 1);
}
