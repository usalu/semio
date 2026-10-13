#!/usr/bin/env python3
"""🪆️ W1 step 3 (split plan 1b + sqlite half of 1d): the child-reference codecs and the generic sqlite native leaves move into the
foundation; the union-only sqlite decode/encode stays with the union file."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *
from use_rewrite import rewrite_uses, rewrite_linear

S = lambda rel: SUBSETS + "/" + rel
SQ = "✉️base/🚪️io/🪶️sqlite/📸️snapshot/"
TC = ("crate", "base_foundation", "io", "text", "codec")
BC = ("crate", "base_foundation", "io", "binary", "codec")
NATIVE = ("crate", "base_foundation", "io", "sqlite", "snapshot", "native")
SUB = lambda s, *rest: ("crate", "standards", "v1", "subsets", s) + rest
remap = {}
pub = lambda t: re.sub(r"(?m)^(\s*)pub\(crate\) ?fn ", r"\1pub fn ", t)


def take(rel, names, dest, insert_use=True):
    p = S(rel)
    src, texts, first = extract_functions(read(p), names)
    keep = used_names(src, names)
    if keep and insert_use:
        src = src[:first] + "use %s::{%s};\n" % ("::".join(dest), ", ".join(keep)) + src[first:]
    write(p, src)
    return texts


TEXT_CHILD = ["enc_ref", "dec_ref", "enc_child", "dec_child", "enc_child_opt", "dec_child_opt"]
BIN_CHILD = ["write_ref", "read_ref", "write_child", "read_child", "write_child_opt", "read_child_opt"]
text_moved = take("📦️object/🚪️io/📝️text/📸️snapshot/🦀️.rs", TEXT_CHILD, TC)
take("🧰️kit/🚪️io/📝️text/📸️snapshot/🦀️.rs", TEXT_CHILD, TC)
bin_moved = take("📦️object/🚪️io/💾️binary/📸️snapshot/🦀️.rs", BIN_CHILD, BC)
take("🧰️kit/🚪️io/💾️binary/📸️snapshot/🦀️.rs", BIN_CHILD, BC)
sql_moved = take("📦️object/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs", ["native_child", "native_reference"], NATIVE)
for s in ("object", "kit"):
    for n in TEXT_CHILD:
        remap[(SUB(s, "io", "text", "snapshot"), n)] = TC + (n,)
    for n in BIN_CHILD:
        remap[(SUB(s, "io", "binary", "snapshot"), n)] = BC + (n,)
    for n in ("native_child", "native_reference"):
        remap[(SUB(s, "io", "sqlite", "snapshot"), n)] = NATIVE + (n,)

tc = S("✉️base/🚪️io/📝️text/🔤️codec/🦀️.rs")
src = read(tc)
cut = src.index("/// ➖️ The `Display` spelling")
child_text = pub("".join(text_moved)).replace("crate::standards::v1::subsets::base::io::text::snapshot::", "")
src = src[:cut] + child_text.rstrip("\n") + "\n\n" + src[cut:]
write(tc, src)
bc = S("✉️base/🚪️io/💾️binary/🔤️codec/🦀️.rs")
write(bc, read(bc).rstrip("\n") + "\n\n" + pub("".join(bin_moved)).rstrip("\n") + "\n")
nat = S(SQ + "📏️native/🦀️.rs")
write(nat, read(nat).rstrip("\n") + "\n\n" + "".join(sql_moved).rstrip("\n") + "\n")

dec_p = S(SQ + "🛬️native/🦀️.rs")
dec_src, dec_fns, _ = extract_functions(read(dec_p), ["decode_snapshot", "binary_snapshot", "text_snapshot"])
write(dec_p, dec_src)
enc_p = S(SQ + "🛫️native/🦀️.rs")
enc_src, enc_fns, _ = extract_functions(read(enc_p), ["encode_snapshot", "subset"])
write(enc_p, enc_src)

union = S(SQ + "🦀️.rs")
u = read(union)
for mount in ('#[path = "📏️native/🦀️.rs"]\npub mod native;\n', '#[path="🫙️projection/🦀️.rs"]\npub(crate)mod projection;\n', '#[path = "🛫️native/🦀️.rs"]\npub(crate) mod native_encoding;\n', '#[path = "🛬️native/🦀️.rs"]\npub(crate) mod native_decoding;\n'):
    assert u.count(mount) == 1, mount
    u = u.replace(mount, "")
u = u.replace("native_decoding::decode_snapshot(", "union_decoding::decode_snapshot(").replace("native_encoding::encode_snapshot(", "union_encoding::encode_snapshot(")
anchor = "use semio_framework_value::{ValueError,ValueRefusalKind};\n"
assert u.count(anchor) == 1
u = u.replace(anchor, anchor + "use crate::base_foundation::io::sqlite::snapshot::{native, native_decoding};\n", 1)
u = u.rstrip("\n") + "\n\n" + (
    "/// 🛬️ Decodes the union envelope's native payload by dispatching on its subset tag to that subset's own decoder.\n"
    "mod union_decoding {\n"
    "use crate::base_foundation::io::sqlite::snapshot::native_decoding::*;\n"
    "use semio_framework_value::{ValueError,ValueRefusalKind};\n"
    "use semio_framework_value::native_decoding::NativeDecodeControl;\n"
    "use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};\n"
    + "".join(dec_fns).rstrip("\n") + "\n}\n\n"
    "/// 🛫️ Encodes the union envelope's native payload by dispatching to the selected subset's own field encoder.\n"
    "mod union_encoding {\n"
    "use crate::base_foundation::io::sqlite::snapshot::native_encoding::*;\n"
    "use semio_framework_value::ValueError;\n"
    "use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase};\n"
    + "".join(enc_fns).rstrip("\n") + "\n}\n")
write(union, u)

found = S("✉️base/🦀️.rs")
f = read(found).rstrip("\n")
assert f.endswith("}\n}") or f.endswith("}")
f = f[: f.rindex("}")].rstrip("\n") + '''
    #[path = "."]
    pub mod sqlite {
        #[path = "."]
        pub mod snapshot {
            #[path = "🚪️io/🪶️sqlite/📸️snapshot/📏️native/🦀️.rs"]
            pub mod native;
            #[path = "🚪️io/🪶️sqlite/📸️snapshot/🛬️native/🦀️.rs"]
            pub mod native_decoding;
            #[path = "🚪️io/🪶️sqlite/📸️snapshot/🛫️native/🦀️.rs"]
            pub mod native_encoding;
            #[path = "🚪️io/🪶️sqlite/📸️snapshot/🫙️projection/🦀️.rs"]
            pub mod projection;
        }
    }
}
'''
write(found, f)

MOVED_SQ = ("native", "native_decoding", "native_encoding", "projection")


def mapper(seg):
    for head in (("crate",), ("semio_s_artifact_stdio_semio",)):
        if tuple(seg[:1]) != head:
            continue
        if list(seg[1:8]) == ["standards", "v1", "subsets", "base", "io", "sqlite", "snapshot"] and len(seg) > 8 and seg[8] in MOVED_SQ:
            return list(head) + ["base_foundation", "io", "sqlite", "snapshot"] + list(seg[8:])
        hit = remap.get((("crate",) + tuple(seg[1:-1]), seg[-1]))
        if hit:
            return list(head) + list(hit[1:])
    return None


linear = [(r"\b(crate|semio_s_artifact_stdio_semio)::standards::v1::subsets::base::io::sqlite::snapshot::(native_decoding|native_encoding|native|projection)\b", r"\1::base_foundation::io::sqlite::snapshot::\2")]
changed = 0
for p in list(semio_rust_files()) + consumer_rust_files():
    s = read(p)
    s2, a = rewrite_linear(s, linear)
    s3, b = rewrite_uses(s2, mapper)
    if s3 != s:
        write(p, s3)
        changed += 1
squeeze_blank_runs([S(r) for r in ["📦️object/🚪️io/📝️text/📸️snapshot/🦀️.rs", "🧰️kit/🚪️io/📝️text/📸️snapshot/🦀️.rs", "📦️object/🚪️io/💾️binary/📸️snapshot/🦀️.rs", "🧰️kit/🚪️io/💾️binary/📸️snapshot/🦀️.rs", "📦️object/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs", SQ + "🛬️native/🦀️.rs", SQ + "🛫️native/🦀️.rs"]])
print("rewrote", changed, "files")
