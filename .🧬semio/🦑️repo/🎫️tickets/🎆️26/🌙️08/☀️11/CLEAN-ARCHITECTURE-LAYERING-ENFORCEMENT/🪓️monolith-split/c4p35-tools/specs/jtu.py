from specs.lib import *
import c4p35x as X

FR = [
    ('🧯' + V + 'retirement-laws', (4, 199), '🧯' + V + ' Retirement laws of the pack-json companion: the retained writer, projection and parser admission, resumption, cancellation and controlled retirement against the independent grammar.'),
    ('📐' + V + 'grammar-laws', (202, 391), '📐' + V + ' Grammar laws of the pack-json companion: the borrowing macro, the literal, number, string and container grammar and their refusals.'),
    ('🌉' + V + 'value-bridge-and-macro-laws', (393, 522), '🌉' + V + ' Value bridge and macro laws of the pack-json companion: the dsl value round trips, the json macro and the serde bridge bytes.'),
    ('🎲' + V + 'property-and-differential-laws', (524, 822), '🎲' + V + ' Property and differential laws of the pack-json companion: the deterministic generator, the round trips, the serde differentials and the float parity.'),
    ('🫳' + V + 'borrowed-source-laws', (824, 1079), '🫳' + V + ' Borrowed source laws of the pack-json companion: the borrowed source and sink, the direct semantic source and the retirement and cancellation accounting.'),
]

SPEC = dict(
    orig='jtu.rs',
    file=JTU + '/🦀️.rs',
    build=lambda t: X.build_include(t, FR),
    expect_extra=['/// ' + d for _, _, d in FR],
    children=[dict(folder=f, mod=None) for f, _, _ in FR],
)
