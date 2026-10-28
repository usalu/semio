extern crate semio_s_artifact_energy_model_diff_patch as patch;
use patch::{FieldPatch, Row, RowPatch, Rows, Slots, Unchanged};
use semio_framework_value::{FromValue, DslValue};

#[derive(Clone, Debug, PartialEq)]
struct Entity { id: u32, name: String }
#[derive(Clone, Debug, PartialEq)]
struct EntityPatch { id: u32, name: Option<String> }
impl Row for Entity { type Key = u32; fn key(&self) -> u32 { self.id } }
impl Unchanged for EntityPatch { fn unchanged(&self) -> bool { self.name.is_none() } }
impl RowPatch for EntityPatch { fn key(&self) -> u32 { self.id } }
impl FieldPatch for EntityPatch {
    type Target = Entity;
    fn commit_onto(&self, target: &mut Entity) -> Result<(), semio_framework_os_kernel::MutationApplyError> { self.name.commit_onto(&mut target.name) }
    fn absorb(&mut self, later: Self) { self.name.absorb(later.name); }
    fn inverse(&self, base: &Entity) -> Self { Self { id: self.id, name: self.name.inverse(&base.name) } }
}
fn entity(id: u32) -> Entity { Entity { id, name: format!("Entity {id}") } }

#[test]
fn production_keyed_rows_match_shared_identity_fixture() {
    let source = std::fs::read_to_string(std::env::var("ENERGY_IDENTITY_FIXTURE").unwrap()).unwrap();
    let cases: Vec<DslValue> = semio_framework_pack_json::from_json_str(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for case in cases {
        let ids = Vec::<u32>::from_value(case["base"].clone()).unwrap();
        let removed = Vec::<u32>::from_value(case["removed"].clone()).unwrap();
        let inserted = Vec::<Vec<u32>>::from_value(case["inserted"].clone()).unwrap();
        let accepted = bool::from_value(case["accepted"].clone()).unwrap();
        let base: Vec<_> = ids.into_iter().map(entity).collect();
        let mut rows = Rows::<EntityPatch>::removing_where(&base, |row| removed.contains(&row.id));
        for entry in inserted { rows.absorb(Rows::inserting(entry[0] as usize, entity(entry[1]))); }
        let mut after = base.clone();
        assert_eq!(rows.commit_onto(&mut after).is_ok(), accepted);
        if accepted {
            assert_eq!(after.iter().map(|row| row.id).collect::<Vec<_>>(), Vec::<u32>::from_value(case["expected"].clone()).unwrap());
            rows.inverse(&base).commit_onto(&mut after).unwrap();
            assert_eq!(after, base);
        }
    }
}

#[test]
fn production_slots_match_shared_assignment_fixture() {
    let source = std::fs::read_to_string(std::env::var("ENERGY_SLOTS_FIXTURE").unwrap()).unwrap();
    let case: DslValue = semio_framework_pack_json::from_json_str(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut slots = Slots::<f64, 3>::from_value(case["earlier"].clone()).unwrap();
    slots.absorb(Slots::from_value(case["later"].clone()).unwrap());
    let mut after = [0.0; 3];
    slots.commit_onto(&mut after).unwrap();
    assert_eq!(after.to_vec(), Vec::<f64>::from_value(case["expected"].clone()).unwrap());
    assert_eq!(slots.0.iter().map(|slot| slot.index).collect::<Vec<_>>(), [0, 2]);
    slots.inverse(&[0.0; 3]).commit_onto(&mut after).unwrap();
    assert_eq!(after, [0.0; 3]);
}

#[test]
fn production_rows_fold_inserted_renames_before_inverse() {
    let base = vec![entity(1)];
    let mut rows = Rows::<EntityPatch>::inserting(1, entity(2));
    rows.absorb(Rows::modifying(EntityPatch { id: 2, name: Some("Changed".into()) }));
    let mut after = base.clone();
    rows.commit_onto(&mut after).unwrap();
    assert_eq!(after[1].name, "Changed");
    rows.inverse(&base).commit_onto(&mut after).unwrap();
    assert_eq!(after, base);
}

#[test]
fn production_slots_reject_invalid_array_indices() {
    let mut after = [0.0; 3];
    assert!(Slots::<f64, 3>::assigning(3, 1.0).commit_onto(&mut after).is_err());
}
