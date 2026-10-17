#!/usr/bin/env python3
"""c4p32: proves the chained row macros of the split BIM terminology expand to the same `app_labels!` token stream as the original table.
Sandbox only: both trees are written under the scratchpad, `semio_framework_ui_locale::app_labels!` is replaced by a stringify probe in the copies.
usage: labelsprobe.py [stage|live]"""
import os, re, sys, shutil, subprocess
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p32 as P
mode = sys.argv[1] if len(sys.argv) > 1 else 'stage'
SB = os.path.join(P.SCR, 'labelsprobe')
shutil.rmtree(SB, ignore_errors=True)
BIMT = '✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology'
PROBE = 'macro_rules! app_labels_probe { ($($t:tt)*) => { pub const LABELS_DUMP: &str = stringify!($($t)*); } }\n'


def prep(text):
    t = text.replace('semio_framework_ui_locale::app_labels!', 'app_labels_probe!')
    cut = t.index('//#endregion 🔖️Labels')
    t = t[:cut] + '//#endregion 🔖️Labels\n'
    i = t.index('macro_rules! bim_labels {')
    return t[:i] + PROBE + t[i:]


def write(rel, text, root):
    p = os.path.join(root, rel)
    assert os.path.realpath(os.path.dirname(p)).startswith(os.path.realpath(SB)), 'outside sandbox'
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, 'w', encoding='utf-8').write(text)


orig = open(os.path.join(T, 'orig', '5.rs.txt'), encoding='utf-8').read()
A = os.path.join(SB, 'orig')
write('lib.rs', prep(orig), A)
B = os.path.join(SB, 'new')
src = os.path.join(P.SCR, 'stage', 'bimterm') if mode == 'stage' else os.path.join(P.REPO, BIMT)
write('lib.rs', prep(open(os.path.join(src, '🦀️.rs'), encoding='utf-8').read()), B)
for d in sorted(os.listdir(src)):
    f = os.path.join(src, d, '🦀️.rs')
    if os.path.isfile(f):
        write(os.path.join(d, '🦀️.rs'), open(f, encoding='utf-8').read(), B)


def expand(root):
    p = subprocess.run(['rustc', '+nightly-2026-07-20', '--edition', '2021', '--crate-type', 'lib', '--crate-name', 'labelsprobe', '-Zunpretty=expanded', os.path.join(root, 'lib.rs')], capture_output=True, text=True, cwd=root)
    m = re.search(r'pub const LABELS_DUMP: &str =\s*"((?:[^"\\]|\\.)*)";', p.stdout, re.S)
    return (m.group(1) if m else None), p.returncode, p.stderr[-600:]


ea, ra, sa = expand(A)
eb, rb, sb = expand(B)
print('orig rc', ra, 'len', len(ea or ''), '| new rc', rb, 'len', len(eb or ''))
if ea is None or eb is None:
    print(sa); print(sb); sys.exit(2)
print('rows orig', ea.count(';'), 'new', eb.count(';'))
print('EXPANSION IDENTICAL' if ea == eb else 'EXPANSION DIFFERS')
sys.exit(0 if ea == eb else 1)
