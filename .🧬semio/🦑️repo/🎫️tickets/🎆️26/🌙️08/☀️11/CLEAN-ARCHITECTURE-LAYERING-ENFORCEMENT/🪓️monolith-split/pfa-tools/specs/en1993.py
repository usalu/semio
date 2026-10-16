F = '✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs'

CALL = '    check_special_elements(document, annex, params, &mut report);'
DOC = '/// 🔩 The joint, fatigue, fire, cold-formed, stainless, plated, shell, through-thickness, tension, high-strength, bridge, tower, pile and crane checks of `check_full_steel_structure`, appended to the same report.'
HEADER = 'fn check_special_elements(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, report: &mut CheckReport) {'


def pre(text):
    lines = text.split('\n')
    assert lines[1332 - 1] == 'pub fn check_full_steel_structure(document: &En1993Snapshot) -> CheckReport {'
    assert lines[1938 - 1] == '    }' and lines[1939 - 1] == ''
    assert lines[1940 - 1].strip() == '// Joints 1-8'
    assert lines[2917 - 1] == '    }' and lines[2918 - 1] == ''
    assert lines[2919 - 1] == '    report' and lines[2920 - 1] == '}'
    head = lines[:1938]
    tail = lines[1939:2917]
    rest = lines[2920:]
    out = head + ['', CALL, '', '    report', '}', '', DOC, HEADER] + tail + ['}'] + rest
    return '\n'.join(out)


SPEC = dict(
    orig='en1993.rs.txt',
    file=F,
    pre=pre,
    pre_extra=[CALL.strip(), DOC.strip(), HEADER.strip(), '}'],
    children=[
        dict(folder='📏️parts', mod='parts', glob=False, ranges=[(107, 902)],
             doc='//! 📏️ EN 1993 clause parts: the clause tables and resistance formulas of EN 1993-1-1 to 1-12 and parts 2 to 6.'),
        dict(folder='🧰️structure-helpers', mod='structure_helpers', ranges=[(904, 1330)],
             doc='//! 🧰️ EN 1993 structure helpers: section and material lookup, member kinds, load combination of scalar forces and member actions, and the referential-integrity checks.'),
        dict(folder='🧱️member-checks', mod='member_checks', ranges=[(1332, 1943)],
             doc='//! 🧱️ EN 1993 member checks: `check_full_steel_structure` verifies every member against EN 1993-1-1 and then runs the special element checks.'),
        dict(folder='🔗️special-element-checks', mod='special_element_checks', ranges=[(1945, 2925)],
             doc='//! 🔗️ EN 1993 special element checks: joints, fatigue, fire, cold-formed members, plated panels, shells, tension components, bridges, towers, piles and crane runways.'),
    ],
    external=['✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs'],
)
