#!/usr/bin/env python3
"""c4p32 file 5: splits the one `bim_labels!` row table into row-provider macros chained by callbacks; writes the stage tree and specs/bimterm.py.
usage: labels.py stage"""
import os, sys, json, collections
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p32 as P

BIMT = '✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology'
V = '️'
CHUNKS = [
    ('🏠', 'model-and-window-labels', 'model', 17, 304, 'Model and window labels of the BIM terminology: the utilities, windows, panels, entity kinds, groups, fields, actions, measures and status rows.'),
    ('🎮', 'command-and-argument-labels', 'commands', 306, 533, 'Command and argument labels of the BIM terminology: the outliner, status and progress rows and the command, argument and measure rows.'),
    ('⚠', 'fault-and-utility-labels', 'faults', 535, 828, 'Fault and utility labels of the BIM terminology: the fault rows, the utility rows, the sweep kinds and the schedule filter rows.'),
    ('🔍', 'review-and-schedule-labels', 'review', 830, 1276, 'Review and schedule labels of the BIM terminology: the schedule, family, diagnostic, clash, rule, issue and envelope rows.'),
]


def build():
    orig = open(os.path.join(T, 'orig', '5.rs.txt'), encoding='utf-8').read()
    L = orig.split('\n')
    assert L[13].strip() == '//#region 🔖️Labels' and L[15].strip() == 'bim_labels! {' and L[1276] == '}' and L[1277].strip() == '//#endregion 🔖️Labels', 'anchors'
    assert L[304].strip() == '' and L[533].strip() == '' and L[828].strip() == ''
    names = ['bim_label_rows_' + c[2] for c in CHUNKS]
    root = L[:12] + ['',
        'macro_rules! bim_labels_emit {',
        '    (; $($rows:tt)*) => {',
        '        bim_labels! { $($rows)* }',
        '    };',
        '}', '']
    for c in CHUNKS:
        root += ['#[macro_use]', '#[path = "%s%s%s/🦀️.rs"]' % (c[0], V, c[1]), 'mod %s;' % c[1].replace('-', '_'), '']
    root += L[13:15] + ['%s! { %s; }' % (names[0], ', '.join(names[1:] + ['bim_labels_emit']))] + L[1277:]
    files = {'🦀️.rs': '\n'.join(root)}
    for (e, k, n, a, b, doc) in CHUNKS:
        body = ['//! %s%s %s See [`super`].' % (e, V, doc), '',
                'macro_rules! bim_label_rows_%s {' % n,
                '    ($next:ident $(, $more:ident)*; $($rows:tt)*) => {',
                '        $next! { $($more),*; $($rows)*'] + L[a - 1:b] + ['        }', '    };', '}', '']
        files[e + V + k + '/🦀️.rs'] = '\n'.join(body)
    return orig, files


def stage():
    orig, files = build()
    d = os.path.join(P.SCR, 'stage', 'bimterm')
    import shutil
    shutil.rmtree(d, ignore_errors=True)
    for rel, text in files.items():
        p = os.path.join(d, rel)
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, 'w', encoding='utf-8').write(text)
    O = collections.Counter(P.norm_line(l) for l in orig.split('\n') if l.strip())
    N = collections.Counter()
    for t in files.values():
        N += collections.Counter(P.norm_line(l) for l in t.split('\n') if l.strip())
    miss, extra = O - N, N - O
    print('missing', dict(miss))
    print('extra', dict(extra))
    spec = dict(orig='5.rs.txt', file=BIMT + '/🦀️.rs', nest=False,
                children=[dict(folder=c[0] + V + c[1], mod=c[1].replace('-', '_'), ranges=[(c[3], c[4])], doc='') for c in CHUNKS],
                external=[], expect_missing=['bim_labels! {'],
                expect_extra=[l for l, n in extra.items() for _ in range(n)])
    with open(os.path.join(T, 'specs', 'bimterm.py'), 'w', encoding='utf-8') as fh:
        fh.write('SPEC = %s\n' % json.dumps(spec, ensure_ascii=False, indent=1).replace('true', 'True').replace('false', 'False'))
    print({k: v.count('\n') + 1 for k, v in files.items()})


if __name__ == '__main__':
    stage()
