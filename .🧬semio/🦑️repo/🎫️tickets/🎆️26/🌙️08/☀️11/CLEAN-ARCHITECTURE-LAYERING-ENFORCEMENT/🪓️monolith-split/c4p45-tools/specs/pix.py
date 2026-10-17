from specs.lib import *

CH = [
    ch('🧱', 'png-chunks-and-header', 'png_chunks_and_header', [(93, 213)],
       'PNG chunks and header of the pixels codec: the signature, the chunk writer and readers, the IHDR record and parser and the sample and row size helpers.'),
    ch('🧮', 'png-filters-and-samples', 'png_filters_and_samples', [(217, 388)],
       'PNG filters and samples of the pixels codec: the Paeth predictor, the row filters and defilters, the Adam7 pass geometry and the sample unpacking to RGBA.'),
    ch('📤', 'png-encoding-core', 'png_encoding_core', [(392, 468)],
       'PNG encoding core of the pixels codec: the RGBA and gray16 encoders, the IDAT packer, the PNG assembler and the whole-image decoder.'),
    ch('🔄', 'scanline-decoder', 'scanline_decoder', [(472, 827)],
       'Scanline decoder of the pixels codec: the resumable PNG scanline decoder with its step, source and stream records.'),
]

SPEC = dict(
    orig='pix.rs',
    file=PIX + '/🦀️.rs',
    children=CH,
    external=mounted(PIX + '/🦀️.rs', PIX + '/📦️packages/🦀️rust/🦀️.rs'),
    expect_missing=['mod deflate {', '}'],
    inner=dict(mod='deflate', indent=4, folder='🗜️deflate', doc='', children=[]),
)
