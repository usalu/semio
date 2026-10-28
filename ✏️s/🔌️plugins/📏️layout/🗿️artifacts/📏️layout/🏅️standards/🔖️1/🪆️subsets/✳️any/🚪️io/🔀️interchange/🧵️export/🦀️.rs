//! 🧵️ Export sessions emit at most one bounded semantic fragment or ZIP chunk per advance.
use super::*;
use semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::stream::StoredZipStream;
use semio_s_artifact_stdio_xml::{schema::snapshot::{XmlDocument,XmlNode},standards::v1_0::subsets::base::io::text::snapshot::xml_document_to_text_checked};

const CHUNK:usize=4096;
#[derive(Debug)]
pub enum InterchangeExportAdvance{Progress{completed_units:u64},Chunk(Vec<u8>),Done}
#[derive(Debug)]
enum Part{Story{index:usize,stage:u8,text:usize},Master{index:usize,stage:u8,frame:usize},Spread{index:usize,stage:u8,page:usize,frame:usize,x:f64},Image{master:bool,page:usize,frame:usize,stage:u8,offset:usize,spread:Option<(usize,usize,f64)>},OverrideImage{page:usize,frame:usize,stage:u8,offset:usize,spread:(usize,usize,f64)},Styles{stage:u8,index:usize},Graphic{stage:u8,page:usize,frame:usize},Design{stage:u8,index:usize,page:usize,layer:usize},Container,Central,Done}
#[derive(Debug)]
enum Kind{Step{session:step::StepExportSession},Idml{started:bool,writer:StoredZipStream,part:Part,layers:Vec<String>}}
#[derive(Debug)]
pub struct InterchangeExportSession{kind:Kind,pending:Vec<u8>,cursor:usize,completed:u64,output_bytes:usize,admission:u8,admission_index:usize,input_bytes:usize,input_items:usize,admission_page:usize,admission_frame:usize,selected_page:Option<usize>,story_plan:Option<idml::text_styles::StoryFragmentPlan>}
fn fragment(node:XmlNode)->Result<Vec<u8>,String>{xml_document_to_text_checked(&XmlDocument{root:Some(node),..XmlDocument::default()}).map(String::into_bytes)}
fn open(name:&str,attrs:Vec<(&str,String)>)->Result<Vec<u8>,String>{let mut bytes=fragment(idml::element(name,attrs,Vec::new()))?;if !bytes.ends_with(b"/>"){return Err("layout.idml.xml-open".into());}bytes.truncate(bytes.len()-2);bytes.push(b'>');Ok(bytes)}
impl InterchangeExportSession{
    pub fn new(format:&str)->Result<Self,String>{let kind=match format{"step"=>Kind::Step{session:step::StepExportSession::new()},"idml"=>Kind::Idml{started:false,writer:StoredZipStream::new(),part:Part::Story{index:0,stage:0,text:0},layers:Vec::new()},_=>return Err("layout.interchange.session-format".into())};Ok(Self{kind,pending:Vec::new(),cursor:0,completed:0,output_bytes:0,admission:0,admission_index:0,input_bytes:0,input_items:0,admission_page:0,admission_frame:0,selected_page:None,story_plan:None})}
    pub fn with_selected_page(mut self,index:usize)->Self{self.selected_page=Some(index);self}
    pub fn completed_units(&self)->u64{self.completed}
    fn publish(&mut self,bytes:Vec<u8>)->Result<InterchangeExportAdvance,String>{self.output_bytes=self.output_bytes.checked_add(bytes.len()).filter(|&bytes|bytes<=MAX_INTERCHANGE_BYTES).ok_or("layout.interchange.byte-limit")?;Ok(InterchangeExportAdvance::Chunk(bytes))}
    fn admit_one(&mut self,doc:&LayoutSnapshot)->Result<(),String>{
        match self.admission{
            0=>{if doc.pages.len()>MAX_INTERCHANGE_PAGES||doc.parent_pages.len()>MAX_INTERCHANGE_PAGES||doc.spreads.len()>MAX_INTERCHANGE_PAGES||doc.spreads.len()+doc.stories.len()+doc.parent_pages.len()+5>1024||doc.name.len()>4096{return Err("layout.interchange.document-envelope".into());}self.admission=1;},
            1=>{if let Some((width,height,frames))=doc.pages.get(self.admission_index).map(|page|(page.width,page.height,page.frames.len()+page.overrides.len())).or_else(||doc.parent_pages.get(self.admission_index.saturating_sub(doc.pages.len())).map(|page|(page.width,page.height,page.frames.len()))){check_dimensions(width,height)?;self.input_items=self.input_items.checked_add(frames).ok_or("layout.interchange.item-limit")?;if self.input_items>MAX_INTERCHANGE_ITEMS{return Err("layout.interchange.item-limit".into());}self.admission_index+=1;}else{self.admission=2;self.admission_index=0;}},
            2=>{if let Some(story)=doc.stories.get(self.admission_index){if story.id.len()>256{return Err("layout.interchange.id-limit".into());}self.input_bytes=self.input_bytes.checked_add(story.content.len()).ok_or("layout.interchange.byte-limit")?;self.admission_index+=1;}else{self.admission=3;self.admission_index=0;}},
            3=>{if let Some(link)=doc.links.get(self.admission_index){if link.id.len()>256||link.path.len()>4096{return Err("layout.interchange.link-envelope".into());}self.input_bytes=self.input_bytes.checked_add(link.proxy_data_url.as_ref().map_or(0,String::len)).ok_or("layout.interchange.byte-limit")?;self.admission_index+=1;}else{self.admission=4;self.admission_index=0;}},
            4=>{if let Some(frames)=doc.pages.get(self.admission_page).map(|page|&page.frames).or_else(||doc.parent_pages.get(self.admission_page.saturating_sub(doc.pages.len())).map(|page|&page.frames)){if let Some(frame)=frames.get(self.admission_frame){if frame.id().len()>256||frame.layer_id().len()>256{return Err("layout.interchange.id-limit".into());}match frame{Frame::Text{story_id,thread_next,..}=>if story_id.len()>256||thread_next.as_ref().is_some_and(|id|id.len()>256){return Err("layout.interchange.id-limit".into());},Frame::Image{link_id,..}=>if link_id.len()>256{return Err("layout.interchange.id-limit".into());},_=>{}}self.admission_frame+=1;}else{self.admission_page+=1;self.admission_frame=0;}}else{self.admission=5;self.admission_index=0;}},
            5=>{if let Some(style)=doc.paragraph_styles.get(self.admission_index){if style.id.len()>256||style.name.len()>256||style.font_family.len()>256{return Err("layout.interchange.style-envelope".into());}self.admission_index+=1;}else{self.admission=6;self.admission_index=0;}},
            6=>{if let Some(style)=doc.character_styles.get(self.admission_index){if style.id.len()>256||style.name.as_ref().is_some_and(|name|name.len()>256)||style.font_family.as_ref().is_some_and(|name|name.len()>256){return Err("layout.interchange.style-envelope".into());}self.admission_index+=1;}else{self.admission=7;self.admission_index=0;}},
            7=>{if let Some(page)=doc.pages.get(self.admission_index){idml::validate_spread_page(doc,page)?;self.admission_index+=1;}else{self.admission=8;self.admission_index=0;self.admission_frame=0;}},
            8=>{if let Some(spread)=doc.spreads.get(self.admission_index){if spread.id.len()>256||spread.name.len()>256||spread.page_ids.len()>MAX_INTERCHANGE_PAGES{return Err("layout.idml.spread-envelope".into());}if let Some(id)=spread.page_ids.get(self.admission_frame){if !doc.pages.iter().any(|page|&page.id==id&&page.spread_id==spread.id){return Err("layout.idml.spread-membership".into());}self.admission_frame+=1;}else{self.admission_index+=1;self.admission_frame=0;}}else{self.admission=9;}},
            _=>{},
        }
        if self.input_bytes>MAX_INTERCHANGE_BYTES{return Err("layout.interchange.byte-limit".into());}Ok(())
    }
    pub fn advance(&mut self,doc:&LayoutSnapshot)->Result<InterchangeExportAdvance,String>{
        self.completed=self.completed.saturating_add(1);
        if self.admission<9{self.admit_one(doc)?;return Ok(InterchangeExportAdvance::Progress{completed_units:self.completed});}
        if self.cursor<self.pending.len(){let end=(self.cursor+CHUNK).min(self.pending.len());let bytes=match &mut self.kind{Kind::Step{..}=>self.pending[self.cursor..end].to_vec(),Kind::Idml{writer,..}=>writer.write_chunk(&self.pending[self.cursor..end])?};self.cursor=end;return self.publish(bytes);}
        if self.pending.capacity()!=0{self.pending=Vec::new();self.cursor=0;return Ok(InterchangeExportAdvance::Progress{completed_units:self.completed});}
        match &mut self.kind{
            Kind::Step{session}=>{match session.advance(doc,self.selected_page)?{step::StepAdvance::Progress=>Ok(InterchangeExportAdvance::Progress{completed_units:self.completed}),step::StepAdvance::Fragment(fragment)=>{self.pending=fragment;Ok(InterchangeExportAdvance::Progress{completed_units:self.completed})},step::StepAdvance::Done=>Ok(InterchangeExportAdvance::Done)}},
            Kind::Idml{started,writer,part,layers}=>{
                if !*started{*started=true;let bytes=writer.known_entry("mimetype",idml::MIME.as_bytes())?;return self.publish(bytes);}
                let mut chunk=None;
                match part{
                    Part::Story{index,stage,text}=>{
                        if *index>=doc.stories.len(){*part=Part::Master{index:0,stage:0,frame:0};}
                        else{let story=&doc.stories[*index];match *stage{
                            0=>{chunk=Some(writer.begin_entry(&format!("Stories/Story_{index}.xml"))?);*stage=1;},
                            1=>{let mut bytes=open("idPkg:Story",vec![("xmlns:idPkg",idml::NS.into()),("DOMVersion","8.0".into())])?;bytes.extend(open("Story",vec![("Self",story.id.clone())])?);self.pending=bytes;*stage=2;},
                            2=>{if *text>=story.content.len(){self.pending=b"</Story></idPkg:Story>".to_vec();*stage=3;}else{if self.story_plan.is_none(){self.story_plan=Some(idml::text_styles::StoryFragmentPlan::new(story,*text)?);}if let Some((end,node))=self.story_plan.as_mut().unwrap().advance(story)?{self.pending=fragment(node)?;*text=end;self.story_plan=None;}}},
                            _=>{chunk=Some(writer.end_entry()?);*index+=1;*stage=0;*text=0;},
                        }}
                    },
                    Part::Master{index,stage,frame}=>{
                        if *index>=doc.parent_pages.len(){*part=Part::Spread{index:0,stage:0,page:0,frame:0,x:0.0};}
                        else{let parent=&doc.parent_pages[*index];match *stage{
                            0=>{chunk=Some(writer.begin_entry(&format!("MasterSpreads/MasterSpread_{index}.xml"))?);*stage=1;},
                            1=>{let mut bytes=open("idPkg:MasterSpread",vec![("xmlns:idPkg",idml::NS.into()),("DOMVersion","8.0".into())])?;bytes.extend(open("MasterSpread",vec![("Self",parent.id.clone()),("Name",parent.name.clone()),("NamePrefix","A".into()),("BaseName",parent.name.clone()),("PageCount","1".into())])?);bytes.extend(fragment(idml::element("Page",vec![("Self",format!("{}-page",parent.id)),("Name",parent.name.clone()),("GeometricBounds",format!("0 0 {} {}",parent.height,parent.width)),("ItemTransform","1 0 0 1 0 0".into()),("AppliedMaster","n".into())],Vec::new()))?);self.pending=bytes;*stage=2;},
                            2=>{if let Some(value)=parent.frames.get(*frame){if matches!(value,Frame::Image{..}){*part=Part::Image{master:true,page:*index,frame:*frame,stage:0,offset:0,spread:None};}else{self.pending=fragment(idml::frame_node(doc,value)?)?;*frame+=1;}}else{self.pending=b"</MasterSpread></idPkg:MasterSpread>".to_vec();*stage=3;}},
                            _=>{chunk=Some(writer.end_entry()?);*index+=1;*stage=0;*frame=0;},
                        }}
                    },
                    Part::Spread{index,stage,page,frame,x}=>{
                        if *index>=doc.spreads.len(){*part=Part::Styles{stage:0,index:0};}else{let spread=&doc.spreads[*index];match *stage{
                            0=>{chunk=Some(writer.begin_entry(&format!("Spreads/Spread_{index}.xml"))?);*stage=1;},
                            1=>{let mut bytes=open("idPkg:Spread",vec![("xmlns:idPkg",idml::NS.into()),("DOMVersion","8.0".into())])?;bytes.extend(open("Spread",vec![("Self",spread.id.clone()),("PageCount",spread.page_ids.len().to_string())])?);bytes.extend(fragment(idml::spread_properties(spread))?);self.pending=bytes;*stage=2;},
                            2=>{if let Some(id)=spread.page_ids.get(*page){let source=doc.pages.iter().find(|page|&page.id==id&&page.spread_id==spread.id).ok_or("layout.idml.spread-membership")?;self.pending=fragment(idml::page_node_at(source,*x))?;*frame=0;*stage=6;}else{self.pending=b"</Spread></idPkg:Spread>".to_vec();*stage=5;}},
                            6=>{let id=spread.page_ids.get(*page).ok_or("layout.idml.spread-membership")?;let(global,source)=doc.pages.iter().enumerate().find(|(_,page)|&page.id==id&&page.spread_id==spread.id).ok_or("layout.idml.spread-membership")?;if *frame<source.overrides.len(){if matches!(idml::overrides::source(doc,source,*frame)?,Frame::Image{..}){*part=Part::OverrideImage{page:global,frame:*frame,stage:0,offset:0,spread:(*index,*page,*x)};}else{self.pending=fragment(idml::overrides::node(doc,source,*frame,*x)?)?;*frame+=1;}}else{*frame=0;*stage=3;}},
                            3=>{let id=spread.page_ids.get(*page).ok_or("layout.idml.spread-membership")?;let(global,source)=doc.pages.iter().enumerate().find(|(_,page)|&page.id==id&&page.spread_id==spread.id).ok_or("layout.idml.spread-membership")?;if let Some(value)=source.frames.get(*frame){if matches!(value,Frame::Image{..}){*part=Part::Image{master:false,page:global,frame:*frame,stage:0,offset:0,spread:Some((*index,*page,*x))};}else{self.pending=fragment(idml::frame_node_at(doc,value,*x)?)?;*frame+=1;}}else{*frame=0;*stage=4;}},
                            4=>{let id=spread.page_ids.get(*page).ok_or("layout.idml.spread-membership")?;let source=doc.pages.iter().find(|page|&page.id==id).ok_or("layout.idml.spread-membership")?;if *frame<source.guides.len(){for node in idml::guides::nodes_for(source,*frame,*x,*page)?{self.pending.extend(fragment(node)?);}*frame+=1;}else{*x+=source.width;*page+=1;*stage=2;}},
                            _=>{chunk=Some(writer.end_entry()?);*index+=1;*stage=0;*page=0;*frame=0;*x=0.0;},
                        }}
                    },
                    Part::Image{master,page,frame,stage,offset,spread}=>{let frames=if *master{doc.parent_pages.get(*page).map(|page|&page.frames)}else{doc.pages.get(*page).map(|page|&page.frames)};let source=frames.and_then(|frames|frames.get(*frame)).ok_or("layout.idml.image-frame")?;match *stage{
                        0=>{self.pending=idml::assets::export_parts(doc,source,spread.as_ref().map_or(0.0,|resume|resume.2))?.0;*stage=1;},
                        1=>{let payload=idml::assets::payload(doc,source)?;if *offset<payload.len(){let end=(*offset+CHUNK).min(payload.len());let bytes=&payload.as_bytes()[*offset..end];if bytes.iter().any(|byte|!byte.is_ascii_alphanumeric()&&!b"+/=".contains(byte)){return Err("layout.idml.image-base64".into());}self.pending=bytes.to_vec();*offset=end;}else{self.pending=idml::assets::export_parts(doc,source,spread.as_ref().map_or(0.0,|resume|resume.2))?.2;*stage=2;}},
                        _=>{*part=if *master{Part::Master{index:*page,stage:2,frame:*frame+1}}else{let(index,page_position,x)=spread.ok_or("layout.idml.spread-resume")?;Part::Spread{index,stage:3,page:page_position,frame:*frame+1,x}};},
                    }},
                    Part::OverrideImage{page,frame,stage,offset,spread}=>{let source=doc.pages.get(*page).ok_or("layout.idml.override-page")?;match *stage{
                        0=>{self.pending=idml::overrides::image_parts(doc,source,*frame,spread.2)?.0;*stage=1;},
                        1=>{let payload=idml::overrides::payload(doc,source,*frame)?;if *offset<payload.len(){let end=(*offset+CHUNK).min(payload.len());let bytes=&payload.as_bytes()[*offset..end];if bytes.iter().any(|byte|!byte.is_ascii_alphanumeric()&&!b"+/=".contains(byte)){return Err("layout.idml.image-base64".into());}self.pending=bytes.to_vec();*offset=end;}else{self.pending=idml::overrides::image_parts(doc,source,*frame,spread.2)?.2;*stage=2;}},
                        _=>{let(index,page_position,x)=*spread;*part=Part::Spread{index,stage:6,page:page_position,frame:*frame+1,x};},
                    }},
                    Part::Styles{stage,index}=>{match *stage{
                        0=>{chunk=Some(writer.begin_entry("Resources/Styles.xml")?);let mut bytes=open("idPkg:Styles",vec![("xmlns:idPkg",idml::NS.into()),("DOMVersion","8.0".into())])?;bytes.extend(open("RootParagraphStyleGroup",vec![("Self","root-paragraph".into())])?);bytes.extend(fragment(idml::element("ParagraphStyle",vec![("Self","ParagraphStyle/$ID/NormalParagraphStyle".into()),("Name","$ID/NormalParagraphStyle".into())],Vec::new()))?);self.pending=bytes;*stage=1;},
                        1=>{if let Some(style)=doc.paragraph_styles.get(*index){self.pending=fragment(idml::text_styles::paragraph_node(style)?)?;*index+=1;}else{let mut bytes=b"</RootParagraphStyleGroup>".to_vec();bytes.extend(open("RootCharacterStyleGroup",vec![("Self","root-character".into())])?);bytes.extend(fragment(idml::element("CharacterStyle",vec![("Self","CharacterStyle/$ID/[No character style]".into()),("Name","$ID/[No character style]".into())],Vec::new()))?);self.pending=bytes;*index=0;*stage=2;}},
                        2=>{if let Some(style)=doc.character_styles.get(*index){self.pending=fragment(idml::text_styles::character_node(style)?)?;*index+=1;}else{self.pending=b"</RootCharacterStyleGroup></idPkg:Styles>".to_vec();*stage=3;}},
                        _=>{chunk=Some(writer.end_entry()?);*part=Part::Graphic{stage:1,page:0,frame:0};},
                    }},
                    Part::Graphic{stage,page,frame}=>{match *stage{
                        0=>{chunk=Some(writer.end_entry()?);*stage=1;},
                        1=>{chunk=Some(writer.begin_entry("Resources/Graphic.xml")?);let mut bytes=open("idPkg:Graphic",vec![("xmlns:idPkg",idml::NS.into()),("DOMVersion","8.0".into())])?;bytes.extend(fragment(idml::element("Swatch",vec![("Self","Swatch/None".into()),("Name","$ID/None".into())],Vec::new()))?);bytes.extend(fragment(idml::element("Color",vec![("Self","Color/Black".into()),("Name","Black".into()),("Model","Process".into()),("Space","RGB".into()),("ColorValue","0 0 0".into())],Vec::new()))?);self.pending=bytes;*stage=2;},
                        2=>{if let Some(frames)=doc.pages.get(*page).map(|page|&page.frames).or_else(||doc.parent_pages.get(page.saturating_sub(doc.pages.len())).map(|page|&page.frames)){if let Some(value)=frames.get(*frame){for color in idml::frame_colors(value)?{self.pending.extend(fragment(color)?);}*frame+=1;}else{*page+=1;*frame=0;}}else{*stage=3;*frame=0;}},
                        3=>{if let Some(style)=doc.character_styles.get(*frame){if let Some(color)=idml::text_styles::character_color(style)?{self.pending=fragment(color)?;}*frame+=1;}else{self.pending=b"</idPkg:Graphic>".to_vec();*stage=4;}},
                        _=>{chunk=Some(writer.end_entry()?);*part=Part::Design{stage:1,index:0,page:0,layer:0};},
                    }},
                    Part::Design{stage,index,page,layer}=>{match *stage{
                        0=>{chunk=Some(writer.end_entry()?);*stage=1;},
                        1=>{chunk=Some(writer.begin_entry("designmap.xml")?);*stage=2;},
                        2=>{self.pending=open("Document",vec![("xmlns:idPkg",idml::NS.into()),("DOMVersion","8.0".into()),("Self","d".into()),("Name",doc.name.clone())])?;*stage=3;},
                        3=>{if let Some(source)=doc.pages.get(*page).map(|page|&page.layers).or_else(||doc.parent_pages.get(page.saturating_sub(doc.pages.len())).map(|page|&page.layers)){if let Some(value)=source.get(*layer){if !layers.contains(&value.id){layers.push(value.id.clone());self.pending=fragment(idml::element("Layer",vec![("Self",value.id.clone()),("Name",value.name.clone()),("Visible",value.visible.to_string()),("Locked",value.locked.to_string())],Vec::new()))?;}*layer+=1;}else{*page+=1;*layer=0;}}else{*stage=4;*index=0;}},
                        4=>{if *index<doc.stories.len(){self.pending=fragment(idml::element("idPkg:Story",vec![("src",format!("Stories/Story_{index}.xml"))],Vec::new()))?;*index+=1;}else{*stage=5;*index=0;}},
                        5=>{if *index<doc.parent_pages.len(){self.pending=fragment(idml::element("idPkg:MasterSpread",vec![("src",format!("MasterSpreads/MasterSpread_{index}.xml"))],Vec::new()))?;*index+=1;}else{*stage=6;*index=0;}},
                        6=>{if *index<doc.spreads.len(){self.pending=fragment(idml::element("idPkg:Spread",vec![("src",format!("Spreads/Spread_{index}.xml"))],Vec::new()))?;*index+=1;}else{self.pending=b"<idPkg:Styles src=\"Resources/Styles.xml\"/><idPkg:Graphic src=\"Resources/Graphic.xml\"/></Document>".to_vec();*stage=7;}},
                        _=>{chunk=Some(writer.end_entry()?);*part=Part::Container;},
                    }},
                    Part::Container=>{chunk=Some(writer.begin_entry("META-INF/container.xml")?);self.pending=fragment(idml::element("container",vec![("version","1.0".into()),("xmlns","urn:oasis:names:tc:opendocument:xmlns:container".into())],vec![idml::element("rootfiles",Vec::new(),vec![idml::element("rootfile",vec![("full-path","designmap.xml".into()),("media-type","text/xml".into())],Vec::new())])]))?;*part=Part::Central;},
                    Part::Central=>{if writer_active(writer){chunk=Some(writer.end_entry()?);}else if let Some(bytes)=writer.finish_one()?{chunk=Some(bytes);}else{*part=Part::Done;}},
                    Part::Done=>return Ok(InterchangeExportAdvance::Done),
                }
                if let Some(chunk)=chunk{return self.publish(chunk);}Ok(InterchangeExportAdvance::Progress{completed_units:self.completed})
            }
        }
    }
    pub fn next_release_bytes(&self)->usize{if self.story_plan.is_some(){return 0;}if self.pending.capacity()!=0{return self.pending.capacity();}match &self.kind{Kind::Step{session}=>session.next_release_bytes(),Kind::Idml{writer,layers,..}=>layers.last().map_or_else(||if layers.capacity()!=0{layers.capacity()*std::mem::size_of::<String>()}else{writer.next_release_bytes()},String::capacity)}}
    pub fn close_one(&mut self,maximum_bytes:usize)->bool{if self.story_plan.take().is_some(){return true;}if self.next_release_bytes()>maximum_bytes{return false;}if self.pending.capacity()!=0{self.pending=Vec::new();self.cursor=0;return true;}match &mut self.kind{Kind::Step{session}=>session.close_one(maximum_bytes),Kind::Idml{writer,layers,..}=>if layers.pop().is_some(){true}else if layers.capacity()!=0{*layers=Vec::new();true}else{writer.close_one(maximum_bytes)}}}
    pub fn terminal_is_empty(&self)->bool{self.story_plan.is_none()&&self.pending.capacity()==0&&match &self.kind{Kind::Step{session}=>session.terminal_is_empty(),Kind::Idml{writer,layers,..}=>layers.capacity()==0&&writer.terminal_is_empty()}}
}
fn writer_active(writer:&StoredZipStream)->bool{writer.entry_is_open()}
