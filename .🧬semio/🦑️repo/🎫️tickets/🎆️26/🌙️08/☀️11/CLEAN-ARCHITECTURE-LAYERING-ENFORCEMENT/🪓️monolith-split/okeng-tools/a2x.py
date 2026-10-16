# 🏭 A2 driver of OS-KERNEL-ENGINE: the WP2/WP3 carve pipeline with (1) the store root as a kernel-owned split source (its items leave to the batch crates, the remainder stays), (2) kernel split sources in general (files whose main crate is outside the batch), (3) known test relocations, (4) timing and a full report.
# usage: a2x.py <name> dump|apply   (crates from okeng-tools/<name>.crates, relocations from okeng-tools/<name>.reloc)
import os, sys, json, time, re, collections
HERE = os.path.dirname(os.path.abspath(__file__))
P56 = HERE + '/../okwp56-tools'
sys.path.insert(0, P56); sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE + '/../okwp23-tools'); sys.path.insert(0, HERE)
import a2lib
from a2lib import *
import a2, mycof, apply as AP
from okrs import items, tokenize
OKD = os.path.dirname(os.path.abspath(__file__))
GEN = OKD + '/../🗑️generated'
ROOT = ns.S + '/🦀️.rs'
KERNEL_KEY = '<kernel-root>'
T0 = time.time()
def log(*a): print('[%6.0fs]' % (time.time() - T0), *a, flush=True)

class KBatch(carve.Batch):
    def fix_kernel_refs(self):
        self.build_kernel_sources()
        super().fix_kernel_refs()

    def build_kernel_sources(self):
        pl = self.plan
        for f in sorted(pl.moved_items):
            if f in pl.file_crate: continue
            self.build_kernel_source(f)

    def build_kernel_source(self, f):
        pl = self.plan; w = pl.w
        self.rw.cur_file = f
        mod = w.files[f]
        src = open(f, encoding='utf8').read()
        moved = [it for it, c in pl.moved_items.get(f, [])]
        its = items(src)
        cut_starts = {it['start'] for it in moved}
        for it in its:
            if it['kind'] == 'mod':
                ch = mod.children.get(it['name'])
                if ch is not None and ch.key == ch.file and pl.file_crate.get(ch.file) is None and ch.file in pl.relocated_files:
                    cut_starts.add(it['start'])
        pieces = []; last = 0
        for it in its:
            if it['start'] in cut_starts:
                pieces.append(src[last:it['start']])
                lm = carve.leading_markers(it['text'])
                pieces.append('\n'.join(lm) + ('\n' if lm else ''))
                last = it['end']
        pieces.append(src[last:])
        body = ''.join(pieces)
        new = self.rw.rewrite_text(body, mod, RW_KERNEL, False, model.short(f) + ' [kernel-source]')
        new = self.add_moved_imports(new, f, None, moved)
        self.out[f] = new
        log('kernel source', model.short(f)[-60:], len(src.split('\n')), '->', len(new.split('\n')), 'moved', len(moved))

import rewrite as _RW
RW_KERNEL = _RW.KERNEL
import builtins
_open0 = builtins.open
def _open_retry(*a, **k):
    for i in range(60):
        try: return _open0(*a, **k)
        except OSError as e:
            if e.errno not in (23, 24): raise
            time.sleep(2)
    return _open0(*a, **k)
builtins.open = _open_retry

_LH = {}
def _lower_hits(self, src, nm):
    key = (len(src), hash(src))
    s = _LH.get(key)
    if s is None:
        s = set(m.group(1) for m in re.finditer(r'(?<![\w.:])([A-Za-z_][A-Za-z0-9_]*)\s*(?:\(|!|::)', src))
        _LH.clear(); _LH[key] = s
    return {nm} if nm in s else set()
_RW.Rewriter._lower_hits = _lower_hits

_sib0 = carve.Batch.sibling_imports
def _sib(self, st, its, body):
    saved = {}
    fm = model.file_model(st.src_file)
    for it in fm.items:
        if it['kind'] == 'mod': saved[id(it)] = it['kind']; it['kind'] = '<mod-skip>'
    try: return _sib0(self, st, its, body)
    finally:
        for it in fm.items:
            if it['kind'] == '<mod-skip>': it['kind'] = 'mod'
carve.Batch.sibling_imports = _sib

KERNEL_HEADS = {'semio_framework_os_kernel', 'store', 'vcs', 'dsl', 'spr', 'semio_format', 'dsl_grammar'}
_resolve0 = ns.resolve
def _resolve(m, segs, depth=0):
    r = _resolve0(m, segs, depth)
    if r.kind == 'extern' and len(segs) > 1 and segs[0] in KERNEL_HEADS and (m.cw is None or m.cw is ns.world()):
        return _resolve0(ns.world().root, ['crate'] + list(segs[1:]), depth + 1)
    return r
ns.resolve = _resolve

OVERRIDES = {
    (ns.S + '/🏗️initialization/📚️catalog/🦀️.rs', 'demand'): 'os-store-initialization-runtime',
}

_mc0 = model.FileModel.main_crate
def _mc(self):
    return KERNEL_KEY if self.path == ROOT else _mc0(self)
model.FileModel.main_crate = _mc

mycof.FILE_CRATE[ns.S + '/🧰️test-support/🦀️.rs'] = 'os-store-test-support'

def prepare():
    model.OVERRIDE.update(OVERRIDES)
    a2.prepare()
    model.FILE_OVERRIDE[ROOT] = KERNEL_KEY

def read_list(path):
    return open(path).read().split() if os.path.exists(path) else []

def run(name):
    crates = read_list(f'{OKD}/{name}.crates')
    reloc = [l.strip() for l in open(f'{OKD}/{name}.reloc', encoding='utf8').read().split('\n') if l.strip()] if os.path.exists(f'{OKD}/{name}.reloc') else []
    log('prepare')
    prepare()
    log('world loaded', len(ns.world().files), 'files')
    before = {f: AP.sha(f) for f in ns.world().files}
    base_text = {f: m.text for f, m in ns.world().files.items() if m.key == m.file}
    carve.P.Plan = a2lib.PredictPlan
    carve.Batch = KBatch
    reloc_abs = [ns.R + '/' + r if not r.startswith('/') else r for r in reloc]
    b = carve.solve(crates, relocate=reloc_abs, auto=os.environ.get('A2_NOAUTO') != '1')
    b.base_text = base_text
    log('solved; relocated', len(b.plan.relocate))
    return b, crates, before

def report(b, name):
    out = []
    P = lambda *a: out.append(' '.join(str(x) for x in a))
    P(len(b.out), 'files', dict(b.widened))
    if b.plan.relocate:
        P('relocated test files:', len(b.plan.relocate))
        for a in sorted(b.plan.relocate): P('   ', a)
    for v in b.rw.rep.violations: P('VIOLATION', v)
    P('violations', len(b.rw.rep.violations))
    P('dropped imports', len(b.rw.rep.dropped_imports))
    for x in b.rw.rep.dropped_imports: P('  dropped', x)
    P('globs explicitised', [(x[0][-60:], x[1][:12]) for x in b.rw.rep.glob_expanded])
    RK = C.RANK
    for (s, d), n in sorted(b.rw.rep.cross.items(), key=lambda kv: str(kv[0])):
        if s in RK and d in RK and RK.index(d) >= RK.index(s):
            P('BACK-EDGE', s, '->', d, n, b.rw.rep.cross_examples[(s, d)])
    for (s, d), n in sorted(b.rw.rep.cross.items(), key=lambda kv: str(kv[0])):
        if s in RK and d not in RK: P('EDGE-OUT-OF-RANK', s, '->', d, n, b.rw.rep.cross_examples[(s, d)])
    P('cross', {f'{s}->{d}': n for (s, d), n in b.rw.rep.cross.items()})
    P('externs', sorted(set(i for (c, i) in b.rw.rep.externs)))
    bad = []
    for (c, text) in sorted(b.rw.rep.extern_paths):
        segs = text.split('::')
        ok = ns.extern_exists(segs[:4])
        if ok is not True and segs[0] not in ('std', 'core', 'alloc'): bad.append((c, text, ok))
    P('extern paths checked', len(b.rw.rep.extern_paths), 'not proven', len(bad))
    for x in bad: P('  EXTERN?', x)
    P('upward files', sorted(b.rw.rep.upward_files)[:40])
    P('notes', b.notes)
    open(f'{GEN}/okeng-report-{name}.txt', 'w', encoding='utf8').write('\n'.join(out) + '\n')
    log('report written', f'{GEN}/okeng-report-{name}.txt', 'violations', len(b.rw.rep.violations))

if __name__ == '__main__':
    name, mode = sys.argv[1], sys.argv[2]
    b, crates, before = run(name)
    report(b, name)
    a2.dump(b, crates, before, f'{GEN}/okeng-out-{name}.json')
    log('dump written')
    if mode == 'apply':
        AP.TOUCHED = OKD + '/touched.txt'
        plan, texts, new_k, ktext = a2.apply(b, crates, before)
        a2.write_all(b, crates, plan, texts, new_k, ktext)
        log('applied')
