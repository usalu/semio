import sys,os,re
sys.path.insert(0,"..")
import rs,gen,build
OUT=gen.OUT
doms=[("Brep","semio_s_artifact_stdio_semio_brep","SemioBrepSnapshot"),("Mesh","semio_s_artifact_stdio_semio_mesh","SemioMeshSnapshot"),("Model","semio_s_artifact_stdio_semio_model","SemioModelSnapshot"),("Value","semio_s_artifact_stdio_semio_value_model","SemioValueSnapshot"),("Document","semio_s_artifact_stdio_semio_document","SemioDocumentSnapshot"),("Cad","semio_s_artifact_stdio_semio_cad","SemioCadSnapshot"),("Drawing","semio_s_artifact_stdio_semio_drawing","SemioDrawingSnapshot"),("Image","semio_s_artifact_stdio_semio_image","SemioImageSnapshot"),("Video","semio_s_artifact_stdio_semio_video_model","SemioVideoSnapshot"),("Audio","semio_s_artifact_stdio_semio_audio_model","SemioAudioSnapshot"),("Animation","semio_s_artifact_stdio_semio_animation","SemioAnimationSnapshot"),("Presentation","semio_s_artifact_stdio_semio_presentation","SemioPresentationSnapshot"),("Flow","semio_s_artifact_stdio_semio_flow","SemioFlowSnapshot"),("Text","semio_s_artifact_stdio_semio_text_model","SemioTextSnapshot"),("Table","semio_s_artifact_stdio_semio_table_model","SemioTableSnapshot"),("Graph","semio_s_artifact_stdio_semio_graph","SemioGraphSnapshot"),("Object","semio_s_artifact_stdio_semio_object","SemioObjectSnapshot"),("Kit","semio_s_artifact_stdio_semio_kit","SemioKitSnapshot")]
s=open(rs.SUBSETS+"✉️base/"+rs.SNAP+"🦀️.rs",encoding="utf-8").read()
a=s.index("mod union_decoding {")
i=s.index("{",a);d=0
while True:
    e=rs.skip(s,i)
    if e is not None:i=e;continue
    if s[i]=="{":d+=1
    elif s[i]=="}":
        d-=1
        if d==0:break
    i+=1
mod=s[a:i+1]
variants="".join(f"{v}({c}::schema::snapshot::{t}),\n" for v,c,t in doms)
lib=f'''#![allow(unused,dead_code)]
pub mod schema{{pub mod snapshot{{
 pub const STDIO_SEMIO_DOCUMENT_SCHEMA:&str="stdio.semio";
 pub struct SemioSnapshot{{pub schema:String,pub subset:SemioSubsetSnapshot}}
 pub enum SemioSubsetSnapshot{{{variants}}}
}}}}
mod semantic{{
 use semio_framework_value::ValueError;use store::sqlite_snapshot::SqliteDatabaseLimits;
 pub fn layout(l:SqliteDatabaseLimits)->Result<(),ValueError>{{Ok(())}}
 pub fn subset_limits(l:SqliteDatabaseLimits,n:usize,tag:&str)->Result<SqliteDatabaseLimits,ValueError>{{Ok(l)}}
 pub fn tag(o:u8)->Result<&'static str,ValueError>{{Ok("brep")}}
}}
{mod}
'''
ex=build.stage_stubs()
assert build.build_base(ex)
ex["semio_s_artifact_stdio_semio_base"]=build.rmeta("semio_s_artifact_stdio_semio_base")
for d,crate,deps,io in build.DOMAINS:
    rc,err=build.build_domain(d,crate,deps,io,ex);assert rc==0,(d,err[:1000])
    ex[crate]=build.rmeta(crate)
p=build.write("semio_envelope.rs",lib)
rc,err=build.rustc("semio_s_artifact_stdio_semio",p,ex)
print("ENVELOPE","OK" if rc==0 else "FAIL");print(err[:8000])
