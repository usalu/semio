//! 🔬️ The text facet is a fixed point: print → parse → print is byte-identical, and an empty body
//! decodes to the default document rather than failing.

use crate::schema::snapshot::Grid3dSnapshot;

#[test]
fn printing_then_parsing_is_a_fixed_point_for_every_example() {
    for snapshot in [crate::examples::blocks::snapshot(), crate::examples::pipes_3d::snapshot()] {
        let text = super::print_dsl(&snapshot);
        let parsed = super::parse_dsl(&text).expect("printed text parses");
        assert_eq!(parsed, snapshot);
        assert_eq!(super::print_dsl(&parsed), text);
    }
}

#[test]
fn an_empty_body_decodes_to_the_default_document() {
    assert_eq!(super::parse_dsl("").expect("empty text decodes"), Grid3dSnapshot::default());
}
