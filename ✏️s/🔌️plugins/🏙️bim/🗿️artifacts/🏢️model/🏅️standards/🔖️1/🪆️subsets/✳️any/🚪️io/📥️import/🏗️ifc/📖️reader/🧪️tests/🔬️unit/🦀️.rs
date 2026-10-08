use super::*;
use semio_s_artifact_stdio_ifc::part21::parse_part21;

const SAMPLE: &str = "ISO-10303-21;
HEADER;
FILE_DESCRIPTION((''),'2;1');
FILE_NAME('','',(''),(''),'','','');
FILE_SCHEMA(('IFC2X3'));
ENDSEC;
DATA;
#1=IFCCARTESIANPOINT((0.,0.,0.));
#2=IFCCARTESIANPOINT((10.,0.,3.));
#3=IFCDIRECTION((0.,1.,0.));
#4=IFCAXIS2PLACEMENT3D(#1,$,$);
#5=IFCAXIS2PLACEMENT3D(#2,$,#3);
#6=IFCLOCALPLACEMENT($,#4);
#7=IFCLOCALPLACEMENT(#6,#5);
#10=IFCCARTESIANPOINT((0.,0.));
#11=IFCCARTESIANPOINT((4.,0.));
#12=IFCCARTESIANPOINT((4.,3.));
#13=IFCCARTESIANPOINT((2.,0.));
#14=IFCAXIS2PLACEMENT2D(#13,$);
#15=IFCCIRCLE(#14,2.);
#16=IFCTRIMMEDCURVE(#15,(#10),(#11),.F.,.CARTESIAN.);
#17=IFCPOLYLINE((#10,#11,#12,#10));
#18=IFCRECTANGLEPROFILEDEF(.AREA.,$,#14,5.,2.);
#19=IFCARBITRARYCLOSEDPROFILEDEF(.AREA.,$,#17);
#20=IFCDIRECTION((0.,0.,1.));
#21=IFCEXTRUDEDAREASOLID(#18,#4,#20,2.5);
#22=IFCSHAPEREPRESENTATION($,'Body','SweptSolid',(#21));
#23=IFCPRODUCTDEFINITIONSHAPE($,$,(#22));
#30=IFCWALL('g',$,'W',$,$,#7,#23,'w-1');
#31=IFCBUILDINGSTOREY('s',$,'S',$,'st-1',#6,$,$,.ELEMENT.,0.);
#32=IFCRELCONTAINEDINSPATIALSTRUCTURE('c',$,$,$,(#30),#31);
#33=IFCOPENINGELEMENT('o',$,'O',$,$,#7,$,'o-1');
#34=IFCRELVOIDSELEMENT('v',$,$,$,#30,#33);
ENDSEC;
END-ISO-10303-21;
";

fn doc() -> Part21Document {
    parse_part21(SAMPLE).expect("the sample parses")
}

#[test]
fn a_placement_chain_composes_from_the_parent_to_the_child() {
    let document = doc();
    let doc = Doc::new(&document);
    let world = doc.world(&Part21Value::Ref(7));
    assert_eq!(world.origin, [10.0, 0.0, 3.0]);
    assert_eq!(world.x, [0.0, 1.0, 0.0], "the reference direction becomes the x axis");
}

#[test]
fn a_trimmed_circle_edge_recovers_its_signed_bulge() {
    let document = doc();
    let doc = Doc::new(&document);
    let edge = doc.edge(&Part21Value::Ref(16)).expect("an edge");
    assert_eq!((edge.start, edge.end), ([0.0, 0.0], [4.0, 0.0]));
    assert!((edge.bulge + 1.0).abs() < 1e-9, "a clockwise half circle has bulge -1: {}", edge.bulge);
}

#[test]
fn a_polyline_profile_loses_its_closing_point() {
    let document = doc();
    let doc = Doc::new(&document);
    let section = doc.section(&Part21Value::Ref(19)).expect("a section");
    assert_eq!(section, Section::Outline { outer: vec![([0.0, 0.0], 0.0), ([4.0, 0.0], 0.0), ([4.0, 3.0], 0.0)], holes: vec![] });
}

#[test]
fn a_rectangle_profile_keeps_its_centre_and_a_body_keeps_its_depth_and_direction() {
    let document = doc();
    let doc = Doc::new(&document);
    assert_eq!(doc.section(&Part21Value::Ref(18)), Some(Section::Rectangle { centre: [2.0, 0.0], width: 5.0, depth: 2.0 }));
    let wall = doc.args(30, "IFCWALL").expect("the wall");
    let body = doc.body(wall).expect("a body");
    assert_eq!((body.depth, body.direction), (2.5, [0.0, 0.0, 1.0]));
}

#[test]
fn the_index_links_containment_voids_and_identity() {
    let document = doc();
    let doc = Doc::new(&document);
    assert_eq!(doc.index.contained.get(&30), Some(&31));
    assert_eq!(doc.index.voids.get(&30), Some(&vec![33]));
    let wall = doc.args(30, "IFCWALL").expect("the wall");
    assert_eq!(Doc::identity(wall, "IFCWALL").as_deref(), Some("w-1"));
    let storey = doc.args(31, "IFCBUILDINGSTOREY").expect("the storey");
    assert_eq!(Doc::identity(storey, "IFCBUILDINGSTOREY").as_deref(), Some("st-1"));
}
