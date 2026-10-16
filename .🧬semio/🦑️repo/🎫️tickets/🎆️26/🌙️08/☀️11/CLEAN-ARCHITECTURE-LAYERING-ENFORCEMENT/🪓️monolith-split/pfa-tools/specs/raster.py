D = '✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🔨️modules/🏠️host/🧰️owned/'
F = D + '🦀️.rs'

def pre(text):
    lines = text.split('\n')
    assert lines[2].startswith('use semio_framework_job_publication::')
    assert lines[3] == '#![allow(clippy::result_large_err)]' and lines[4] == '#![allow(unexpected_cfgs)]'
    return '\n'.join(lines[:2] + lines[3:5] + [lines[2]] + lines[5:])


SPEC = dict(
    orig='raster.rs.txt',
    file=F,
    pre=pre,
    children=[
        dict(folder='🔓️owned-field-decode', mod='owned_field_decode',
             ranges=[(17, 23), (25, 231), (2012, 2045), (3198, 3243)],
             doc='//! 🔓️ raster owned field decode: the pack decoders, the macro-generated snapshot and mutation field decode authorities, the envelope owned-field catalog and the rejected conflict authority.'),
        dict(folder='♻️retirement', mod='retirement',
             ranges=[(657, 796)],
             doc='//! ♻️ raster retirement: the snapshot and mutation retirement factories, the displaced value, the retirement slot and the grant helpers.'),
        dict(folder='🚀️store-initialization', mod='store_initialization',
             ranges=[(233, 654), (1688, 2008)],
             doc='//! 🚀️ raster store initialization: the document store owners, the store initialization authority with its phase machine and the initialization job.'),
        dict(folder='🧬️snapshot-clone', mod='snapshot_clone',
             ranges=[(798, 1036), (2361, 2560), (2562, 2882), (3279, 3288)],
             doc='//! 🧬️ raster snapshot clone: the snapshot, dsl value and layer clone authorities that copy a snapshot incrementally.'),
        dict(folder='📐️bounds-census', mod='bounds_census',
             ranges=[(2047, 2132), (2134, 2214), (2216, 2349), (2351, 2359), (2884, 2989), (2991, 3107), (3245, 3277), (3290, 3292), (3296, 3302)],
             doc='//! 📐️ raster bounds census: the owner totals, the map key cursor, the snapshot, layer and dsl value bounds authorities, the layer locator and the traversal frame.'),
        dict(folder='🧩️mutation-candidate', mod='mutation_candidate',
             ranges=[(1038, 1686), (3109, 3190)],
             doc='//! 🧩️ raster mutation candidate: the mutation candidate authority with its phase machine and the one-item apply.'),
    ],
    external=[D + '🧪️tests/🦀️.rs'],
)
