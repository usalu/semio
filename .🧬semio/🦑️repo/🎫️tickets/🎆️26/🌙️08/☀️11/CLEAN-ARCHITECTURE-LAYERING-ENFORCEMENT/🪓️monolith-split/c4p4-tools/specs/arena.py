ALIASES = ('pub type DrawingNativeLayers =', 'pub type DrawingNativeText =', 'pub type DrawingNativeSegments =')


def pre(text):
    lines = text.split('\n')
    done = 0
    for i, line in enumerate(lines):
        if line.startswith(ALIASES):
            assert line.endswith('{usize::MAX}'), line
            lines[i] = line + '>;'
            done += 1
    assert done == 3, done
    return '\n'.join(lines)


SPEC = dict(
    pre=pre,
    orig='arena.rs.txt',
    file='✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🏟️arena/🦀️.rs',
    children=[
        dict(folder='🔓️owned-field-decode', mod='owned_field_decode',
             ranges=[(23, 338)],
             doc='//! 🔓️ Drawing arena owned-field decode: the retirement factories, the decoded field retirement, the macro-generated owned-field authorities and the envelope owned-field catalog.'),
        dict(folder='🏊️mutation-arena-pool', mod='mutation_arena_pool',
             ranges=[(356, 1038)],
             doc='//! 🏊️ Drawing mutation arena pool: the arena owner and builder, the pool with its slots and bootstrap job, the process state and the borrow functions.'),
        dict(folder='📐️bounds-census', mod='bounds_census',
             ranges=[(1040, 1257)],
             doc='//! 📐️ Drawing bounds census: the traversal frame, the snapshot owner totals, the fixed owner census, the asset bounds cursor and the snapshot bounds authority.'),
        dict(folder='🧬️snapshot-clone', mod='snapshot_clone',
             ranges=[(20, 21), (1259, 1477), (1601, 1607)],
             doc='//! 🧬️ Drawing snapshot clone: the native clone authority, the clone work totals, the layer clone work authority and the initial snapshot clone grant.'),
        dict(folder='🧭️layer-locator', mod='layer_locator',
             ranges=[(1479, 1599)],
             doc='//! 🧭️ Drawing layer locator: the layer address and the locator that finds a layer by identifier and rebuilds its container.'),
    ],
    external='auto',
)
