//! 🔬️ The viewer's preview renders the authored grid over the same pure projection the editor uses.

use crate::viewer::grid3d::modes::view::windows::preview::*;

#[test]
fn every_example_renders_a_non_empty_authored_scene() {
    for document in [crate::examples::blocks::snapshot(), crate::examples::pipes_3d::snapshot()] {
        let node = render(&document).expect("viewer preview renders");
        assert!(!format!("{node:?}").is_empty());
    }
}

#[test]
fn the_opening_pose_frames_the_document() {
    let document = crate::examples::blocks::snapshot();
    let (position, target) = framed_camera(&document);
    assert_ne!(position, target);
}

#[test]
fn portable_authored_preview_never_invents_an_unsolved_assignment() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️authored-preview/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let mut document=Grid3dSnapshot::default();
        let size=row["size"].as_array().unwrap();
        document.width=size[0].as_u64().unwrap() as u32;
        document.height=size[1].as_u64().unwrap() as u32;
        document.depth=size[2].as_u64().unwrap() as u32;
        document.cell_sizes_x=vec![1.0;document.width as usize];
        document.cell_sizes_y=vec![1.0;document.height as usize];
        document.cell_sizes_z=vec![1.0;document.depth as usize];
        document.tiles=row["tiles"].as_array().unwrap().iter().map(|id|crate::Grid3dTile {id:id.as_str().unwrap().into(),weight:1.0,..Default::default()}).collect();
        document.pinned=row["pins"].as_array().unwrap().iter().map(|pin|crate::Grid3dPinnedCell {x:pin[0].as_u64().unwrap() as u32,y:pin[1].as_u64().unwrap() as u32,z:pin[2].as_u64().unwrap() as u32,tile_id:pin[3].as_str().unwrap().into()}).collect();
        document.masked=row["masked"].as_array().unwrap().iter().map(|cell|crate::Grid3dCell {x:cell[0].as_u64().unwrap() as u32,y:cell[1].as_u64().unwrap() as u32,z:cell[2].as_u64().unwrap() as u32}).collect();
        let observed=authored_assignments(&document).into_iter().map(|((x,y,z),tile)|serde_json::json!([x,y,z,tile])).collect::<Vec<_>>();
        assert_eq!(serde_json::Value::Array(observed),row["expected"],"{}",row["id"]);
        render(&document).expect("authored viewer surface");
    }
}
