#!/usr/bin/env python3
"""➡️ W1 step 6c: paths to the moved conversion APIs (encode/decode, brep dwg, the renamed video/audio/animation leaves)."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *
from use_rewrite import rewrite_uses, rewrite_linear

LEAF_MODS = {}
for s in ("video", "audio", "animation"):
    root = read(SUBSETS + "/" + EMOJI[s] + "/🚪️io/📥️import/🦀️.rs") + read(SUBSETS + "/" + EMOJI[s] + "/🚪️io/📤️export/🦀️.rs")
    for m in re.finditer(r'pub mod (\w+) \{\n\s*#\[path = "\."\]\n\s*pub mod (\w+) \{\n\s*#\[path = "\."\]\n\s*pub mod any \{\n\s*#\[path = "(🧩️deserializers|🧵️serializers)/', root):
        fmt, ver, kind = m.group(1), m.group(2), m.group(3)
        key = "import" if kind == "🧩️deserializers" else "export"
        old = "%s_%s" % (fmt, "deserializer" if key == "import" else "serializer")
        LEAF_MODS[(s, old)] = ["io", key, "deserializers" if key == "import" else "serializers", "artifacts", fmt, ver, "any"]


def mapper(seg):
    head = seg[:1]
    if head not in (["crate"], ["semio_s_artifact_stdio_semio"]) or seg[1:4] != ["standards", "v1", "subsets"] or len(seg) < 7:
        return None
    s, rest = seg[4], seg[5:]
    if rest[:1] != ["io"]:
        return None
    if s in ("mesh", "drawing") and rest[1:2] in (["encode_" + s], ["decode_" + s]):
        return seg[:6] + ["export" if rest[1].startswith("encode") else "import"] + rest[1:]
    if s == "brep" and rest[1:2] == ["dwg"]:
        if rest[2:3] == ["import"]:
            return seg[:6] + ["import", "dwg", "import"] + rest[3:]
        if rest[2:3] == ["export"]:
            return seg[:6] + ["export", "dwg", "export"] + rest[3:]
    if (s, rest[1]) in LEAF_MODS:
        return seg[:5] + LEAF_MODS[(s, rest[1])] + rest[2:]
    return None


linear = []
for s in ("mesh", "drawing"):
    linear += [(r"\b(crate|semio_s_artifact_stdio_semio)::standards::v1::subsets::%s::io::encode_%s\b" % (s, s), r"\1::standards::v1::subsets::%s::io::export::encode_%s" % (s, s)),
               (r"\b(crate|semio_s_artifact_stdio_semio)::standards::v1::subsets::%s::io::decode_%s\b" % (s, s), r"\1::standards::v1::subsets::%s::io::import::decode_%s" % (s, s))]
linear += [(r"\b(crate|semio_s_artifact_stdio_semio)::standards::v1::subsets::brep::io::dwg::import\b", r"\1::standards::v1::subsets::brep::io::import::dwg::import"),
           (r"\b(crate|semio_s_artifact_stdio_semio)::standards::v1::subsets::brep::io::dwg::export\b", r"\1::standards::v1::subsets::brep::io::export::dwg::export")]
for (s, old), new in LEAF_MODS.items():
    linear.append((r"\b(crate|semio_s_artifact_stdio_semio)::standards::v1::subsets::%s::io::%s\b" % (s, old), r"\1::standards::v1::subsets::%s::%s" % (s, "::".join(new))))
changed = {}
for p in list(semio_rust_files()) + consumer_rust_files():
    s0 = read(p)
    s1, a = rewrite_linear(s0, linear)
    s2, b = rewrite_uses(s1, mapper)
    if s2 != s0:
        write(p, s2)
        changed[p] = a + b
for p, n in changed.items():
    print(n, os.path.relpath(p, REPO)[-110:])
print(len(LEAF_MODS), "leaf modules")
