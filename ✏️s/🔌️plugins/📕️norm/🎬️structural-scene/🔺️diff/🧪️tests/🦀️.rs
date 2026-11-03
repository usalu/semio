use super::*;
use crate::mock_projection::*;
use crate::projection::SceneProjector;

#[test]
fn annotation_and_legend_changes_are_carried() {
    let mut p = SceneProjector::new(MockProjection::default());
    let mut d = doc(2);
    let a = p.project(&d, &eval(&[]), &ctx(1)).unwrap();
    d.members[0].length = 9.0;
    let b = p.project(&d, &eval(&[]), &ctx(2)).unwrap();
    let delta = diff(&a, &b);
    assert_eq!(delta.annotations.len(), 1);
    assert!(matches!(delta.annotations[0], AnnotationPatch::Replace { .. }));
    assert!(delta.legends.is_none() && delta.layers.is_none());
    assert_eq!(apply(&delta, &a).unwrap(), b);
}
