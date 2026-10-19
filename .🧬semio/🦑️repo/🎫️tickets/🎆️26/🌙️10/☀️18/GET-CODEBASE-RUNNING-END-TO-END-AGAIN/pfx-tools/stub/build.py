import re,os,sys,subprocess
sys.path.insert(0,"..")
import rs,gen
OUT=gen.OUT
def write(name,text):
    p=os.path.join(OUT,name);open(p,"w",encoding="utf-8").write(text);return p
def rustc(crate,src,externs,extra=()):
    cmd=["rustc","+nightly-2026-07-20","--edition","2021","--crate-type","lib","--crate-name",crate,"--emit=metadata","--cap-lints","warn","-L","dependency="+OUT,"--out-dir",OUT,src]
    for k,v in externs.items():cmd+=["--extern",f"{k}={v}"]
    cmd+=list(extra)
    r=subprocess.run(cmd,capture_output=True,text=True)
    return r.returncode,r.stderr
def rmeta(crate):return os.path.join(OUT,f"lib{crate}.rmeta")
def stage_stubs():
    here=gen.HERE
    ex={}
    for crate,f in (("semio_framework_artifact_reference","reference_stub.rs"),("semio_framework_value","value_stub.rs")):
        p=write(crate+".rs",open(os.path.join(here,f),encoding="utf-8").read())
        rc,err=rustc(crate,p,dict(ex));assert rc==0,err[:2000]
        ex[crate]=rmeta(crate)
    p=write("store.rs",open(os.path.join(here,"store_stub.rs"),encoding="utf-8").read())
    rc,err=rustc("store",p,dict(ex));assert rc==0,err[:3000]
    ex["store"]=rmeta("store")
    return ex
GEOM='''
#[derive(Clone,Copy,Debug,Default,PartialEq)] pub struct SemioPoint2{pub x:f64,pub y:f64}
#[derive(Clone,Copy,Debug,Default,PartialEq)] pub struct SemioPoint3{pub x:f64,pub y:f64,pub z:f64}
#[derive(Clone,Copy,Debug,Default,PartialEq)] pub struct SemioUv{pub u:f64,pub v:f64}
#[derive(Clone,Copy,Debug,Default,PartialEq)] pub struct SemioRgba{pub r:f32,pub g:f32,pub b:f32,pub a:f32}
#[derive(Clone,Copy,Debug,Default,PartialEq)] pub struct SemioQuaternion{pub x:f64,pub y:f64,pub z:f64,pub w:f64}
#[derive(Clone,Copy,Debug,Default,PartialEq)] pub struct SemioTransform{pub translation:SemioPoint3,pub rotation:SemioQuaternion,pub scale:SemioPoint3}
impl SemioTransform{pub fn identity()->Self{Self::default()}}
pub mod native{pub struct NativeF64(pub f64);pub struct NativeF32(pub f32);impl std::fmt::Display for NativeF64{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{Ok(())}} impl std::fmt::Display for NativeF32{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{Ok(())}}}
'''
def base_native_source():
    real=rs.SUBSETS+"✉️base/"+rs.SNAP+"🛬️native/🦀️.rs"
    s=open(real,encoding="utf-8").read()
    s=s.replace('#[path="🫙️workspace/🦀️.rs"]\npub mod workspace;','''pub mod workspace{
 use semio_framework_value::{ValueError,NativeDecodeControl};
 use semio_framework_value::retirement::RetireOwned;
 pub fn decode<T:RetireOwned>(control:&mut NativeDecodeControl<'_>,empty:impl FnOnce()->T,read:impl FnOnce(&mut T,&mut NativeDecodeControl<'_>)->Result<(),ValueError>)->Result<T,ValueError>{unimplemented!()}
}''')
    a=s.index("impl<T:semio_framework_value::retirement::RetireOwned> Drop for Owned<T>")
    i=s.index("{",a);d=0
    while True:
        e=rs.skip(s,i)
        if e is not None:i=e;continue
        if s[i]=="{":d+=1
        elif s[i]=="}":
            d-=1
            if d==0:break
        i+=1
    s=s[:a]+s[i+1:]
    return write("base_native.rs",s)
def build_base(ex):
    nat=base_native_source()
    enc=write('base_encode.rs',open(rs.SUBSETS+'✉️base/'+rs.SNAP+'🛫️native/🦀️.rs',encoding='utf-8').read().replace('#[derive(semio_framework_value::RetireOwned)]\n',''))
    lib=f'''#![allow(unused,dead_code)]
pub mod schema{{pub mod geometry{{{GEOM}}} pub mod child{{pub fn validate_semio_child_identity(_c:&str,_t:&semio_framework_artifact_reference::ArtifactRef,_s:&str)->Result<(),String>{{Ok(())}}}}}}
pub mod io{{
 pub mod sqlite{{pub mod snapshot{{#[path="{nat}"] pub mod native_decoding; #[path="{enc}"] pub mod native_encoding;}}}}
 pub mod binary{{pub mod codec{{pub fn read_transform(r:&mut store::ByteReader<'_>)->Result<crate::schema::geometry::SemioTransform,String>{{unimplemented!()}}}}}}
 pub mod text{{pub mod codec{{pub fn parse_u32(s:&str)->Result<u32,String>{{unimplemented!()}}}}}}
}}
'''
    p=write("base_lib.rs",lib)
    ex2=dict(ex);ex2["semio_framework_os_kernel"]=ex["store"]
    rc,err=rustc("semio_s_artifact_stdio_semio_base",p,ex2);
    if rc!=0:print("BASE FAIL\n",err[:6000])
    return rc==0
DOMAINS=[
 ("🔢️value","semio_s_artifact_stdio_semio_value_model",[],""),
 ("📑️document","semio_s_artifact_stdio_semio_document",[],""),
 ("🌊️flow","semio_s_artifact_stdio_semio_flow",[],""),
 ("🔤️text","semio_s_artifact_stdio_semio_text_model",[],"pub mod binary{pub mod snapshot{pub fn mark_kind_from_tag(t:u8)->Result<crate::schema::snapshot::SemioTextMarkKind,String>{unimplemented!()}}}"),
 ("🔊️audio","semio_s_artifact_stdio_semio_audio_model",[],"pub mod binary{pub mod snapshot{pub fn format_from_tag(t:u8)->Result<crate::schema::snapshot::SemioAudioFormat,String>{unimplemented!()}}}"),
 ("🎬️video","semio_s_artifact_stdio_semio_video_model",[],"pub mod binary{pub mod snapshot{pub fn kind_from_tag(t:u8)->Result<crate::schema::snapshot::SemioVideoStreamKind,String>{unimplemented!()}}}"),
 ("🖼️image","semio_s_artifact_stdio_semio_image",[],"pub mod binary{pub mod snapshot{pub fn colorspace_from_tag(t:u8)->Result<crate::schema::snapshot::SemioColorspace,String>{unimplemented!()}}} pub mod text{pub mod snapshot{pub fn parse_u8(s:&str)->Result<u8,String>{unimplemented!()}}}"),
 ("🎞️animation","semio_s_artifact_stdio_semio_animation",[],""),
 ("🏛️model","semio_s_artifact_stdio_semio_model",[],""),
 ("📐️cad","semio_s_artifact_stdio_semio_cad",[],""),
 ("🖊️drawing","semio_s_artifact_stdio_semio_drawing",[],""),
 ("🧊️brep","semio_s_artifact_stdio_semio_brep",[],""),
 ("🔺️mesh","semio_s_artifact_stdio_semio_mesh",[],""),
 ("📦️object","semio_s_artifact_stdio_semio_object",[],""),
 ("🧰️kit","semio_s_artifact_stdio_semio_kit",[],""),
 ("📊️table","semio_s_artifact_stdio_semio_table_model",["semio_s_artifact_stdio_semio_value_model"],""),
 ("🕸️graph","semio_s_artifact_stdio_semio_graph",["semio_s_artifact_stdio_semio_value_model"],""),
 ("📽️presentation","semio_s_artifact_stdio_semio_presentation",["semio_s_artifact_stdio_semio_document"],""),
]
EXTRA_SCHEMA={
 "📦️object":"pub struct SemioBrepSnapshot;pub struct SemioMeshSnapshot;pub struct SemioValueSnapshot;impl SemioObjectSnapshot{pub fn validate(&self)->Result<(),String>{Ok(())}}",
 "🧰️kit":"pub struct SemioObjectSnapshot;pub struct SemioModelSnapshot;pub struct SemioValueSnapshot;",
}
def build_domain(d,crate,deps,io_extra,ex):
    schema=gen.sanitize(open(rs.SUBSETS+d+"/🧬️schema/📸️snapshot/🦀️.rs",encoding="utf-8").read())
    uses="pub use semio_s_artifact_stdio_semio_base::schema::geometry::*;\n"
    if "semio_s_artifact_stdio_semio_value_model" in deps:uses+="pub use semio_s_artifact_stdio_semio_value_model::schema::snapshot::*;\n"
    if "semio_s_artifact_stdio_semio_document" in deps:uses+="use semio_s_artifact_stdio_semio_document::schema::snapshot::DocBlock;\n"
    nat=rs.native(d)
    lib=f'''#![allow(unused,dead_code)]
pub mod schema{{pub mod snapshot{{
{uses}{schema}
{EXTRA_SCHEMA.get(d,"")}
}}}}
pub mod io{{
 pub mod sqlite{{pub mod snapshot{{
  pub(crate) fn admit_layout(_:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),semio_framework_value::ValueError>{{Ok(())}}
  pub(crate) fn admit_binary(_:&[u8],_:&mut semio_framework_value::NativeDecodeControl<'_>,_:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),semio_framework_value::ValueError>{{Ok(())}}
  pub(crate) fn admit_document(_:&str,_:&mut semio_framework_value::NativeDecodeControl<'_>,_:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),semio_framework_value::ValueError>{{Ok(())}}
  #[path="{nat}"] pub mod native_decoding;
 }}}}
 {io_extra}
}}
'''
    p=write(crate+".rs",lib)
    e=dict(ex)
    rc,err=rustc(crate,p,e)
    return rc,err
def main(only=None):
    ex=stage_stubs()
    if not build_base(ex):return
    ex["semio_s_artifact_stdio_semio_base"]=rmeta("semio_s_artifact_stdio_semio_base")
    for d,crate,deps,io_extra in DOMAINS:
        if only and d not in only and crate not in only:
            continue
        rc,err=build_domain(d,crate,deps,io_extra,ex)
        print("==",d,"OK" if rc==0 else "FAIL")
        if rc!=0:print(err[:int(os.environ.get("ERRMAX","5000"))])
        else:
            ex[crate]=rmeta(crate)
            warn=[l for l in err.split("\n") if l.startswith("warning")]
            if warn:print("  warnings:",len(warn))
if __name__=="__main__":
    main(sys.argv[1:] or None)
