from specs.lib import *

CH = [
    ch('📦', 'container-codecs', 'container_codecs', [(400, 1344)],
       'Container codecs of the value codec: options, slices, vectors, deques, fixed arrays, boxes, hash and ordered sets, string-keyed maps, hash maps, the dynamic value, phantom data and the pair and triple tuples.'),
]

SPEC = dict(
    orig='cod.rs',
    file=COD + '/🦀️.rs',
    children=CH,
    external=mounted(COD + '/🦀️.rs', M + '/🌱️value/🌳️intrinsic/🦀️.rs'),
)
