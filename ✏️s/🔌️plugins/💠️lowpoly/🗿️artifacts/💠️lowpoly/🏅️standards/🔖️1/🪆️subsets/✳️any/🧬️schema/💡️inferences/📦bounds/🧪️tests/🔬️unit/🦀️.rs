
use super::*;
use crate::{LOWPOLY_DOCUMENT_SCHEMA, LowpolyPaintLayer, LowpolyTransform};

fn object(id: &str, position: [f32; 3]) -> LowpolyObject {
    LowpolyObject { id: id.into(), name: id.into(), transform: LowpolyTransform { position, ..LowpolyTransform::default() }, smooth_shading: false, mesh: None, paint_layers: vec![LowpolyPaintLayer::new("Base")] }
}

#[semio_framework_async_macros::async_test]
async fn empty_document_has_no_bounds() {
    assert!(scene_bounds(&LowpolySnapshot::default()).is_none());
}

#[semio_framework_async_macros::async_test]
async fn two_objects_produce_their_enclosing_box() {
    let snapshot = LowpolySnapshot { schema: LOWPOLY_DOCUMENT_SCHEMA.into(), objects: vec![object("a", [-1.0, 0.0, 2.0]), object("b", [3.0, -4.0, 5.0])] };
    let bounds = scene_bounds(&snapshot).expect("two objects bound");
    assert_eq!(bounds, LowpolyBounds { min: [-1.0, -4.0, 2.0], max: [3.0, 0.0, 5.0] });
}
