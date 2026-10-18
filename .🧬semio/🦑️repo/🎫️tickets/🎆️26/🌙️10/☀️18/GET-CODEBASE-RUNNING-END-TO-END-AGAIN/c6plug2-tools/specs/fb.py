from specs.lib import *
import re as _re
F = FILES['fb']
OLD = '        struct $name(SessionCapture);'
NEW = '        pub(super) struct $name(pub(super) SessionCapture);'
def pre(t):
    assert t.count(OLD) == 6
    return t.replace(OLD, NEW)
RANGES = [
    ('🔶', 'primitives-curves-surfaces-and-sweeps', 'primitives_curves_surfaces_and_sweeps', (234, 477),
     'Primitives, curves, surfaces and sweeps: the primitive, curve, surface and sweep operators of the brep extension.'),
    ('🔪', 'booleans-and-transforms', 'booleans_and_transforms', (481, 711),
     'Booleans and transforms: the boolean operation macro with its kernel plan job and the compound cut and transform operators.'),
    ('🧲', 'features-intersect-evaluate-and-topology', 'features_intersect_evaluate_and_topology', (715, 1062),
     'Features, intersect, evaluate and topology: the feature, intersection, curve and surface evaluation and topology operators.'),
    ('📏', 'measure-utilities-and-io', 'measure_utilities_and_io', (1066, 1276),
     'Measure, utilities and io: the measure and utility operators and the step, stl, obj and dwg export and import operators.'),
]
_orig = open(os.path.join(T, 'orig', 'fb.rs'), encoding='utf-8').read().split('\n')
_NAME = _re.compile(r'^(?:geo|num|point|vec|text|boolean)_operation!\((\w+),')
_STRUCT = _re.compile(r'^struct (\w+)\(SessionCapture\);')
def names(a, b):
    out = []
    for l in _orig[a - 1:b]:
        m = _NAME.match(l)
        if m:
            out.append(m.group(1))
    return out
CH = [ch(e, k, m, [r], d) for e, k, m, r, d in RANGES]
CH.append(ch('🧾', 'register-operators', 'register_operators', [(1281, 2384)],
             'Register operators: the registration of the brep schemas and operators, the extension manifest json and the module registry.'))
PP = []
for e, k, m, r, d in RANGES:
    ns = [n for n in names(*r)]
    if ns:
        PP.append(('mod %s;' % m, 'mod %s;\nuse %s::{%s};' % (m, m, ', '.join(sorted(ns)))))
SPEC = dict(orig='fb.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='fb.rs'), pre=pre, patch_parent=PP,
            expect_missing=[OLD] * 6, expect_extra=[NEW] * 6 + ['use %s::{%s};' % (m, ', '.join(sorted(names(*r)))) for e, k, m, r, d in RANGES if names(*r)])
