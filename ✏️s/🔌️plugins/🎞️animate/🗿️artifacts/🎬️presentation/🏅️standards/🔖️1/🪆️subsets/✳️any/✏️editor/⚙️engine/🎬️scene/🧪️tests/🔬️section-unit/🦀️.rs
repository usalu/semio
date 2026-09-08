mod tests {
    use super::*;

    #[test]
    fn section_duration_is_non_negative() {
        let s = Section::new("intro", 0.0, 2.5);
        assert!((s.duration() - 2.5).abs() < 1e-9);
    }

    #[test]
    fn section_list_tracks_open_close() {
        let mut list = SectionList::new();
        list.begin_section("main", false);
        list.end_section(10.0);
        assert_eq!(list.sections.len(), 1);
        assert_eq!(list.sections[0].name, "main");
    }
}
