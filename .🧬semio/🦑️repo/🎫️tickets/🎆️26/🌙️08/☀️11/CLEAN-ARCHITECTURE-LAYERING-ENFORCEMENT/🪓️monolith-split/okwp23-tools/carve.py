# 🔧 Output builder of one carve batch: new texts of every file the batch touches (crate files, staging files, split sources, kernel mount shims). Pure: reads the live tree, returns {path: text}; apply.py writes with guards.
import os, re, sys, collections, hashlib
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
import ns, model, registry, assign, plan as P, rewrite as RW, widen
import okusetree as U
from okrs import tokenize, items

MARK = re.compile(r'^\s*//#(end)?region\b(.*)$')
FE0F = '️'

def ident_of(key): return 'semio_framework_' + key.replace('-', '_')

def lines_of(text): return text.split('\n')

def leading_markers(item_text):
    """region marker lines that sit in the leading comment block of an item (before its first code token)"""
    out = []
    for l in item_text.split('\n'):
        s = l.strip()
        if MARK.match(l): out.append(l)
        elif s.startswith('//') or s == '' or s.startswith('#['):
            if s.startswith('#['): break
            continue
        else: break
    return out

def strip_markers(item_text):
    ls = item_text.split('\n'); out = []; lead = True
    for l in ls:
        s = l.strip()
        if lead and MARK.match(l): continue
        if not (s == '' or s.startswith('//')): lead = False
        out.append(l)
    return '\n'.join(out)

def region_stack_before(src, offset):
    st = []
    for l in src[:offset].split('\n'):
        m = MARK.match(l)
        if m:
            if m.group(1):
                if st: st.pop()
            else: st.append(m.group(2).strip())
    return st

def idents_in(text):
    return {t[1] for t in tokenize(text) if t[0] == 'id'}

class Batch:
    def __init__(self, crates, relocate=()):
        registry.load()
        self.plan = P.Plan(crates, relocate=relocate)
        self.rw = RW.Rewriter(self.plan)
        self.out = {}          # path -> new text
        self.new_files = set()
        self.notes = []

    # ------------------------------------------------------------------ helpers
    def relpath(self, frm_file, to_file):
        return os.path.relpath(to_file, os.path.dirname(frm_file))

    def module_decl(self, mod_item_text_attrs, name, relpath, vis='pub'):
        return f'{mod_item_text_attrs}#[path = "{relpath}"]\n{vis} mod {name};'

    # ------------------------------------------------------------------ main files
    def build(self):
        pl = self.plan
        self.notes = []
        for f, crate in sorted(pl.file_crate.items()):
            self.build_file(f, crate)
        for (f, crate), st in sorted(pl.staging.items(), key=lambda kv: kv[1].path):
            self.build_staging(st)
        self.build_roots()
        self.build_relocated()
        self.fix_kernel_refs()
        return self.out

    def fix_kernel_refs(self):
        """kernel files (outside the batch) that name relocated test modules by their old path follow the move"""
        pl = self.plan
        if not pl.relocate: return
        self.rw.kernel_fix = True
        try:
            for f, mod in pl.w.files.items():
                if f in pl.file_crate or f in pl.relocated_files or f in self.out: continue
                src = open(f, encoding='utf8').read()
                if not any(name in src for name in ('tests', 'reader', 'borrowed', 'native_source')): continue
                self.rw.cur_file = f
                new = self.rw.rewrite_text(src, mod, RW.KERNEL, False, model.short(f) + ' [kernel-fix]')
                if new != src: self.out[f] = new
        finally:
            self.rw.kernel_fix = False

    def is_test_file(self, f):
        m = self.plan.w.files.get(f)
        while m is not None:
            if any('test' in c for c in m.cfg): return True
            m = m.parent
        return False

    def build_relocated(self):
        pl = self.plan
        for T in sorted(pl.relocate):
            m = pl.w.files[T]; host = m.parent
            hn = host.name or os.path.basename(os.path.dirname(host.file))
            pl.reloc_name[T] = f'{hn}_{m.name}'
        for T in sorted(pl.relocate):
            self.rw.cur_file = T
            mod = pl.w.files[T]
            src = open(T, encoding='utf8').read()
            self.out[T] = self.rw.rewrite_text(src, mod, RW.KERNEL, False, model.short(T) + ' [relocated]')

    def parent_in_same_crate(self, f, crate):
        m = self.plan.w.files[f]
        return m.parent is not None and self.plan.file_crate.get(m.parent.file) == crate

    def build_file(self, f, crate):
        pl = self.plan; w = pl.w
        self.rw.cur_file = f
        mod = w.files[f]
        src = open(f, encoding='utf8').read()
        relative_ok = f != pl.root_file[crate] and self.parent_in_same_crate(f, crate)
        moved = [it for it, c in pl.moved_items.get(f, []) if c != crate] if f in pl.moved_items else []
        its = items(src)
        cut_starts = {it['start'] for it in moved}
        for it in its:
            if it['kind'] == 'mod':
                ch = mod.children.get(it['name'])
                if ch is not None and ch.key == ch.file and pl.file_crate.get(ch.file) not in (None, crate):
                    cut_starts.add(it['start'])
                elif ch is not None and ch.key == ch.file and pl.file_crate.get(ch.file) is None and ch.file in pl.relocated_files:
                    cut_starts.add(it['start'])
        pieces = []; last = 0
        for it in its:
            if it['start'] in cut_starts:
                pieces.append(src[last:it['start']])
                lm = leading_markers(it['text'])
                pieces.append('\n'.join(lm) + ('\n' if lm else ''))
                last = it['end']
        pieces.append(src[last:])
        body = ''.join(pieces) if cut_starts else src
        new = self.rw.rewrite_text(body, mod, crate, relative_ok, model.short(f))
        if moved:
            new = self.add_moved_imports(new, f, crate, moved)
        self.out[f] = new

    def add_moved_imports(self, text, f, crate, moved):
        pl = self.plan
        names_by = collections.defaultdict(list)
        used = idents_in(re.sub(r'(?m)^\s*(pub(\([^)]*\))?\s+)?use\b[^;]*;', '', text))
        for it in moved:
            c = it['crate']
            st = pl.staging.get((f, c))
            if not st: continue
            if it['kind'] in ('impl', 'use', 'mod', None): continue
            nm = it['name']
            if not nm: continue
            if f in pl.file_crate and self.rw.rank(c) >= self.rw.rank(pl.file_crate[f]): continue
            vis = 'pub' if re.match(r'\s*pub\s+(?!\()', widen.vis_text(it['text'])) else 'pub(crate)'
            if f not in pl.file_crate: names_by[(c, st.modname, 'pub' if vis == 'pub' else 'pub(crate)')].append(nm)
            elif nm in used or widen.vis_text(it['text']).startswith('pub'): names_by[(c, st.modname, 'pub' if widen.vis_text(it['text']).startswith('pub') else 'private')].append(nm)
            if f not in pl.file_crate or nm in used: self.rw.rep.need_pub.add((f, nm, f))
        lines = []
        this = pl.file_crate.get(f)
        for (c, modname, vis), names in names_by.items():
            names = sorted(set(names))
            path = (f'crate::{modname}' if this == c else ident_of(c) + '::' + modname)
            v = '' if vis == 'private' else vis + ' '
            lines.append(f'{v}use {path}::{{{", ".join(names)}}};')
            if this != c: self.rw.rep.cross[(this, c)] += 1
        if not lines: return text
        its = items(text)
        pos = 0
        for it in its:
            if it['kind'] == 'use': pos = it['end']
        if pos == 0:
            ls = text.split('\n'); i = 0
            while i < len(ls) and (ls[i].startswith('//!') or not ls[i].strip()): i += 1
            return '\n'.join(ls[:i]) + '\n' + '\n'.join(lines) + '\n' + '\n'.join(ls[i:])
        return text[:pos] + '\n' + '\n'.join(lines) + text[pos:]

    # ------------------------------------------------------------------ staging
    def build_staging(self, st):
        pl = self.plan
        self.rw.cur_file = st.src_file
        f = st.src_file; src = open(f, encoding='utf8').read()
        mod = pl.w.files[f]
        its = items(src)
        sel = {it['start']: it for it in st.items}
        uses = [self.strip_pub_use(it) for it in its if it['kind'] in ('use', 'extern')]
        blocks = []
        run = []
        def flush():
            if not run: return
            first = run[0]
            stack = region_stack_before(src, first['start'] + (len(first['text']) - len(first['text'].lstrip('\n'))))
            texts = [strip_markers(it['text']) for it in run]
            body = '\n\n'.join(texts)
            head = ''.join(f'//#region {n}\n' for n in stack)
            tail = ''.join(f'//#endregion {n}\n' for n in reversed(stack))
            blocks.append(head + body + '\n' + tail)
            run.clear()
        order = [it for it in its if it['start'] in sel]
        prev = None
        for it in order:
            if run and not self.adjacent(its, run[-1], it): flush()
            run.append(it)
        flush()
        body = '\n'.join(blocks)
        pruned = self.prune_uses([it['text'] for it in uses], body)
        sib = self.sibling_imports(st, its, body)
        combined = '\n'.join(pruned + sib) + '\n' + body
        combined = '\n'.join(l for l in combined.split('\n') if not l.lstrip().startswith('//!'))
        rewritten = self.rw.rewrite_text(combined, mod, st.crate, False, model.short(f) + ' [staging]', in_place=False)
        name = mod.name or os.path.basename(os.path.dirname(f))
        header = f'//! 📥️ Items of `{name}` that belong to `{st.crate}` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `{os.path.relpath(f, ns.M)}`.\n'
        text = header + rewritten.rstrip('\n') + '\n'
        self.out[st.path] = text
        self.new_files.add(st.path)

    def sibling_imports(self, st, its, body):
        """staged code names items that stayed in (or went from) the same source file: import them from where they landed"""
        pl = self.plan
        f = st.src_file; mod = pl.w.files[f]
        fm = model.file_model(f)
        ids = idents_in(body)
        mine = {x['start'] for x in st.items}
        lines = []
        done = set()
        for it in fm.items:
            if it['kind'] not in widen.DEF_KINDS or not it['name'] or it['start'] in mine: continue
            nm = it['name']
            if nm not in ids or nm in done: continue
            c = it['crate'] if it['crate'] in pl.batch else None
            keeper = pl.file_crate.get(f)
            loc_crate = c if (c and c != keeper) else keeper
            if loc_crate is None: continue
            if c and c != keeper and (f, c) in pl.staging: path_mod = pl.staging[(f, c)].modname; chain = [path_mod]
            else: chain = pl.mod_chain(f) or []
            if loc_crate == st.crate and chain == [st.modname]: continue
            done.add(nm)
            path = (['crate'] if loc_crate == st.crate else [ident_of(loc_crate)]) + chain + [nm]
            if loc_crate != st.crate:
                self.rw.rep.cross[(st.crate, loc_crate)] += 1
                self.rw.rep.need_pub.add((f, nm, f))
            lines.append('use ' + '::'.join(path) + ';')
        return lines

    def strip_pub_use(self, it):
        text = it['text']
        toks = tokenize(text)
        k = widen._code_start(toks)
        if k < len(toks) and toks[k][1] == 'pub':
            j = k + 1
            while j < len(toks) and toks[j][0] == 'ws': j += 1
            e = k
            if j < len(toks) and toks[j][1] == '(':
                d = 0; e = j
                while e < len(toks):
                    if toks[e][1] == '(': d += 1
                    elif toks[e][1] == ')':
                        d -= 1
                        if d == 0: break
                    e += 1
            else: e = k
            end = toks[e][3]
            while end < len(text) and text[end] in ' \t': end += 1
            text = text[:toks[k][2]] + text[end:]
        d2 = dict(it); d2['text'] = text
        return d2

    def adjacent(self, its, a, b):
        ia = its.index(a); ib = its.index(b)
        between = its[ia + 1:ib]
        return all(x['kind'] in (None,) and not x['text'].strip().startswith('#') and 'region' in x['text'] or x['kind'] is None and not x['text'].strip() for x in between) if between else True

    def fix_self_refs(self, code, st):
        return code

    def prune_uses(self, use_texts, code):
        body_ids = idents_in(code)
        out = []
        for t in use_texts:
            m = re.search(r'^(.*?\buse\b)\s*(.*?);\s*$', t.strip(), re.S)
            if not m: out.append(t); continue
            head, tree = m.group(1), m.group(2)
            if re.match(r'^\s*pub\b', head) is None:
                leaves = U.parse(tree)
                keep = []
                for path, alias, glob in leaves:
                    if glob: keep.append((path, alias, glob)); continue
                    local = alias or (path[-1] if path[-1] != 'self' else path[-2])
                    if local == '_' or local in body_ids: keep.append((path, alias, glob))
                if not keep: continue
                if len(keep) != len(leaves):
                    lines = U.emit(keep)
                    attrs = head[:head.rindex('use')]
                    out.extend(attrs + l for l in lines)
                    continue
            out.append(t.strip())
        return out

    # ------------------------------------------------------------------ crate roots
    def build_roots(self):
        pl = self.plan
        for crate in pl.batch:
            root = pl.root_file[crate]
            text = self.out.get(root)
            if text is None and os.path.exists(root): text = open(root, encoding='utf8').read()
            if text is None:
                self.notes.append(('root-missing', crate, root)); continue
            decls = []
            for f, c in sorted(pl.file_crate.items()):
                if c != crate or f == root: continue
                m = pl.w.files[f]
                if m.parent is not None and pl.file_crate.get(m.parent.file) == crate: continue
                decls.append(self.root_decl(f, m, crate))
            for (sf, c), st in sorted(pl.staging.items(), key=lambda kv: kv[1].path):
                if c != crate: continue
                decls.append(f'#[path = "{self.relpath(root, st.path)}"]\npub mod {st.modname};')
            text = self.insert_decls(text, decls)
            self.out[root] = text

    def root_decl(self, f, m, crate):
        pl = self.plan
        parent = m.parent
        attrs = ''
        if parent is not None:
            pit = None
            for it in items(open(parent.file, encoding='utf8').read()):
                if it['kind'] == 'mod' and it['name'] == m.name: pit = it; break
            if pit:
                cfgs = [a for a in pit['attrs'] if re.match(r'#\s*\[\s*cfg', a)]
                attrs = ''.join(a + '\n' for a in cfgs)
        rel = self.relpath(pl.root_file[crate], f)
        return f'{attrs}#[path = "{rel}"]\npub mod {m.name};'

    def insert_decls(self, text, decls):
        if not decls: return text
        lines = text.split('\n'); i = 0
        while i < len(lines) and (lines[i].startswith('//!') or lines[i].startswith('#![') or not lines[i].strip() or (lines[i].startswith('//') and not lines[i].startswith('///') and i == 0)): i += 1
        # a leading inner-attribute may span lines: keep going while inside `#![ ... ]`
        j = 0
        while j < len(lines):
            s = lines[j]
            if s.startswith('#!['):
                d = s.count('[') - s.count(']'); k = j
                while d > 0 and k + 1 < len(lines):
                    k += 1; d += lines[k].count('[') - lines[k].count(']')
                j = k + 1; i = max(i, j); continue
            if s.startswith('//!') or not s.strip(): j += 1; i = max(i, j); continue
            break
        return '\n'.join(lines[:i]) + '\n' + '\n'.join(decls) + '\n' + '\n'.join(lines[i:])

    # ------------------------------------------------------------------ kernel references, widening, shims
    def kernel_refs(self):
        """defs of the batch referenced by name from files that stay in the kernel (reached through the mount shims)"""
        import edges
        pl = self.plan; w = pl.w
        marks = set()
        for f, mod in w.files.items():
            if f in pl.file_crate: continue
            try: fm = model.file_model(f)
            except Exception: continue
            for it in fm.items:
                if it['kind'] in ('mod', None, 'extern'): continue
                seen = set()
                for segs, pos in edges.name_uses(edges.item_tokens(fm, it)) if it['kind'] != 'use' else self.use_paths(it):
                    key = tuple(segs[:6])
                    if key in seen: continue
                    seen.add(key)
                    r = ns.resolve(mod, segs)
                    if r.kind == 'def' and r.mod.file in pl.file_crate: marks.add((r.mod.file, r.name))
        return marks

    def use_paths(self, it):
        body = re.search(r'\buse\b(.*);', ''.join(x[1] for x in tokenize(it['text']) if x[0] != 'comment'), re.S)
        if not body: return []
        out = []
        for path, alias, glob in U.parse(body.group(1)):
            if path: out.append((list(path), 0))
        return out

    CENSUS_CFG = '#[cfg(any(test, feature = "hot-path-census"))]'

    def census(self):
        """the test-only hot-path census of the store root moves to os-store-ledger behind the `hot-path-census` feature (ledger and replay count into it)"""
        pl = self.plan
        if 'os-store-ledger' not in pl.batch: return
        root = self.component_file()
        text = self.out.get(root) or open(root, encoding='utf8').read()
        its = items(text)
        mi = [i for i in its if i['kind'] == 'mod' and i['name'] == 'hot_path_census']
        if not mi: return
        it = mi[0]
        code = it['text']
        a = code.index('mod hot_path_census')
        open_i = code.index('{', a)
        body = code[open_i + 1: code.rindex('}')]
        docs = [l.strip()[3:].strip() for l in code[:code.index('#[cfg(test)]')].split('\n') if l.strip().startswith('///')]
        lines = [l[4:] if l.startswith('    ') else l for l in body.strip('\n').split('\n')]
        newp = pl.node['os-store-ledger'] + '/🧮️hot-path-census/🦀️.rs'
        header = '//! ' + (docs[0] if docs else '🧮️ Test-only census of whole-history work on the mutation path.') + '\n' + ''.join('//! ' + d + '\n' for d in docs[1:])
        header += '//! Compiled for the crate\'s own unit tests and, behind the `hot-path-census` feature, for the kernel tests that count whole-history work across crates.\n'
        self.out[newp] = header + '\n' + '\n'.join(lines).replace('pub(crate) fn count', 'pub fn count') + '\n'
        self.new_files.add(newp)
        repl = '#[cfg(test)]\npub(crate) use semio_framework_os_store_ledger::hot_path_census;'
        self.out[root] = text[:it['start']] + repl + text[it['end']:]
        self.census_decl = f'{self.CENSUS_CFG}\n#[path = "🧮️hot-path-census/🦀️.rs"]\npub mod hot_path_census;'
        for crate, old, new in (('os-store-ledger', '#[cfg(test)] use super::hot_path_census;', ''),
                                ('os-store-replay', '#[cfg(test)] use crate::hot_path_census;', self.CENSUS_CFG + ' use semio_framework_os_store_ledger::hot_path_census;')):
            if crate not in pl.batch: continue
            f = pl.root_file[crate]
            x = self.out[f]
            assert x.count(old) == 1, (crate, 'census import')
            x = x.replace(old, new)
            x, n = re.subn(r'#\[cfg\(test\)\](\s*)hot_path_census::count', lambda m: self.CENSUS_CFG + m.group(1) + 'hot_path_census::count', x)
            self.out[f] = x
        self.rw.rep.violations = [v for v in self.rw.rep.violations if 'hot_path_census' not in v[2]]
        if 'os-store-ledger' in pl.batch:
            rf = pl.root_file['os-store-ledger']
            self.out[rf] = self.insert_decls(self.out[rf], [self.census_decl])

    def finish(self):
        pl = self.plan; rep = self.rw.rep
        self.census()
        want = collections.defaultdict(set)     # file -> names to widen
        for (f, name, key) in rep.need_pub: want[f].add(name)
        for (f, name) in self.kernel_refs(): want[f].add(name)
        self.widened = collections.Counter()
        for f, names in want.items():
            todo = set(names)
            targets = [f] + [st.path for (sf, c), st in pl.staging.items() if sf == f]
            for tp in targets:
                if tp not in self.out: continue
                self.out[tp], done = widen.widen_items(self.out[tp], todo, kinds=widen.DEF_KINDS)
                self.widened['items'] += len(done)
        for f in rep.need_pub_mod:
            m = pl.w.files[f]
            P = m.parent
            if P is not None and P.file in self.out and pl.file_crate.get(P.file) == pl.file_crate.get(f):
                self.out[P.file] = widen.widen_mod_decls(self.out[P.file], {m.name})
        for crate in pl.batch:
            for f, c in pl.file_crate.items():
                if c == crate: self.out[f] = widen.widen_use_vis(self.out[f])
            for (sf, c), st in pl.staging.items():
                if c == crate: self.out[st.path] = widen.widen_use_vis(self.out[st.path])
        self.widen_members()
        self.build_shims()

    def widen_members(self):
        pl = self.plan
        per_file = {}
        def uses_of(path):
            if path not in per_file:
                try: per_file[path] = widen.member_uses(open(path, encoding='utf8').read() if path not in self.out else self.out[path])
                except Exception: per_file[path] = set()
            return per_file[path]
        kernel_files = [f for f in pl.w.files if f not in pl.file_crate]
        for crate in pl.batch:
            outside = set()
            for f in kernel_files: outside |= uses_of(f)
            for f, c in pl.file_crate.items():
                if c != crate: outside |= uses_of(f)
            for (sf, c), st in pl.staging.items():
                if c != crate: outside |= uses_of(st.path)
            for f, c in pl.file_crate.items():
                if c != crate: continue
                self.out[f], n = widen.widen_members(self.out[f], outside); self.widened['members'] += n
            for (sf, c), st in pl.staging.items():
                if c != crate: continue
                self.out[st.path], n = widen.widen_members(self.out[st.path], outside); self.widened['members'] += n

    def build_shims(self):
        pl = self.plan
        by_parent = collections.defaultdict(list)
        for f, c in pl.file_crate.items():
            m = pl.w.files[f]
            P = m.parent
            if P is None or pl.file_crate.get(P.file) == c: continue
            if P.file in pl.file_crate: continue          # parent inside the batch (other crate): the decl was cut by build_file
            by_parent[P.file].append((f, c, m))
        for pf, lst in by_parent.items():
            text = self.out.get(pf) or open(pf, encoding='utf8').read()
            its = items(text)
            edits = []
            for f, c, m in lst:
                dec = next((it for it in its if it['kind'] == 'mod' and it['name'] == m.name), None)
                if dec is None:
                    self.notes.append(('decl-missing', pf, m.name)); continue
                if dec['start'] in {e[0] for e in edits}: continue
                chain = pl.mod_chain(f)
                target = ident_of(c) + ''.join('::' + s for s in chain)
                vis = widen_vis(dec['text'])
                extra = [(st.crate, st.modname) for (sf, sc), st in sorted(pl.staging.items(), key=lambda kv: kv[1].path) if sf == f and st.crate != c]
                raw = self.rw.rep.shim_extra.get(f, [])
                attrs = ''.join(a + '\n' for a in dec['attrs'] if re.match(r'#\s*\[\s*cfg', a))
                lead = '\n'.join(leading_markers(dec['text']))
                lead = lead + '\n' if lead else ''
                if extra or raw:
                    body = ['    pub use ' + target + '::*;'] + ['    pub use ' + ident_of(xc) + '::' + xm + '::*;' for xc, xm in extra] + ['    ' + r for r in raw]
                    code = f'{attrs}{vis}mod {m.name} {{\n' + '\n'.join(body) + '\n}'
                else:
                    last = chain[-1] if chain else None
                    code = f'{attrs}{vis}use {target}' + ('' if last == m.name else f' as {m.name}') + ';'
                edits.append((dec['start'], dec['end'], lead + code))
            for s, e, r in sorted(edits, reverse=True): text = text[:s] + r + text[e:]
            self.out[pf] = text
        if pl.relocate:
            root = self.component_file()
            text = self.out.get(root) or open(root, encoding='utf8').read()
            block = ['', '// 🧪️ Test modules of the carved crates that need test doubles the kernel root still owns; they moved here from their host modules (PLAN-OS-KERNEL-CRATES.md, 9.2) until the companion test crates land.']
            for T in sorted(pl.relocate):
                block.append('#[cfg(test)]\n#[path = "' + os.path.relpath(T, os.path.dirname(root)) + '"]\nmod ' + pl.reloc_name[T] + ';')
            self.out[root] = text.rstrip('\n') + '\n' + '\n'.join(block) + '\n'

    def component_file(self):
        return ns.S + '/🦀️.rs'

def widen_vis(decl_text):
    toks = tokenize(decl_text)
    k = widen._code_start(toks)
    if toks[k][1] == 'pub':
        j = k + 1
        while j < len(toks) and toks[j][0] == 'ws': j += 1
        if j < len(toks) and toks[j][1] == '(':
            d = 0; e = j
            while e < len(toks):
                if toks[e][1] == '(': d += 1
                elif toks[e][1] == ')':
                    d -= 1
                    if d == 0: break
                e += 1
            return decl_text[toks[k][2]:toks[e][3]] + ' '
        return 'pub '
    return ''

def solve(crates, relocate=(), auto=True):
    """Build a batch, relocating test files that need kernel-owned test doubles until no violation names a test file."""
    reloc = set(relocate)
    b = Batch(crates, relocate=sorted(reloc))
    b.build(); b.finish()
    for _ in range(6):
        more = {v[0] for v in b.rw.rep.violations if v[0] and b.is_test_file(v[0]) and v[0] in b.plan.file_crate}
        more |= {f for f in b.rw.rep.upward_files if b.is_test_file(f) and f in b.plan.file_crate}
        for (cf, nm) in b.rw.rep.upward_via_desc:
            stack = list(b.plan.w.files[cf].children.values())
            while stack:
                c = stack.pop(); stack.extend(c.children.values())
                if c.key == c.file and c.file in b.plan.file_crate and b.is_test_file(c.file) and nm in idents_in(open(c.file, encoding='utf8').read()): more.add(c.file)
        if not auto or not more or more <= reloc: break
        reloc |= more
        b = Batch(crates, relocate=sorted(reloc))
        b.build(); b.finish()
    return b

if __name__ == '__main__':
    import json
    dump = None
    args = sys.argv[1:]
    if '--dump' in args:
        i = args.index('--dump'); dump = args[i + 1]; del args[i:i + 2]
    b = solve([a for a in args if not a.startswith('--') and not a.endswith('.json')], auto='--no-auto-relocate' not in args)
    out = b.out
    print(len(out), 'files', dict(b.widened))
    if b.plan.relocate:
        print('relocated test files:', len(b.plan.relocate))
        for a in sorted(b.plan.relocate): print('   ', model.short(a))
    for v in b.rw.rep.violations[:80]: print('VIOLATION', v)
    print('dropped imports', len(b.rw.rep.dropped_imports)); [print('  dropped', x) for x in b.rw.rep.dropped_imports[:60]]
    print('globs explicitised', [(x[0][-60:], x[1][:12]) for x in b.rw.rep.glob_expanded])
    import okcrates
    RK = okcrates.RANK
    for (s, d), n in sorted(b.rw.rep.cross.items()):
        if s in RK and d in RK and RK.index(d) >= RK.index(s):
            print('BACK-EDGE', s, '->', d, n, b.rw.rep.cross_examples[(s, d)][:4])
    print('cross', dict(b.rw.rep.cross))
    print('externs', sorted(set(i for (c, i) in b.rw.rep.externs)))
    bad = []
    for (c, text) in sorted(b.rw.rep.extern_paths):
        segs = text.split('::')
        ok = ns.extern_exists(segs[:4])
        if ok is not True and segs[0] not in ('std', 'core', 'alloc'): bad.append((c, text, ok))
    print('extern paths checked', len(b.rw.rep.extern_paths), 'not proven', len(bad))
    for x in bad[:60]: print('  EXTERN?', x)
    if b.notes: print('notes', b.notes[:10])
    if dump:
        json.dump({'batch': b.plan.batch, 'out': out, 'new': sorted(b.new_files), 'roots': b.plan.root_file, 'fold': {st.path: sf for (sf, c), st in b.plan.staging.items()}, 'externs': sorted(f'{c}|{p}' for (c, p) in b.rw.rep.extern_paths), 'relocated': sorted(b.plan.relocate)}, open(dump, 'w'), ensure_ascii=False)
