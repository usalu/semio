use crate::concept::ConceptFullDto;
use crate::events::KitEvent;
use crate::guid::Guid;
use crate::kit::{KitFullDto, KitStore};

#[test]
fn concept_set_name_emits() {
    let g = Guid::new_v7();
    let kit = KitStore::from_full_dto(KitFullDto {
        guid: Guid::new_v7(),
        name: "k".into(),
        concepts: vec![ConceptFullDto {
            guid: g.clone(),
            name: "c".into(),
            description: None,
            order: None,
        }],
        ..Default::default()
    });
    let mut rx = kit.read().unwrap().subscribe();
    let c = {
        let kr = kit.read().unwrap();
        kr.concepts[0].clone()
    };
    c.write().unwrap().set_name("c2".into());
    let evs = super::common::drain(&mut rx);
    assert!(evs.iter().any(|e| matches!(e, KitEvent::FieldChanged { field: "name", .. })));
}
