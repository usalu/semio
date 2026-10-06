use crate::standards::v1::subsets::any::io::text::snapshot::*;
use crate::GENERATION_3D_SCHEMA;
use semio_framework_os_kernel::os_store::test_support;
use store::ArtifactDsl;

#[test]
fn dsl_round_trip_empty_projection() {
    test_support::assert_dsl_round_trip_cold(&Generation3dSnapshot::default(), Generation3dSnapshot::retire_cold);
    test_support::assert_dsl_pack_equivalence_cold(&Generation3dSnapshot::default(), Generation3dSnapshot::retire_cold);
}

#[test]
fn dsl_round_trip_every_bundled_example() {
    for text in [
        GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT,
        GENERATION3D_EXAMPLE_RECT_EXTRUDE_TEXT,
        GENERATION3D_EXAMPLE_SPHERE_TORUS_TEXT,
        GENERATION3D_EXAMPLE_BOX_FILLET_TEXT,
        GENERATION3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT,
        GENERATION3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT,
        GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT,
        GENERATION3D_EXAMPLE_BOX_SHELL_TEXT,
    ] {
        let projection = crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new(Generation3dSnapshot::parse_dsl(text).expect("parse bundled example"));
        test_support::assert_dsl_round_trip_cold(&*projection, Generation3dSnapshot::retire_cold);
        test_support::assert_dsl_pack_equivalence_cold(&*projection, Generation3dSnapshot::retire_cold);
    }
}

#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

    let mut store = crate::store_fixture::document_store(Generation3dSnapshot::default()).await;
    use crate::standards::v1::subsets::any::schema::mutations::create_widget::CreateWidget;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![Generation3dMutation::CreateWidget(CreateWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } })], transaction: None }).await.expect("apply");
    let edit: &Edit<Generation3dMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    test_support::assert_command_envelope_round_trip::<Generation3dSnapshot, Generation3dMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
    crate::store_fixture::close(store);
}

#[test]
fn retained_snapshot_text_projection_borrows_original_graph_and_ranked_generation_values(){
    use semio_framework_dsl_record::native_encoding::RetainedFieldProjection;use semio_framework_value::{NativeEncodeControl,ValueRefusalKind,SnapshotRetirementStep,DslValue};use semio_framework_value::retirement::owned_retirement;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧫️fixtures/🧵️continuation/🔣️.json")).unwrap();
    let text=fixture["textUnit"].as_str().unwrap().repeat(fixture["projection"]["textRepeats"].as_u64().unwrap()as usize);
    for source_text in [GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT,GENERATION3D_EXAMPLE_RECT_EXTRUDE_TEXT,GENERATION3D_EXAMPLE_MESH_WORKBENCH_TEXT]{
        let mut source=crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new(parse_dsl(source_text).unwrap());
        source.host_snapshot.widgets.push(Widget::InputNote{id:"large-escaped-note".into(),text:text.clone()});
        let params=Dictionary::new().insert("null",NeuralValue::Atom(Atom::Null)).insert("boolean",NeuralValue::Atom(Atom::Boolean(true))).insert("integer",NeuralValue::Atom(Atom::Integer(42))).insert("decimal",NeuralValue::Atom(Atom::Decimal(1.5))).insert("text",NeuralValue::Atom(Atom::String(text.clone()))).insert("nested",NeuralValue::Dictionary(Dictionary::new().insert("original",NeuralValue::Atom(Atom::String(text.clone())))));
        let tree=Tree{neurons:vec![Neuron{id:"source-neuron".into(),kind:"source.kind".into(),params:params.clone(),tree:Some(Box::new(Tree{neurons:vec![Neuron{id:"nested-neuron".into(),kind:"nested.kind".into(),params:params.clone(),tree:None}],synapses:Vec::new()}))}],synapses:vec![Synapse{id:"source-edge".into(),from:"source-neuron".into(),to:"nested-neuron".into(),from_port:"source-port".into(),to_port:"target-port".into()}]};
        let mut flow=FlowUi{camera:CameraJson{x:1.5,y:-2.5,zoom:0.75},nodes:OrderedMap::new(),previews:vec![FlowPreviewGui{id:"source-preview".into(),source:Some(FlowChannelRef{neuron:"source-neuron".into(),channel:"source-port".into()}),mode:"mesh".into(),preview:params.clone(),expanded:["z".into(),"null".into(),text.clone()].into(),layout:Some(WidgetLayout{x:3.5,y:-4.5})},FlowPreviewGui{id:"empty-preview".into(),source:None,mode:"text".into(),preview:Dictionary::new(),expanded:OrderedSet::new(),layout:None}]};
        for (index,chrome) in [NodeChrome::Plain{preview:true},NodeChrome::Slider{label:text.clone(),min:-1.0,max:2.0,step:0.125,value:1.5},NodeChrome::Note{text:text.clone()},NodeChrome::Image{src:text.clone()},NodeChrome::Variable{name:text.clone(),schema:"number".into()}].into_iter().enumerate(){flow.nodes.insert(format!("original-node-{index}"),FlowNodeGui{layout:WidgetLayout{x:index as f64,y:-1.5},chrome});}
        source.host_snapshot.widgets.extend([Widget::Neuron{id:"original-params".into(),neuron_kind:"source.kind".into(),preview:true,input_ports:vec![text.clone()],output_ports:vec!["null".into()],params},Widget::InputImage{id:"original-image".into(),src:text.clone()},Widget::Variable{id:"original-variable".into(),name:text.clone(),schema:"number".into()},Widget::OutputAction{id:"original-action".into(),action:text.clone()},Widget::Cluster{id:"original-cluster".into(),name:text.clone(),tree,flow}]);
        source.generation.cold_builder_mut().unwrap().generations.push(FormGeneration{id:"retained-values".into(),name:text.clone(),values:[("original-é-🧊".into(),DslValue::Object(vec![("nested".into(),DslValue::Array(vec![DslValue::String(text.clone()),DslValue::Bytes(vec![0,255,42])]))]))].into()});
        let expected=generation3d_document_to_dsl(&source).__dsl_to_record();assert!(print_dsl(&source).contains(&serde_json::to_string(&text).unwrap()));
        for budget in fixture["budgets"].as_array().unwrap(){let mut cursor=RetainedFieldProjection::new(&*source);let mut accept=|_|true;let mut control=NativeEncodeControl::new(33554432,&mut accept);assert!(cursor.step(&*source,1,&mut control).unwrap().is_none());let actual=loop{let before=cursor.position();assert!(cursor.step(&*source,0,&mut control).unwrap().is_none());assert_eq!(cursor.position(),before);if let Some(value)=cursor.step(&*source,budget.as_u64().unwrap()as usize,&mut control).unwrap(){break value}};assert_eq!(actual,semio_framework_dsl_record::FieldValue::Record(expected.clone()));let mut close=owned_retirement(actual);while !close.terminal_is_empty(){close.close_step(1,3).unwrap();}let mut close=owned_retirement(cursor);while !close.terminal_is_empty(){if let SnapshotRetirementStep::Pending{released_bytes,..}=close.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}}
        for stop in [0,1,8,64,256]{let mut cursor=RetainedFieldProjection::new(&*source);let live=std::cell::Cell::new(true);let mut accept=|_|live.get();let mut control=NativeEncodeControl::new(33554432,&mut accept);for _ in 0..stop{assert!(cursor.step(&*source,1,&mut control).unwrap().is_none())}let before=cursor.position();live.set(false);assert_eq!(cursor.step(&*source,1,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(cursor.position(),before);let mut close=owned_retirement(cursor);while !close.terminal_is_empty(){close.close_step(1,3).unwrap();}}
    }
    eprintln!("[DEBUG] Original Generation3d graph text source budgets1/8/256, ranked generations/layout/dictionaries, zero/cancel and3byte retirement passed");
}
