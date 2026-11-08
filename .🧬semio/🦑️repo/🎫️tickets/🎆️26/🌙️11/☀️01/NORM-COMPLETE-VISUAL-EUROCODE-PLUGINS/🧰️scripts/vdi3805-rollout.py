import os, shutil
ART = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts'
S = '🏅️standards/🔖️1/🪆️subsets/✳️any'
SRC = f'{ART}/🪶️en1999/{S}'
DST = f'{ART}/🏭️vdi3805/{S}'
SW = [('EN-1999', 'VDI-3805'), ('EN 1999', 'VDI 3805'), ('En1999', 'Vdi3805'), ('EN1999', 'VDI3805'), ('en1999', 'vdi3805'),
      ('the projected steel scene with utilisation colouring, supports, loads and joints', 'the projected catalogue scene with utilisation colouring, product bodies and parametric geometry'),
      ('buckling curves, LTB, M–N interaction, classification, S–N and fire curves of the selected check\'s entity', 'characteristic curves, radiator output, valve pressure drop and catalogue utilisation of the selected check\'s product')]
def swap(t):
    for a, b in SW: t = t.replace(a, b)
    return t
def copy_tree(rel, skip=()):
    for root, _, files in os.walk(f'{SRC}/{rel}'):
        for n in files:
            if n == '📌️.empty.md' or n in skip: continue
            p = os.path.join(root, n)
            t = os.path.join(DST, rel, os.path.relpath(p, f'{SRC}/{rel}'))
            os.makedirs(os.path.dirname(t), exist_ok=True)
            open(t, 'w', encoding='utf8').write(swap(open(p, encoding='utf8').read()))
W = '✏️editor/🎭️modes/✏️edit/🪟️windows'
V = '👁️viewer/🎭️modes/👁️view/🪟️windows'
for old in ('📥️inputs', '📊️results'): shutil.rmtree(f'{DST}/{W}/{old}', ignore_errors=True)
shutil.rmtree(f'{DST}/{V}/📊️report', ignore_errors=True)
for w in ('✅️checks', '📈️diagrams', '🌲️outliner', '🧾️properties', '🧊️viewport'): copy_tree(f'{W}/{w}')
for w in ('✅️checks', '📈️diagrams', '🧊️viewport'): copy_tree(f'{V}/{w}')
for rel in ('✏️editor/🎭️modes/✏️edit', '👁️viewer/🎭️modes/👁️view'):
    for f in ('🦀️.rs', '🧪️tests/🔬️unit/🦀️.rs'):
        os.makedirs(os.path.dirname(f'{DST}/{rel}/{f}'), exist_ok=True)
        open(f'{DST}/{rel}/{f}', 'w', encoding='utf8').write(swap(open(f'{SRC}/{rel}/{f}', encoding='utf8').read()))
copy_tree('✏️editor/🧮️incremental')
for f in ('✏️editor/🦀️.rs', '✏️editor/📦️packages/🦀️rust/Cargo.toml', '👁️viewer/🦀️.rs', '👁️viewer/📦️packages/🦀️rust/Cargo.toml', '👁️viewer/🧪️tests/🔬️unit/🦀️.rs'):
    open(f'{DST}/{f}', 'w', encoding='utf8').write(swap(open(f'{SRC}/{f}', encoding='utf8').read()))
