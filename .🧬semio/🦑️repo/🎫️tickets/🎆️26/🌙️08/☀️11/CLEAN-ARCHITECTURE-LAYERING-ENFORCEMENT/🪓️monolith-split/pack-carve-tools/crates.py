"""📇 Registry of the carved pack crates (node = taxonomy folder holding the lib root, package = `<node>/📦️packages/🦀️rust`)."""
from lib import *

PACKAGE = "📦️packages/🦀️rust"
CRATES = [
    # key, cargo name, node (relative to the pack module), description
    ("format", "semio-framework-pack-format", "📐️format", "Pack container framing: header, footer, segments, symbols, chunk table, manifest, writer, reader and recovery."),
    ("retained", "semio-framework-pack-format-retained", "📐️format/🎮️retained", "Retained canonical Pack source admission cursors: pages, segments, varints and anchors, bounded and cancellable byte by byte."),
    ("catalog", "semio-framework-pack-format-catalog", "📐️format/🗂️catalog", "Retained canonical Pack catalog cursors: manifest, symbol table, chunk table and inline symbols, bounded and cancellable byte by byte."),
    ("schema", "semio-framework-pack-record-schema", "🌱️value/🧬️schema", "Record wire vocabulary: tags, element tags, shape navigation and the canonical schema graph hash."),
    ("rretained", "semio-framework-pack-record-retained", "🌱️value/🛬️decode", "Retained record decoding: value and record-body cursors, source decode, receiving and stack storage."),
    ("encode", "semio-framework-pack-record-encode", "🌱️value/🛫️encode", "Controlled record encoding: borrowed projection, symbol discovery, operation pages and owned wire cursors."),
    ("preflight", "semio-framework-pack-record-preflight", "🌱️value/🫳️preflight", "Borrowed record preflight: exact document size forecast from an immutable projection."),
    ("record", "semio-framework-pack-record", "🌱️value", "Record codec: canonical document and record body encode and decode, scalar witness and intrinsic adapters."),
    ("intrinsic", "semio-framework-pack-intrinsic", "🌱️intrinsic", "Intrinsic Body dispatch and retained document clone."),
    ("async", "semio-framework-pack-async", "⏳️async", "Runtime-neutral async pack access: AsyncPackSource, cancellation and the read scheduler."),
    ("http", "semio-framework-pack-http", "🌐️http", "HTTP range-request access layer for pack files."),
    ("io", "semio-framework-pack-io", "🔌️io", "Native file I/O for the pack container family."),
]
BY_KEY = {c[0]: c for c in CRATES}

def ident(key): return BY_KEY[key][1].replace("-", "_")
def cname(key): return BY_KEY[key][1]
def node(key): return P + BY_KEY[key][2]
def pkgdir(key): return node(key) + "/" + PACKAGE
def libroot(key): return node(key) + "/🦀️.rs"
def short_id(key): return BY_KEY[key][1][len("semio-framework-"):]
