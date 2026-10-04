
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
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️mesh-child/🔣️.json")).expect("closed admission contract");
    let row=&fixture["tiles"][1]["media"];
    let text=|value:&serde_json::Value|value.as_str().expect("identity").to_string();
    let child=store::ArtifactChild::new(text(&row["childId"]),store::os_io::ArtifactRef{artifact_id:text(&row["artifactId"]),dialect:store::os_io::ArtifactDialect{artifact_kind:text(&row["artifactKind"]),standard:text(&row["standard"]),subset:text(&row["subset"])}});
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

