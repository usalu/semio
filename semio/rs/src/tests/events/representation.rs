use crate::events::KitEvent;
use crate::file::{FileFullDto, FileIdDto};
use crate::guid::Guid;
use crate::kit::{KitFullDto, KitStore};
use crate::representation::RepresentationFullDto;
use crate::typ::TypeFullDto;

#[test]
fn representation_set_url_emits() {
    let fg = Guid::new_v7();
    let rg = Guid::new_v7();
    let tg = Guid::new_v7();
    let kit = KitStore::from_full_dto(KitFullDto {
        guid: Guid::new_v7(),
        name: "k".into(),
        files: vec![FileFullDto {
            guid: fg.clone(),
            url: "https://f".into(),
            ..Default::default()
        }],
        types: vec![TypeFullDto {
            guid: tg.clone(),
            name: "t".into(),
            representations: vec![RepresentationFullDto {
                guid: rg.clone(),
                url: "https://r".into(),
                file: Some(FileIdDto { guid: fg.clone() }),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    });
    let mut rx = kit.read().unwrap().subscribe();
    let r = {
        let kr = kit.read().unwrap();
        let t = kr.types[0].clone();
        let tr = t.read().unwrap();
        tr.representation(rg.as_str()).unwrap().clone()
    };
    r.write().unwrap().set_url("https://r2".into());
    let evs = super::common::drain(&mut rx);
    assert!(evs.iter().any(|e| matches!(e, KitEvent::FieldChanged { field: "url", .. })));
}
