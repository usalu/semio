extern crate self as semio_framework_3d;
pub mod mesh {pub use semio_bevel_modeling::*;}

use semio_framework_mesh_engine::io::text::PolygonSourcePreparation;
use semio_framework_value::{AuthoringWallet,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError};
use serde_json::{Value,json};

struct EvalError(String);
impl EvalError {
    #[allow(non_snake_case)]
    fn Retained(error:ValueError)->Self{Self(error.to_string())}
}
fn invalid(error:impl std::fmt::Display)->EvalError{EvalError(error.to_string())}

fn run_pair(fixture:&str,operation:&'static str,which:usize,expected:[usize;2]){
    let path=std::path::PathBuf::from(std::env::var("SEMIO_MESH_UNION_FIXTURE").unwrap()).with_file_name(fixture);
    let neutral:Value=serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let row=&neutral["cases"][which];
    let faces:Vec<Vec<u32>>=serde_json::from_value(neutral["faces"].clone()).unwrap();
    let cube=|bounds:&Value,faces:&[Vec<u32>]|{
        let [[x,y,z],[a,b,c]]:[[f32;3];2]=serde_json::from_value(bounds.clone()).unwrap();
        json!({"vertices":[[x,y,z],[a,y,z],[a,b,z],[x,b,z],[x,y,c],[a,y,c],[a,b,c],[x,b,c]],"faces":faces}).to_string()
    };
    let mut stock_faces=faces.clone();
    if let Some(i)=row["splitStockOpening"].as_u64(){
        let i=i as usize;
        let [a,b,c,d]:[u32;4]=stock_faces[i].as_slice().try_into().unwrap();
        stock_faces.splice(i..=i, [vec![a,b,c],vec![a,c,d]]);
    }
    let left=cube(&neutral["left"],&stock_faces);
    let right=cube(&row["right"],&faces);
    run_text_pair(&left,&right,operation,expected);
}

fn run_text_pair(left:&str,right:&str,operation:&'static str,expected:[usize;2]){
    let source=finish_text_pair(left,right,operation);
    assert_eq!([source.vertices.len(),source.faces.len()],expected);
}

fn finish_text_pair(left:&str,right:&str,operation:&'static str)->semio_framework_mesh_engine::PolygonMeshSource {
    let inputs=[left,right];
    let stock=RetainedCloneGrant{maximum_items:100_000,maximum_copy_bytes:1<<27,maximum_capacity_bytes:1<<27,maximum_release_bytes:1<<27,maximum_depth:256};
    let mut wallet=AuthoringWallet::new(stock,1<<28);
    let mut pair=production::CombineSource::new(operation);
    assert_eq!(pair.phase(),"mesh-combine-left");
    for _ in 0..100_000 {
        let d=pair.demand(inputs,4096).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,
            maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let staged=wallet.run_invoked(grant,128,|effect|pair.step(inputs,effect)).unwrap().unwrap().output;
        let receipt=pair.take_progress();
        assert!(receipt.fits(grant));
        wallet.post(receipt).unwrap();
        if let Some(source)=staged.unwrap_or_else(|error|panic!("{}",error.0)){
            assert_eq!(pair.phase(),format!("mesh-{operation}"));
            return source;
        }
    }
    panic!("retained Flow pair did not settle");
}

#[test]
fn retained_flow_union_publishes_a_closed_boundary(){
    run_pair("➕️convex-union.json","union",0,[12,10]);
}

#[test]
fn retained_flow_corner_difference_publishes_a_closed_boundary(){
    run_pair("➖️convex-difference.json","difference",6,[14,11]);
}

#[test]
fn retained_flow_slab_difference_publishes_two_closed_shells(){
    run_pair("➖️convex-difference.json","difference",7,[16,12]);
}

#[test]
fn retained_flow_octant_difference_publishes_a_closed_boundary(){
    run_pair("➖️convex-difference.json","difference",12,[19,15]);
}

#[test]
fn retained_flow_tunnel_difference_publishes_a_closed_boundary(){
    run_pair("➖️convex-difference.json","difference",16,[24,20]);
}

#[test]
fn retained_flow_blind_pocket_difference_publishes_a_closed_boundary(){
    run_pair("➖️convex-difference.json","difference",20,[28,22]);
}

#[test]
fn retained_flow_enclosed_difference_keeps_two_closed_shells(){
    run_pair("➖️convex-difference.json","difference",23,[16,12]);
}

#[test]
fn retained_flow_partial_union_stitches_both_authored_boundaries(){
    run_pair("➕️convex-union.json","union",7,[24,20]);
}

#[test]
fn retained_flow_union_stitches_five_crossing_supports(){
    run_pair("➕️convex-union.json","union",12,[28,22]);
}

#[test]
fn retained_flow_nonprismatic_recesses_keep_one_closed_surface(){
    run_pair("➖️convex-difference.json","difference",15,[20,16]);
    run_pair("➖️convex-difference.json","difference",19,[24,19]);
}

#[test]
fn retained_flow_pocket_preserves_split_stock_opening(){
    run_pair("➖️convex-difference.json","difference",24,[28,24]);
}

#[test]
fn retained_flow_prismatic_difference_keeps_all_authored_cap_faces() {
    let path=std::path::PathBuf::from(std::env::var("SEMIO_MESH_UNION_FIXTURE").unwrap())
        .with_file_name("🔺️prismatic-difference.json");
    let law:Value=serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let left=json!({"vertices":law["stock"]["positions"],"faces":law["stock"]["faces"]}).to_string();
    for row in law["cases"].as_array().unwrap() {
        let spec=&law["profiles"][row["profile"].as_str().unwrap()];
        let profile:Vec<[f32;2]>=serde_json::from_value(spec["footprint"].clone()).unwrap();
        let heights:[f32;2]=serde_json::from_value(row["height"].clone()).unwrap();
        let vertices:Vec<_>=heights.into_iter().flat_map(|z|profile.iter().map(move |&[x,y]|[x,y,z])).collect();
        let right=json!({"vertices":vertices,"faces":spec["cutterFaces"]}).to_string();
        let expected=["vertices","faces"].map(|key|row[key].as_u64().unwrap() as usize);
        run_text_pair(&left,&right,"difference",expected);
    }
}


#[test]
fn retained_flow_difference_publishes_every_disconnected_shell(){
    let fixture=compound::fixture();
    for row in fixture["cases"].as_array().unwrap(){
        let [left,right]=compound::operands(&fixture["faces"],row,"vertices").map(|source|source.to_string());
        let result=finish_text_pair(&left,&right,"difference");
        compound::verify(&result.vertices,&result.faces,row);
    }
}


#[test]
fn retained_flow_union_sews_both_orders_of_placed_stock(){
    let fixture=compound::fixture_named("🧩️placed-union.json");
    for row in fixture["cases"].as_array().unwrap(){
        let sources=compound::operands(&fixture["faces"],row,"vertices").map(|source|source.to_string());
        for order in [[0,1],[1,0]]{
            let result=finish_text_pair(&sources[order[0]],&sources[order[1]],"union");
            compound::verify(&result.vertices,&result.faces,row);
        }
    }
}

#[test]
fn retained_flow_handles_the_contained_concave_cutter_in_both_orders(){
    let law=compound::fixture_named("🕳️nested-prismatic-difference.json");
    let sources=compound::prism_operands(&law,"vertices").map(|value|value.to_string());
    for (operation,expected) in [("difference",&law),("intersect",&law["intersection"]),("union",&law["union"])]{
        let output=finish_text_pair(&sources[0],&sources[1],operation);
        compound::verify(&output.vertices,&output.faces,expected);
        let output=finish_text_pair(&sources[1],&sources[0],operation);
        if operation=="difference"{assert!(output.faces.is_empty());}
        else{compound::verify(&output.vertices,&output.faces,expected);}
    }
}

#[test]
fn retained_flow_intersects_the_prism_through_one_original_cap(){
    let law=compound::fixture_named("🕳️nested-prismatic-difference.json");
    let [_,prism]=compound::prism_operands(&law,"vertices").map(|value|value.to_string());
    let slab=compound::prism_enclosure(&law,"clipped","vertices").to_string();
    for [a,b] in [[&slab,&prism],[&prism,&slab]]{
        let result=finish_text_pair(a,b,"intersect");
        compound::verify(&result.vertices,&result.faces,&law["clipped"]);
    }
}

#[test]
fn retained_flow_difference_splits_the_concave_prism(){
    let law=compound::fixture_named("🕳️nested-prismatic-difference.json");
    let [_,prism]=compound::prism_operands(&law,"vertices").map(|value|value.to_string());
    let cap=compound::prism_enclosure(&law,"split","vertices").to_string();
    let result=finish_text_pair(&prism,&cap,"difference");
    compound::verify(&result.vertices,&result.faces,&law["split"]);
}

#[test]
fn retained_flow_clips_the_prism_along_one_original_wall(){
    let law=compound::fixture_named("🕳️nested-prismatic-difference.json");
    let [_,prism]=compound::prism_operands(&law,"vertices").map(|value|value.to_string());
    let wall=compound::prism_enclosure(&law,"lateral","vertices").to_string();
    for (a,b,operation,expected) in [(&prism,&wall,"intersect","intersection"),
        (&wall,&prism,"intersect","intersection"),(&prism,&wall,"difference","difference")]{
        let result=finish_text_pair(a,b,operation);
        compound::verify(&result.vertices,&result.faces,&law["lateral"][expected]);
    }
}
