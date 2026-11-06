//! 🧪️ Cold authoring graphs preserve original rows and refuse invalid references.
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::{snapshot::{Fem3dSnapshot,Fem3dSnapshotValue},structure::{FemElement,FemSection}};
use semio_s_fem_node::volumetric::FemNodeValue;
use semio_s_fem_material::volumetric::FemMaterialValue;
fn text(value:&serde_json::Value)->String{value.as_str().unwrap().to_owned()}
#[test]
fn reference_graph_fixture_builds_and_retires_owned_persistent_rows(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧪️tests/🛬️receiving/❄️cold/🧫️fixtures/🔣️.json")).unwrap();
 for case in fixture["cases"].as_array().unwrap().iter().filter(|case|case["dimension"]==3){
  let mut input=Fem3dSnapshotValue::default();
  input.nodes=case["nodes"].as_array().unwrap().iter().map(|id|FemNodeValue{id:text(id),x:0.,y:0.,z:0.,}).collect();
  input.materials=vec![FemMaterialValue{id:text(&case["material"]),name:"material".into(),e:1.,nu:0.2,rho:1.,g:1.,}];
  input.sections=vec![FemSection{id:text(&case["section"]),name:"section".into(),area:1.,iy:1.,iz:1.,j:1.,}];
  let e=&case["element"];input.elements=vec![FemElement::Bar{id:"e".into(),start:text(&e["start"]),end:text(&e["end"]),material_id:text(&e["material"]),section_id:text(&e["section"])}];
  let result=Fem3dSnapshot::from_reference(input);
  assert_eq!(result.is_ok(),case["valid"].as_bool().unwrap());
  if let Ok(snapshot)=result{assert_eq!(snapshot.nodes.len(),2);assert_eq!(snapshot.materials.len(),1);<Fem3dSnapshot as semio_framework_value::FromValue>::retire_decoded(snapshot);}
 }
}

#[test]
fn cold_builder_crosses_page_boundaries_and_retires_its_roots(){
 for count in [0,1,16,17,257]{
  let mut value=Fem3dSnapshotValue::default();
  value.nodes=(0..count).map(|i|FemNodeValue{id:format!("n{i}"),x:1.,y:2.,z:3.,}).collect();
  let snapshot=Fem3dSnapshot::from_reference(value).unwrap();
  assert_eq!(snapshot.nodes.len(),count);
  if count>0{assert_eq!(snapshot.nodes.get(count-1).unwrap().id.as_str(),format!("n{}",count-1));}
  <Fem3dSnapshot as semio_framework_value::FromValue>::retire_decoded(snapshot);
 }
}
