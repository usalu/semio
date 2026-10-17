from specs.lib import *

CH = [
    ch('🧮', 'transforms-and-tables', 'transforms_and_tables', [(65, 174), (476, 570)],
       'Errors, transforms and constant tables of the JPEG codec: the typed error, the zigzag order, the 8x8 DCT and inverse DCT, the quantization and standard Huffman tables and the colour conversion.', glob=False),
    ch('🔡', 'huffman-and-bit-io', 'huffman_and_bit_io', [(178, 472)],
       'Entropy coding of the JPEG codec: the Huffman table builder, the bit writer and reader, the sign extension and the block encoder and decoder.'),
    ch('✍', 'encoder', 'encoder', [(574, 825)],
       'Encoder of the JPEG codec: the box downsampler, the JFIF APP0 writer, the frame components and the baseline encoder with its Huffman table writer.'),
    ch('🔎', 'header-and-decode-entry', 'header_and_decode_entry', [(829, 1155)],
       'Header parsing and decode entry points of the JPEG codec: the JFIF APP0 parser, the byte source, the decode entry points, the header record and the marker-segment header parser.'),
    ch('🪜', 'step-decoder', 'step_decoder', [(1156, 1335)],
       'Stepwise decoder of the JPEG codec: the resumable decoder state and its scan, restart and output steps.'),
]

SPEC = dict(
    orig='jpg.rs.txt',
    file=JPG + '/🦀️.rs',
    nest=False,
    external=sub(JPG, skip=folders(CH)),
    outside=outside('jpg'),
    children=CH,
)
