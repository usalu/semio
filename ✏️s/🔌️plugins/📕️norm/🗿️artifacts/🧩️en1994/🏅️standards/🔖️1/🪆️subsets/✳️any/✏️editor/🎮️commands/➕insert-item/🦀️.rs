//! 📜️ Norm command — `insert-item`.

use semio_s_artifact_norm_en1994_mutations::En1994Mutation;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value::{DslValue, ToValue as _};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_norm_en1994_model::{CharacteristicAction, ColumnAction, SlabAction, Stud};
use semio_s_artifact_norm_en1994_checks::part_studs;
use semio_s_plugin_norm_app_surface::{parse_path, NormEdit, PathSegment};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "insert-item")]
pub struct InsertItem {
    pub path: String,
    pub index: u32,
    pub value_json: Option<String>,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
/// ➕ The row an `Add item` without a value inserts: a fresh action `A-<n>` of the beam, column or slab the path names (a visible placeholder load the dev edits, not a normative value).
fn default_action(document: &En1994Snapshot, path: &str) -> Option<DslValue> {
    let segments = parse_path(path).ok()?;
    let [PathSegment::Field(collection), parent, PathSegment::Field(field)] = segments.as_slice() else { return None };
    fn row<'a, T>(rows: &'a [T], at: &PathSegment, id: impl Fn(&T) -> &str) -> Option<&'a T> {
        match at {
            PathSegment::Index(index) => rows.get(*index),
            PathSegment::Id(wanted) => rows.iter().find(|r| id(r) == wanted),
            PathSegment::Field(_) => None,
        }
    }
    let fresh = |taken: Vec<&str>| (1..).map(|n| format!("A-{n}")).find(|id| !taken.contains(&id.as_str()));
    if field != "actions" {
        return None;
    }
    match collection.as_str() {
        "beams" => {
            let beam = row(&document.beams, parent, |b| b.id.as_str())?;
            Some(CharacteristicAction::imposed_area(&fresh(beam.actions.iter().map(|a| a.id.as_str()).collect())?, semio_s_artifact_norm_en1994_model::ActionStage::Composite, "B", 1.0e3).to_value())
        }
        "slabs" => {
            let slab = row(&document.slabs, parent, |s| s.id.as_str())?;
            Some(SlabAction::imposed_area(&fresh(slab.actions.iter().map(|a| a.id.as_str()).collect())?, semio_s_artifact_norm_en1994_model::ActionStage::Composite, "B", 1.0e3).to_value())
        }
        "columns" => {
            let column = row(&document.columns, parent, |c| c.id.as_str())?;
            Some(ColumnAction::forces(&fresh(column.actions.iter().map(|a| a.id.as_str()).collect())?, semio_s_artifact_norm_en1994_model::ActionStage::Composite, semio_s_artifact_norm_en1994_model::ActionKind::Imposed, "B", 1.0e5, 1.0e4).to_value())
        }
        _ => None,
    }
}

/// 🔩️ The stud an `Add item` or a palette click on the stud collection inserts: the given row or a copy of the last stud, on its beam; a stud that would sit on another stud of its beam (a duplicate) goes into the widest gap of the weaker half span instead, and a taken or empty id is replaced by the first free `<beam>-stud-<n>`.
fn placed_stud(document: &En1994Snapshot, given: Option<&DslValue>) -> Result<Stud, String> {
    let given = given.map(|value| semio_framework_value::FromValue::from_value(value.clone()).map_err(|error| format!("{error:?}"))).transpose()?;
    let template: Option<Stud> = given.or_else(|| document.studs.last().cloned());
    let beam = template.as_ref().and_then(|s| document.beams.iter().find(|b| b.id == s.beam_id)).or_else(|| document.beams.first()).ok_or_else(|| "the document has no beam to place a stud on".to_string())?;
    let mut stud = template.unwrap_or_else(|| Stud::typical("", &beam.id, 0.0));
    stud.beam_id = beam.id.clone();
    let set = part_studs::of(beam, &document.studs, document.annex);
    if set.rows.iter().any(|row| (row.x_m - stud.x_m).abs() <= part_studs::STATION_TOLERANCE_M) {
        stud.x_m = set.next_position_m();
    }
    if stud.id.is_empty() || document.studs.iter().any(|s| s.id == stud.id) {
        stud.id = (0..).map(|n| format!("{}-stud-{n}", beam.id)).find(|id| !document.studs.iter().any(|s| &s.id == id)).unwrap_or_default();
    }
    Ok(stud)
}

/// 🎯️ The edit resolver of the editor: the family's [`semio_s_artifact_norm_en1994_mutations::resolve_edit`] after a stud insert got its place (see [`placed_stud`]).
pub fn resolve_edit(document: &En1994Snapshot, edit: &NormEdit) -> Result<Vec<En1994Mutation>, String> {
    match edit {
        NormEdit::InsertItem { path, index, value } if path == "studs" => {
            let placed = NormEdit::InsertItem { path: path.clone(), index: *index, value: Some(placed_stud(document, value.as_ref())?.to_value()) };
            semio_s_artifact_norm_en1994_mutations::resolve_edit(document, &placed)
        }
        other => semio_s_artifact_norm_en1994_mutations::resolve_edit(document, other),
    }
}

pub fn handle(payload: &InsertItem, doc: &ArtifactView<'_, En1994Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1994Mutation, NoConfigMutation>, Fault> {
    let resolve = |document: &En1994Snapshot, edit: &NormEdit| resolve_edit(document, edit);
    match payload.value_json.as_deref() {
        None | Some("null") => match default_action(doc.snapshot, &payload.path) {
            Some(value) => semio_s_plugin_norm_app_surface::handle_insert_item(doc.snapshot, &payload.path, payload.index as usize, Some(value), resolve),
            None => semio_s_plugin_norm_app_surface::dispatch_insert_item(doc.snapshot, &payload.path, payload.index as usize, None, resolve),
        },
        json => semio_s_plugin_norm_app_surface::dispatch_insert_item(doc.snapshot, &payload.path, payload.index as usize, json, resolve),
    }
}
//#endregion 🔖️Handler

#[cfg(test)]
mod tests {
    use super::*;
    use semio_s_plugin_norm_app_surface::NormEdit;

    #[test]
    fn an_add_item_without_a_value_inserts_a_fresh_action_row_the_resolver_turns_into_the_insert_kind() {
        let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
        for (path, kind) in [("beams[id=beam-B1].actions", "insert-beam-action"), ("slabs[0].actions", "insert-slab-action"), ("columns[id=col-C1].actions", "insert-column-action")] {
            let value = default_action(&document, path).unwrap_or_else(|| panic!("{path}"));
            let mutations = semio_s_artifact_norm_en1994_mutations::resolve_edit(&document, &NormEdit::InsertItem { path: path.into(), index: 0, value: Some(value) }).unwrap_or_else(|e| panic!("{path}: {e}"));
            assert_eq!(mutations.len(), 1, "{path}");
            assert!(format!("{:?}", mutations[0]).starts_with(&kind.split('-').map(|w| w[..1].to_uppercase() + &w[1..]).collect::<String>()), "{path}");
        }
        assert!(default_action(&document, "studs").is_none());
        assert!(default_action(&document, "beams[id=missing].actions").is_none());
    }

    #[test]
    fn a_duplicated_stud_goes_into_the_widest_gap_with_a_fresh_id_and_a_given_free_position_is_kept() {
        let document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
        let last = document.studs.last().expect("the example beam carries studs").clone();
        let duplicate = placed_stud(&document, Some(&last.to_value())).expect("duplicate");
        assert_ne!(duplicate.id, last.id);
        assert!(document.studs.iter().all(|s| (s.x_m - duplicate.x_m).abs() > part_studs::STATION_TOLERANCE_M));
        assert_eq!((duplicate.diameter_m, duplicate.height_m, duplicate.f_u_pa), (last.diameter_m, last.height_m, last.f_u_pa));
        let free = Stud { id: "mine".into(), x_m: last.x_m + 0.0123, ..last.clone() };
        let kept = placed_stud(&document, Some(&free.to_value())).expect("free position");
        assert_eq!((kept.id.as_str(), kept.x_m), ("mine", free.x_m));
        let mutations = resolve_edit(&document, &NormEdit::InsertItem { path: "studs".into(), index: document.studs.len(), value: None }).expect("resolves");
        assert!(format!("{:?}", mutations[0]).starts_with("InsertStud"));
    }

    #[test]
    fn the_fresh_action_id_skips_the_taken_ones() {
        let mut document = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
        let taken = SlabAction::imposed_area("A-1", semio_s_artifact_norm_en1994_model::ActionStage::Composite, "B", 1.0e3);
        document.slabs[0].actions.push(taken);
        let DslValue::Object(entries) = default_action(&document, "slabs[id=slab-S1].actions").expect("row") else { panic!("object") };
        assert!(entries.iter().any(|(key, value)| key.as_str() == "id" && format!("{value:?}").contains("A-2")));
    }
}
