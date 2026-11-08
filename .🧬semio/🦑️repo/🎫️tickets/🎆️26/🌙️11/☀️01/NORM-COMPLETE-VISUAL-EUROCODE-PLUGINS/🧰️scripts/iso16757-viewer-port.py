import os, re, shutil
ART = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts'
S = '🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer'
SRC = f'{ART}/🪨️en1996/{S}'
DST = f'{ART}/📇️iso16757/{S}'
SWAPS = [('EN 1996', 'ISO 16757'), ('En1996', 'Iso16757'), ('en1996', 'iso16757'), ('EN1996', 'ISO16757')]
EX = re.compile(r'(?:Iso16757Snapshot|En1996Snapshot)::(?:example\([^)]*\)|noncompliant_multi_fail\(\)|compliant_clay_wall\(\)|full_showcase\(\))')
def swap(t):
    for a, b in SWAPS:
        t = t.replace(a, b)
    t = EX.sub(lambda m: 'Iso16757Snapshot::broken_fixture()' if 'noncompliant' in m.group(0) else 'Iso16757Snapshot::default()', t)
    return t.replace('semio_s_plugin_norm_compliance::AnnexChoice::De', '')
shutil.rmtree(f'{DST}/🎭️modes/👁️view/🪟️windows/📊️report', ignore_errors=True)
rels = ['🦀️.rs', '🟦️.ts', '🧪️tests/🔬️unit/🦀️.rs', '🎭️modes/👁️view/🦀️.rs', '📦️packages/🦀️rust/Cargo.toml']
for root, _, files in os.walk(f'{SRC}/🎭️modes/👁️view/🪟️windows'):
    for n in files:
        rels.append(os.path.relpath(os.path.join(root, n), SRC))
for rel in rels:
    os.makedirs(os.path.dirname(f'{DST}/{rel}'), exist_ok=True)
    open(f'{DST}/{rel}', 'w', encoding='utf8').write(swap(open(f'{SRC}/{rel}', encoding='utf8').read()))
