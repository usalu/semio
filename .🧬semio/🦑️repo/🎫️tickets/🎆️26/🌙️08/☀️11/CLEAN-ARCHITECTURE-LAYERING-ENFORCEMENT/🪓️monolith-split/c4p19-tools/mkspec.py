import os, sys, json
T = os.path.dirname(os.path.abspath(__file__))
REPO = '/Users/ueli/Documents/semio'
V = '️'
N = '✏️s/🔌️plugins/📕️norm/🗿️artifacts'
S = '🏅️standards/🔖️1/🪆️subsets/✳️any'
PB = '✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural'


def ch(emoji, kebab, mod, ranges, doc, **kw):
    return dict(folder=emoji + V + kebab, mod=mod, ranges=ranges, doc='//! %s%s %s See [`super`].' % (emoji, V, doc), **kw)


def facet_patch(children, extra=()):
    old = 'rust: include_str!("🦀️.rs"),'
    parts = ['include_str!("🦀️.rs")'] + ['include_str!("%s/🦀️.rs")' % c['folder'] for c in children]
    return (old, 'rust: concat!(%s),' % ', '.join(parts))


def spec(n, file, children, **kw):
    d = dict(orig='%d.rs.txt' % n, file=file, nest=False, children=children, external=kw.pop('external', []))
    d.update(kw)
    return d


TABLE = {}

c = [
    ch('🩹', 'list-row-patches', 'list_row_patches', [(21, 442)], 'Positional list deltas of the ISO 16757 diff: the keyed sparse patches and row deltas of the catalogue lists (product groups, classes, series, products, indexes, property definitions, subjects).'),
    ch('🗂', 'keyed-row-sections', 'keyed_row_sections', [(444, 744)], 'Keyed row sections of the ISO 16757 diff: the geometry objects, selection constraints and part number inputs rows and the script limits patch.'),
    ch('🔁', 'apply-and-inverse', 'apply_and_inverse', [(746, 807), (852, 1033)], 'Apply and inverse of the ISO 16757 diff: the single id value patches, the central applier hook and the algebra that returns the negative diff.'),
]
TABLE['iso16757'] = spec(1, N + '/📇️iso16757/' + S + '/🧬️schema/🔺️diff/🦀️.rs', c, patch_parent=[facet_patch(c)])

c = [
    ch('🏗', 'beam-rows', 'beam_rows', [(21, 368)], 'Beam sections of the EN 1994 diff: the beam action rows and the keyed beam row patches.'),
    ch('🏛', 'column-rows', 'column_rows', [(370, 661)], 'Column sections of the EN 1994 diff: the column action rows and the keyed column row patches.'),
    ch('🧱', 'slab-rows', 'slab_rows', [(663, 954)], 'Slab sections of the EN 1994 diff: the slab action rows and the keyed slab row patches.'),
    ch('🔁', 'apply-and-inverse', 'apply_and_inverse', [(983, 1077)], 'Apply and inverse of the EN 1994 diff: the central applier hook and the algebra that returns the negative diff.'),
]
TABLE['en1994'] = spec(2, N + '/🧩️en1994/' + S + '/🧬️schema/🔺️diff/🦀️.rs', c, patch_parent=[facet_patch(c)])

c = [
    ch('🧮', 'resistance-helpers', 'resistance_helpers', [(16, 241)], 'Pure DIN 4108 and EN ISO 6946 helpers of the compliance checks: the surface resistances, the layer and total resistances, the U-value and the insulation thickness.'),
    ch('🧱', 'part-2-minimum-insulation', 'part_2', [(244, 889)], 'DIN 4108-2 of the compliance checks: the minimum thermal insulation, the mould criterion and the surface temperature checks.', modbody=True, glob=False),
    ch('💧', 'part-3-moisture-protection', 'part_3', [(893, 1078)], 'DIN 4108-3 of the compliance checks: the vapour pressure and the Glaser condensation checks.', modbody=True, glob=False),
    ch('📊', 'part-4-design-values', 'part_4', [(1082, 1226)], 'DIN 4108-4 of the compliance checks: the design thermal conductivity tables and the catalogue checks.', modbody=True, glob=False),
    ch('🌡', 'part-6-u-value-checks', 'part_6', [(1230, 1611)], 'DIN 4108-6 and EN ISO 6946 of the compliance checks: the bridged U-value and the U-value checks.', modbody=True, glob=False),
    ch('🌬', 'part-7-airtightness', 'part_7', [(1615, 1657)], 'DIN 4108-7 of the compliance checks: the n50 limit and the airtightness check.', modbody=True, glob=False),
    ch('🌉', 'bb-2-thermal-bridges', 'bb_2', [(1661, 1853)], 'DIN 4108 Beiblatt 2 of the compliance checks: the thermal bridge categories and the equivalence checks.', modbody=True, glob=False),
    ch('🔧', 'part-10-application-types', 'part_10', [(1857, 2079)], 'DIN 4108-10 of the compliance checks: the application type codes and the product property rows.', modbody=True, glob=False),
]
TABLE['din4108'] = spec(3, N + '/🧱️din4108/' + S + '/🧬️schema/💡️inferences/⚖️checks/🦀️.rs', c, wiring='each')

c = [
    ch('📏', 'factors-and-geometry', 'factors_and_geometry', [(31, 302)], 'Factors and geometry of the EN 1996 masonry checks: the annex parameters, the characteristic strengths and the slenderness, eccentricity and reduction factors.'),
    ch('🔥', 'fire-and-design-effects', 'fire_and_design_effects', [(306, 527)], 'Fire resistance and design effects of the EN 1996 masonry checks: the minimum fire thickness, the mortar strength, the combination factors and the governing design effects.'),
    ch('🛡', 'conformance-and-integrity', 'conformance_and_integrity', [(531, 791)], 'Material conformance and document integrity of the EN 1996 masonry checks: the conformance test, the duplicate id and referential integrity checks and the building entry point.'),
    ch('🧱', 'evaluate-wall', 'evaluate_wall', [(793, 1594)], 'The wall evaluation of the EN 1996 masonry checks: the clause-by-clause compliance assessment of one wall.'),
]
TABLE['en1996'] = spec(4, N + '/🪨️en1996/' + S + '/🧬️schema/⚖️masonry/🦀️.rs', c, prune_unused_reexports=True)

c = [
    ch('🌲', 'material-and-annex-factors', 'material_and_annex_factors', [(16, 392)], 'Material and annex factors of the EN 1995 timber checks: the strength class properties, the annex parameters and the modification, creep, size, buckling and shear factors.'),
    ch('🔩', 'fasteners-and-johansen', 'fasteners_and_johansen', [(394, 536)], 'Fasteners of the EN 1995 timber checks: the characteristic embedment and yield values, the Johansen load-carrying capacities, the effective number and the spacing minima.'),
    ch('🧮', 'load-combinations', 'load_combinations', [(538, 917)], 'Load combinations of the EN 1995 timber checks: the action internals, the member and connection combination enumeration and the deflection.'),
    ch('🧰', 'assessment-vocabulary', 'assessment_vocabulary', [(919, 1120)], 'Assessment vocabulary of the EN 1995 timber checks: the governing combination, the grain and lever tables and the solve and finish steps of one assessment.'),
    ch('🏗', 'structure-and-members', 'structure_and_members', [(1122, 1567)], 'Structure and member assessments of the EN 1995 timber checks: the integrity checks, the structure entry point and the member, floor and fire assessments.'),
    ch('🌉', 'bridge-and-connections', 'bridge_and_connections', [(1569, 1745)], 'Footbridge member and connection assessments of the EN 1995 timber checks.'),
]
TABLE['en1995'] = spec(5, N + '/🪵️en1995/' + S + '/🧬️schema/⚖️timber/🦀️.rs', c, prune_unused_reexports=True)

c = [
    ch('🔬', 'resistance-worked-examples', 'resistance_worked_examples', [(11, 169)], 'Worked resistance examples of the EN 1999 compliance tests: the hand-derived buckling, alloy, moment, fatigue, fire and cold-formed values.'),
    ch('✅', 'compliance-gates', 'compliance_gates', [(171, 344)], 'Compliance gates of the EN 1999 compliance tests: the compliant and non-compliant example reports, the remedy laws, the bundled DSL and the Python oracle.'),
    ch('🧬', 'facet-and-asset-parity', 'facet_and_asset_parity', [(346, 525)], 'Facet and asset parity of the EN 1999 compliance tests: the example snapshot schema, the bundled assets, the field metadata and the facet leaves.'),
    ch('🔎', 'leaf-influence-and-material-laws', 'leaf_influence_and_material_laws', [(527, 830)], 'Leaf influence and material laws of the EN 1999 compliance tests: the metadata rows, the editable leaf influence, the asset regeneration and the material id laws.'),
    ch('🧭', 'path-helpers', 'path_helpers', [(832, 1008)], 'Path helpers of the EN 1999 compliance tests: the family directory, the oracle script and the snapshot path grammar and editors.'),
    ch('🚦', 'gate-parity', 'gate_parity', [(1010, 1166)], 'Gate parity of the EN 1999 compliance tests: every failing remedy clears its status.'),
]
TABLE['en1999'] = spec(6, N + '/🪶️en1999/' + S + '/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs', c, prune_unused_reexports=True)

c = [
    ch('🧰', 'seismic-helpers', 'seismic_helpers', [(67, 97)], 'Helpers of the EN 1998 inference: the quantity and localisation constructors, the id selector and the annex resolution.'),
    ch('🌍', 'full-seismic-check', 'full_seismic_check', [(99, 1071)], 'The full seismic check of the EN 1998 inference: the clause-by-clause evaluation across the claimed parts.'),
    ch('⚖', 'part-evaluators', 'part_evaluators', [(1073, 1454)], 'Part evaluators of the EN 1998 inference: the integrity checks and the bridge, assessment, silo, foundation and tower evaluations.'),
]
TABLE['en1998'] = spec(7, N + '/🫨️en1998/' + S + '/🧬️schema/💡️inferences/🦀️.rs', c, patch_parent=[facet_patch(c)],
                      external=[N + '/🫨️en1998/' + S + '/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs', N + '/🫨️en1998/' + S + '/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs'])

c = [
    ch('🎒', 'payload-codec-and-diff', 'payload_codec_and_diff', [(145, 327)], 'Payload of the procedural module: the render payload record with its codec and the sparse payload diff.'),
    ch('🧱', 'learning-program-and-geometry', 'learning_program_and_geometry', [(99, 130), (331, 566)], 'Learning program and geometry ownership of the procedural module: the authored program, the flow widget helpers and the geometry handle owner.'),
    ch('👁', 'preview-and-params-body', 'preview_and_params_body', [(568, 678), (745, 839)], 'Preview and parameters of the procedural module: the evaluated preview payload and body, the media buttons and the parameter controls.'),
    ch('💾', 'media-export-and-command', 'media_export_and_command', [(680, 743), (843, 871)], 'Media exchange of the procedural module: the solid export and import handlers and the command schema.'),
]
TABLE['procedural'] = spec(8, PB + '/🦀️.rs', c, explicit_reexport=True,
                           external=[PB + '/🚪️io/🦀️.rs', PB + '/🚪️io/💾️binary/🦀️.rs', PB + '/🚪️io/📝️text/🦀️.rs'], external_test=[PB + '/🧪️tests/🔬️unit/🦀️.rs'])

if __name__ == '__main__':
    for k, v in TABLE.items():
        with open(os.path.join(T, 'specs', k + '.py'), 'w', encoding='utf-8') as fh:
            fh.write('SPEC = %s\n' % json.dumps(v, ensure_ascii=False, indent=1).replace('true', 'True').replace('false', 'False'))
        print('wrote', k)
