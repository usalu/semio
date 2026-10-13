#!/usr/bin/env python3
"""🔤️ W1 step 2 (split plan 1a): the text and binary codec primitives move into the foundation's `🔤️codec` leaves."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *
from use_rewrite import rewrite_uses, rewrite_linear, _mask

S = lambda rel: SUBSETS + "/" + rel
TEXT_CODEC = S("✉️base/🚪️io/📝️text/🔤️codec/🦀️.rs")
BIN_CODEC = S("✉️base/🚪️io/💾️binary/🔤️codec/🦀️.rs")
TC = ("crate", "base_foundation", "io", "text", "codec")
BC = ("crate", "base_foundation", "io", "binary", "codec")
SUB = lambda s, *rest: ("crate", "standards", "v1", "subsets", s) + rest

DRAW_TEXT = ["hex_encode", "hex_decode", "enc_str", "dec_str", "parse_f64", "parse_f32", "enc_bool", "parse_bool", "enc_list", "dec_list", "encode_option", "decode_option", "enc_point2", "dec_point2", "enc_point3", "dec_point3", "enc_quaternion", "dec_quaternion", "enc_transform", "dec_transform", "enc_rgba", "dec_rgba"]
BASE_TEXT = ["split_top_level", "strip_brackets", "enc_opt", "dec_opt", "enc_indexed_triple", "dec_indexed_triple", "enc_named_triple", "dec_named_triple", "enc_named_added", "dec_named_added"]
DRAW_BIN = ["write_bytes_lp", "read_bytes_lp", "write_str_lp", "read_str_lp", "write_point2", "read_point2", "write_point3", "read_point3", "write_quaternion", "read_quaternion", "write_transform", "read_transform", "read_f32_le", "write_rgba", "read_rgba", "write_bool", "read_bool"]
PLAIN = [("📐️cad/🚪️io/📝️text/🔺️diff/🦀️.rs", "cad", ("io", "text", "diff"), ["parse_f64", "enc_point2", "dec_point2"]),
         ("🔺️mesh/🚪️io/📝️text/🔺️diff/🦀️.rs", "mesh", ("io", "text", "diff"), ["parse_f32", "enc_rgba", "dec_rgba"]),
         ("🏛️model/🚪️io/📝️text/🔺️diff/🦀️.rs", "model", ("io", "text", "diff"), ["enc_point3", "dec_point3", "enc_quat", "dec_quat", "enc_transform", "dec_transform"])]
ROOT_MOVES = [("🖊️drawing/🚪️io/📝️text/📸️snapshot/🦀️.rs", "drawing", ("io", "text", "snapshot"), DRAW_TEXT),
              ("🌊️flow/🚪️io/📝️text/📸️snapshot/🦀️.rs", "flow", ("io", "text", "snapshot"), ["enc_f64", "dec_f64"]),
              ("🎬️video/🚪️io/📝️text/📸️snapshot/🦀️.rs", "video", ("io", "text", "snapshot"), ["dec_bool"]),
              ("🔺️mesh/🚪️io/📝️text/📸️snapshot/🦀️.rs", "mesh", ("io", "text", "snapshot"), ["parse_u32"]),
              ("📑️document/🚪️io/📝️text/🧬️mutations/🦀️.rs", "document", ("io", "text", "mutations"), ["parse_usize"])]

remap = {}
sections = {"root": [], "plain": [], "bin": []}


RECOVER = "--recover" in sys.argv
GEO = [(r"\b(crate)::standards::v1::subsets::base::schema::(geometry|triples|child)\b", r"\1::base_foundation::schema::\2")]


def source_of(p):
    return rewrite_linear(head_text(p), GEO)[0] if RECOVER else read(p)


def move(rel, subset_mod, names, dest, section, wrapper=None):
    p = S(rel)
    src = source_of(p)
    src, texts, first = extract_functions(src, names)
    sections[section].extend(texts)
    for n in names:
        remap[(subset_mod, n)] = dest + (n,)
    if RECOVER:
        return
    keep = used_names(src, names)
    if keep:
        line = "use %s::{%s};\n" % ("::".join(dest), ", ".join(keep))
        src = src[:first] + line + src[first:]
    write(p, src)


for rel, s, mod, names in ROOT_MOVES:
    move(rel, SUB(s, *mod), names, TC, "root")
for rel, s, mod, names in PLAIN:
    move(rel, SUB(s, *mod), names, TC + ("plain",), "plain")
move("🖊️drawing/🚪️io/💾️binary/📸️snapshot/🦀️.rs", SUB("drawing", "io", "binary", "snapshot"), DRAW_BIN, BC, "bin")

base_text = S("✉️base/🚪️io/📝️text/📸️snapshot/🦀️.rs")
src = source_of(base_text)
a = src.index("#[allow(unused_imports)]\nmod snapshot_wire2_codec {")
b = src.index("pub use snapshot_wire2_codec::*;\n") + len("pub use snapshot_wire2_codec::*;\n")
block = src[a:b]
src = src[:a] + src[b:]
if not RECOVER:
    write(base_text, src)
body = block.split("use crate::base_foundation::schema::triples::*;\n", 1)[1].rsplit("}\npub use snapshot_wire2_codec::*;", 1)[0].strip("\n")
sections["root"].insert(0, body + "\n\n")
for n in BASE_TEXT:
    remap[(SUB("base", "io", "text", "snapshot"), n)] = TC + (n,)

audio = S("🔊️audio/🚪️io/📝️text/🔺️diff/🦀️.rs")
if not RECOVER:
    src, _, _ = extract_functions(read(audio), ["split_top_level", "strip_brackets"])
    write(audio, src)
for n in ("split_top_level", "strip_brackets"):
    remap[(SUB("audio", "io", "text", "diff"), n)] = TC + (n,)
for n in ("dec_str", "enc_str", "hex_decode"):
    remap[(SUB("value", "io", "text", "diff"), n)] = TC + (n,)
for n in ("enc_list", "dec_list"):
    remap[(SUB("model", "io", "text", "diff"), n)] = TC + (n,)
for n in ("enc_channel", "dec_channel"):
    remap[(SUB("animation", "io", "text", "snapshot"), n)] = "DROP"


def pub(text):
    return re.sub(r"(?m)^(\s*)pub\(crate\) fn ", r"\1pub fn ", text)


write(TEXT_CODEC, "//! 🔤️ Text codec primitives of the semio family: bracketed lists and options, hex strings, the indexed and named\n//! diff triples, and the geometry records in their native (exact round-trip) spelling; `plain` holds the Rust\n//! `Display` spelling the subset diff codecs use.\n\n"
      "use crate::base_foundation::schema::triples::*;\n"
      "use crate::base_foundation::schema::geometry::native::{NativeF32, NativeF64};\n"
      "use crate::base_foundation::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};\n\n"
      + pub("".join(sections["root"])).rstrip("\n") + "\n\n"
      "/// ➖️ The `Display` spelling of the geometry records and scalars, as the subset diff codecs print and parse them.\npub mod plain {\n"
      "use super::{split_top_level, strip_brackets};\n"
      "use crate::base_foundation::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};\n\n"
      "// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9\npub fn parse_f64(s: &str) -> Result<f64, String> {\n    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())\n}\n\n"
      + pub("".join(t for t in sections["plain"] if not re.search(r"fn parse_f64\b", t))).rstrip("\n") + "\n}\n")
write(BIN_CODEC, "//! 💾️ Binary codec primitives of the semio family: varint length-prefixed strings and bytes, and the geometry records\n//! as little-endian floats.\n\n"
      "use crate::base_foundation::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};\n\n"
      + pub("".join(sections["bin"])).rstrip("\n") + "\n")

found = read(SUBSETS + "/✉️base/🦀️.rs")
found += '''
#[path = "."]
pub mod io {
    #[path = "."]
    pub mod text {
        #[path = "🚪️io/📝️text/🔤️codec/🦀️.rs"]
        pub mod codec;
    }
    #[path = "."]
    pub mod binary {
        #[path = "🚪️io/💾️binary/🔤️codec/🦀️.rs"]
        pub mod codec;
    }
}
'''
write(SUBSETS + "/✉️base/🦀️.rs", found)


def mapper(seg):
    for head in (("crate",), ("semio_s_artifact_stdio_semio",)):
        if tuple(seg[:1]) != head:
            continue
        key = ("crate",) + tuple(seg[1:-1])
        hit = remap.get((key, seg[-1]))
        if hit == "DROP":
            return ["__DROP__"]
        if hit:
            return list(head) + list(hit[1:])
    return None


changed = 0
for p in list(semio_rust_files()) + consumer_rust_files():
    s = read(p)
    s2, n = rewrite_uses(s, mapper)
    if n:
        s2 = re.sub(r"(?m)^[ \t]*use __DROP__;\n", "", s2)
        s2 = re.sub(r"__DROP__, |, __DROP__", "", s2)
        write(p, s2)
        changed += 1
print("rewrote", changed, "files;", len(remap), "mapped items")
