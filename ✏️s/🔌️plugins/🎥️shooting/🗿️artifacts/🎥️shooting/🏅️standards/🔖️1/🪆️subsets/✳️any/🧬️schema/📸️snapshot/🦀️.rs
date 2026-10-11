//! 🎥️ Persisted Shooting fields and literal composed emblem ownership.
use crate::{ShootingAsset,ShootingEmblemChild,ShootingSavedCamera,ShootingSceneLighting,ShootingShot,SHOOTING_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
/// 📸️ Complete Shooting document snapshot, with ordered records and optional child reference.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all="camelCase")]
#[artifact_schema(id="s.shooting.shooting")]
#[dsl(extension="shooting")]
#[dsl(layout="lines")]
pub struct ShootingSnapshot{
 #[state(artifact)]
 pub schema:String,
 #[state(artifact)]
 #[value(default)]
 #[dsl(table)]
 pub assets:Vec<ShootingAsset>,
 #[state(artifact)]
 #[value(default)]
 #[dsl(table)]
 pub saved_cameras:Vec<ShootingSavedCamera>,
 #[state(artifact)]
 #[value(default)]
 #[dsl(block)]
 pub scene:ShootingSceneLighting,
 #[state(artifact)]
 #[value(default)]
 #[dsl(table)]
 pub shots:Vec<ShootingShot>,
 #[state(artifact)]
 #[value(default)]
 pub active_shot_id:String,
 #[state(artifact)]
 #[value(default)]
 pub active_asset_id:String,
 #[state(artifact)]
 #[child(kind="s.stdio.semio")]
 #[value(default,skip_serializing_if="Option::is_none")]
 pub emblem:Option<ShootingEmblemChild>,
}
impl Default for ShootingSnapshot{
 fn default()->Self{Self{schema:SHOOTING_DOCUMENT_SCHEMA.into(),assets:Vec::new(),saved_cameras:Vec::new(),scene:ShootingSceneLighting::default(),shots:Vec::new(),active_shot_id:String::new(),active_asset_id:String::new(),emblem:None}}
}



