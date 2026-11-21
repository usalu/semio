#[derive(Default)]
struct FixtureState {
    pages: Vec<PdfPage>,
    info: PdfInfo,
    language: Option<String>,
}

#[test]
fn ordinary_archive_inverse_uses_actual_leaf_algorithms_over_fixture_observations() {
    use dsl::original_receiving::receive_to_completion;
    let laws = object_archive_cases();
    for pair in laws["ordinaryLeaves"]["pairs"].as_array().unwrap() {
        let tag = pair["tag"].as_u64().unwrap() as u8;
        let [forward, inverse] = ["forward", "inverse"].map(|side| {
            let frame = leaf_archive_frame(tag, &pair[side]);
            receive_to_completion::<PdfMutation>(&frame, (0, frame.len()), image_archive_receiving_stock()).unwrap()
        });
        let mut base = FixtureState::default();
        let actual = match (&forward, &inverse) {
            (PdfMutation::SetPageRotation(edit), PdfMutation::SetPageRotation(prior)) => {
                let mut page = PdfPage::new(200.0, 100.0);
                page.rotate = i32::from(prior.rotation);
                base.pages.push(page);
                edit.inverse(&base)
            }
            (PdfMutation::SetInfo(edit), PdfMutation::SetInfo(prior)) => {
                base.info = prior.info.clone();
                edit.inverse(&base)
            }
            (PdfMutation::SetLanguage(edit), PdfMutation::SetLanguage(prior)) => {
                base.language = prior.language.clone();
                edit.inverse(&base)
            }
            _ => panic!("unexpected typed fixture pair"),
        };
        assert_eq!(actual.unwrap(), vec![inverse]);
    }
}
