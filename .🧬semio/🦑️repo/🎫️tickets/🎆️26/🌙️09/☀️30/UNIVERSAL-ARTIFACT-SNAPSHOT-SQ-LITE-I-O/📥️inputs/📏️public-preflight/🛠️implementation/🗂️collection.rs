//! 🗂️ Held preflight borrows folders, entries and the exact document/blob variant payload.
use super::*;
#[path="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/📏️public-preflight/🛠️implementation/🦀️.rs"]mod census;
pub(super)fn check(value:&CollectionSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 let total=rows(value)?;let mut c=census::Census::new(control,CollectionSnapshot::SQLITE_SCHEMA,total)?;
 c.entity(1)?;c.labels(&["collection","schema","name","folders","entries"])?;c.text(&value.schema)?;c.text(&value.name)?;
 for row in &value.folders{c.entity(3)?;c.labels(&["id","parent-id","name"])?;c.text(&row.id)?;c.optional(row.parent_id.as_deref())?;c.text(&row.name)?;}
 for row in &value.entries{
  c.entity(3)?;c.labels(&["id","folder-id","name","kind-id","body"])?;c.text(&row.id)?;c.optional(row.folder_id.as_deref())?;c.text(&row.name)?;c.text(&row.kind_id)?;
  match row.body.as_ref(){
   ArtifactBody::Document{schema,document_id}=>{c.text("document")?;c.entity(1)?;c.labels(&["document","schema","document-id"])?;c.text(schema)?;c.text(document_id)?;}
   ArtifactBody::Blob{blob}=>{c.text("blob")?;c.entity(3)?;c.labels(&["blob","hash","size","media-type"])?;c.text(&blob.hash)?;c.text(&blob.media_type)?;}
  }
 }
 c.finish(encoding,Some(CollectionSnapshot::__DSL_ENVELOPE_ID))
}

