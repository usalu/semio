const ticket=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O";
const root="✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/";
const fixture={slot:"tiles",kind:"s.stdio.semio",many:true,tiles:[{id:"inline\0引用😀",media:{kind:"mesh",positions:[0,0,0,1,0,0,0,1,0],indices:[0,1,2]}},{id:"child引用😀",media:{kind:"meshChild",childId:"mesh-child\0引用😀",artifactId:"mesh-artifact引用😀",artifactKind:"s.stdio.semio",standard:"v1",subset:"mesh"}}],expected:[["tiles","mesh-child\0引用😀","mesh-artifact引用😀","s.stdio.semio","v1","mesh"]]};
const schemaPath=root+"🦀️.rs",before=await Bun.file(schemaPath).text();
const definitions=[
"/// 🧸️ Projects each tile through its declared media branch.",
"impl semio_framework_schema_composition::ChildFieldRefs for Tile {",
" const MANY:bool=false;",
" fn visit_child_field<'a,V:semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self,slot:&'static str,visitor:&mut V)->Result<(),V::Error>{visitor.step()?;self.media.visit_child_field(slot,visitor)}",
"}",
"/// 🧸️ Inline geometry contributes no child identity.",
"impl semio_framework_schema_composition::ChildFieldRefs for TileMedia3d {",
" const MANY:bool=false;",
" fn visit_child_field<'a,V:semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self,slot:&'static str,visitor:&mut V)->Result<(),V::Error>{visitor.step()?;match self{Self::Mesh{..}=>Ok(()),Self::MeshChild{child}=>child.visit_child_field(slot,visitor)}}",
"}",
"/// 👁️ Admits a nested tile media child into the genuine captured source.",
"impl<R:semio_framework_schema_composition::ChildReadSource> semio_framework_schema_composition::ChildFieldReadAdmission<R> for Tile {",
" fn admit_child_field(&self,slot:&'static str,source:&mut R)->Result<(),R::Error>{source.step()?;semio_framework_schema_composition::ChildFieldReadAdmission::admit_child_field(&self.media,slot,source)}",
"}",
"/// 👁️ Retains the exact Semio mesh snapshot type at the source admission.",
"impl<R:semio_framework_schema_composition::ChildReadSource> semio_framework_schema_composition::ChildFieldReadAdmission<R> for TileMedia3d {",
" fn admit_child_field(&self,slot:&'static str,source:&mut R)->Result<(),R::Error>{source.step()?;if let Self::MeshChild{child}=self{source.step()?;source.child::<SemioMeshSnapshot>(slot,semio_framework_schema_composition::ChildRefFields{child_id:&child.child_id,artifact_id:&child.target.artifact_id,artifact_kind:&child.target.dialect.artifact_kind,standard:&child.target.dialect.standard,subset:&child.target.dialect.subset})?;}Ok(())}",
"}",""].join("\n");
const needle="    pub tiles: Vec<Tile>,";
if(before.split(needle).length!==2)throw Error("exact tiles field guard");
const after=before.replace(needle,'    #[child(kind = "s.stdio.semio")]\n'+needle)+"\n"+definitions;
const testPath=root+"🧪️tests/🔬️unit/🦀️.rs",testBefore=await Bun.file(testPath).text();
const test=[
"/// 🧸️ Every nested mesh identity reaches capture while inline coordinates remain unchanged.",
"#[test]",
"fn declared_nested_mesh_child_projection_preserves_full_identity_and_inline_media(){",
" use semio_framework_schema_composition::{ArtifactCompositionFields,ChildRefFields,ChildRefVisitor};",
' let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️mesh-child/🔣️.json")).expect("closed nested media contract");',
' let text=|value:&serde_json::Value|value.as_str().expect("text").to_string();',
' let tiles=fixture["tiles"].as_array().expect("tiles").iter().map(|row|{let media=&row["media"];Tile{id:text(&row["id"]),label:None,weight:1.0,media:match media["kind"].as_str().expect("kind"){',
' "mesh"=>TileMedia3d::Mesh{positions:media["positions"].as_array().expect("positions").iter().map(|v|v.as_f64().expect("coordinate")).collect(),indices:media["indices"].as_array().expect("indices").iter().map(|v|u32::try_from(v.as_u64().expect("index")).expect("u32")).collect(),color:None},',
' "meshChild"=>TileMedia3d::MeshChild{child:store::ArtifactChild::new(text(&media["childId"]),store::os_io::ArtifactRef{artifact_id:text(&media["artifactId"]),dialect:store::os_io::ArtifactDialect{artifact_kind:text(&media["artifactKind"]),standard:text(&media["standard"]),subset:text(&media["subset"])}})},',
' _=>panic!("closed media branch"),}}}).collect();',
" let document=Wfc3dSnapshot{tiles,..Wfc3dSnapshot::default()};",
" struct Projection{rows:Vec<serde_json::Value>}",
" impl<'a> ChildRefVisitor<'a> for Projection{type Error=();fn step(&mut self)->Result<(),Self::Error>{Ok(())}fn child(&mut self,slot:&'static str,value:ChildRefFields<'a>)->Result<(),Self::Error>{self.rows.push(serde_json::json!([slot,value.child_id,value.artifact_id,value.artifact_kind,value.standard,value.subset]));Ok(())}}",
' let mut projection=Projection{rows:Vec::new()};document.visit_child_refs(&mut projection).expect("bounded projection");',
' assert_eq!(serde_json::json!(projection.rows),fixture["expected"]);',
' let slots=Wfc3dSnapshot::child_slots();assert_eq!(slots.len(),1);assert_eq!(slots[0].name,fixture["slot"].as_str().expect("slot"));assert_eq!(slots[0].kind,fixture["kind"].as_str().expect("kind"));assert_eq!(slots[0].many,fixture["many"].as_bool().expect("many"));',
' let TileMedia3d::Mesh{positions,indices,color}=&document.tiles[0].media else{panic!("original inline branch");};assert_eq!(serde_json::json!(positions),fixture["tiles"][0]["media"]["positions"]);assert_eq!(serde_json::json!(indices),fixture["tiles"][0]["media"]["indices"]);assert_eq!(*color,None);',
' let oracle:Vec<serde_json::Value>=fixture["tiles"].as_array().expect("tiles").iter().filter_map(|row|{let media=&row["media"];(media["kind"]=="meshChild").then(||serde_json::json!([fixture["slot"],media["childId"],media["artifactId"],media["artifactKind"],media["standard"],media["subset"]]))}).collect();assert_eq!(serde_json::json!(oracle),serde_json::json!(projection.rows));',
' eprintln!("[DEBUG] Declared nested tile media preserves complete mesh child identity and original inline coordinates");',
"}",""].join("\n");
const pairs=[{path:root+"🧪️tests/🧫️fixtures/🪆️mesh-child/🔣️.json",before:"",after:JSON.stringify(fixture,null,2)+"\n"},{path:testPath,before:testBefore,after:testBefore+"\n"+test},{path:schemaPath,before,after}];
for(const p of pairs.filter(p=>p.path.endsWith(".rs"))){const proc=Bun.spawn(["rustfmt","--edition","2024","--emit","stdout","--config","skip_children=true"],{stdin:new Blob([p.after]),stdout:"ignore",stderr:"pipe"});const e=await new Response(proc.stderr).text();if(await proc.exited!==0)throw Error(e);}
await Bun.write(ticket+"/📥️inputs/wfc3d-declared-nested-mesh-child-projection-held-pairs.json",JSON.stringify({state:"demand first; producer held; no genuine frame or geometry admission credit",pairs},null,2));
for(const p of pairs.slice(0,2)){const current=await Bun.file(p.path).exists()?await Bun.file(p.path).text():"";if(current!==p.before)throw Error("guard "+p.path);await Bun.write(p.path,p.after);}
console.log("closed fixture and original-law-preserving demand2 mounted; schema producer held");

