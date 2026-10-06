use super::*;

#[test]
fn portable_force_cases_match_sqlite_for_every_hop_size(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){for maximum_units in[1,2,3,64,256]{
  let mut positions:Vec<Vec2>=case["positions"].as_array().unwrap().iter().map(|row|Vec2::new(row[0].as_f64().unwrap(),row[1].as_f64().unwrap())).collect();
  let radii:Vec<f64>=case["radii"].as_array().unwrap().iter().map(|value|value.as_f64().unwrap()).collect();
  let edges:Vec<(usize,usize)>=case["edges"].as_array().unwrap().iter().map(|row|(row[0].as_u64().unwrap()as usize,row[1].as_u64().unwrap()as usize)).collect();
  let pins:Vec<Option<Vec2>>=case["pins"].as_array().unwrap().iter().map(|row|if row.is_null(){None}else{Some(Vec2::new(row[0].as_f64().unwrap(),row[1].as_f64().unwrap()))}).collect();
  let p=&case["parameters"];let parameters=ForceLayoutOptions{iterations:p["iterations"].as_u64().unwrap()as u32,ideal_edge_length:p["idealEdgeLength"].as_f64().unwrap(),repulsion_strength:p["repulsionStrength"].as_f64().unwrap(),spring_strength:p["springStrength"].as_f64().unwrap(),gravity:p["gravity"].as_f64().unwrap(),center_x:p["centerX"].as_f64().unwrap(),center_y:p["centerY"].as_f64().unwrap(),time_step:p["timeStep"].as_f64().unwrap(),velocity_damping:p["velocityDamping"].as_f64().unwrap(),max_speed:p["maxSpeed"].as_f64().unwrap(),..Default::default()};
  let mut velocities=vec![Vec2::ZERO;positions.len()];let mut forces=vec![Vec2::ZERO;positions.len()];let mut state=ForceLayoutState::default();let mut yes=|_|true;let mut control=NativeDecodeControl::new(usize::MAX,&mut yes);let mut hops=0;
  while !state.advance(maximum_units,&mut positions,&radii,&edges,&pins,&mut velocities,&mut forces,&parameters,&mut control).unwrap(){hops+=1;assert!(hops<10000);}
  for(index,position)in positions.iter().enumerate(){let expected=&case["expected"][index];assert!((position.x-expected[0].as_f64().unwrap()).abs()<=1e-9*position.x.abs().max(1.0));assert!((position.y-expected[1].as_f64().unwrap()).abs()<=1e-9*position.y.abs().max(1.0));}
 }}
}

#[test]
fn canceled_hop_keeps_every_borrowed_owner(){
 let mut positions=vec![Vec2::new(-1.0,0.0),Vec2::new(1.0,0.0)];let radii=vec![1.0,1.0];let edges=vec![(0,1)];let pins=vec![None,None];let mut velocities=vec![Vec2::ZERO;2];let mut forces=vec![Vec2::ZERO;2];let mut state=ForceLayoutState::default();let mut no=|_|false;let mut control=NativeDecodeControl::new(0,&mut no);
 assert!(matches!(state.advance(1,&mut positions,&radii,&edges,&pins,&mut velocities,&mut forces,&ForceLayoutOptions::default(),&mut control),Err(error)if error.kind==ValueRefusalKind::Canceled));
 assert_eq!(positions.len(),2);assert_eq!(velocities.len(),2);assert_eq!(forces.len(),2);
}
