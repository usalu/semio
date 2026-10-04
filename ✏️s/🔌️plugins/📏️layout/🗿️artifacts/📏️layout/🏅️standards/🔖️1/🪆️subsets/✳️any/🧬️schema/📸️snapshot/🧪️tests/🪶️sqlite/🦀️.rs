//! 📏️ Exact Layout native capability before the owned relational implementation.
use super::*;
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_layout_actual_declaration_publishes_exact_native_codec(){
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("layout").label("Layout owned SQLite").version("0.0.1").package_id("semio:layout").artifact(crate::declaration().unwrap()).try_build().unwrap();
    let codec=store::document_codec(LAYOUT_DOCUMENT_SCHEMA).await.unwrap().unwrap();
    let provider=codec.snapshot_sqlite.expect("actual Layout declaration must publish semantic SQLite capability");
    assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<LayoutSnapshot>()));
    let snapshot=crate::standards::v1::subsets::any::schema::default_document();
    let dialect=store::os_io::ArtifactDialect{artifact_kind:"s.layout.layout".into(),standard:"1".into(),subset:"*".into()};
    let payload=store::os_io::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot));
    let database=(provider.export)(LAYOUT_DOCUMENT_SCHEMA,&dialect,&payload,&mut store::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,store::sqlite_snapshot::SqliteDatabaseLimits::default())).unwrap().value;
    assert!(database.table("layout_document").is_ok());
}
#[test]
fn sqlite_snapshot_layout_declared_owner_capability(){let snapshot=crate::standards::v1::subsets::any::schema::default_document();let provider=<LayoutSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().expect("Layout must publish its complete typed semantic SQLite capability");let dialect=store::os_io::ArtifactDialect{artifact_kind:"s.layout.layout".into(),standard:"1".into(),subset:"*".into()};let payload=store::os_io::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot));let result=(provider.export)(LAYOUT_DOCUMENT_SCHEMA,&dialect,&payload,&mut store::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,store::sqlite_snapshot::SqliteDatabaseLimits::default())).unwrap();assert!(result.value.table("layout_document").is_ok());}

fn component_fixture()->crate::LayoutDrawingChild {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas,DrawLayer,DrawNode,SemioPoint2};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    let identity=fixture["childIdentity"].as_array().unwrap();
    let target=store::os_io::ArtifactRef{dialect:store::os_io::ArtifactDialect{artifact_kind:identity[1].as_str().unwrap().into(),standard:identity[2].as_str().unwrap().into(),subset:identity[3].as_str().unwrap().into()},artifact_id:identity[4].as_str().unwrap().into()};
    crate::LayoutDrawingChild{
        handle:store::ArtifactChild::new(identity[0].as_str().unwrap().into(),target),
        content:crate::SemioDrawingSnapshot{schema:fixture["drawingSchema"].as_str().unwrap().into(),canvas:DrawCanvas{width:-0.0,height:1.0,background:None},styles:vec![],layers:vec![DrawLayer{id:String::new(),name:String::new(),visible:false,root:DrawNode::Image{at:SemioPoint2{x:0.0,y:-0.0},width:1.0,height:2.0,mime:"image/private".into(),bytes:vec![0,127,255]}}]},
    }
}
#[test]
fn sqlite_snapshot_layout_inline_drawing_controlled_metadata(){
    let mut callback=|_|true;
    let mut decode=semio_framework_value::NativeDecodeControl::new(1024*1024,&mut callback);
    let shape=<crate::LayoutDrawingChild as semio_framework_dsl_record::DslField>::shape_controlled(&mut decode).unwrap();
    let semio_framework_dsl_record::Shape::Record(producer)=shape else{panic!("inline Drawing record")};
    let spec=producer.decode(&mut decode).unwrap();
    assert_eq!(spec.fields.len(),2);assert_eq!(spec.fields[0].key,"handle");assert_eq!(spec.fields[1].key,"content");
    let mut encode_callback=|_|true;let mut encode=semio_framework_value::NativeEncodeControl::new(1024*1024,&mut encode_callback);
    assert_eq!(producer.encode(&mut encode).unwrap().fields.len(),2);
    assert!(producer.decode(&mut semio_framework_value::NativeDecodeControl::new(1,&mut |_|true)).is_err());
    assert!(producer.encode(&mut semio_framework_value::NativeEncodeControl::new(1,&mut |_|true)).is_err());
}
#[test]
fn sqlite_snapshot_layout_inline_drawing_owned_fields_controlled_input_output(){
    let expected=component_fixture();let value=<crate::LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value(&expected);
    let restored=<crate::LayoutDrawingChild as semio_framework_dsl_record::DslField>::from_value_controlled(&value,&mut semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut |_|true)).unwrap();
    assert_eq!(restored,expected);assert_eq!(restored.content.canvas.width.to_bits(),(-0.0f64).to_bits());
    let output=<crate::LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value_controlled(&expected,&mut semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut |_|true)).unwrap();
    assert_eq!(output,value);
    assert!(<crate::LayoutDrawingChild as semio_framework_dsl_record::DslField>::from_value_controlled(&value,&mut semio_framework_value::NativeDecodeControl::new(1,&mut |_|true)).is_err());
    assert!(<crate::LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value_controlled(&expected,&mut semio_framework_value::NativeEncodeControl::new(1,&mut |_|true)).is_err());
    assert!(<crate::LayoutDrawingChild as semio_framework_dsl_record::DslField>::from_value_controlled(&value,&mut semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut |_|false)).is_err());
    assert!(<crate::LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value_controlled(&expected,&mut semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut |_|false)).is_err());
}
#[test]
fn sqlite_snapshot_layout_inline_drawing_long_native_copy_is_interior_controlled(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    let total=fixture["intrinsicBytes"].as_u64().unwrap() as usize;
    let mut expected=component_fixture();expected.content.schema="x".repeat(total);
    let value=<crate::LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value(&expected);
    let mut input=false;let mut input_callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{if event.total==total&&event.completed>=65536&&event.completed<total{input=true;false}else{true}};
    assert!(<crate::LayoutDrawingChild as semio_framework_dsl_record::DslField>::from_value_controlled(&value,&mut semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut input_callback)).is_err());
    assert!(input,"must cancel during literal inline Drawing ownership copy");
    let mut output=false;let mut output_callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{if event.total==total&&event.completed>=65536&&event.completed<total{output=true;false}else{true}};
    assert!(<crate::LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value_controlled(&expected,&mut semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut output_callback)).is_err());
    assert!(output,"must cancel during literal inline Drawing projection copy");
}

fn complete_fixture(word:f64,color:f32)->LayoutSnapshot{
    use crate::*;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    let maximum=fixture["maximumU64"].as_str().unwrap().parse::<u64>().unwrap();
    let bounds=LayoutBounds{x:word,y:word,width:word,height:word,rotation:word};let rect=LayoutRect{x:word,y:word,width:word,height:word};
    let layers=vec![Layer{id:String::new(),name:String::new(),visible:false,locked:true,object_ids:vec![String::new(),"!".into()]}];
    let frames=vec![Frame::Rect{id:String::new(),layer_id:String::new(),bounds:bounds.clone(),locked:None,visible:Some(false),fill:Some([color;4]),stroke:None},Frame::Text{id:String::new(),layer_id:String::new(),bounds:bounds.clone(),locked:Some(true),visible:None,story_id:String::new(),thread_next:Some(String::new()),columns:u32::MAX,inset:rect.clone(),wrap_mode:String::new()},Frame::Image{id:String::new(),layer_id:String::new(),bounds:bounds.clone(),locked:Some(false),visible:Some(true),link_id:String::new()}];
    let mut value=empty_layout_snapshot();value.schema=fixture["schema"].as_str().unwrap().into();value.grid=GridSettings{baseline_grid:word,baseline_offset:word,snap_to_baseline:false};
    value.paragraph_styles=vec![ParagraphStyle{id:String::new(),name:String::new(),font_family:String::new(),font_size:word,font_weight:u32::MAX,leading:word,tracking:word,alignment:String::new()}];
    value.character_styles=vec![CharacterStyle{id:String::new(),name:None,font_family:Some(String::new()),font_size:Some(word),font_weight:None,italic:Some(false),color:Some([color;4]),tracking:None}];
    value.stories=vec![TextStory{id:String::new(),content:"語\0".into(),style_runs:vec![TextStyleRun{start:maximum,end:0,paragraph_style_id:Some(String::new()),character_style_id:None}]}];
    value.links=vec![ImageLink{id:String::new(),path:String::new(),hash:String::new(),width:u32::MAX,height:0,dpi:1,color_profile:None,state:Some(String::new()),proxy_data_url:Some(String::new()),artifact_kind:fixture["imageArtifactKind"].as_str().unwrap().into(),artifact_ref:fixture["imageArtifactRef"].as_str().unwrap().into()}];
    value.parent_pages=vec![ParentPage{id:String::new(),name:String::new(),width:word,height:word,layer_ids:vec![String::new(),String::new()],layers:layers.clone(),frames:frames.clone()}];
    value.spreads=vec![Spread{id:String::new(),name:String::new(),page_ids:vec![String::new(),"!".into()]}];
    value.pages=vec![Page{id:String::new(),name:String::new(),spread_id:String::new(),parent_page_id:Some(String::new()),width:word,height:word,margins:PageMargins{top:word,right:word,bottom:word,left:word},columns:PageColumns{count:u32::MAX,gutter:word},guides:vec![rect],layer_ids:vec![String::new()],layers,frames,overrides:vec![PageOverride{object_id:String::new(),bounds:Some(bounds),visible:Some(false),locked:None},PageOverride{object_id:String::new(),bounds:None,visible:None,locked:Some(true)}]}];
    value.print_target=Some(String::new());value.data_fields=Some(crate::FormDictionary{entries:vec![
 crate::FormDictionaryEntry{question_id:String::new(),value:semio_framework_value::DslValue::String("intrinsic data field\0".into())},
 crate::FormDictionaryEntry{question_id:"null".into(),value:semio_framework_value::DslValue::Null},
 crate::FormDictionaryEntry{question_id:"boolean".into(),value:semio_framework_value::DslValue::Bool(true)},
 crate::FormDictionaryEntry{question_id:"unsigned".into(),value:semio_framework_value::DslValue::uint(u64::MAX)},
 crate::FormDictionaryEntry{question_id:"signed".into(),value:semio_framework_value::DslValue::int(i64::MIN)},
 crate::FormDictionaryEntry{question_id:"float".into(),value:semio_framework_value::DslValue::float(word)},
 crate::FormDictionaryEntry{question_id:"bytes".into(),value:semio_framework_value::DslValue::Bytes(vec![0,127,128,255])},
 crate::FormDictionaryEntry{question_id:"array".into(),value:semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::Null,semio_framework_value::DslValue::uint(u64::MAX)])},
 crate::FormDictionaryEntry{question_id:"object".into(),value:semio_framework_value::DslValue::Object(vec![("repeat".into(),semio_framework_value::DslValue::float(word)),(String::new(),semio_framework_value::DslValue::Bytes(vec![0,255])),("repeat".into(),semio_framework_value::DslValue::Null)])},
 ]});value.background_drawing=Some(component_fixture());
    value.referenced_model=Some(store::ArtifactLink{target:store::os_io::ArtifactRef{dialect:store::os_io::ArtifactDialect{artifact_kind:String::new(),standard:"!/@".into(),subset:String::new()},artifact_id:String::new()},role:String::new(),pin:store::LinkPin::Snapshot{blob:store::BlobRef{hash:String::new(),size:maximum,media_type:String::new()}}});value
}
#[test]
fn sqlite_snapshot_layout_complete_owned_fields_words_and_literals_cross_both_native_encodings(){
    use store::{ArtifactDsl,ArtifactPack};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    let codec=<LayoutSnapshot as ArtifactPack>::sqlite_snapshot_codec().expect("complete Layout native capability");
    let dialect=store::os_io::ArtifactDialect{artifact_kind:"s.layout.layout".into(),standard:"1".into(),subset:"*".into()};
    for(index,bits)in fixture["float64Bits"].as_array().unwrap().iter().enumerate(){
        let word=u64::from_str_radix(bits.as_str().unwrap(),16).unwrap();let color=u32::from_str_radix(fixture["float32Bits"][index].as_str().unwrap(),16).unwrap();let expected=complete_fixture(f64::from_bits(word),f32::from_bits(color));
        for encoding in [store::sqlite_snapshot::SnapshotEncoding::Binary,store::sqlite_snapshot::SnapshotEncoding::Text]{
            let payload=match encoding{store::sqlite_snapshot::SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(expected.encode_pack()),store::sqlite_snapshot::SnapshotEncoding::Text=>store::os_io::IoPayload::Text(expected.print_dsl())};
            let mut callback=|_|true;let mut control=store::sqlite_snapshot::SqliteSnapshotControl::new(&mut callback,store::sqlite_snapshot::SqliteDatabaseLimits::default());
            let database=(codec.export)(LAYOUT_DOCUMENT_SCHEMA,&dialect,&payload,&mut control).unwrap().value;
            assert_eq!(database.table("layout_frame").unwrap().rows.len(),6);assert_eq!(database.table("layout_image_link").unwrap().rows[0].text(12).unwrap(),fixture["imageArtifactKind"].as_str().unwrap());
            assert_eq!(database.table("layout_scalar64").unwrap().rows[0].integer(2).unwrap() as u64,word);
            let result=(codec.import)(LAYOUT_DOCUMENT_SCHEMA,&dialect,database,encoding,&mut control).unwrap().value;
            let restored=match result{store::os_io::IoPayload::Binary(bytes)=>LayoutSnapshot::decode_pack(&bytes).unwrap(),store::os_io::IoPayload::Text(text)=>LayoutSnapshot::parse_dsl(&text).unwrap()};
            assert_eq!(restored.grid.baseline_grid.to_bits(),word);assert_eq!(restored.character_styles[0].color.unwrap()[0].to_bits(),color);assert_eq!(restored.stories[0].style_runs[0].start,expected.stories[0].style_runs[0].start);assert_eq!(restored.links,expected.links);assert_eq!(restored.background_drawing.as_ref(),expected.background_drawing.as_ref());assert_eq!(restored.referenced_model,expected.referenced_model);assert_eq!(restored.data_fields,expected.data_fields);
            assert!(restored.encode_pack()==expected.encode_pack(),"complete derived owner fields must survive relational reconstruction");
        }
    }
}
#[test]
fn sqlite_snapshot_layout_deep_inline_drawing_component_has_owned_controlled_boundaries(){
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::DrawNode;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();let depth=fixture["componentDepth"].as_u64().unwrap() as usize;
    let mut expected=component_fixture();let mut node=std::mem::take(&mut expected.content.layers[0].root);
    for _ in 0..depth{let mut parent=DrawNode::default();let DrawNode::Group{children,..}=&mut parent else{panic!("declared default group")};children.push(node);node=parent;}
    expected.content.layers[0].root=node;
    let value=<crate::LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value_controlled(&expected,&mut semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut |_|true)).unwrap();
    let restored=<crate::LayoutDrawingChild as semio_framework_dsl_record::DslField>::from_value_controlled(&value,&mut semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut |_|true)).unwrap();
    assert_eq!(restored.handle,expected.handle);let mut current=&restored.content.layers[0].root;let mut count=0;
    while let DrawNode::Group{children,..}=current{assert_eq!(children.len(),1);current=&children[0];count+=1;}
    assert_eq!(count,depth);let DrawNode::Image{bytes,..}=current else{panic!("complete image leaf")};assert_eq!(bytes,&[0,127,255]);
    <crate::SemioDrawingSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(expected.content);<crate::SemioDrawingSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(restored.content);
}

#[test]
fn sqlite_snapshot_layout_inline_drawing_all_node_fields_preserve_ieee_words(){
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode,PathSegment,SemioPoint2};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioTransform;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    for bits in fixture["float64Bits"].as_array().unwrap(){
        let word=u64::from_str_radix(bits.as_str().unwrap(),16).unwrap();let number=f64::from_bits(word);let point=SemioPoint2{x:number,y:-0.0};
        let mut transform=SemioTransform::identity();transform.translation.x=number;
        let mut expected=component_fixture();expected.content.layers[0].root=DrawNode::Group{transform,children:vec![
            DrawNode::Path{segments:vec![PathSegment::MoveTo{to:point},PathSegment::LineTo{to:point},PathSegment::CubicTo{c1:point,c2:point,to:point},PathSegment::QuadTo{c:point,to:point},PathSegment::ArcTo{rx:number,ry:number,x_rotation:number,large_arc:false,sweep:true,to:point},PathSegment::Close],style:Some(String::new())},
            DrawNode::Text{value:"English Deutsch\0文".into(),at:point,style:None},
            DrawNode::Image{at:point,width:number,height:-0.0,mime:String::new(),bytes:vec![0,127,255]},
        ]};
        let output=<crate::LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value_controlled(&expected,&mut semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut |_|true)).unwrap();
        let restored=<crate::LayoutDrawingChild as semio_framework_dsl_record::DslField>::from_value_controlled(&output,&mut semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut |_|true)).unwrap();
        assert_eq!(restored.handle,expected.handle);
        assert!(store::ArtifactPack::encode_pack(&restored.content)==store::ArtifactPack::encode_pack(&expected.content),"every authored Drawing node and path field must remain exact");
        let DrawNode::Group{transform,children}=&restored.content.layers[0].root else{panic!("literal group")};
        assert_eq!(transform.translation.x.to_bits(),word);
        let DrawNode::Image{width,bytes,..}=&children[2] else{panic!("literal image")};assert_eq!(width.to_bits(),word);assert_eq!(bytes,&[0,127,255]);
        <crate::SemioDrawingSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(expected.content);<crate::SemioDrawingSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(restored.content);
    }
}

#[path="💰️backing/🦀️.rs"]mod backing;
