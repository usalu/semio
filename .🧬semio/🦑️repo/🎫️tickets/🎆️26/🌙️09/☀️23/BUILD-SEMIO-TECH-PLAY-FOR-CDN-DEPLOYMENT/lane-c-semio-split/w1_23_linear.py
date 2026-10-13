#!/usr/bin/env python3
"""➡️ W1 steps 2-3 follow-up: qualified (non-`use`) paths to the moved codec items point at their foundation home."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *
from use_rewrite import rewrite_linear

TC = "base_foundation::io::text::codec"
BC = "base_foundation::io::binary::codec"
NAT = "base_foundation::io::sqlite::snapshot::native"
moves = []
def add(mod, names, dest):
    for n in names:
        moves.append((mod, n, dest))
add("drawing::io::text::snapshot", ["hex_encode", "hex_decode", "enc_str", "dec_str", "parse_f64", "parse_f32", "enc_bool", "parse_bool", "enc_list", "dec_list", "encode_option", "decode_option", "enc_point2", "dec_point2", "enc_point3", "dec_point3", "enc_quaternion", "dec_quaternion", "enc_transform", "dec_transform", "enc_rgba", "dec_rgba"], TC)
add("flow::io::text::snapshot", ["enc_f64", "dec_f64"], TC)
add("video::io::text::snapshot", ["dec_bool"], TC)
add("mesh::io::text::snapshot", ["parse_u32"], TC)
add("document::io::text::mutations", ["parse_usize"], TC)
add("cad::io::text::diff", ["parse_f64", "enc_point2", "dec_point2"], TC + "::plain")
add("mesh::io::text::diff", ["parse_f32", "enc_rgba", "dec_rgba"], TC + "::plain")
add("model::io::text::diff", ["enc_point3", "dec_point3", "enc_quat", "dec_quat", "enc_transform", "dec_transform"], TC + "::plain")
add("model::io::text::diff", ["enc_list", "dec_list"], TC)
add("drawing::io::binary::snapshot", ["write_bytes_lp", "read_bytes_lp", "write_str_lp", "read_str_lp", "write_point2", "read_point2", "write_point3", "read_point3", "write_quaternion", "read_quaternion", "write_transform", "read_transform", "read_f32_le", "write_rgba", "read_rgba", "write_bool", "read_bool"], BC)
add("base::io::text::snapshot", ["split_top_level", "strip_brackets", "enc_opt", "dec_opt", "enc_indexed_triple", "dec_indexed_triple", "enc_named_triple", "dec_named_triple", "enc_named_added", "dec_named_added"], TC)
add("audio::io::text::diff", ["split_top_level", "strip_brackets"], TC)
add("value::io::text::diff", ["dec_str", "enc_str", "hex_decode"], TC)
for s in ("object", "kit"):
    add(s + "::io::text::snapshot", ["enc_ref", "dec_ref", "enc_child", "dec_child", "enc_child_opt", "dec_child_opt"], TC)
    add(s + "::io::binary::snapshot", ["write_ref", "read_ref", "write_child", "read_child", "write_child_opt", "read_child_opt"], BC)
    add(s + "::io::sqlite::snapshot", ["native_child", "native_reference"], NAT)
rules = [(r"\b(crate|semio_s_artifact_stdio_semio)::standards::v1::subsets::" + re.escape(mod) + "::" + n + r"\b(?!::)", r"\1::" + dest + "::" + n) for mod, n, dest in moves]
changed = 0
for p in list(semio_rust_files()) + consumer_rust_files():
    s = read(p)
    s2, c = rewrite_linear(s, rules)
    if c:
        write(p, s2)
        changed += c
print("linear rewrites:", changed)
