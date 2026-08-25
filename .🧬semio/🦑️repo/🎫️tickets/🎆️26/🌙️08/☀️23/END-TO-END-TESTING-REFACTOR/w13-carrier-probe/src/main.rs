//! 🔬 Ticket probe: is each semio subset's committed `.dsl.semio` / `.pack.semio` example a
//! BYTE-EXACT carrier — i.e. does re-printing / re-encoding the parsed snapshot reproduce the
//! committed file exactly? Prints one line per subset so the byte law each `mutate-semio-*`
//! identity handler is about to assert is chosen from evidence rather than from hope.

use std::fs;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets")
}

fn report(name: &str, printed: &str, dsl: &[u8], packed: &[u8], pack: &[u8]) {
    let text_ok = printed.as_bytes() == dsl;
    let pack_ok = packed == pack;
    let at = printed.as_bytes().iter().zip(dsl.iter()).position(|(a, b)| a != b);
    println!("{name}: dsl_exact={text_ok} (out {} in {} first-diff {:?}) pack_exact={pack_ok} (out {} in {})", printed.len(), dsl.len(), at, packed.len(), pack.len());
    if !text_ok {
        let offset = at.unwrap_or(0).saturating_sub(40);
        println!("   out …{}", printed.get(offset..(offset + 160).min(printed.len())).unwrap_or(""));
        println!("   in  …{}", String::from_utf8_lossy(&dsl[offset..(offset + 160).min(dsl.len())]));
    }
}

macro_rules! probe_dsl {
    ($name:literal, $dir:literal, $parse:path, $print:path) => {{
        let dsl = fs::read(root().join(concat!($dir, "/🖼️assets/🗣️example.dsl.semio"))).expect(concat!($name, " dsl"));
        let text = String::from_utf8(dsl.clone()).expect("utf-8");
        match $parse(&text) {
            Ok(snapshot) => report($name, &$print(&snapshot), &dsl, &[], &[]),
            Err(error) => println!("{}: PARSE FAILED {}", $name, error),
        }
    }};
}

macro_rules! probe_both {
    ($name:literal, $dir:literal, $parse:path, $print:path, $decode:path, $encode:path) => {{
        let dsl = fs::read(root().join(concat!($dir, "/🖼️assets/🗣️example.dsl.semio"))).expect(concat!($name, " dsl"));
        let pack = fs::read(root().join(concat!($dir, "/🖼️assets/🎒️example.pack.semio"))).expect(concat!($name, " pack"));
        let text = String::from_utf8(dsl.clone()).expect("utf-8");
        match $parse(&text) {
            Ok(snapshot) => {
                let agree = match $decode(&pack) {
                    Ok(other) => other == snapshot,
                    Err(_) => false,
                };
                report($name, &$print(&snapshot), &dsl, &$encode(&snapshot), &pack);
                println!("   pack_decodes_to_same_snapshot={agree}");
            }
            Err(error) => println!("{}: PARSE FAILED {}", $name, error),
        }
    }};
}

fn main() {
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets as s;
    probe_dsl!("animation", "✳️any/📚️examples/🚶️walk", s::animation::schema::snapshot::parse_semio_animation_dsl, s::animation::schema::snapshot::print_semio_animation_dsl);
    probe_dsl!("audio", "✳️any/📚️examples/🎵️tone", s::audio::schema::snapshot::parse_semio_audio_dsl, s::audio::schema::snapshot::print_semio_audio_dsl);
    probe_dsl!("video", "✳️any/📚️examples/🎥️clip", s::video::schema::snapshot::parse_semio_video_dsl, s::video::schema::snapshot::print_semio_video_dsl);
    probe_both!("cad", "✳️any/📚️examples/📐️drawing", s::cad::schema::snapshot::parse_semio_cad_dsl, s::cad::schema::snapshot::print_semio_cad_dsl, s::cad::schema::snapshot::decode_semio_cad_pack, s::cad::schema::snapshot::encode_semio_cad_pack);
    probe_both!("document", "✳️any/📚️examples/📄️memo", s::document::schema::snapshot::parse_semio_document_dsl, s::document::schema::snapshot::print_semio_document_dsl, s::document::schema::snapshot::decode_semio_document_pack, s::document::schema::snapshot::encode_semio_document_pack);
    probe_both!("drawing", "✳️any/📚️examples/🖍️sketch", s::drawing::schema::snapshot::parse_semio_drawing_dsl, s::drawing::schema::snapshot::print_semio_drawing_dsl, s::drawing::schema::snapshot::decode_semio_drawing_pack, s::drawing::schema::snapshot::encode_semio_drawing_pack);
    probe_both!("flow", "✳️any/📚️examples/🌊️pipeline", s::flow::schema::snapshot::parse_semio_flow_dsl, s::flow::schema::snapshot::print_semio_flow_dsl, s::flow::schema::snapshot::decode_semio_flow_pack, s::flow::schema::snapshot::encode_semio_flow_pack);
    probe_both!("graph", "✳️graph/📚️examples/🕸️wires", s::graph::schema::snapshot::parse_semio_graph_dsl, s::graph::schema::snapshot::print_semio_graph_dsl, s::graph::schema::snapshot::decode_semio_graph_pack, s::graph::schema::snapshot::encode_semio_graph_pack);
    probe_both!("kit", "✳️kit/📚️examples/🪑️furniture", s::kit::schema::snapshot::parse_semio_kit_dsl, s::kit::schema::snapshot::print_semio_kit_dsl, s::kit::schema::snapshot::decode_semio_kit_pack, s::kit::schema::snapshot::encode_semio_kit_pack);
    probe_both!("model", "✳️any/📚️examples/🏢️building", s::model::schema::snapshot::parse_semio_model_dsl, s::model::schema::snapshot::print_semio_model_dsl, s::model::schema::snapshot::decode_semio_model_pack, s::model::schema::snapshot::encode_semio_model_pack);
    probe_both!("object", "✳️object/📚️examples/📦️crate", s::object::schema::snapshot::parse_semio_object_dsl, s::object::schema::snapshot::print_semio_object_dsl, s::object::schema::snapshot::decode_semio_object_pack, s::object::schema::snapshot::encode_semio_object_pack);
    probe_both!("presentation", "✳️any/📚️examples/📽️deck", s::presentation::schema::snapshot::parse_semio_presentation_dsl, s::presentation::schema::snapshot::print_semio_presentation_dsl, s::presentation::schema::snapshot::decode_semio_presentation_pack, s::presentation::schema::snapshot::encode_semio_presentation_pack);
    probe_both!("table", "✳️table/📚️examples/📃️sheet", s::table::schema::snapshot::parse_semio_table_dsl, s::table::schema::snapshot::print_semio_table_dsl, s::table::schema::snapshot::decode_semio_table_pack, s::table::schema::snapshot::encode_semio_table_pack);
    probe_both!("text", "✳️text/📚️examples/📃️note", s::text::schema::snapshot::parse_semio_text_dsl, s::text::schema::snapshot::print_semio_text_dsl, s::text::schema::snapshot::decode_semio_text_pack, s::text::schema::snapshot::encode_semio_text_pack);
    probe_both!("mesh", "✳️mesh/📚️examples/🧊️cube", s::mesh::schema::snapshot::parse_mesh_dsl, s::mesh::schema::snapshot::print_mesh_dsl, s::mesh::schema::snapshot::decode_mesh_pack, s::mesh::schema::snapshot::encode_mesh_pack);
    probe_both!("image", "✳️any/📚️examples/🖼️swatch", s::image::schema::snapshot::parse_semio_image_dsl, s::image::schema::snapshot::print_semio_image_dsl, s::image::schema::snapshot::decode_semio_image_pack, s::image::schema::snapshot::encode_semio_image_pack);
    probe_both!("envelope", "✳️any/📚️examples/🌐️envelope", s::any::schema::snapshot::parse_semio_envelope_dsl, s::any::schema::snapshot::print_semio_envelope_dsl, s::any::schema::snapshot::decode_semio_envelope_pack, s::any::schema::snapshot::encode_semio_envelope_pack);
    probe_both!("brep", "✳️any/📚️examples/🧊️solid", s::brep::schema::snapshot::parse_semio_brep_dsl, s::brep::schema::snapshot::print_semio_brep_dsl, s::brep::schema::snapshot::decode_semio_brep_pack, s::brep::schema::snapshot::encode_semio_brep_pack);
}
