use crate::events::KitEvent;

#[test]
fn file_set_mime_emits() {
    let (kit, fg) = super::common::kit_with_file();
    let mut rx = kit.read().unwrap().subscribe();
    let f = {
        let kr = kit.read().unwrap();
        kr.file(fg.as_str()).unwrap().clone()
    };
    f.write().unwrap().set_mime(Some("image/png".into()));
    let evs = super::common::drain(&mut rx);
    assert!(evs.iter().any(|e| matches!(e, KitEvent::FieldChanged { field: "mime", .. })));
}
