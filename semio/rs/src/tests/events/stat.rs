use crate::design::DesignFullDto;
use crate::events::KitEvent;
use crate::guid::Guid;
use crate::kit::{KitFullDto, KitStore};
use crate::piece::PieceFullDto;
use crate::stat::StatFullDto;
use crate::typ::{TypeFullDto, TypeIdDto};

#[test]
fn stat_set_description_emits() {
    let type_guid = Guid::new_v7();
    let design_guid = Guid::new_v7();
    let piece_guid = Guid::new_v7();
    let stat_guid = Guid::new_v7();
    let kit = KitStore::from_full_dto(KitFullDto {
        guid: Guid::new_v7(),
        name: "k".into(),
        types: vec![TypeFullDto {
            guid: type_guid.clone(),
            name: "typ".into(),
            ..Default::default()
        }],
        designs: vec![DesignFullDto {
            guid: design_guid.clone(),
            name: "des".into(),
            pieces: vec![PieceFullDto {
                guid: piece_guid.clone(),
                r#type: Some(TypeIdDto {
                    guid: type_guid.clone(),
                }),
                ..Default::default()
            }],
            stats: vec![StatFullDto {
                guid: stat_guid.clone(),
                key: "sk".into(),
                value: "sv".into(),
                description: None,
                unit: None,
            }],
            ..Default::default()
        }],
        ..Default::default()
    });
    let mut rx = kit.read().unwrap().subscribe();
    let s = {
        let kr = kit.read().unwrap();
        let d = kr.designs[0].clone();
        let dr = d.read().unwrap();
        dr.stats[0].clone()
    };
    s.write().unwrap().set_description(Some("d".into()));
    let evs = super::common::drain(&mut rx);
    assert!(evs
        .iter()
        .any(|e| matches!(e, KitEvent::FieldChanged { field: "description", .. })));
}
