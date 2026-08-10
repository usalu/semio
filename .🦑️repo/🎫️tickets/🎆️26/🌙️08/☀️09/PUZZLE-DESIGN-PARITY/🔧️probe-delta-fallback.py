from pathlib import Path
root = next(Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins').glob('*puzzle*'))
art2d = next(p for p in (root/'🗿️artifacts').iterdir() if p.name.endswith('2d'))
mut = art2d / '🧬️mutations' / '🦀️component.rs'
t = mut.read_text()
# find tests around granular delta
i = t.find('granular delta must not fall back')
print(t[i-800:i+600] if i>=0 else 'no test')
# Puzzle2dSnapshot struct - does from_value of sparse fixture work
comp = art2d / '🦀️component.rs'
ct = comp.read_text()
i = ct.find('pub struct Puzzle2dSnapshot')
print('\n==== snapshot ====\n', ct[i:i+1200])
# 3d equivalent delta
art3d = next(p for p in (root/'🗿️artifacts').iterdir() if p.name.endswith('3d'))
for f in art3d.rglob('*mutations*component.rs'):
    tt=f.read_text()
    if 'document_delta_operations' in tt or 'SetSnapshot' in tt:
        print('3d mut', f)
        j=tt.find('fn puzzle3d_document_delta')
        if j<0: j=tt.find('fallback')
        print(tt[j:j+1500] if j>=0 else tt[:500])
