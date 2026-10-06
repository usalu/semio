#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use crate::standards::v1::subsets::any::schema::inferences::Fem2dInference;
    use protocol::Inference;

    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(text).expect("example dsl parses");
    assert_eq!(Fem2dInference::infer(&snapshot).expect("valid materialized inference fixture"), Fem2dInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[test]
fn inference_default_law() {
    use crate::standards::v1::subsets::any::schema::inferences::Fem2dInference;
    use crate::Fem2dSnapshot;
    use protocol::Inference;

    assert_eq!(Fem2dInference::infer(&Fem2dSnapshot::default()).expect("valid materialized inference fixture"), Fem2dInference::default());
}
//#endregion 🧪️InferenceLaws
