mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn presentation_scene_counts_slides() {
        let scene = PresentationScene {
            schema: PRESENTATION_SCENE_SCHEMA.into(),
            title: "Demo".into(),
            sections: vec![PresentationSection {
                id: "s1".into(),
                title: "Intro".into(),
                slides: vec![
                    PresentationSlide { id: "a".into(), title: "A".into(), scene_hash: None, timeline_sections: Vec::new() },
                    PresentationSlide { id: "b".into(), title: "B".into(), scene_hash: Some("abc123".into()), timeline_sections: vec![Section::new("main", 0.0, 5.0)] },
                ],
            }],
            deck: None,
        };
        assert_eq!(scene.slide_count(), 2);
        assert_eq!(scene.scene_hashes(), vec!["abc123".to_string()]);
    }
}
