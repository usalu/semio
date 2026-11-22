import re, os, sys
V = '️'
S = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/'
E = S + '⚙️engine/📥️lifted/🏪️store/'
N = S + '⚙️engine/🔭️derived-replay/'
LIFTED = E + '🦀️.rs'
REPORT = S + '🔁️replay/🎮️operation/🧬️native/🔗️history/📜️report/🦀️.rs'
OPER = S + '🔁️replay/🎮️operation/🌱️derived/🧩️store/🦀️.rs'
CKPT = E + '📸️checkpoint-read/🦀️.rs'
for p in (N, N + '🔭️history-preview/', N + '📜️report-replay/', N + '🧩️operation/', N + '📸️checkpoint-read/'):
    os.makedirs(p, exist_ok=True)

L = open(LIFTED).read().split('\n')
start = next(i for i, l in enumerate(L) if 'pub fn finished_history_preview_read_demands' in l)
end = next(i for i, l in enumerate(L) if l.startswith('    pub fn projection_event'))
region = L[start:end]
imports = [l for l in L if re.match(r'^(#\[cfg\(target_arch = "wasm32"\)\]|use )', l) and 'use semio_os_store_engine_owners::DocumentStoreOwners' not in l]

item_start = re.compile(r'^    (///|#\[|(pub(\([a-z]+\))? )?(async )?fn )')
items = []
cur = None
prev_doc = False
for l in region:
    m = item_start.match(l)
    is_pre = bool(m) and (l.startswith('    ///') or l.startswith('    #[') and 'fn ' not in l)
    if m and not prev_doc:
        cur = []
        items.append(cur)
    if cur is None:
        continue
    cur.append(l)
    prev_doc = is_pre
items = [i for i in items if any(re.search(r'\bfn \w+', x) for x in i)]
def head(it):
    t = '\n'.join(it)
    return t[:t.index('{', t.index(' fn '))]
def name(it):
    return re.search(r'\bfn (\w+)', '\n'.join(it)).group(1)

def to_crate(txt):
    return re.sub(r'^(\s+)(pub\(super\)|pub\(crate\)|) ?fn ', lambda m: m.group(1) + 'pub fn ', txt, flags=re.M)

def split_params(sig):
    a = sig[sig.index('(', sig.index(' fn ')) + 1:]
    depth = 1; out = []; cur = ''
    for c in a:
        if c in '([<{': depth += 1
        elif c in ')]>}':
            if c == '>' and cur.endswith('-'): cur += c; continue
            depth -= 1
            if depth == 0: break
        if c == ',' and depth == 1: out.append(cur); cur = ''
        else: cur += c
    if cur.strip(): out.append(cur)
    return out

def args(sig):
    r = []
    for p in split_params(sig):
        p = p.strip()
        if 'self' in p.split(':')[0]: continue
        r.append(re.match(r'(?:mut\s+)?(\w+)\s*:', p).group(1))
    return ', '.join(r)

def impl_h(m, d):
    return f"impl<'a, P, {m}, Log> StoreDerived<'a, P, {m}, Log>\nwhere\n    P: semio_os_spr::StoreState,\n    {m}: semio_os_spr::StoreMutationFor<P>,\n    Log: std::ops::{d}<Target = StoreLog<P, {m}>>,\n{{"

def blocks(its, join='\n\n'):
    rd = [i for i in its if '&mut self' not in head(i)]
    wr = [i for i in its if '&mut self' in head(i)]
    out = ''
    if rd: out += impl_h('Mutation','Deref') + '\n' + '\n'.join('\n'.join(i) for i in rd) + '\n}\n'
    if wr: out += '\n' + impl_h('Mutation','DerefMut') + '\n' + '\n'.join('\n'.join(i) for i in wr) + '\n}\n'
    return out

keep_helpers = ('admit_derived_source', 'seal_derived_snapshot', 'applied_revision', 'applied_edit', 'step_history_read_plan', 'live_base', 'convergence_checkpoints')
tail_start = next(i for i, l in enumerate(L) if l.startswith('/// ⛽️ Computational work'))
tail = L[tail_start:]
fpr = tail[:next(i for i, l in enumerate(tail) if l.startswith('#[path="📸️'))]
helper_src = '\n'.join(L[next(i for i, l in enumerate(L) if 'fn admit_derived_source' in l) - 0: start])
helpers = re.findall(r'(    fn (?:admit_derived_source|seal_derived_snapshot|applied_revision|applied_edit|step_history_read_plan|live_base|convergence_checkpoints)\b.*?\n    }\n)', helper_src, flags=re.S)
helpers = [h.replace('    fn ', '    pub fn ', 1) for h in helpers]

body_lifted = '//! 🔭️ Derived history previews and report replays borrow the Store frontier without moving its cursor.\nuse super::*;\n\n' + impl_h('Mutation','Deref') + '\n' + '\n'.join(helpers) + '\n}\n\n' + blocks([i for i in items]) + '\n' + '\n'.join(fpr) + '\n'
body_lifted = to_crate(body_lifted)
open(N + '🔭️history-preview/🦀️.rs', 'w').write(body_lifted)

# operation
t = open(OPER).read()
t = re.sub(r'impl<P,M> ArtifactStore<P,M>\nwhere [^\n]*\{', impl_h('M','Deref'), t)
t = to_crate(t.replace('(crate) fn', ' fn'))
open(N + '🧩️operation/🦀️.rs', 'w').write(t)

# checkpoint
t = open(CKPT).read()
t = re.sub(r'impl<P,Mutation> ArtifactStore<P,Mutation>\nwhere [^\n]*\{', impl_h('Mutation','Deref'), t)
t = to_crate(t)
open(N + '📸️checkpoint-read/🦀️.rs', 'w').write(t)

# report
t = open(REPORT).read().split('\n')
hi = next(i for i, l in enumerate(t) if l.startswith('impl<P,M> ArtifactStore<P,M>'))
body = t[hi + 1:-2] if t[-1] == '' else t[hi + 1:-1]
cut = next(i for i, l in enumerate(body) if 'fn close_original_report_replacement' in l)
# docs preceding close fn
while body[cut - 1].lstrip().startswith('///'): cut -= 1
rd, wr = body[:cut], body[cut:]
out = '\n'.join(t[:hi]) + '\n' + impl_h('M','Deref') + '\n' + '\n'.join(rd) + '\n}\n' + impl_h('M','DerefMut') + '\n' + '\n'.join(wr) + '\n}\n'
out = to_crate(out)
open(N + '📜️report-replay/🦀️.rs', 'w').write(out)

# delegates in engine
dl = []
for i in items:
    h = head(i)
    n = name(i)
    if not re.search(r'^\s*pub fn ', h, flags=re.M): continue
    mut = '&mut self' in h
    docs = [x for x in i if x.startswith('    ///')]
    sig = h.rstrip()
    dl.append('    /// 🪞️ Delegates to [`StoreDerived::%s`].\n    #[inline] %s { self.%s().%s(%s) }\n' % (n, re.sub(r'^\s*(#\[[^\]]*\]\s*)*', '', sig).strip(), 'derived_mut' if mut else 'derived', n, args(sig)))
open(N + 'delegates.rs.txt', 'w').write('\n'.join(dl))
print(len(items), [name(i) for i in items])
