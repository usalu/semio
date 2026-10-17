//! 🧪️ Every authored shape coordinate preserves all sibling geometry fields.
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_model::DrawingShapeBody;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_model::shape_geometry::*;
#[test]
fn shape_coordinates_shared_fixtures_and_rejection_atomicity() {
    let rows:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../../../🧪️tests/🧫️fixtures/🔷️shape/✏️coordinates/🔣️.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let kind=row["kind"].as_str().unwrap();let field=ShapeCoordinateField::parse(row["field"].as_str().unwrap()).unwrap();
        assert_eq!(field.as_str(),row["field"].as_str().unwrap());
        let semio_s_artifact_draw_drawing_model::DrawingLayerNode::Shape(template)=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Test")).to_string().into()).expect("nonempty authored identity"), "Test") else {unreachable!()};
        let mut shape=DrawingShapeBody {base:template.base,shape_kind:kind.into(),rect:None,ellipse:None,circle:None,line:None,polygon:None};
        match kind {"rect"=>shape.rect=Some(serde_json::from_value(row["before"].clone()).unwrap()),"ellipse"=>shape.ellipse=Some(serde_json::from_value(row["before"].clone()).unwrap()),"circle"=>shape.circle=Some(serde_json::from_value(row["before"].clone()).unwrap()),"line"=>shape.line=Some(serde_json::from_value(row["before"].clone()).unwrap()),"polygon"=>shape.polygon=Some(serde_json::from_value(row["before"].clone()).unwrap()),_=>unreachable!()}
        let before=shape.clone();let index=row["index"].as_u64().map(|value|value as usize);let value=row["value"].as_f64().unwrap();
        if row["error"]==true {assert!(set_shape_coordinate(&mut shape,field,index,value).is_err());assert_eq!(shape,before);continue;}
        let previous=shape_coordinate(&shape,&field,index).unwrap();set_shape_coordinate(&mut shape,field,index,value).unwrap();
        assert_eq!(shape_coordinate(&shape,&field,index).unwrap(),value);assert_eq!(serde_json::to_value(&shape).unwrap()[kind],row["after"]);
        set_shape_coordinate(&mut shape,field,index,previous).unwrap();assert_eq!(shape,before);
        assert!(set_shape_coordinate(&mut shape,field,index,f64::NAN).is_err());assert_eq!(shape,before);
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
