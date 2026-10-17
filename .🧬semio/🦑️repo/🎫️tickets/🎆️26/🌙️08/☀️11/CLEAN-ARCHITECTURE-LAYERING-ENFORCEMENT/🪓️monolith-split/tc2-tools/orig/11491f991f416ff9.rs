//! 🔤️ Authored text selects one exact shipped family without inferred substitution.
#[derive(Clone,Copy,Debug,PartialEq,Eq,semio_framework_value::RetainedClone,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test,derive(serde::Serialize,serde::Deserialize))]
#[value(rename_all="camelCase")]
#[cfg_attr(test,serde(rename_all="camelCase"))]
pub enum DrawingFontFamily{Anta,KellySlab,ShareTechMono,NotoEmoji}
impl DrawingFontFamily{
 pub const fn as_str(self)->&'static str{match self{Self::Anta=>"anta",Self::KellySlab=>"kellySlab",Self::ShareTechMono=>"shareTechMono",Self::NotoEmoji=>"notoEmoji"}}
 pub const fn catalog_family(self)->&'static str{match self{Self::Anta=>"Anta",Self::KellySlab=>"Kelly Slab",Self::ShareTechMono=>"Share Tech Mono",Self::NotoEmoji=>"Noto Emoji"}}
 pub fn from_catalog_family(value:&str)->Result<Self,&'static str>{match value{"Anta"=>Ok(Self::Anta),"Kelly Slab"=>Ok(Self::KellySlab),"Share Tech Mono"=>Ok(Self::ShareTechMono),"Noto Emoji"=>Ok(Self::NotoEmoji),_=>Err("Unsupported authored font family")}}
 pub fn parse(value:&str)->Result<Self,&'static str>{match value{"anta"=>Ok(Self::Anta),"kellySlab"=>Ok(Self::KellySlab),"shareTechMono"=>Ok(Self::ShareTechMono),"notoEmoji"=>Ok(Self::NotoEmoji),_=>Err("Unsupported authored font family")}}
}
