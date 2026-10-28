//! 📐️ STEP planar XY projection through the canonical AP214 graph and faceted B-rep codec.
use super::*;
use semio_s_artifact_stdio_contract::part21::{parse_part21,Part21Document};
#[path="🧵️export/🦀️.rs"]
mod content_export;
#[path="📐️geometry/🦀️.rs"]
mod geometry;
#[path="🖍️background/🦀️.rs"]
mod background;
pub(super) use content_export::{StepExportSession,StepAdvance};
use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::io::projection::project_edges;
use semio_framework_io_ports::{VectorScene,VectorCanvas,VectorRgba,VectorLayer,VectorNode,VectorStyle,VectorTransform,VectorSegment,VectorPoint2};
use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::io::brep::{analyze_brep_mesh, brep_mesh_to_part21, BrepFace, BrepMesh, BrepVertex};
#[path="📥️import/🦀️.rs"]
mod content_import;
pub(super) use content_import::import;
pub(super) fn export(doc:&LayoutSnapshot,control:&mut InterchangeControl<'_>)->Result<InterchangeResult<Vec<u8>>,String>{let mut session=StepExportSession::new();let mut bytes=Vec::new();let mut completed=0usize;loop{checkpoint(control,completed,completed.saturating_add(1))?;completed+=1;match session.advance(doc,None)?{StepAdvance::Fragment(fragment)=>{if bytes.len()+fragment.len()>MAX_INTERCHANGE_BYTES{return Err("layout.interchange.byte-limit".into());}bytes.extend(fragment);},StepAdvance::Progress=>{},StepAdvance::Done=>break,}}while !session.terminal_is_empty(){session.close_one(session.next_release_bytes());}Ok(InterchangeResult{value:bytes,fidelity:"projection",diagnostics:vec![diagnostic("layout.step.draughting","The page solid, visible resolved parent/body rectangle geometry, RGB fill/curve styles, positioned Unicode text annotations and image placement/source references are emitted as actual AP214 entities in millimetres. ISO3098 substitutes text fonts; typography flow, paint alpha and image pixels are not embedded. Background layers use borrowed scene events, nested transforms, bounded path batches, default-size text and image placement/MIME annotations. Curved segments are projected by deterministic samples; endpoint arcs use at most64segments; compound and over-envelope fills currently refuse explicitly.")]})}
pub(super) fn sheet_graph(page:&Page)->Result<Part21Document,String>{check_dimensions(page.width,page.height)?;let(w,h,depth)=(page.width*25.4/72.0,page.height*25.4/72.0,0.1);let vertices=[[0.0,0.0,0.0],[w,0.0,0.0],[w,h,0.0],[0.0,h,0.0],[0.0,0.0,depth],[w,0.0,depth],[w,h,depth],[0.0,h,depth]].into_iter().map(|[x,y,z]|BrepVertex{x,y,z}).collect();let faces=[[3,2,1,0],[4,5,6,7],[0,1,5,4],[1,2,6,5],[2,3,7,6],[3,0,4,7]].into_iter().map(|indices|BrepFace{indices:indices.to_vec()}).collect();let mut graph=brep_mesh_to_part21(&BrepMesh{vertices,faces});if let Some(instance)=graph.instances.iter_mut().find(|instance|instance.is_type("MANIFOLD_SOLID_BREP")){if let Some((_,args))=instance.entities.iter_mut().find(|(name,_)|name=="MANIFOLD_SOLID_BREP"){args[0]=semio_s_artifact_stdio_contract::part21::Part21Value::Str(format!("Layout page {}",page.id));}}Ok(graph)}
