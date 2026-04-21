use crate::author::AuthorFullDto;
use crate::events::KitEvent;
use crate::guid::Guid;
use crate::kit::{KitFullDto, KitStore};

#[test]
fn author_set_email_emits() {
    let ag = Guid::new_v7();
    let kit = KitStore::from_full_dto(KitFullDto {
        guid: Guid::new_v7(),
        name: "k".into(),
        authors: vec![AuthorFullDto {
            guid: ag.clone(),
            name: "n".into(),
            email: "e@x".into(),
            role: None,
            rank: None,
        }],
        ..Default::default()
    });
    let mut rx = kit.read().unwrap().subscribe();
    let a = kit.read().unwrap().authors[0].clone();
    a.write().unwrap().set_email("e2@x".into());
    let evs = super::common::drain(&mut rx);
    assert!(evs.iter().any(|e| matches!(e, KitEvent::FieldChanged { field: "email", .. })));
}
