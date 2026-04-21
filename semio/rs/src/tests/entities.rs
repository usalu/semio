use crate::author::AuthorStore;

#[test]
fn author_setter_invalidates_local_hash() {
    let mut a = AuthorStore::from_full_dto(crate::author::AuthorFullDto {
        guid: crate::Guid::new_v7(),
        name: "n".into(),
        email: "e".into(),
        role: None,
        rank: None,
    });
    let h0 = a.hash();
    a.set_name("n2".into());
    let h1 = a.hash();
    assert_ne!(h0, h1);
}
