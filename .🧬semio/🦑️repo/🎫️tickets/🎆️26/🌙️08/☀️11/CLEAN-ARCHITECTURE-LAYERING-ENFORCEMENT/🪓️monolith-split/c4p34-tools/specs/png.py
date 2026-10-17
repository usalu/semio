from specs.lib import *

CH = [
    ch('🧾', 'records-and-chunk-io', 'records_and_chunk_io', [(16, 87), (134, 204)],
       'Records and chunk framing of the PNG codec: the chunk, projection, address, layout and preview records, the signature, the CRC and the chunk writer and reader.'),
    ch('✅', 'header-and-structure', 'header_and_structure', [(207, 390)],
       'Header and structure checks of the PNG codec: the IHDR parser, the chunk ordering rules, the structure validation and the sample and row size helpers.'),
    ch('🎞', 'filters-and-adam7', 'filters_and_adam7', [(394, 515)],
       'Scanline filters and interlacing of the PNG codec: the Paeth predictor, the row filters and defilters, the filter choice and the Adam7 pass geometry.'),
    ch('🖼', 'unpack-and-ancillary', 'unpack_and_ancillary', [(519, 658)],
       'Sample unpacking and ancillary chunks of the PNG codec: the sample unpack and scale, the RGBA conversion, the background chunk and the text chunk writer.'),
    ch('🧭', 'layout-and-projection', 'layout_and_projection', [(663, 891)],
       'Layout and projection of the PNG codec: the empty file, the chunk addresses, the layout and preview readers, the decode and encode entry points, the native sample writer and the authored projection.'),
    ch('✍', 'native-encode', 'native_encode', [(893, 1001)],
       'Native encoding of the PNG codec: the Latin-1 helpers, the controlled chunk collector, the text chunk encoder and the controlled image encoder and projection.'),
    ch('🔓', 'native-decode', 'native_decode', [(1003, 1266)],
       'Native decoding of the PNG codec: the read error and the controlled image decoder.'),
]

SPEC = dict(
    orig='png.rs.txt',
    file=PNG + '/🦀️.rs',
    nest=False,
    keep_empty=('🧪️CodecTests',),
    external=sub(PNG, skip=folders(CH)),
    outside=outside('png'),
    children=CH,
)
