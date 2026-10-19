#[derive(Clone,Debug,PartialEq,Eq,Default)]
pub struct ArtifactDialect{pub artifact_kind:String,pub standard:String,pub subset:String}
#[derive(Clone,Debug,PartialEq,Eq,Default)]
pub struct ArtifactRef{pub artifact_id:String,pub dialect:ArtifactDialect}
