use super::*;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::{txt::v_utf_8::any as txt_out};
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any as txt_in;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::{encode_drawing, SemioDrawingFormat};

const SCENE: &str = r#"{"width":100,"height":100,"nodes":[
  {"transform":[2,0,0,2,10,20],"node":{"kind":"rect","x":0,"y":0,"width":10,"height":10},"fill":{"kind":"solid","color":[1,0,0,1]},"opacity":1},
  {"transform":[1,0,0,1,0,0],"node":{"kind":"path","segments":[{"kind":"move","to":[0,0]},{"kind":"cubic","ctrl1":[5,-5],"ctrl2":[15,5],"to":[20,0]}]},"stroke":{"color":[0,0,1,1],"width":2,"cap":"butt","join":"miter","dash":[]},"opacity":0.5},
  {"transform":[1,0,0,1,0,0],"node":{"kind":"circle","cx":30,"cy":30,"r":5},"fill":{"kind":"solid","color":[0,1,0,1]},"opacity":1},
  {"transform":[1,0,0,1,0,0],"node":{"kind":"text","x":0,"y":45,"content":"hi","size":12},"opacity":1}
]}"#;

#[test]
fn scene_nodes_become_a_drawing_with_their_transforms_applied() {
    let drawing = semio_drawing_from_scenes(&[SCENE.to_string()]).expect("drawing");
    assert_eq!(drawing.styles.len(), 4);
    assert_eq!(drawing.styles[1].stroke.map(|c| c.a), Some(0.5));
    let svg = String::from_utf8(encode_drawing(&drawing, SemioDrawingFormat::Svg).expect("svg")).expect("utf-8");
    assert!(svg.contains(">hi<"), "{svg}");
    let dxf = String::from_utf8(encode_drawing(&drawing, SemioDrawingFormat::Dxf).expect("dxf")).expect("dxf text");
    assert_eq!(dxf.matches("CIRCLE").count(), 1, "the circle stays a circle");
    let curve_dip = 15.0 * (0.5 - 3.0f64.sqrt() / 6.0) * (0.5 + 3.0f64.sqrt() / 6.0) * (3.0f64.sqrt() / 3.0);
    assert_eq!(drawing.canvas.width, 35.0 + 32.0);
    assert!((drawing.canvas.height - (45.0 + curve_dip + 32.0)).abs() < 0.05, "{}", drawing.canvas.height);
}

#[test]
fn an_evaluation_without_geometry_is_refused() {
    assert!(semio_drawing_from_scenes(&[]).is_err());
    assert!(semio_drawing_from_scenes(&[r#"{"error":"unknown handle"}"#.to_string()]).is_err());
}

#[test]
fn txt_is_the_exact_dsl_carrier() {
    let document = crate::standards::v1::subsets::any::schema::default_snapshot();
    let bytes = txt_out::serialize_bytes(&document).expect("txt export");
    let back = txt_in::deserialize_bytes(&bytes).expect("txt import");
    assert_eq!(<crate::Generation2dSnapshot as store::ArtifactDsl>::print_dsl(&back), <crate::Generation2dSnapshot as store::ArtifactDsl>::print_dsl(&document));
    back.retire_cold();
    document.retire_cold();
}
