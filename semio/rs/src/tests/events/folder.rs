use crate::events::KitEvent;
use crate::folder::FolderFullDto;
use crate::guid::Guid;
use crate::kit::{KitFullDto, KitStore};

#[test]
fn folder_set_description_emits() {
    let g = Guid::new_v7();
    let kit = KitStore::from_full_dto(KitFullDto {
        guid: Guid::new_v7(),
        name: "k".into(),
        folders: vec![FolderFullDto {
            guid: g.clone(),
            path: "/p".into(),
            description: None,
        }],
        ..Default::default()
    });
    let mut rx = kit.read().unwrap().subscribe();
    let f = {
        let kr = kit.read().unwrap();
        kr.folders[0].clone()
    };
    f.write().unwrap().set_description(Some("d".into()));
    let evs = super::common::drain(&mut rx);
    assert!(evs
        .iter()
        .any(|e| matches!(e, KitEvent::FieldChanged { field: "description", .. })));
}
