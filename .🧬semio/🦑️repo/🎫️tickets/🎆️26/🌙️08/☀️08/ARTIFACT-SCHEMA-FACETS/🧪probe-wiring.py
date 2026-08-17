from pathlib import Path
LP = next(p for p in Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins').iterdir() if 'lowpoly' in p.name)

mut = LP / '🗿️artifacts' / '💠️lowpoly' / '🧬️mutations' / '� p.name)

mut = LP / '🗿️artifacts' / '💠️lowpoly' / '🧬️mutations' / '🦀️component.rs'
for i,l in enumerate(mut.read_text().splitlines(),1):
    if any(k in l for k in ('MutationDiff','fn diff','impl Mutation','type Diff','fn apply_lowpoly','to_diff')):
        print(f'mut:{i}:{l}')

eng = next((LP / '�','fn apply_lowpoly','to_diff')):
        print(f'mut:{i}:{l}')

eng = next((LP / '🗿️artifacts' / '💠️lowpoly' / '⚙️engine').glob('*.rs'))
for i,l in enumerate(eng.read_text().splitlines(),1):
    if any(k in l for k in ('DocumentApp','type Artifact','type Snapshot','fn snapshot','LowpolyArtifact','MutationDiff')):
        print(f'eng:{i}:{l}')

app = LP / '🎛️apps' / '�𝒽lowpoly' / '🦀️component.rs'
app = LP / '🎛️apps' / '💠️lowpoly' / '🦀️component.rs'
for i,l in enumerate(app.read_text().splitlines(),1):
    if any(k in l for k in ('DocumentApp','type Artifact','type Snapshot','MutationDiff','fn apply')):
        print(f'app:{i}:{l}')

print('--- mutation dirs ---')
for p in sorted((LP / '🗿️artifacts' / '� / '🧬️mutations').iterdir()):
for p in sorted((LP / '🗿️artifacts' / '�l}')

print('--- mutation dirs ---')
for p in sorted((LP / '🗿️artifacts' / '� / '🧬️mutations').iterdir()):
for p in sorted((LP / '🗿️artifacts' / '💠️lowpoly' / '🧬️mutations').iterdir()):
    print(p.name)

# print objects-add diff
for p in (LP / '🗿️artifacts' / '� for p in (LP / '🗿️artifacts' / '💠️lowpoly' / '🧬️mutations').rglob('*/🔺️diff/�artifacts' / '💠️lowpoly' / '🧬️mutations').rglob('*/🔺️diff/🦀️component.rs'):
    if 'objects-add' in str(p):
        print('===', p)
        print(p.read_text())
