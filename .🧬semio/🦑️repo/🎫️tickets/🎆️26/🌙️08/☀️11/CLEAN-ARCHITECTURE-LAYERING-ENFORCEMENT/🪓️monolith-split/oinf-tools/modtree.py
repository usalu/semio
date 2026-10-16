"""Module tree walker following #[path] attributes (cfg ignored)."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from oinf_rs import *

def path_attr(attrs):
    for a in attrs:
        m = re.match(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]', a)
        if m: return m.group(1)
    return None

def walk(root_file):
    out = []
    def process(items, text, modpath, base, cfgs, file):
        for it in items:
            if it.kind != 'mod': continue
            pa = path_attr(it.attrs)
            c2 = cfgs + it.cfg
            inline = it.text.rstrip().endswith('}')
            if inline:
                nb = os.path.normpath(os.path.join(base, pa if pa else it.name))
                out.append(dict(modpath=modpath + (it.name,), file=None, cfgs=c2, inline=True, parent=file))
                inner_start = text.index('{', it.start) + 1
                inner = text[inner_start:it.end-1]
                sub, _ = chunk_file(inner)
                process(sub, inner, modpath + (it.name,), nb, c2, file)
            else:
                if not pa: raise SystemExit(f'mod {it.name} in {file} has no path attr')
                child = os.path.normpath(os.path.join(base, pa))
                rec(child, modpath + (it.name,), c2)
    def rec(file, modpath, cfgs):
        text = open(file, encoding='utf-8').read()
        items, _ = chunk_file(text)
        out.append(dict(modpath=modpath, file=file, cfgs=cfgs, inline=False))
        process(items, text, modpath, os.path.dirname(file), cfgs, file)
    rec(root_file, ('crate',), [])
    return out

INFROOT = os.environ.get("OINF_ROOT", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite").rstrip("/") + "/"
LIBROOT = INFROOT + "📦️packages/🦀️rust/🦀️.rs"
if __name__ == '__main__':
    for e in walk(LIBROOT):
        f = e['file']
        print('::'.join(e['modpath']), '|', (f[len(INFROOT):] if f and f.startswith(INFROOT) else f), '|', ' '.join(c[:50] for c in e['cfgs'] if 'target_arch' not in c))
