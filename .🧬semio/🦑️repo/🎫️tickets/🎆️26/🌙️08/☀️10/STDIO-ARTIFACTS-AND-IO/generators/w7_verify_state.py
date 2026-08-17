from pathlib import Path

hits = []
for p in Path('.').rglob('*mimes.csv'):
    s = str(p)
    if any(x in s for x in ['node_modules', '/target/', '/dist/', 'fixture', 'storybook-static']):
        continue
    hits.append(p)
print('MIMES HITS:')
for p in sorted(hits):
    print(' ', p, 'bytes', p.stat().st_size)

ui_root = Path('🧰️framework/🔨️modules/🖱️ui')
assets_root = Path('�
assets_root = Path('�')
assets_root = Path('�')
