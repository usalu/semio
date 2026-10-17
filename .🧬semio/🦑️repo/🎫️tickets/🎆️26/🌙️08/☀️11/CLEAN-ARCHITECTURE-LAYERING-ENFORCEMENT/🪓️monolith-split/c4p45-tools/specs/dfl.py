from specs.lib import *

CH = [
    ch('📋', 'errors-and-tables', 'errors_and_tables', [(11, 88)],
       'Errors and tables of the deflate codec: the inflate error causes, the length and distance base tables, the code length order and the canonical code builders.', glob=False),
    ch('🔁', 'bit-io-and-huffman', 'bit_io_and_huffman', [(90, 260)],
       'Bit I/O and Huffman of the deflate codec: the bit reader and writer and the canonical Huffman decode table.'),
    ch('🧱', 'inflate-state', 'inflate_state', [(262, 469)],
       'Inflate state of the deflate codec: the outcome, the phase, the retained allocation and close records, the retained history and the inflater record.'),
    ch('🚰', 'inflater', 'inflater', [(471, 905)],
       'Inflater of the deflate codec: the resumable byte-granular inflate state machine and the one-shot inflate function.'),
    ch('📤', 'deflate-encoder', 'deflate_encoder', [(909, 1088)],
       'Deflate encoder of the deflate codec: the controlled LZ77 and fixed-Huffman compressor with its progress, control and one-shot entry point.'),
]

SPEC = dict(
    orig='dfl.rs',
    file=DFL + '/🦀️.rs',
    children=CH,
    external=mounted(DFL + '/🦀️.rs', DFL + '/📦️packages/🦀️rust/🦀️.rs'),
)
