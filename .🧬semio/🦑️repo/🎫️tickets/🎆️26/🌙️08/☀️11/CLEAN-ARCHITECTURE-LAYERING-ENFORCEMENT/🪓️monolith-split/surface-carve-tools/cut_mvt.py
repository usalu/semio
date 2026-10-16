import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import tf
TM = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map"
src = TM + "/🧪️tests/🔬️unit/🦀️.rs"
t = open(src, encoding="utf-8").read()
names = ["owned_vector_tile_inventory_agrees_with_independent_protobuf_corpus", "decode_mvt_empty_bytes_yields_empty_tile", "decode_demotile_fixture_has_named_layers", "demotile_fixture_countries_use_multi_ring_polygons", "fixture_linestrings_split_at_moveto", "demotile_z5_has_countries_and_centroids", "demotile_z0_has_many_country_features", "vector_label_anchor_uses_line_midpoint_for_transportation_names"]
blocks = tf.take(t, names)
assert len(blocks) == len(names), len(blocks)
body = "\n\n".join(blocks)
body = body.replace("super::vector_tiles::", "super::")
body = body.replace("    use super::vector_tiles::GeomType;\n", "")
body = body.replace('''    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🗺️vector-tiles/🔣️.json")).expect("vector tile corpus");''', '''    let corpus: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(fixture_root().join("🔣️.json")).expect("vector tile corpus file")).expect("vector tile corpus");''')
body = body.replace("assert_eq!(rows.len(), VECTOR_TILE_FIXTURES.len());", "assert_eq!(rows.len(), VECTOR_TILE_KEYS.len());")
body = body.replace("assert_eq!(bytes.len() as u64, row[\"bytes\"]", "assert_eq!(bytes.len() as u64, row[\"bytes\"]")
body = body.replace("let tile = decode_mvt(bytes).expect(\"owned vector tile\");", "let tile = decode_mvt(&bytes).expect(\"owned vector tile\");")
body = body.replace("    for (_, bytes) in VECTOR_TILE_FIXTURES {\n        let tile = decode_mvt(&bytes).expect(\"decode\");", "    for key in VECTOR_TILE_KEYS {\n        let tile = decode_mvt(&vector_tile_fixture(key)).expect(\"decode\");")
body = body.replace("    use super::{feature_label_anchor, GeomType, VectorFeature};\n", "")
header = '''//! 🧪️ Unit proofs of the MVT decoder against the six owned protobuf tiles, read at run time from the tiled-map fixture folder so no tile bytes are compiled into the crate.

use super::{decode_mvt, feature_label, feature_label_anchor, GeomType, VectorFeature};
use std::path::PathBuf;

const VECTOR_TILE_KEYS: [&str; 6] = ["0-0-0", "1-1-0", "2-2-1", "3-4-2", "4-8-5", "5-17-11"];

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../🧪️tests/🧫️fixtures/🗺️vector-tiles")
}

fn vector_tile_fixture(key: &str) -> Vec<u8> {
    std::fs::read(fixture_root().join(key).join("🛰️.pbf")).expect("owned vector tile fixture")
}

'''
out_dir = TM + "/🧱️vector-tiles/📦️mvt-decoder/🧪️tests/🔬️unit"
os.makedirs(out_dir, exist_ok=True)
open(out_dir + "/🦀️.rs", "w", encoding="utf-8").write(header + body.replace("feature_label(&f.properties", "feature_label(&f.properties") + "\n")
# drop from host tests
t2 = tf.drop(t, names + ["VECTOR_TILE_FIXTURES", "vector_tile_fixture"])
open(src, "w", encoding="utf-8").write(t2)
print("moved", len(blocks), "len", len(t.split("\n")), "->", len(t2.split("\n")))
