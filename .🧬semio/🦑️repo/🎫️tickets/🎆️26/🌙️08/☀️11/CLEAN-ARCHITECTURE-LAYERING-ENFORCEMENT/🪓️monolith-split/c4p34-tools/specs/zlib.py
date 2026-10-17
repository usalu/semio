from specs.lib import *

CH = [
    ch('🔢', 'adler-bit-io-and-huffman', 'adler_bit_io_and_huffman', [(17, 193)],
       'Checksum, bit IO and Huffman of the zlib codec: the Adler-32 checksum, the bit writer and reader, the canonical code builder, the Huffman decoder and the fixed code lengths.', glob=False),
    ch('🔎', 'lz77-matching', 'lz77_matching', [(200, 294)],
       'LZ77 matching of the zlib codec: the length and distance tables, the hash chain constants, the hash, the longest match search and the length and distance symbols.', glob=False),
    ch('🧵', 'deflate-encode-job', 'deflate_encode_job', [(296, 504)],
       'Resumable deflate encoder of the zlib codec: the encode job record and its step, match and block emission.'),
    ch('📥', 'retained-decoder', 'retained_decoder', [(506, 682)],
       'Retained zlib decoder of the zlib codec: the step result, the inflate refusal and the decoder that resumes across steps.', keep_pub=True),
    ch('📤', 'retained-encoder', 'retained_encoder', [(684, 877)],
       'Retained zlib encoder of the zlib codec: the encoder that resumes across steps and owns its output wallet.', keep_pub=True),
    ch('🚚', 'publication-and-job-steps', 'publication_and_job_steps', [(886, 1035)],
       'Publication hand-off and job steps of the zlib codec: the delivered publication retirement and the interactive job steps of the deflate encoder.'),
    ch('🔐', 'flat-close-and-checkpoint', 'flat_close_and_checkpoint', [(1038, 1160)],
       'Close protocol and checkpoints of the zlib codec: the flat close admission and receipt, the flat backing, the checkpoint writers and reader and the raw deflate entry.'),
    ch('🎚', 'tuned-encode', 'tuned_encode', [(1164, 1374)],
       'Tuned encoder of the zlib codec: the tuned frame, the tuned deflate job with its interactive steps and the deterministic and high-search raw deflate entries.'),
    ch('🔓', 'inflate-and-zlib-frame', 'inflate_and_zlib_frame', [(1376, 1552)],
       'Inflate and zlib framing of the zlib codec: the stored, fixed and dynamic block inflation, the raw inflate and the zlib compress and decompress entries.'),
]

IO = ZLIB.rsplit('/', 1)[0]

SPEC = dict(
    orig='zlib.rs.txt',
    file=ZLIB + '/🦀️.rs',
    nest=False,
    keep_empty=('🧪️CodecTests',),
    external=sub(IO + '/🗜️dynamic', keep_root=True) + sub(IO + '/📤️publication', keep_root=True) + sub(IO + '/🧪️tests', keep_root=True),
    outside=outside('zlib'),
    children=CH,
)


SPEC['foreign_live'] = dict(
    note='2026-10-17 08:51 a concurrent deflate carve (peer, crate semio-s-artifact-stdio-deflate-zlib) replaced the publication mount and removed the codec_tests mount in the installed parent',
    expect_missing=['#[path = "../📤️publication/🦀️.rs"]', 'pub mod publication;', '#[cfg(test)]', '#[path = "../🧪️tests/🔬️codec/🦀️.rs"]', 'mod codec_tests;'],
    expect_extra=['pub use semio_s_artifact_stdio_deflate_examples::standards::v_rfc1950::subsets::any::io::zlib::publication;'],
)
