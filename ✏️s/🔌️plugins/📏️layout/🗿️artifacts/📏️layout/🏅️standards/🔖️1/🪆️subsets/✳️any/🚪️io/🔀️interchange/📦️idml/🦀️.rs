//! 📦️ IDML package semantics over stdio ZIP/XML; no parallel container or markup parser.
use super::*;
#[path = "🔤️text/🦀️.rs"]
pub(super) mod text_styles;
#[path = "🖼️assets/🦀️.rs"]
pub(super) mod assets;
#[path = "📃️parents/🦀️.rs"]
pub(super) mod parents;
#[path = "📐️guides/🦀️.rs"]
pub(super) mod guides;
#[path = "📎️overrides/🦀️.rs"]
pub(super) mod overrides;
use std::collections::{BTreeMap, HashSet};
use semio_s_artifact_stdio_zip::{ZipSnapshot, schema::snapshot::ZipEntry};
use semio_s_artifact_stdio_xml::{schema::snapshot::{XmlAttr, XmlDocument, XmlNode}, standards::v1_0::subsets::base::io::text::snapshot::{xml_document_from_text, xml_document_to_text_checked}};
pub(super) const MIME: &str = "application/vnd.adobe.indesign-idml-package";
pub(super) const NS: &str = "http://ns.adobe.com/AdobeInDesign/idml/1.0/packaging";
fn local(node: &XmlNode) -> &str { match node { XmlNode::Element { name, .. } => name.rsplit(':').next().unwrap_or(name), _ => "" } }
fn attr<'a>(node: &'a XmlNode, key: &str) -> Option<&'a str> { match node { XmlNode::Element { attrs, .. } => attrs.iter().find(|attr| attr.name == key).map(|attr| attr.value.as_str()), _ => None } }
fn children(node: &XmlNode) -> &[XmlNode] { match node { XmlNode::Element { children, .. } => children, _ => &[] } }
fn find<'a>(node: &'a XmlNode, name: &str, out: &mut Vec<&'a XmlNode>, depth: usize) -> Result<(), String> {
    if depth > 128 || out.len() >= MAX_INTERCHANGE_ITEMS { return Err("layout.idml.node-limit".into()); }
    if local(node) == name { out.push(node); }
    for child in children(node) { find(child, name, out, depth + 1)?; }
    Ok(())
}
fn nodes<'a>(node: &'a XmlNode, name: &str) -> Result<Vec<&'a XmlNode>, String> { let mut out = Vec::new(); find(node, name, &mut out, 0)?; Ok(out) }
fn text(node: &XmlNode, out: &mut String) {
    match node { XmlNode::Text { text } | XmlNode::CData { text } => out.push_str(text), _ => for child in children(node) { text(child, out); } }
}
fn safe_member(name: &str) -> Result<(), String> {
    if name.is_empty() || name.starts_with('/') || name.contains('\\') || name.contains(':') || name.split('/').any(|part| part == ".." || part == "." || part.is_empty()) { Err("layout.idml.member-path".into()) } else { Ok(()) }
}
fn read_xml(entries: &BTreeMap<String, Vec<u8>>, name: &str) -> Result<XmlNode, String> {
    safe_member(name)?;
    let bytes = entries.get(name).ok_or_else(|| format!("layout.idml.missing-member:{name}"))?;
    let source = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let doc = xml_document_from_text(source)?;
    if doc.doctype.is_some() { return Err("layout.idml.doctype".into()); }
    doc.root.ok_or_else(|| "layout.idml.missing-root".into())
}
fn numbers(value: &str, count: usize) -> Result<Vec<f64>, String> {
    let values = value.split_whitespace().map(str::parse::<f64>).collect::<Result<Vec<_>, _>>().map_err(|_| "layout.idml.numeric-value")?;
    if values.len() != count || values.iter().any(|value| !value.is_finite()) { return Err("layout.idml.numeric-value".into()); }
    Ok(values)
}
fn matrix(node: &XmlNode) -> Result<[f64; 6], String> { let values = numbers(attr(node, "ItemTransform").unwrap_or("1 0 0 1 0 0"), 6)?; Ok([values[0], values[1], values[2], values[3], values[4], values[5]]) }
fn point(matrix: [f64; 6], x: f64, y: f64) -> (f64, f64) { (matrix[0]*x + matrix[2]*y + matrix[4], matrix[1]*x + matrix[3]*y + matrix[5]) }
fn geometry(node: &XmlNode) -> Result<(LayoutBounds, f64, f64), String> {
    let m = matrix(node)?;
    let (left, top, right, bottom) = if let Some(value) = attr(node, "GeometricBounds") { let v = numbers(value, 4)?; (v[1], v[0], v[3], v[2]) } else {
        let points = nodes(node, "PathPointType")?;
        if points.len() != 4 { return Err("layout.idml.non-rectangular-path".into()); }
        for node in &points {let anchor=attr(node,"Anchor").ok_or("layout.idml.path-anchor")?;let expected=numbers(anchor,2)?;for key in ["LeftDirection","RightDirection"]{if let Some(value)=attr(node,key){if numbers(value,2)?!=expected{return Err("layout.idml.curved-path".into());}}}}
        let points = points.iter().map(|node| numbers(attr(node, "Anchor").ok_or("layout.idml.path-anchor")?, 2)).collect::<Result<Vec<_>,_>>()?;
        let left = points.iter().map(|p|p[0]).fold(f64::INFINITY,f64::min);
        let top = points.iter().map(|p|p[1]).fold(f64::INFINITY,f64::min);
        let right = points.iter().map(|p|p[0]).fold(f64::NEG_INFINITY,f64::max);
        let bottom = points.iter().map(|p|p[1]).fold(f64::NEG_INFINITY,f64::max);
        if points.iter().any(|p| (p[0]!=left && p[0]!=right)||(p[1]!=top && p[1]!=bottom)) { return Err("layout.idml.non-rectangular-path".into()); }
        (left,top,right,bottom)
    };
    let sx = m[0].hypot(m[1]); let sy = m[2].hypot(m[3]);
    if sx <= 0.0 || sy <= 0.0 || (m[0]*m[2]+m[1]*m[3]).abs() > 1e-8*sx*sy || m[0]*m[3]-m[1]*m[2] <= 0.0 { return Err("layout.idml.skew-or-reflection".into()); }
    let (cx,cy) = point(m,(left+right)/2.0,(top+bottom)/2.0);
    Ok((LayoutBounds { x:cx-(right-left)*sx*0.5, y:cy-(bottom-top)*sy*0.5, width:(right-left)*sx, height:(bottom-top)*sy, rotation:m[1].atan2(m[0]) },cx,cy))
}
fn identity(node: &XmlNode) -> Result<String, String> { attr(node,"Self").filter(|id|!id.is_empty()).map(str::to_string).ok_or_else(||"layout.idml.missing-identity".into()) }
fn paint_colors(entries:&BTreeMap<String,Vec<u8>>,root:&XmlNode)->Result<BTreeMap<String,[f32;4]>,String>{
    let mut colors=BTreeMap::from([("Color/Black".into(),[0.0,0.0,0.0,1.0])]);
    for reference in nodes(root,"Graphic")?{let Some(source)=attr(reference,"src")else{continue};let part=read_xml(entries,source)?;for color in nodes(&part,"Color")?{if attr(color,"Space")!=Some("RGB"){continue;}let values=numbers(attr(color,"ColorValue").ok_or("layout.idml.color-value")?,3)?;if values.iter().any(|value|*value<0.0||*value>255.0){return Err("layout.idml.color-value".into());}colors.insert(identity(color)?,[values[0]as f32/255.0,values[1]as f32/255.0,values[2]as f32/255.0,1.0]);}}
    Ok(colors)
}
fn opacity(node:&XmlNode,scope:&str)->Result<f32,String>{let Some(setting)=children(node).iter().find(|child|local(child)==scope)else{return Ok(1.0)};let Some(blend)=nodes(setting,"BlendingSetting")?.first().copied()else{return Ok(1.0)};if attr(blend,"BlendMode").is_some_and(|mode|mode!="Normal"){return Err("layout.idml.blend-mode".into());}let value=attr(blend,"Opacity").unwrap_or("100").parse::<f32>().map_err(|_|"layout.idml.opacity")?;if !value.is_finite()||!(0.0..=100.0).contains(&value){return Err("layout.idml.opacity".into());}Ok(value/100.0)}
fn transparency(name:&str,alpha:f32)->XmlNode{element(name,Vec::new(),vec![element("BlendingSetting",vec![("BlendMode","Normal".into()),("Opacity",(alpha*100.0).to_string())],Vec::new())])}
fn paint(node:&XmlNode,key:&str,colors:&BTreeMap<String,[f32;4]>)->Result<Option<[f32;4]>,String>{match attr(node,key){None|Some("Swatch/None")|Some("$ID/None")=>Ok(None),Some(id)=>{let mut color=colors.get(id).copied().ok_or_else(||format!("layout.idml.unsupported-color:{id}"))?;color[3]*=opacity(node,"TransparencySetting")?*opacity(node,if key=="FillColor"{"FillTransparencySetting"}else{"StrokeTransparencySetting"})?;Ok(Some(color))}}}
pub(super) fn frame_colors(frame:&Frame)->Result<Vec<XmlNode>,String>{let mut out=Vec::new();if let Frame::Rect{id,fill,stroke,..}=frame{for (name,color) in [("fill",fill),("stroke",stroke)]{if let Some(color)=color{if color.iter().any(|channel|!channel.is_finite()||*channel<0.0||*channel>1.0){return Err("layout.idml.paint-alpha".into());}out.push(element("Color",vec![("Self",format!("Color/{id}-{name}")),("Name",format!("{id}-{name}")),("Model","Process".into()),("Space","RGB".into()),("ColorValue",format!("{} {} {}",color[0]*255.0,color[1]*255.0,color[2]*255.0))],Vec::new()));}}}Ok(out)}
pub(super) fn frame_node(doc:&LayoutSnapshot,frame:&Frame)->Result<XmlNode,String>{
    let b=frame.bounds();let(sin,cos)=b.rotation.sin_cos();let tx=b.x+b.width*0.5-cos*b.width*0.5+sin*b.height*0.5;let ty=b.y+b.height*0.5-sin*b.width*0.5-cos*b.height*0.5;
    let mut attrs=vec![("Self",frame.id().into()),("ItemLayer",frame.layer_id().into()),("ItemTransform",format!("{cos} {sin} {} {cos} {tx} {ty}",-sin)),("Locked",frame.locked().to_string()),("Visible",frame.visible().to_string())];
    let name=match frame{
        Frame::Text{story_id,thread_next,..}=>{attrs.push(("FillColor","Swatch/None".into()));attrs.push(("StrokeColor","Swatch/None".into()));attrs.push(("ParentStory",story_id.clone()));attrs.push(("PreviousTextFrame",doc.pages.iter().flat_map(|page|&page.frames).chain(doc.parent_pages.iter().flat_map(|page|&page.frames)).find_map(|candidate|match candidate{Frame::Text{id,thread_next:Some(next),..}if next==frame.id()=>Some(id.clone()),_=>None}).unwrap_or_else(||"n".into())));attrs.push(("NextTextFrame",thread_next.clone().unwrap_or_else(||"n".into())));"TextFrame"},
        Frame::Rect{id,fill,stroke,..}=>{frame_colors(frame)?;attrs.push(("FillColor",fill.map_or_else(||"Swatch/None".into(),|_|format!("Color/{id}-fill"))));attrs.push(("StrokeColor",stroke.map_or_else(||"Swatch/None".into(),|_|format!("Color/{id}-stroke"))));"Rectangle"},
        Frame::Image{..}=>{attrs.push(("ContentType","GraphicType".into()));attrs.push(("FillColor","Swatch/None".into()));attrs.push(("StrokeColor","Swatch/None".into()));"Rectangle"},
    };let mut content=vec![path_geometry(b.width,b.height)];content.extend(text_styles::frame_preferences(frame)?);if let Frame::Rect{fill,stroke,..}=frame{if let Some(fill)=fill{content.push(transparency("FillTransparencySetting",fill[3]));}if let Some(stroke)=stroke{content.push(transparency("StrokeTransparencySetting",stroke[3]));}}Ok(element(name,attrs,content))
}
pub(super) fn import(bytes: &[u8], name: &str, control: &mut InterchangeControl<'_>) -> Result<InterchangeResult<LayoutSnapshot>, String> {
    let archive = semio_s_artifact_stdio_idml::IdmlPackage::decode(bytes).map_err(|error|error.replacen("stdio.idml.","layout.idml.",1))?.into_archive();
    if archive.entries.len() > MAX_INTERCHANGE_ITEMS { return Err("layout.idml.member-limit".into()); }
    let stored_mimetype=archive.entries.first().is_some_and(|entry|entry.name=="mimetype"&&entry.metadata.compression_method==0);
    let mut entries = BTreeMap::new(); let mut total=0usize;
    for entry in archive.entries {
        if entry.name.ends_with('/')&&entry.data.is_empty(){safe_member(entry.name.trim_end_matches('/'))?;continue;}
        safe_member(&entry.name)?;
        total=total.checked_add(entry.data.len()).ok_or("layout.interchange.byte-limit")?;
        if total>MAX_INTERCHANGE_BYTES { return Err("layout.interchange.byte-limit".into()); }
        if entries.insert(entry.name,entry.data).is_some(){return Err("layout.idml.duplicate-member".into());}
    }
    if !stored_mimetype||entries.get("mimetype").map(Vec::as_slice)!=Some(MIME.as_bytes()) {return Err("layout.idml.mimetype".into());}
    let root=read_xml(&entries,"designmap.xml")?;
    if local(&root)!="Document" {return Err("layout.idml.designmap".into());}
    let version=attr(&root,"DOMVersion").ok_or("layout.idml.dom-version")?;
    let mut doc=document(name); let mut seen=HashSet::new();
    let mut diagnostics=vec![diagnostic("layout.idml.subset",&format!("IDML DOMVersion {version}: editable rectangular pages/frames and Unicode stories. Embedded PNG/JPEG, one-page parent templates, paragraph/character typography and RGB paint are mapped. Groups, nonrectangular paths, cropping/effects, color profiles, nested masters and threaded master overrides require review."))];
    let colors=paint_colors(&entries,&root)?;
    text_styles::import_styles(&mut doc,&entries,&root,&colors)?;
    let layers=nodes(&root,"Layer")?;
    let mut refs=Vec::new();
    for node in nodes(&root,"Story")? { if let Some(src)=attr(node,"src") {refs.push(src.to_string());} }
    for src in refs {
        let part=read_xml(&entries,&src)?;
        for story in nodes(&part,"Story")? {
            let id=identity(story)?; if !seen.insert(format!("story:{id}")){return Err("layout.idml.duplicate-identity".into());}
            let imported=text_styles::import_story(story,&mut doc,&colors)?;doc.stories.push(imported);
        }
    }
    parents::import(&mut doc,&entries,&root,&layers,&colors,&mut seen)?;
    let spreads=nodes(&root,"Spread")?;
    for (si,reference) in spreads.iter().enumerate() {
        checkpoint(control,si,spreads.len())?;
        let src=attr(reference,"src").ok_or("layout.idml.spread-reference")?;
        let part=read_xml(&entries,src)?;
        let spread=nodes(&part,"Spread")?.first().copied().ok_or("layout.idml.spread")?;
        let spread_id=identity(spread)?;let mut page_ids=Vec::new();let first=doc.pages.len();let mut origins=Vec::new();let mut recovered_overrides=HashSet::new();
        for source in nodes(spread,"Page")? {
            if doc.pages.len()>=MAX_INTERCHANGE_PAGES{return Err("layout.interchange.page-limit".into());}
            let id=identity(source)?; if !seen.insert(format!("page:{id}")){return Err("layout.idml.duplicate-identity".into());}
            let (b,_,_)=geometry(source)?;check_dimensions(b.width,b.height)?;
            if b.rotation.abs()>1e-8{return Err("layout.idml.rotated-page".into());}
            let mut page=page(id.clone(),b.width,b.height);page.parent_page_id=attr(source,"AppliedMaster").filter(|id|*id!="n").map(str::to_string);if page.parent_page_id.as_ref().is_some_and(|id|!doc.parent_pages.iter().any(|parent|&parent.id==id)){return Err("layout.idml.missing-master".into());}page.name=attr(source,"Name").unwrap_or(&id).into();page.spread_id=spread_id.clone();
            if let Some(margin)=children(source).iter().find(|node|local(node)=="MarginPreference"){let scalar=|key:&str|->Result<f64,String>{let number=attr(margin,key).unwrap_or("0").parse::<f64>().map_err(|_|"layout.idml.page-margin")?;if !number.is_finite(){return Err("layout.idml.page-margin".into());}Ok(number)};page.margins=PageMargins{top:scalar("Top")?,right:scalar("Right")?,bottom:scalar("Bottom")?,left:scalar("Left")?};page.columns=PageColumns{count:attr(margin,"ColumnCount").unwrap_or("1").parse().map_err(|_|"layout.idml.page-columns")?,gutter:scalar("ColumnGutter")?};}
            if !layers.is_empty(){page.layers=layers.iter().map(|node|Ok(Layer{id:format!("{}:{}",id,identity(node)?),name:attr(node,"Name").unwrap_or("Layer").into(),visible:attr(node,"Visible")!=Some("false"),locked:attr(node,"Locked")==Some("true"),object_ids:Vec::new()})).collect::<Result<Vec<_>,String>>()?;page.layer_ids=page.layers.iter().map(|layer|layer.id.clone()).collect();}
            overrides::read_page(&doc,&mut page,source,attr(spread,"ShowMasterItems")!=Some("false"))?;origins.push((b.x,b.y,b.width,b.height));page_ids.push(id);doc.pages.push(page);
        }
        for source in children(spread).iter().filter(|node|matches!(local(node),"Rectangle"|"TextFrame"|"Polygon"|"Oval"|"Group")) {
            if matches!(local(source),"Group"|"Oval"|"Polygon"){diagnostics.push(diagnostic("layout.idml.unsupported-item",local(source)));continue;}
            let (mut b,cx,cy)=match geometry(source){Ok(value)=>value,Err(error)=>{diagnostics.push(diagnostic("layout.idml.geometry",&error));continue;}};
            let index=origins.iter().position(|&(x,y,w,h)|cx>=x&&cx<=x+w&&cy>=y&&cy<=y+h).ok_or("layout.idml.pasteboard-item")?;
            let image=nodes(source,"Image")?.first().copied();let image_link=image.map(|source|assets::import(&mut doc,source)).transpose()?;
            let page=&mut doc.pages[first+index];b.x-=origins[index].0;b.y-=origins[index].1;
            let id=identity(source)?;if !seen.insert(format!("frame:{id}")){return Err("layout.idml.duplicate-identity".into());}
            let layer=attr(source,"ItemLayer").and_then(|id|page.layers.iter().position(|layer|layer.id==format!("{}:{id}",page.id))).unwrap_or(0);
            let layer_id=page.layers[layer].id.clone();let locked=Some(attr(source,"Locked")==Some("true"));let visible=Some(attr(source,"Visible")!=Some("false"));
            let frame=if let Some(link_id)=image_link{Frame::Image{id:id.clone(),layer_id,bounds:b,link_id,locked,visible}}else if local(source)=="TextFrame" {
                let story_id=attr(source,"ParentStory").ok_or("layout.idml.parent-story")?.to_string();if !doc.stories.iter().any(|story|story.id==story_id){return Err("layout.idml.missing-story".into());}
                let(columns,inset,wrap_mode)=text_styles::read_frame_preferences(source)?;Frame::Text{id:id.clone(),layer_id,bounds:b,story_id,thread_next:attr(source,"NextTextFrame").filter(|id|*id!="n").map(str::to_string),columns,inset,wrap_mode,locked,visible}
            }else{
                Frame::Rect{id:id.clone(),layer_id,bounds:b,fill:paint(source,"FillColor",&colors)?,stroke:paint(source,"StrokeColor",&colors)?,locked,visible}
            };
            if let Some(parent)=page.parent_page_id.as_ref().and_then(|id|doc.parent_pages.iter().find(|parent|&parent.id==id)){if let Some(source_id)=overrides::recover(page,parent,source,&frame)?{if !recovered_overrides.insert((page.id.clone(),source_id)){return Err("layout.idml.duplicate-override-copy".into());}continue;}}page.layers[layer].object_ids.push(id);page.frames.push(frame);
        }
        guides::import(spread,&mut doc.pages[first..],&origins)?;
        doc.spreads.push(Spread{id:spread_id,name:spread_name(spread,si)?,page_ids});
    }
    for frames in doc.pages.iter().map(|page|&page.frames).chain(doc.parent_pages.iter().map(|page|&page.frames)){for frame in frames {if let Frame::Text{thread_next:Some(next),..}=frame{if !doc.pages.iter().flat_map(|page|&page.frames).chain(doc.parent_pages.iter().flat_map(|page|&page.frames)).any(|frame|frame.id()==next){return Err("layout.idml.missing-thread-frame".into());}}}}
    let thread_map=doc.pages.iter().flat_map(|page|&page.frames).chain(doc.parent_pages.iter().flat_map(|page|&page.frames)).filter_map(|frame|match frame{Frame::Text{id,thread_next,..}=>Some((id.as_str(),thread_next.as_deref())),_=>None}).collect::<BTreeMap<_,_>>();
    for start in thread_map.keys(){let mut visited=HashSet::new();let mut current=Some(*start);while let Some(id)=current{if !visited.insert(id){return Err("layout.idml.thread-cycle".into());}current=thread_map.get(id).copied().flatten();}}
    Ok(InterchangeResult{value:doc,fidelity:"editable-subset",diagnostics})
}
pub(super) fn element(name:&str,attrs:Vec<(&str,String)>,children:Vec<XmlNode>)->XmlNode{XmlNode::Element{name:name.into(),attrs:attrs.into_iter().map(|(name,value)|XmlAttr{name:name.into(),value}).collect(),children}}
fn member(archive:&mut ZipSnapshot,name:&str,node:XmlNode)->Result<(),String>{let doc=XmlDocument{root:Some(node),..XmlDocument::default()};let data=xml_document_to_text_checked(&doc)?.into_bytes();archive.entries.push(ZipEntry{name:name.into(),data,..ZipEntry::default()});Ok(())}
pub(super) fn package(name:&str,node:XmlNode)->XmlNode{element(&format!("idPkg:{name}"),vec![("xmlns:idPkg",NS.into()),("DOMVersion","8.0".into())],vec![node])}
pub(super) fn frame_node_at(doc:&LayoutSnapshot,frame:&Frame,x:f64)->Result<XmlNode,String>{let mut node=frame_node(doc,frame)?;let mut transform=matrix(&node)?;transform[4]+=x;if let XmlNode::Element{attrs,..}=&mut node{if let Some(attr)=attrs.iter_mut().find(|attr|attr.name=="ItemTransform"){attr.value=transform.iter().map(f64::to_string).collect::<Vec<_>>().join(" ");}}Ok(node)}
pub(super) fn page_node_at(page:&Page,x:f64)->XmlNode{let mut node=page_node(page);if let XmlNode::Element{attrs,..}=&mut node{if let Some(attr)=attrs.iter_mut().find(|attr|attr.name=="ItemTransform"){attr.value=format!("1 0 0 1 {x} 0");}}node}
pub(super) fn spread_properties(spread:&Spread)->XmlNode{element("Properties",Vec::new(),vec![element("Label",Vec::new(),vec![element("KeyValuePair",vec![("Key","semio:spread-name".into()),("Value",spread.name.clone())],Vec::new())])])}
fn spread_name(node:&XmlNode,index:usize)->Result<String,String>{Ok(nodes(node,"KeyValuePair")?.iter().find(|node|attr(node,"Key")==Some("semio:spread-name")).and_then(|node|attr(node,"Value")).map(str::to_string).unwrap_or_else(||format!("Spread {}",index+1)))}
pub(super) fn validate_spread_page(doc:&LayoutSnapshot,page:&Page)->Result<(),String>{let spread=doc.spreads.iter().find(|spread|spread.id==page.spread_id).ok_or("layout.idml.missing-spread")?;if spread.page_ids.iter().filter(|id|*id==&page.id).count()!=1{return Err("layout.idml.spread-membership".into());}Ok(())}
pub(super) fn page_node(page:&Page)->XmlNode{element("Page",vec![("Self",page.id.clone()),("Name",page.name.clone()),("GeometricBounds",format!("0 0 {} {}",page.height,page.width)),("ItemTransform","1 0 0 1 0 0".into()),("AppliedMaster",page.parent_page_id.clone().unwrap_or_else(||"n".into())),("MasterPageTransform","1 0 0 1 0 0".into()),("OverrideList",page.overrides.iter().map(|item|item.object_id.as_str()).collect::<Vec<_>>().join(" "))],vec![element("MarginPreference",vec![("Top",page.margins.top.to_string()),("Right",page.margins.right.to_string()),("Bottom",page.margins.bottom.to_string()),("Left",page.margins.left.to_string()),("ColumnCount",page.columns.count.to_string()),("ColumnGutter",page.columns.gutter.to_string())],Vec::new())])}
pub(super) fn path_geometry(width:f64,height:f64)->XmlNode{
    let points=[[0.0,0.0],[width,0.0],[width,height],[0.0,height]].into_iter().map(|[x,y]|{let p=format!("{x} {y}");element("PathPointType",vec![("Anchor",p.clone()),("LeftDirection",p.clone()),("RightDirection",p)],Vec::new())}).collect();
    element("Properties",Vec::new(),vec![element("PathGeometry",Vec::new(),vec![element("GeometryPathType",vec![("PathOpen","false".into())],vec![element("PathPointArray",Vec::new(),points)])])])
}
pub(super) fn export(doc:&LayoutSnapshot,control:&mut InterchangeControl<'_>)->Result<InterchangeResult<Vec<u8>>,String>{
    let mut archive=ZipSnapshot::default();let mut mime=ZipEntry{name:"mimetype".into(),data:MIME.as_bytes().to_vec(),..ZipEntry::default()};mime.metadata.compression_method=0;archive.entries.push(mime);
    let mut design=Vec::new();let mut seen_layers=HashSet::new();
    for source in doc.pages.iter().map(|page|&page.layers).chain(doc.parent_pages.iter().map(|page|&page.layers)){for layer in source{if seen_layers.insert(layer.id.as_str()){design.push(element("Layer",vec![("Self",layer.id.clone()),("Name",layer.name.clone()),("Visible",layer.visible.to_string()),("Locked",layer.locked.to_string())],Vec::new()));}}}
    for (index,story) in doc.stories.iter().enumerate(){
        let source=format!("Stories/Story_{index}.xml");design.push(element("idPkg:Story",vec![("src",source.clone())],Vec::new()));
        let mut content=Vec::new();let mut cursor=0;while cursor<story.content.len(){let(end,node)=text_styles::story_fragment(story,cursor)?;content.push(node);cursor=end;}
        let node=element("Story",vec![("Self",story.id.clone())],content);
        member(&mut archive,&source,package("Story",node))?;
    }
    for (index,parent) in doc.parent_pages.iter().enumerate(){let source=format!("MasterSpreads/MasterSpread_{index}.xml");design.push(element("idPkg:MasterSpread",vec![("src",source.clone())],Vec::new()));member(&mut archive,&source,parents::node(doc,parent)?)?;}
    for page in &doc.pages{validate_spread_page(doc,page)?;}
    for(index,spread)in doc.spreads.iter().enumerate(){checkpoint(control,index,doc.spreads.len())?;let source=format!("Spreads/Spread_{index}.xml");design.push(element("idPkg:Spread",vec![("src",source.clone())],Vec::new()));let mut items=vec![spread_properties(spread)];let mut x=0.0;
        for(position,id)in spread.page_ids.iter().enumerate(){let page=doc.pages.iter().find(|page|&page.id==id&&page.spread_id==spread.id).ok_or("layout.idml.spread-membership")?;items.push(page_node_at(page,x));for index in 0..page.overrides.len(){items.push(overrides::node(doc,page,index,x)?);}for frame in &page.frames{let mut node=frame_node_at(doc,frame,x)?;if matches!(frame,Frame::Image{..}){assets::attach(doc,frame,&mut node)?;}items.push(node);}for index in 0..page.guides.len(){items.extend(guides::nodes_for(page,index,x,position)?);}x+=page.width;}
        member(&mut archive,&source,package("Spread",element("Spread",vec![("Self",spread.id.clone()),("PageCount",spread.page_ids.len().to_string())],items)))?;
    }
    let mut paragraphs=vec![element("ParagraphStyle",vec![("Self","ParagraphStyle/$ID/NormalParagraphStyle".into()),("Name","$ID/NormalParagraphStyle".into())],Vec::new())];
    for style in &doc.paragraph_styles{paragraphs.push(text_styles::paragraph_node(style)?);}
    let mut characters=vec![element("CharacterStyle",vec![("Self","CharacterStyle/$ID/[No character style]".into()),("Name","$ID/[No character style]".into())],Vec::new())];
    for style in &doc.character_styles{characters.push(text_styles::character_node(style)?);}
    member(&mut archive,"Resources/Styles.xml",element("idPkg:Styles",vec![("xmlns:idPkg",NS.into()),("DOMVersion","8.0".into())],vec![element("RootParagraphStyleGroup",vec![("Self","root-paragraph".into())],paragraphs),element("RootCharacterStyleGroup",vec![("Self","root-character".into())],characters)]))?;
    let mut graphic=vec![element("Swatch",vec![("Self","Swatch/None".into()),("Name","$ID/None".into())],Vec::new()),element("Color",vec![("Self","Color/Black".into()),("Name","Black".into()),("Model","Process".into()),("Space","RGB".into()),("ColorValue","0 0 0".into())],Vec::new())];
    for frame in doc.pages.iter().flat_map(|page|&page.frames).chain(doc.parent_pages.iter().flat_map(|page|&page.frames)){graphic.extend(frame_colors(frame)?);}
    for style in &doc.character_styles{if let Some(color)=text_styles::character_color(style)?{graphic.push(color);}}
    member(&mut archive,"Resources/Graphic.xml",element("idPkg:Graphic",vec![("xmlns:idPkg",NS.into()),("DOMVersion","8.0".into())],graphic))?;
    design.push(element("idPkg:Graphic",vec![("src","Resources/Graphic.xml".into())],Vec::new()));
    design.push(element("idPkg:Styles",vec![("src","Resources/Styles.xml".into())],Vec::new()));
    member(&mut archive,"designmap.xml",element("Document",vec![("xmlns:idPkg",NS.into()),("DOMVersion","8.0".into()),("Self","d".into()),("Name",doc.name.clone())],design))?;
    member(&mut archive,"META-INF/container.xml",element("container",vec![("version","1.0".into()),("xmlns","urn:oasis:names:tc:opendocument:xmlns:container".into())],vec![element("rootfiles",Vec::new(),vec![element("rootfile",vec![("full-path","designmap.xml".into()),("media-type","text/xml".into())],Vec::new())])]))?;
    let value=semio_s_artifact_stdio_idml::IdmlPackage::from_archive(archive)?.encode()?;
    Ok(InterchangeResult{value,fidelity:"editable-subset",diagnostics:vec![diagnostic("layout.idml.subset","IDML DOMVersion 8.0 pages/spreads, one-page parent templates, rectangular/text/image frames, Unicode styled stories, opacity and raster bytes are exported. Nested/threaded master semantics, color profiles, print/data semantics and nonrectangular background traces require review.")]})
}
