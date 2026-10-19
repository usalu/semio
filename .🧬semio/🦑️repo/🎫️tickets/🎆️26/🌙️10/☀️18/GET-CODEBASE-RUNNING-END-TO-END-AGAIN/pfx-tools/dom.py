import re,subprocess,sys
import rs
HEADER_IMPORT="use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::{self as native,workspace};"
def decode_text(old):
    ty=re.search(r"->Result<(\w+),ValueError>",old).group(1)
    schema=re.search(r"STDIO_\w+_DOCUMENT_SCHEMA",old).group(0)
    vis=re.search(r"(pub(?:\(crate\))?)\s*fn\s+decode",old).group(1)
    return ("/// 🛬️ Separates complete SQL cells from the same caller's actual native allocation backing.\n"
    +vis+" fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut NativeDecodeControl<'_>)->Result<%s,ValueError>{\n"
    " crate::io::sqlite::snapshot::admit_layout(control.limits())?;\n"
    " native::with_body(payload,%s,control,native_control,|body,decoder,limits|workspace::decode(decoder,empty,|output,decoder|match body{native::Body::Binary(body)=>binary_into(output,body,decoder,limits),native::Body::Text(body)=>document_into(output,body,decoder,limits)}))\n"
    "}")%(ty,schema),ty
def apply(domain,binary,document,remove=(),keep_docs=True,extra_imports=()):
    p=rs.native(domain)
    s=open(p,encoding="utf-8").read()
    old=rs.get(p,"decode")
    new,ty=decode_text(old)
    rs.replace(p,"decode",new)
    rs.replace(p,"binary",binary) if re.search(r"(?m)^[ \t]*(pub(\([a-z]+\))?[ \t]*)?fn[ \t]+binary\b",open(p,encoding="utf-8").read()) else None
    rs.replace(p,"document",document)
    for r in remove:rs.replace(p,r,"")
    s=open(p,encoding="utf-8").read()
    s=re.sub(r"pub\(crate\) ?fn empty\(",r"pub fn empty(",s)
    s=re.sub(r"use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding as native;",HEADER_IMPORT,s)
    open(p,"w",encoding="utf-8").write(s)
    return ty
def fmt(domain):
    p=rs.native(domain)
    r=subprocess.run(["rustfmt","--edition","2021","--emit","stdout","--config","skip_children=true",p],capture_output=True,text=True)
    return r.returncode,r.stderr[:2000]
