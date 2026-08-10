from pathlib import Path
root = next(Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins').glob('*puzzle*'))
appdir=next(p for p in next(root.glob('*apps*')).iterdir() if p.name.endswith('2d'))
t=(appdir/'🦀️component.rs').read_text()
i=t.find('fn handle(')
print(t[i:i+9000])
# also operations_from_fixture
for key in ['operations_from_fixture', 'puzzle2d_operations', 'Emit::', 'fn scene_for', 'diff_fixture']:
    print('\nKEY', key, t.find(key))
    j=t.find(key)
    if j>=0:
        print(t[j:j+2000])
