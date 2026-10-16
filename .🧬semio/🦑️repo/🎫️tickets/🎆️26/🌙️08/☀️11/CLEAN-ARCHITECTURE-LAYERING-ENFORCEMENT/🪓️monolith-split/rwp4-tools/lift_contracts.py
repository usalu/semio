# 🛗️ WP4 lifts for `contracts` (plan 5.2 / Appendix H): the shell find-item family and `ShellHubRemoteV1`, written verbatim as `📥️lifted` files of the destination crate (the Shell originals stay until `contracts` mounts them).
import os, re, sys
os.environ['RWP4_SRC'] = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/orig-shell/🎯️targets/🧊️wgpu'
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from assign import *
from t1lib import *
from s0 import widen_head
home, files, items = analyse()
F = {f['slug']: f for f in files}
CONTRACTS = T + '/🤝️contracts/📥️lifted'
PARENT = F['🦀️.rs']

def uses_for(ids):
    out = []
    for u in PARENT['units']:
        if u.kind == 'use':
            txt = '\n'.join(PARENT['src'].lines[u.first:u.last + 1])
            bt = re.sub(r'\bas\s+(\w+)', r'::\1', '\n'.join(l for l in blank(txt).split('\n') if not l.lstrip().startswith('#')))
            cand = {x for x in re.findall(r'([A-Za-z_]\w*)(?!\w|\s*::)', re.sub(r'\b(use|pub)\b', ' ', bt)) if x not in ('crate', 'super', 'self')}
            if cand & ids: out.append(txt)
    return out

def write(slug_dir, title, selected, f):
    lines = []
    for u in selected:
        seg = list(f['src'].lines[u.first:u.last + 1])
        seg[u.code - u.first] = re.sub(r'^(\s*)(?:pub(?:\([^)]*\))?\s+)?(?=(?:async\s+)?(?:fn|struct|enum|const|static|type)\b)', r'\1pub ', seg[u.code - u.first]) if u.kind != 'impl' and u.kind != 'thread_local!' else seg[u.code - u.first]
        lines += seg + ['']
    body = '\n'.join(lines).rstrip() + '\n'
    ids = set(re.findall(r'[A-Za-z_]\w*', blank(body)))
    head = [f'//! {title}', '//!', '//! Lifted verbatim out of the Shell (RENDERER-WP4, plan 5.2): visibility widened to `pub`, bodies unchanged. `contracts` mounts it with one `mod` line; the Shell keeps its originals until then and re-exports from `contracts` afterwards.', '']
    text = '\n'.join(head + uses_for(ids) + ['', body])
    p = CONTRACTS + '/' + slug_dir + '/🦀️.rs'
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, 'w', encoding='utf-8').write(text)
    print(p[len(R) + 1:][-70:], len(text.split('\n')))

find_names = {'ShellFindItem', 'SHELL_FIND_ITEM_CAPACITY', 'SHELL_FIND_PAYLOAD_BYTES', 'ShellFindItems', 'ActiveShellFindItems', 'ShellFindItemBinding', 'try_push_find_item'}
f = F['🔖️identity-and-types']
sel = [u for u in f['units'] if (u.name in find_names) or (u.kind == 'thread_local!' and 'ActiveShellFindItems' in '\n'.join(f['src'].lines[u.code:u.last + 1]))]
write('🔖️identity-and-types', '🔎️ The shell find items: the bounded, thread-bound list of palette find rows with its owner binding.', sel, f)
g = F['🧱️chrome-paint-and-navbar']
write('🧱️chrome-paint-and-navbar', '🔗️ The hub remote status the footer pill and the hub projection share.', [u for u in g['units'] if u.name == 'ShellHubRemoteV1'], g)
