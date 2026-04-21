use crate::events::KitEvent;
use crate::guid::Guid;
use crate::kit::{KitFullDto, KitStore};
use crate::quality::QualityFullDto;

#[test]
fn quality_set_key_emits() {
    let g = Guid::new_v7();
    let kit = KitStore::from_full_dto(KitFullDto {
        guid: Guid::new_v7(),
        name: "k".into(),
        qualities: vec![QualityFullDto {
            guid: g.clone(),
            key: "k1".into(),
            ..Default::default()
        }],
        ..Default::default()
    });
    let mut rx = kit.read().unwrap().subscribe();
    let q = {
        let kr = kit.read().unwrap();
        kr.qualities[0].clone()
    };
    q.write().unwrap().set_key("k2".into());
    let evs = super::common::drain(&mut rx);
    assert!(evs.iter().any(|e| matches!(e, KitEvent::FieldChanged { field: "key", .. })));
}
