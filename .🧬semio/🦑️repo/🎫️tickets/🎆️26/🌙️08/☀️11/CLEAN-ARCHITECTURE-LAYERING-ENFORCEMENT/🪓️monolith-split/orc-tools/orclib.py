"""Shared helpers of the orc (OS-PACK-RECORD-CODEC-DEDUPE) tools: paths, item outline, normalised item comparison."""
import os, re, sys
ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G = F + "/🗑️generated"
OS = os.environ.get("ORC_OS_ROOT", ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack")
FW = ROOT + "/🧰️framework/🔨️modules/🎒️pack"
sys.path.insert(0, F + "/frs-tools")
import rslex


def read(p):
    with open(p, encoding="utf-8") as f:
        return f.read()


def items(path):
    text = read(path)
    lines = text.split("\n")
    out = []
    for it in rslex.outline(text):
        body = "\n".join(lines[it["start"]:it["end"] + 1])
        out.append(dict(it, body=body, file=path))
    return out


PATHMAP = [
    (r"crate::format::", "semio_framework_pack_format::"),
    (r"protocol::value::", "semio_framework_value::"),
    (r"protocol::codec::", "semio_framework_replication_codec::"),
    (r"pack::format::", "semio_framework_pack_format::"),
    (r"crate::KIND_", "semio_framework_replication_codec::KIND_"),
    (r"crate::ByteRange", "semio_framework_replication_codec::ByteRange"),
]


def norm(body, paths=True):
    s = re.sub(r"//[^\n]*", "", body)
    s = re.sub(r"pub\((?:super|crate)\)\s*", "", s)
    s = re.sub(r"^\s*pub\s+", "", s, flags=re.M)
    if paths:
        for a, b in PATHMAP:
            s = re.sub(a, b, s)
    s = re.sub(r"\s+", "", s)
    return s
