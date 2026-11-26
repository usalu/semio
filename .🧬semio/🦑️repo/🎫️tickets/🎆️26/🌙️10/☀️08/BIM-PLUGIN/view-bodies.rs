use std::{env, fs, io::{self, Write}};
use bim_graph::standards::v1::subsets::any::schema::inferences::model_graph::{instance, kinds};
use bim_diff::standards::v1::subsets::any::schema::inferences::view_linework::frame::Frame;
use bim_spaces::standards::v1::subsets::any::schema::inferences::view_linework::hidden_lines::{bodies, Facing};
use pack_json::{from_json_str, JsonMemberPolicy};
fn main() {
    let source = fs::read_to_string(env::args_os().nth(1).unwrap()).unwrap();
    let model = from_json_str(&source, JsonMemberPolicy::Reject).unwrap();
    let mut out = io::BufWriter::new(io::stdout().lock());
    instance::probe::<{kinds::SOLIDS},_>(None,&model,|projection| {
        for (view_id,view) in model.views.iter() {
            let Some(frame) = view.plane.as_ref().and_then(Frame::of) else { continue };
            for body in bodies(&frame,view.depth,projection.element_solids.iter().map(|(k,v)|(k.as_str(),v))) {
                write!(out,"{view_id}\t{}\t",body.element).unwrap();
                for tri in body.tris.iter().filter(|tri|tri.facing==Facing::Front) {
                    for p in tri.drawn { write!(out,"{} {} ",p[0],p[1]).unwrap(); }
                }
                writeln!(out).unwrap();
            }
        }
    }).unwrap();
}
