extern crate self as semio_plugin;
extern crate self as semio_framework;

mod ui { pub use crate::window_resource; }
type UiAssemblyResult<T> = Result<T, &'static str>;

#[derive(Clone)]
struct UiText(String);
impl UiText {
    fn try_from_str(text: &str) -> Option<Self> { (text.len() <= UI_TEXT_MAX_BYTES).then(|| Self(text.into())) }
    fn clipped(text: &str) -> Self { Self(text.chars().take(100).collect()) }
}
#[derive(Clone)]
struct Label(UiText);
fn ui_label(text: &str) -> UiAssemblyResult<Label> { ui_text(text).map(Label) }
fn ui_text(text: &str) -> UiAssemblyResult<UiText> { UiText::try_from_str(text).ok_or("ui.text") }
fn ui_assembly_error(code: &'static str) -> &'static str { code }
fn graph_hover_args(target: &str, granularity: &str) -> UiAssemblyResult<String> {
    assert_eq!(granularity,"handle");
    Ok(target.into())
}
const INTERACTION_HOVER_ACTION_ID: &str = "interactionHover";
struct ActionFactory;
impl ActionFactory {
    fn new(_: &str) -> Self { Self }
    fn action(&self, name: &str, args: Option<String>) -> UiAssemblyResult<(String,Option<String>)> {
        Ok((name.into(),args))
    }
}
enum Trigger { HoverPreview }
enum ScrollAxes { Vertical }
enum Sizing { Fill }
struct ScrollLayout { axes: ScrollAxes, sizing: Sizing, unused: () }
impl Default for ScrollLayout {
    fn default() -> Self { Self { axes: ScrollAxes::Vertical,sizing:Sizing::Fill,unused:() } }
}
#[derive(Default)]
enum LayoutSpec { #[default] Leaf, Scroll(ScrollLayout) }
#[derive(Default)]
struct BuiltNode {
    highlighted: bool,
    key: String,
    kind: &'static str,
    label: String,
    hover: Option<String>,
    source: Option<(String,String)>,
    children: Vec<BuiltNode>,
    layout: LayoutSpec,
}
type TreeItemBuilder = BuiltNode;
impl BuiltNode {
    fn default_open(self, _:bool) -> Self { self }
    fn highlighted(mut self, highlighted:bool) -> Self { self.highlighted=highlighted; self }
    fn description(self, _: UiText) -> Self { self }
    fn granularity(self, _: UiText) -> Self { self }
    fn grow(self, _: bool) -> Self { self }
    fn try_on_with(mut self, _: Trigger, action:String, target:String) -> Result<Self,()> {
        assert_eq!(action,INTERACTION_HOVER_ACTION_ID);
        self.hover=Some(target);
        Ok(self)
    }
}
fn tree_item_builder(label:Label)->BuiltNode { BuiltNode{kind:"treeItem",label:label.0.0,..Default::default()} }
fn text(label:Label)->BuiltNode { BuiltNode{kind:"text",label:label.0.0,..Default::default()} }
fn column()->BuiltNode { BuiltNode{kind:"column",..Default::default()} }
fn ui_id(mut b:BuiltNode,id:&str)->UiAssemblyResult<BuiltNode>{b.key=id.into();Ok(b)}
fn ui_child(mut b:BuiltNode,child:BuiltNode)->UiAssemblyResult<BuiltNode>{b.children.push(child);Ok(b)}
fn ui_build(b:BuiltNode)->UiAssemblyResult<BuiltNode>{Ok(b)}
struct TextView { text:String, language: Option<()> }
struct TextWindowKit;
impl TextWindowKit {
    fn render_read_only(key:&str,v:&TextView)->UiAssemblyResult<BuiltNode>{ui_id(text(Label(UiText::clipped(&v.text))),key)}
}
#[derive(Clone)]
pub struct TreeWindowRequest { pub body_key:String,pub node_key:String,pub open:Option<bool>,pub offset:u32,pub rows:u32 }
struct ViewModel { window_id:Option<String>,tree_windows:Vec<TreeWindowRequest> }
struct TreeWindows<'a> {
    resource_scope: Option<(&'a ViewModel,String)>,
    path: std::cell::RefCell<Vec<String>>,
    reserved: std::cell::Cell<usize>,
    remaining: std::cell::Cell<usize>,
}
impl TreeWindows<'_> {
    fn path_of(&self,key:&str)->String { self.path.borrow().iter().map(String::as_str).chain([key]).collect::<Vec<_>>().join("␟") }
    fn nodes_remaining(&self)->usize { self.remaining.get() }
    fn debit(&self,nodes:usize){self.remaining.set(self.remaining.get()-nodes);}
}
fn tree_window_indexed_item(windows:&TreeWindows<'_>,mut item:BuiltNode,id:&str,_:bool,total:usize,mut row:impl FnMut(usize)->UiAssemblyResult<BuiltNode>)->UiAssemblyResult<BuiltNode>{
    windows.path.borrow_mut().push(id.into());
    for index in 0..total {
        if id == "dataUrl" { break; }
        let result=row(index)?;
        item.children.push(result);
    }
    windows.path.borrow_mut().pop();
    Ok(item)
}
struct PanelTreeBuilder(BuiltNode);
impl PanelTreeBuilder {
    fn new(namespace:&str)->UiAssemblyResult<Self>{Ok(Self(BuiltNode{kind:"tree",key:namespace.into(),..Default::default()}))}
    fn window_section<T>(mut self,windows:&TreeWindows<'_>,id:&str,_:Option<Label>,_:bool,rows:&[T],mut row:impl FnMut(&T)->UiAssemblyResult<BuiltNode>)->UiAssemblyResult<Self>{
        windows.path.borrow_mut().push(id.into());
        for entry in rows { self.0.children.push(row(entry)?); }
        windows.path.borrow_mut().pop();
        Ok(self)
    }
    fn interaction_domain(self,_:&str,_:&str)->UiAssemblyResult<Self>{Ok(self)}
    fn build(self)->UiAssemblyResult<BuiltNode>{Ok(self.0)}
}
pub(crate) fn image_with_source(id:&str,source:&str,alt:Label)->UiAssemblyResult<BuiltNode>{
    image_with_retained_source(id,"",source,alt)
}
fn image_with_retained_source(id:&str,key:&str,source:&str,alt:Label)->UiAssemblyResult<BuiltNode>{
    Ok(BuiltNode{ kind:"image",key:id.into(),label:alt.0.0,source:Some((key.into(),source.into())),..Default::default() })
}
mod semio_plugin_window_transient {
    pub(super) use super::{image_with_source, paged_text_carrier_nodes, WindowResourceRequests};
}
fn direct<'a>(node:&'a BuiltNode,key:&str)->&'a BuiltNode { node.children.iter().find(|entry| entry.key==key).unwrap() }
fn main() {
    use semio_framework_pack_json::{parse,JsonMemberPolicy};
    use channel_values::{scene_with_channel_values,ChannelValueLabels,ChannelValueRow};
    let source=std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let fixture=parse(include_str!(CHANNEL_IMAGE_FIXTURE),JsonMemberPolicy::Reject).unwrap();
    let short=fixture["image"]["dataUrl"].as_str().unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        assert_eq!(channel_values::channel_image_source(&row["value"]),row["image"].as_bool().unwrap().then_some(short));
    }
    let channels=&fixture["channels"];
    let labels=ChannelValueLabels::new(false,"Input","Output");
    let native_value=semio_framework_pack_json::object([("schema".into(),"image".into()),("dataUrl".into(),source.clone().into()),("width".into(),fixture["image"]["width"].clone())]);
    let rows=[
        ChannelValueRow{id:channels["output"]["id"].as_str().unwrap().into(),label:"source@image".into(),input:false,value:&native_value},
        ChannelValueRow{id:channels["input"]["id"].as_str().unwrap().into(),label:"sink@image".into(),input:true,value:&fixture["image"]},
    ];
    let window="procedural3d-preview";
    let body="preview";
    let view=ViewModel{window_id:Some(window.into()),tree_windows:vec![]};
    let project=|view:&ViewModel| {
        let windows=TreeWindows{resource_scope:Some((view,body.into())),path:Default::default(),reserved:std::cell::Cell::new(4),remaining:std::cell::Cell::new(20)};
        let scene=scene_with_channel_values(text(ui_label("Scene").unwrap()),&windows,"procedural3d-view","tree",&labels, &rows, &[]).unwrap();
        assert_eq!(20-windows.nodes_remaining(),WindowResourceRequests::image_page_nodes() + 2);
        scene
    };
    let scene=project(&view);
    let tree=direct(direct(&scene,"preview.channels"),"tree");
    let output=direct(direct(tree,&rows[0].id),"out");
    let input=direct(direct(tree,&rows[1].id),"in");
    let out_preview=direct(output,"image-preview");
    let in_preview=direct(input,"image-preview");
    assert_eq!(out_preview.hover.as_deref(),Some(rows[0].id.as_str()));
    assert_eq!(in_preview.hover.as_deref(),Some(rows[1].id.as_str()));
    assert_eq!(output.children.len(),4);
    assert!(output.children.iter().any(|child| child.label=="dataUrl"));
    let image=&out_preview.children[0];
    assert_eq!(image.label,channels["output"]["alt"].as_str().unwrap());
    assert_eq!(in_preview.children[0].source.as_ref().unwrap().1,short);
    assert_ne!(image.key,in_preview.children[0].key);
    let first=window_resource::WindowResourcePage::parse(&image.source.as_ref().unwrap().0).unwrap();
    assert!(first.resource.matches_owner(window,&image.key));
    assert_eq!(first.index,1);
    let all=(1..=first.resource.pages()).map(|index| {
        let view=ViewModel{window_id:Some(window.into()),tree_windows:vec![TreeWindowRequest{body_key:body.into(),node_key:first.resource.request_key(),open:None,offset:index,rows:1}]};
        let scene=project(&view);
        let output=direct(direct(direct(direct(&scene,"preview.channels"),"tree"),&rows[0].id),"out");
        let page=direct(output,"image-preview").children[0].source.as_ref().unwrap();
        assert_eq!(page.1,window_resource::WindowResourcePage{resource:first.resource,index}.text(&source).unwrap());
        page.0.clone()
    }).collect::<Vec<_>>();
    println!("{{\"imageKey\":{:?},\"pages\":[{}]}}",image.key,all.iter().map(|page|format!("{page:?}")).collect::<Vec<_>>().join(","));
}
