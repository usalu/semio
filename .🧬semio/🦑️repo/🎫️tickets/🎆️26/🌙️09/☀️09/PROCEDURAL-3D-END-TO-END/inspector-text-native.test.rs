extern crate self as semio_ui_contract;
extern crate self as semio_s_procedural_core_semantic_ui;
extern crate self as semio_s_artifact_procedural_3d_set_widget_input;
const GENERATION_3D_PLAY_APP_ID:&str="procedural3d";

mod plugin_app_close_prelude {
    pub(crate) use crate::{UiValue, UI_TEXT_MAX_BYTES, UI_VALUE_MAX_ITEMS};
}
mod semio_s_artifact_procedural_3d_terminology {
    use crate::Generation3dLabels;
    pub fn generation3d_input_name<'a>(_: &Generation3dLabels, _: &str, label:&'a str)->&'a str { label }
}
struct PluginAssemblyError;
impl PluginAssemblyError { fn new(code:&'static str,_:&str)->&'static str {code} }
type UiValuePair = (&'static str, UiValue);
enum UiValue { Text(String), Number(f64), Map(Vec<UiValuePair>) }
fn ui_value_text(value:&str)->UiAssemblyResult<UiValue>{Ok(UiValue::Text(ui_text(value)?.0))}
fn ui_value_map(values: impl IntoIterator<Item=UiValuePair>) -> UiAssemblyResult<UiValue> { Ok(UiValue::Map(values.into_iter().collect())) }
#[derive(Clone,Copy)]
enum InputKind { LongText }
fn input(_:InputKind)->BuiltNode { BuiltNode{kind:"input",..Default::default()} }
impl BuiltNode {
    fn try_child(self,child:BuiltNode)->Result<Self,()> { ui_child(self,child).map_err(|_|()) }
    fn try_build(self)->Result<Self,()> { Ok(self) }
    fn value(mut self,value:UiText)->Self { self.source=Some((value.0,String::new())); self }
    fn commit(mut self,value:UiText)->Self { self.source.as_mut().unwrap().1=value.0; self }
    fn try_label(mut self,label:&str)->Result<Self,()> { self.label=label.into();Ok(self) }
    fn try_with_children(mut self,children:[BuiltNode;1])->Result<Self,()> { self.children.extend(children); Ok(self) }
}
fn generation_text_action(mut control:BuiltNode,_:&str,action:&str,args:UiValue)->UiAssemblyResult<BuiltNode>{
    assert_eq!(action,"setWidgetInput");
    let UiValue::Map(args)=args else { panic!("input arg map") };
    let fields=args.into_iter().map(|(key,value)| { let value=match value{UiValue::Text(value)=>value,UiValue::Number(value)=>value.to_string(),_=>panic!("scalar arg")};format!("{key}={value}") }).collect::<Vec<_>>();
    control.hover=Some(fields.join(";"));
    Ok(control)
}
pub(crate) fn input_value_body_key(text:&str)->String { format!("framework.input.value.{}.{:016x}",text.len(),window_resource::WindowResource::owner_digest("",text)) }
pub(crate) fn paged_text_carrier(key:&str,value:&str)->UiAssemblyResult<BuiltNode>{Ok(BuiltNode{kind:"carrier",key:key.into(),label:value.into(),..Default::default()})}

struct Generation3dLabels { input_value_unavailable:String }
fn inspector_hover_row(key:&str,label:impl AsRef<str>,hover:Option<(GraphTargetKind,&str)>)->UiAssemblyResult<TreeItemBuilder>{
    let mut row=ui_id(tree_item_builder(ui_label(label.as_ref())?),key)?;
    row.hover=hover.map(|(kind,key)|kind.target(key));
    Ok(row)
}
fn test_text(root:&BuiltNode,out:&mut Vec<String>){
    if root.key=="content" { out.push(root.label.clone()); }
    for node in &root.children { test_text(node,out); }
}
fn main(){
    use semio_framework_pack_json::{parse,to_dsl_value,JsonMemberPolicy,Value};
    use semio_framework_value::DslValue;
    let fixture=parse(&std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(),JsonMemberPolicy::Reject).unwrap();
    let value=fixture["display"]["value"].as_str().unwrap();
    let expected_integer=fixture["display"]["integerSpelling"].as_str().unwrap();
    let int=parse(expected_integer,JsonMemberPolicy::Reject).unwrap();
    assert_eq!(reflected_value(&int).as_deref(),Some(expected_integer));

    let windows=TreeWindows{resource_scope:None,path:Default::default(),reserved:std::cell::Cell::new(0),remaining:std::cell::Cell::new(30)};
    let labels=Generation3dLabels{input_value_unavailable:"Unavailable".into()};
    let output=reflected_output(&windows,"output","Result",&Value::String(value.into()),0,&labels,Some((GraphTargetKind::Handle,"note@text")), false).unwrap();
    assert_eq!(output.hover.as_deref(),Some("h:note@text"));
    assert!(output.children.len()>1);
    assert!(output.children.iter().all(|child|child.hover.as_deref()==Some("h:note@text")));
    let mut observed=Vec::new();
    test_text(&output,&mut observed);
    assert_eq!(observed.concat(),value);
    let short=reflected_output(&windows,"output","Result",&Value::String("short".into()),0,&labels,None, false).unwrap();
    assert_eq!(short.label,"Result: short");

    let boundary="é".repeat(128);
    assert_eq!(reflected_value(&Value::String(boundary.clone())).as_deref(),Some(boundary.as_str()));
    assert!(reflected_value(&Value::String(boundary + "x")).is_none());

    let first=fixture["edit"]["first"].as_str().unwrap();
    let next=fixture["edit"]["next"].as_str().unwrap();
    let args=||ui_value_map([("widgetId",UiValue::Text("note".into())),("channel",UiValue::Text("text".into()))]).unwrap();
    let control=retained_text_control("note.input","Value",InputKind::LongText,first,"procedural3d","setWidgetInput",args()).unwrap();
    let descriptor=control.source.as_ref().unwrap();
    assert_eq!(control.key,"note.input");
    assert_eq!(control.label,"Value");
    assert_eq!(descriptor.0,control.children[0].key);
    assert_eq!(descriptor.1,"blurRetainedBody");
    assert_eq!(control.children[0].label,first);
    assert_eq!(control.hover.as_deref(),Some("widgetId=note;channel=text"));
    let field=editable_text_input(&windows,"item.4","Item 5","operator","notes",Some(4),first,true).unwrap();
    assert_eq!(field.hover.as_deref(),Some("h:operator@!inotes"));
    assert_eq!(field.children[0].hover.as_deref(),Some("widgetId=operator;channel=notes;index=4;operation=set"));
    assert_eq!(windows.nodes_remaining(),30-paged_text_carrier_nodes(first));
    let hidden=TreeWindows{resource_scope:None,path:Default::default(),reserved:std::cell::Cell::new(0),remaining:std::cell::Cell::new(0)};
    let fallback=editable_text_input(&hidden,"item.4","Item 5","operator","notes",Some(4),first,true).unwrap();
    assert_eq!(fallback.hover.as_deref(),Some("h:operator@!inotes"));
    assert!(fallback.children.iter().all(|page|page.hover.as_deref()==Some("h:operator@!inotes")));
    let note=editable_text_input(&windows,"note","Note","note","text",None,"source",false).unwrap();
    assert_eq!(note.hover.as_deref(),Some("h:note@text"));
    let mut fallback_pages=Vec::new();
    test_text(&fallback,&mut fallback_pages);
    assert_eq!(fallback_pages.concat(),first);
    let short=retained_text_control("note.input","Value",InputKind::LongText,"again","procedural3d","setWidgetInput",args()).unwrap();
    assert_eq!(short.source,Some(("again".into(),"blurRetained".into())));
    assert!(short.children.is_empty());
    assert_eq!(widget_input_action_value(&to_dsl_value(&fixture["edit"]["wire"])).unwrap(),next);
    assert_eq!(widget_input_action_value(&DslValue::Bool(false)).unwrap(),"false");
    assert!(widget_input_action_value(&DslValue::Array(vec![DslValue::Array(vec![DslValue::String("".into())])])).is_err());
    assert!(widget_input_action_value(&DslValue::Array(vec![DslValue::Array(vec![DslValue::String("p".repeat(513))])])).is_err());
    let page=DslValue::Array(vec![DslValue::String("x".repeat(512));256]);
    let limit=DslValue::Array(vec![page.clone();8]);
    assert_eq!(widget_input_action_value(&limit).unwrap().len(),MAX_WIDGET_INPUT_TEXT_BYTES);
    let too_long=DslValue::Array(vec![page;9]);
    assert!(widget_input_action_value(&too_long).is_err());
}
