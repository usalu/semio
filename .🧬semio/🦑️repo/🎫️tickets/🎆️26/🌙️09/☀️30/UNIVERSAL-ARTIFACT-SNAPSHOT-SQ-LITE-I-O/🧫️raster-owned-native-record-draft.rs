//! 🖨️ Literal typed Raster native graph; staged before genuine owning capability RED.
#[derive(dsl::DslRecord)]
#[dsl(extension="raster")]
struct RasterNativeDocument{
 schema:String,
 id:String,
 title:Option<String>,
 layers:Vec<RasterNativeLayer>,
 assets:Vec<RasterNativeAsset>,
 values:Vec<RasterNativeValue>,
 items:Vec<RasterNativeItem>,
 members:Vec<RasterNativeMember>,
}
#[derive(dsl::DslRecord)]
struct RasterNativeAsset{key:String,child:RasterAssetChild}
#[derive(dsl::DslRecord)]
struct RasterNativeLayer{
 parent:Option<u64>,
 ordinal:u64,
 kind:String,
 id:String,
 name:String,
 visible:bool,
 locked:bool,
 opacity:f32,
 blend:String,
 #[dsl(block)]
 transform:RasterTransform,
 #[dsl(block)]
 mask:Option<RasterLayerMask>,
 width:Option<u32>,
 height:Option<u32>,
 image:Option<String>,
 adjustment:Option<String>,
 parameters:Vec<RasterNativeParameter>,
}
#[derive(dsl::DslRecord)]
struct RasterNativeParameter{key:String,root:u64}
#[derive(dsl::DslRecord)]
struct RasterNativeValue{
 kind:String,
 boolean:Option<bool>,
 signed:Option<i64>,
 unsigned:Option<u64>,
 float:Option<f64>,
 text:Option<String>,
 #[dsl(base64)]
 bytes:Option<Vec<u8>>,
}
#[derive(dsl::DslRecord)]
struct RasterNativeItem{array:u64,ordinal:u64,value:u64}
#[derive(dsl::DslRecord)]
struct RasterNativeMember{object:u64,ordinal:u64,key:String,value:u64}
