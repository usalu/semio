//! 🧪️ The snapshot's own laws: a fresh document names its schema, the addressing helpers find what
//! exists and nothing else, and the canonical insertion index keeps a collection sorted.

use super::*;

fn document() -> Wfc3dSnapshot {
    crate::examples::two_room_corridor::snapshot()
}

#[test]
fn a_default_document_is_empty_but_names_its_own_schema() {
    let snapshot = Wfc3dSnapshot::default();
    assert_eq!(snapshot.schema, WFC3D_DOCUMENT_SCHEMA);
    assert_eq!(snapshot.seed, 0);
    assert!(snapshot.slots.is_empty() && snapshot.edges.is_empty() && snapshot.tiles.is_empty() && snapshot.rules.is_empty());
}

#[test]
fn the_addressing_helpers_find_every_member_by_id() {
    let document = document();
    assert_eq!(slot_index(&document, "room-a"), Some(1));
    assert_eq!(slot_index(&document, "ghost"), None);
    assert_eq!(edge_index(&document, "edge-corridor-b"), Some(1));
    assert_eq!(tile_index(&document, "room"), Some(1));
    assert_eq!(rule_index(&document, "rule-room-corridor"), Some(0));
    assert_eq!(rule_index(&document, "rule-room-room"), None, "the corridor example admits exactly one pair and states nothing else");
}

/// 🔤️ The canonical index is a count of strictly-smaller ids, so inserting there keeps the vector
/// sorted and makes the position an author-independent function of the id alone.
#[test]
fn the_canonical_index_is_where_the_id_belongs_in_sorted_order() {
    let document = document();
    assert_eq!(canonical_slot_index(&document, "aardvark"), 0);
    assert_eq!(canonical_slot_index(&document, "room-c"), 3);
    assert_eq!(canonical_slot_index(&document, "room-ab"), 2, "\"room-ab\" sorts after \"corridor\" and \"room-a\", but BEFORE \"room-b\"");
    assert_eq!(canonical_edge_index(&document, "edge-a-b"), 0);
    assert_eq!(canonical_tile_index(&document, "stair"), 2);
    assert_eq!(canonical_rule_index(&document, "rule-aa"), 0);
}

/// 🥽️ A tile's media defaults to inline geometry, never to a child handle — a document that authored
/// no media must still be self-contained.
#[test]
fn tile_media_defaults_to_inline_geometry() {
    assert!(matches!(TileMedia3d::default(), TileMedia3d::Mesh { .. }));
    assert_eq!(Tile::default().weight, 0.0, "the derived Default is the zero record; every authored tile sets a real weight");
}

#[test]
fn every_example_slot_carries_a_positive_box() {
    for slot in &document().slots {
        assert!(slot.width > 0.0 && slot.height > 0.0 && slot.depth > 0.0, "slot {} must have a positive extent", slot.id);
    }
}

/// 🧸️ Every nested mesh identity reaches capture while inline coordinates remain unchanged.
#[test]
fn declared_nested_mesh_child_projection_preserves_full_identity_and_inline_media(){
 use semio_framework_schema_composition::{ArtifactCompositionFields,ChildRefFields,ChildRefVisitor};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🪆️mesh-child/🔣️.json")).expect("closed nested media contract");
 let text=|value:&serde_json::Value|value.as_str().expect("text").to_string();
 let tiles=fixture["tiles"].as_array().expect("tiles").iter().map(|row|{let media=&row["media"];Tile{id:text(&row["id"]),label:None,weight:1.0,media:match media["kind"].as_str().expect("kind"){
 "mesh"=>TileMedia3d::Mesh{positions:media["positions"].as_array().expect("positions").iter().map(|v|v.as_f64().expect("coordinate")).collect(),indices:media["indices"].as_array().expect("indices").iter().map(|v|u32::try_from(v.as_u64().expect("index")).expect("u32")).collect(),color:None},
 "meshChild"=>TileMedia3d::MeshChild{child:store::ArtifactChild::new(text(&media["childId"]),semio_framework_artifact_reference::ArtifactRef{artifact_id:text(&media["artifactId"]),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:text(&media["artifactKind"]),standard:text(&media["standard"]),subset:text(&media["subset"])}})},
 _=>panic!("closed media branch"),}}}).collect();
 let document=Wfc3dSnapshot{tiles,..Wfc3dSnapshot::default()};
 struct Projection{rows:Vec<serde_json::Value>}
 impl<'a> ChildRefVisitor<'a> for Projection{type Error=();fn step(&mut self)->Result<(),Self::Error>{Ok(())}fn child(&mut self,slot:&'static str,value:ChildRefFields<'a>)->Result<(),Self::Error>{self.rows.push(serde_json::json!([slot,value.child_id,value.artifact_id,value.artifact_kind,value.standard,value.subset]));Ok(())}}
 let mut projection=Projection{rows:Vec::new()};document.visit_child_refs(&mut projection).expect("bounded projection");
 assert_eq!(serde_json::json!(projection.rows),fixture["expected"]);
 let slots=Wfc3dSnapshot::child_slots();assert_eq!(slots.len(),1);assert_eq!(slots[0].name,fixture["slot"].as_str().expect("slot"));assert_eq!(slots[0].kind,fixture["kind"].as_str().expect("kind"));assert_eq!(slots[0].many,fixture["many"].as_bool().expect("many"));
 let TileMedia3d::Mesh{positions,indices,color}=&document.tiles[0].media else{panic!("original inline branch");};assert_eq!(serde_json::json!(positions),fixture["tiles"][0]["media"]["positions"]);assert_eq!(serde_json::json!(indices),fixture["tiles"][0]["media"]["indices"]);assert_eq!(*color,None);
 let oracle:Vec<serde_json::Value>=fixture["tiles"].as_array().expect("tiles").iter().filter_map(|row|{let media=&row["media"];(media["kind"]=="meshChild").then(||serde_json::json!([fixture["slot"],media["childId"],media["artifactId"],media["artifactKind"],media["standard"],media["subset"]]))}).collect();assert_eq!(serde_json::json!(oracle),serde_json::json!(projection.rows));
 eprintln!("[DEBUG] Declared nested tile media preserves complete mesh child identity and original inline coordinates");
}


/// 👁️ Typed nested admission preserves exact identities and the original refusal.
#[test]
fn declared_nested_mesh_child_admission_requires_exact_type_dialect_and_retained_prefix() {
    use semio_framework_schema_composition::{ChildFieldReadAdmission,ChildReadSource,ChildRefFields};
    use semio_framework_value::ValueRefusalKind;
    #[derive(Debug,PartialEq)]
    struct Refusal { kind:ValueRefusalKind, ordinal:usize }
    struct Source { accepted:Vec<serde_json::Value>, maximum:usize }
    impl ChildReadSource for Source {
        type Error=Refusal;
        fn step(&mut self)->Result<(),Self::Error>{Ok(())}
        fn child<S:Send+Sync+'static>(&mut self,slot:&'static str,fields:ChildRefFields<'_>)->Result<(),Self::Error>{
            let ordinal=self.accepted.len();
            if std::any::TypeId::of::<S>()!=std::any::TypeId::of::<SemioMeshSnapshot>()||fields.artifact_kind!="s.stdio.semio"||fields.standard!="v1"||fields.subset!="mesh"{return Err(Refusal{kind:ValueRefusalKind::InvalidValue,ordinal});}
            if ordinal==self.maximum{return Err(Refusal{kind:ValueRefusalKind::OwnershipLimit,ordinal});}
            self.accepted.push(serde_json::json!([slot,fields.child_id,fields.artifact_id,fields.artifact_kind,fields.standard,fields.subset]));
            Ok(())
        }
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🪆️mesh-child/🔣️.json")).expect("closed admission contract");
    let row=&fixture["tiles"][1]["media"];
    let text=|value:&serde_json::Value|value.as_str().expect("identity").to_string();
    let child=store::ArtifactChild::new(text(&row["childId"]),semio_framework_artifact_reference::ArtifactRef{artifact_id:text(&row["artifactId"]),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:text(&row["artifactKind"]),standard:text(&row["standard"]),subset:text(&row["subset"])}});
    let mut second=child.clone();second.child_id="second-mesh-child引用😀".to_string();second.target.artifact_id="second-mesh-artifact引用😀".to_string();
    let second_expected=serde_json::json!(["tiles","second-mesh-child引用😀","second-mesh-artifact引用😀","s.stdio.semio","v1","mesh"]);
    let mut tiles=vec![Tile{media:TileMedia3d::default(),..Tile::default()},Tile{media:TileMedia3d::MeshChild{child:child.clone()},..Tile::default()},Tile{media:TileMedia3d::MeshChild{child:second},..Tile::default()}];
    for maximum in [0,1] {
        let mut source=Source{accepted:Vec::new(),maximum};
        assert_eq!(tiles.admit_child_field("tiles",&mut source),Err(Refusal{kind:ValueRefusalKind::OwnershipLimit,ordinal:maximum}));
        assert_eq!(source.accepted.len(),maximum);
        if maximum==1{assert_eq!(source.accepted[0],fixture["expected"][0]);}
    }
    let mut source=Source{accepted:Vec::new(),maximum:2};
    tiles.admit_child_field("tiles",&mut source).expect("exact snapshot source");
    assert_eq!(source.accepted,vec![fixture["expected"][0].clone(),second_expected]);
    let TileMedia3d::MeshChild{child}=&mut tiles[2].media else{panic!("second declared child");};
    child.target.dialect.subset="image".to_string();
    let mut source=Source{accepted:Vec::new(),maximum:2};
    assert_eq!(tiles.admit_child_field("tiles",&mut source),Err(Refusal{kind:ValueRefusalKind::InvalidValue,ordinal:1}));
    assert_eq!(source.accepted,vec![fixture["expected"][0].clone()]);
    eprintln!("[DEBUG] Exact Semio mesh admission retains accepted identity prefix and typed original refusal");
}

