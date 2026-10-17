from specs.lib import *

SPEC = dict(
    orig='fmt.rs',
    file=FMT + '/🦀️.rs',
    nest=False,
    keep_empty=('🧪️Tests',),
    external=[
        FMT + '/🛫️encoding/🦀️.rs',
        FMT + '/🧪️tests/🔬️unit/🦀️.rs',
        FMT + '/🧪️tests/🔣️absolute-varint/🦀️.rs',
    ],
    children=[
        ch('🧮', 'controlled-allocation', 'controlled_allocation', [(25, 114)],
           'Controlled allocation of the pack format: the allocation layout and size checks, the controlled vector, the inflater owner, the CRC and the controlled payload and segment appenders.'),
        ch('🧱', 'header-and-footer', 'header_and_footer', [(117, 274)],
           'Header and footer of the pack format: the magic, version and flag constants, the header record with its codec and the footer record with its checksum.'),
        ch('🧩', 'segments-and-symbols', 'segments_and_symbols', [(278, 493)],
           'Segments and symbols of the pack format: the retained varints, the segment codec, the segment decoder and the symbol table codec.'),
        ch('📜', 'chunk-table-and-manifest', 'chunk_table_and_manifest', [(497, 655)],
           'Chunk table and manifest of the pack format: the chunk table entry and decoder, the manifest records and their span, encode, parse and resolve helpers.'),
        ch('✍', 'pack-writer', 'pack_writer', [(664, 953)],
           'Pack writer: the write options, the writer, the identity segment and chunk writers and the writer implementation.'),
        ch('📖', 'pack-reader-and-recovery', 'pack_reader_and_recovery', [(957, 1303), (1308, 1365)],
           'Pack reader and recovery: the superblock, the pack file with its identity chunk cursor, the footer-only reader and the forward-scan recovery.'),
    ],
)
