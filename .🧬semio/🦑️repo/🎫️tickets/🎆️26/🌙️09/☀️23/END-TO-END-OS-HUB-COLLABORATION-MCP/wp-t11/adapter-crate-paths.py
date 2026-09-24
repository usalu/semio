"""🧭️ Rewrites `crate::X` in stdio case adapters to the artifact crate that exports `X` at its root.

The adapter is compiled as a separate host crate, so `crate::` names the host, never the artifact.
Only names the artifact crate root exports and the adapter does not define are rewritten."""
import os, re, sys, glob
ROOT = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts'
write = '--write' in sys.argv
def exports(fmt_dir):
    text = open(os.path.join(fmt_dir, '🦀️.rs'), encoding='utf-8').read()
    names = set(re.findall(r'^pub (?:const|static|fn|struct|enum|mod|type|trait) ([A-Za-z_][A-Za-z0-9_]*)', text, re.M))
    for body in re.findall(r'^pub use ([^;]+);', text, re.M):
        tail = body.split('::')[-1]
        if tail.startswith('{'):
            names |= {n.strip().split(' as ')[-1] for n in tail.strip('{}').split(',') if n.strip() and n.strip() != '*'}
        elif tail != '*':
            names.add(tail.split(' as ')[-1].strip())
    return names
def crate_name(fmt_dir):
    return re.search(r'^name = "([^"]+)"', open(os.path.join(fmt_dir, '📦️packages/🦀️rust/Cargo.toml'), encoding='utf-8').read(), re.M).group(1).replace('-', '_')
total = 0
for fmt in sorted(os.listdir(ROOT)):
    fmt_dir = os.path.join(ROOT, fmt)
    if not os.path.isfile(os.path.join(fmt_dir, '🦀️.rs')) or not os.path.isfile(os.path.join(fmt_dir, '📦️packages/🦀️rust/Cargo.toml')):
        continue
    names, krate = exports(fmt_dir), crate_name(fmt_dir)
    for path in glob.glob(os.path.join(fmt_dir, '🏅️standards/*/🪆️subsets/*/🧪️tests/*/🦀️.rs')):
        if not os.path.isfile(os.path.join(os.path.dirname(path), '🥒️.feature')):
            continue
        text = open(path, encoding='utf-8').read()
        local = set(re.findall(r'(?:fn|struct|enum|const|static|mod|type) ([A-Za-z_][A-Za-z0-9_]*)', text))
        def group(m):
            items = [i.strip() for i in m.group(1).split(',') if i.strip()]
            return f'{krate}::{{{m.group(1)}}}' if items and all(i.split(' as ')[0] in names and i.split(' as ')[0] not in local for i in items) else m.group(0)
        def single(m):
            return f'{krate}::{m.group(1)}' if m.group(1) in names and m.group(1) not in local else m.group(0)
        new = re.sub(r'(?<![\w$])crate::\{([^}]*)\}', group, text)
        new = re.sub(r'(?<![\w$])crate::([A-Za-z_][A-Za-z0-9_]*)', single, new)
        if new != text:
            n = sum(1 for a, b in zip(text.splitlines(), new.splitlines()) if a != b)
            total += n
            print(f'{n:3} {os.path.relpath(path, ROOT)}')
            if write:
                open(path, 'w', encoding='utf-8').write(new)
print('lines', total, 'written' if write else 'dry-run')
