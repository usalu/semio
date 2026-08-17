from pathlib import Path
root = next(Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins').glob('*puzzle*'))
for rel in [
    '🗿️artifacts/◻2d/🔺️diff/🦀️component.rs',
    '🗿️artifacts/◻2d/🧬️mutations/🦀️component.rs',
    '🗿️artifacts/◻2d/🧬️mutations/📍set-node/🦠️mutation/🦀️component.rs',
]:
    f = root/rel
    print('\n========', rel, 'len', f.stat().st_size)
    t=f.read_text()
    print(t[:4000])
    if len(t)>4000:
        print('...[mid]...')
        print(t[4000:8000])
