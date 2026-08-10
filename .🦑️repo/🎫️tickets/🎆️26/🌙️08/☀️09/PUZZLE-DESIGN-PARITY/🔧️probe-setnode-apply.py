from pathlib import Path
root = next(Path('/Users/ueli/Documents/semio/✏️s/🔌️plugins').glob('*puzzle*'))
art2d = next(p for p in (root/'🗿️artifacts').iterdir() if p.name.endswith('2d'))
mut = next((art2d/'🧬️mutations').glob('*component.rs'))
# actually mutations component is at art2d/mutations/
mut = art2d / '🧬️mutations' / '🦀️component.rs'
print('mut exists', mut.exists(), mut)
t = mut.read_text()
print('pub delta', 'pub fn puzzle2d_document_delta_operations' in t)
i = t.find('impl Mutation for Puzzle2dMutation')
print(t[i:i+4000])
print('\n==== apply_puzzle2d_operation_to_value ====\n')
i = t.find('fn apply_puzzle2d_operation_to_value')
print(t[i:i+2000])
# SetNode mutation struct fields
i = t.find('SetNode')
print('\n==== enum region ====\n')
print(t[t.find('enum Puzzle2dMutation'):t.find('enum Puzzle2dMutation')+2000])
