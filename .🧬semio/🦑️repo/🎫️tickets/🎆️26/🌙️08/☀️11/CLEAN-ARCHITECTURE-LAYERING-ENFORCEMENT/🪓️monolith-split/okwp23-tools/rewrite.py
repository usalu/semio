# ✍️ Token-level path rewriter of the WP2/WP3 carve: rewrites `crate::`/`super::` paths and `use` trees of a file to their post-state locations; collects violations and visibility needs.
import os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
import ns, model
import okusetree as U
from okrs import tokenize, items

KERNEL_ALIASES = {'semio_framework_os_kernel', 'store', 'vcs', 'dsl', 'spr', 'semio_format', 'dsl_grammar'}
LEAD_KW = {'use', 'as', 'in', 'pub', 'where', 'for', 'impl', 'dyn', 'return', 'mut', 'ref', 'let', 'match', 'if', 'else', 'move', 'loop', 'while', 'break', 'const', 'static', 'mod', 'fn', 'struct', 'enum', 'type', 'trait', 'unsafe', 'async', 'await', 'extern', 'crate'}

def ident_of(key): return 'semio_framework_' + key.replace('-', '_')

DELETE = object()
KERNEL = '<kernel>'

class Report:
    def __init__(self):
        self.violations = []     # (file, path, reason)
        self.need_pub = set()    # (def file, name)
        self.need_pub_mod = set()  # (file-module file) whose mount must be pub
        self.externs = collections.Counter()   # (crate, ident) first-party/extern idents used per crate
        self.cross = collections.Counter()     # (src crate, dst crate)
        self.glob_expanded = []  # (file, names)
        self.dropped_imports = []
        self.upward_files = set()
        self.shim_extra = collections.defaultdict(list)
        self.upward_via_desc = []
        self.extern_paths = collections.Counter()
        self.cross_examples = collections.defaultdict(list)
        self.unresolved = []

class Rewriter:
    def __init__(self, plan, report=None):
        self.plan = plan
        self.rep = report or Report()
        self.cur_file = None
        self.kernel_fix = False
        self.w = plan.w

    def crate_of_file(self, f): return self.plan.file_crate.get(f)

    shim_heads = True

    def shim_head_module(self, m, cur_crate):
        """a module name that reaches this file through a glob and is only used as a path head: a glob-shim module, or a file module that moved to another crate"""
        if self.is_glob_shim(m): return True
        return m.key == m.file and self.plan.file_crate.get(m.file) not in (None, cur_crate)

    @staticmethod
    def is_glob_shim(m):
        """an inline module that only re-exports other crates through globs: `pub mod replay { pub use a::*; pub use b::c::*; }`"""
        return m.key != m.file and not m.defs and not m.children and bool(m.globs) and not m.uses

    def post_path(self, r, cur_crate, ctx_file, commit=True):
        """-> (text, info) for a resolved path result r; None when it cannot be expressed"""
        if r.kind == 'extern':
            return '::'.join(r.extern), 'extern'
        if r.kind == ns.EXTERN_GLOB:
            cand = ns.disambiguate(r)
            if not cand: return None, 'extern-glob-unresolved'
            return '::'.join(cand[0]), 'extern'
        if r.kind in ('def', 'mod'):
            if cur_crate == KERNEL and r.mod.file in self.plan.relocated_files:
                top = r.mod
                while top.parent is not None and not (top.key == top.file and top.file in self.plan.relocate): top = top.parent
                nm = self.plan.reloc_name.get(top.file)
                if nm:
                    rel = r.mod.path()[len(top.path()):]
                    segs = ['super', nm] + rel + ([r.name] if r.kind == 'def' else []) + list(r.rest)
                    return '::'.join(segs), 'relocated'
            if r.kind == 'def': loc = self.plan.location(r.mod, r.name, r.item)
            else: loc = self.plan.location(r.mod)
            if loc is None and r.kind == 'mod' and not r.mod.defs and not r.mod.children and len(r.mod.globs) == 1:
                g = ns.resolve(r.mod, r.mod.globs[0][0])
                if g.kind == 'extern' and len(g.extern) == 1:
                    return '::'.join(g.extern + list(r.rest)), 'extern'
            if loc is None:
                return None, ('kernel' if cur_crate == KERNEL else 'outside-batch')
            crate, chain, name = loc
            segs = list(chain) + ([name] if name else []) + list(r.rest)
            if crate == cur_crate:
                return '::'.join(['crate'] + segs), 'same'
            if commit:
                if self.rank(crate) >= self.rank(cur_crate) and self.cur_file: self.rep.upward_files.add(self.cur_file)
                self.rep.cross[(cur_crate, crate)] += 1
                if len(self.rep.cross_examples[(cur_crate, crate)]) < 6: self.rep.cross_examples[(cur_crate, crate)].append((ctx_file[-60:], '::'.join([ident_of(crate)] + segs)[-90:]))
                if r.kind == 'def': self.rep.need_pub.add((r.mod.file, r.name, r.mod.key))
                self.mark_mod_chain(r.mod, crate)
            return '::'.join([ident_of(crate)] + segs), 'cross'
        return None, r.kind

    def mark_mod_chain(self, m, crate):
        cur = m
        while cur is not None:
            if cur.key == cur.file and cur.parent is not None and self.plan.file_crate.get(cur.file) == crate and cur.file != self.plan.root_file[crate]:
                self.rep.need_pub_mod.add(cur.file)
            cur = cur.parent

    # ------------------------------------------------------------------ one source text
    def rewrite_text(self, src, ctx, cur_crate, relative_ok, file_label, moved_names=None, in_place=True):
        """src: text of items (a file body or a subset); ctx: ns.Mod the text belongs to (pre-state); returns new text."""
        self._in_place = in_place
        toks = tokenize(src)
        sig = [i for i, t in enumerate(toks) if t[0] not in ('ws', 'comment')]
        edits = []   # (start, end, replacement)
        n = len(sig)
        # inline module context: stack of (depth_at_open, Mod)
        stack = []
        depth = 0
        k = 0
        def cur_mod():
            return stack[-1][1] if stack else ctx
        pending_mod = None
        while k < n:
            i = sig[k]; kd, tx, a, b = toks[i]
            if kd == 'punct':
                if tx == '{':
                    depth += 1
                    if pending_mod is not None:
                        cm = cur_mod().children.get(pending_mod)
                        stack.append((depth, cm if cm is not None else cur_mod())); pending_mod = None
                elif tx == '}':
                    if stack and stack[-1][0] == depth: stack.pop()
                    depth -= 1
                elif tx == ';': pending_mod = None
                k += 1; continue
            if kd != 'id':
                k += 1; continue
            if tx == 'mod' and k + 2 < n and toks[sig[k + 1]][0] == 'id':
                pending_mod = toks[sig[k + 1]][1]
                k += 2; continue
            if tx == 'use':
                prev = toks[sig[k - 1]][1] if k else ''
                # statement start: previous token one of ; { } ] pub ) or beginning
                end = k + 1; d = 0
                while end < n:
                    t2 = toks[sig[end]]
                    if t2[1] == '{': d += 1
                    elif t2[1] == '}': d -= 1
                    elif t2[1] == ';' and d == 0: break
                    end += 1
                a0 = toks[i][2]; b0 = toks[sig[end]][3]
                tree_src = src[toks[sig[k + 1]][2]:toks[sig[end]][2]] if end > k + 1 else ''
                is_pub = any(toks[sig[q]][1] == 'pub' for q in range(max(0, k - 6), k) if toks[sig[q]][1] in ('pub',)) and not any(toks[sig[q]][1] in (';', '}', '{') for q in range(max(0, k - 6), k) if sig[q] > 0 and False)
                prevs = []
                q = k - 1
                while q >= 0 and toks[sig[q]][1] not in (';', '{', '}') and len(prevs) < 8:
                    prevs.append(toks[sig[q]][1]); q -= 1
                is_pub = 'pub' in prevs
                rep = self.rewrite_use(tree_src, cur_mod(), cur_crate, relative_ok, file_label, src, toks, sig, is_pub)
                if rep is DELETE:
                    ks = k
                    qs = k - 1
                    while qs >= 0 and toks[sig[qs]][1] not in (';', '{', '}'): qs -= 1
                    ks = qs + 1
                    edits.append((toks[sig[ks]][2], toks[sig[end]][3], ''))
                elif isinstance(rep, list):
                    prefix = self.stmt_prefix(src, toks, sig, k)
                    body = (';\n' + prefix + 'use ').join(rep)
                    edits.append((toks[sig[k + 1]][2], toks[sig[end]][2], body))
                elif rep is not None: edits.append((toks[sig[k + 1]][2], toks[sig[end]][2], rep))
                k = end + 1; continue
            if tx in ('crate', 'super') or tx in KERNEL_ALIASES or tx == 'self':
                lead = k >= 2 and toks[sig[k - 1]][1] == ':' and toks[sig[k - 2]][1] == ':'
                if lead:
                    # `::head`: leading only when the token before the `::` pair is not an identifier / closing bracket
                    pre = toks[sig[k - 3]] if k >= 3 else None
                    cont = pre is not None and ((pre[0] == 'id' and pre[1] not in LEAD_KW) or pre[1] in ('>', ')', ']'))
                    if cont: k += 1; continue
                if tx in KERNEL_ALIASES and not lead and not (k + 2 < n and toks[sig[k + 1]][1] == ':' and toks[sig[k + 2]][1] == ':'):
                    k += 1; continue
                if k > 0 and toks[sig[k - 1]][1] == '$' : k += 1; continue
                if tx in KERNEL_ALIASES:
                    prevtok = toks[sig[k - 1]][1] if k else ''
                    if prevtok == '.' : k += 1; continue
                # collect the path
                segs = [tx]; j = k + 1; last = k
                while j + 2 <= n - 1 and toks[sig[j]][1] == ':' and toks[sig[j + 1]][1] == ':' and toks[sig[j + 2]][0] == 'id':
                    segs.append(toks[sig[j + 2]][1]); last = j + 2; j += 3
                if len(segs) == 1 and tx != 'self':
                    k += 1; continue
                start_tok = sig[k - 2] if lead else sig[k]
                end_tok = sig[last]
                start_pos = toks[start_tok][2]; end_pos = toks[end_tok][3]
                new = self.rewrite_path(segs, cur_mod(), cur_crate, relative_ok, file_label, lead)
                if new is not None and new != src[start_pos:end_pos]:
                    edits.append((start_pos, end_pos, new))
                k = last + 1; continue
            if kd == 'id' and k + 2 < n and toks[sig[k + 1]][1] == ':' and toks[sig[k + 2]][1] == ':':
                prevtok = toks[sig[k - 1]][1] if k else ''
                before2 = toks[sig[k - 2]][1] if k >= 2 else ''
                if prevtok not in ('.', '$', "'") and not (prevtok == ':' and before2 == ':'):
                    cm = cur_mod().children.get(tx)
                    if cm is None and cur_crate != KERNEL and self.shim_heads:
                        lk = ns.lookup(cur_mod(), tx)
                        if lk is not None and lk.kind == 'mod' and not lk.rest and self.shim_head_module(lk.mod, cur_crate):
                            segs = [tx]; j = k + 1; last = k
                            while j + 2 <= n - 1 and toks[sig[j]][1] == ':' and toks[sig[j + 1]][1] == ':' and toks[sig[j + 2]][0] == 'id':
                                segs.append(toks[sig[j + 2]][1]); last = j + 2; j += 3
                            new = self.rewrite_path(segs, cur_mod(), cur_crate, relative_ok, file_label)
                            if new is not None:
                                edits.append((toks[sig[k]][2], toks[sig[last]][3], new))
                            k = last + 1; continue
                    if cm is not None and cm.key == cm.file:
                        cc = self.plan.file_crate.get(cm.file)
                        if cc is not None and (cc != cur_crate or not self._in_place):
                            segs = [tx]; j = k + 1; last = k
                            while j + 2 <= n - 1 and toks[sig[j]][1] == ':' and toks[sig[j + 1]][1] == ':' and toks[sig[j + 2]][0] == 'id':
                                segs.append(toks[sig[j + 2]][1]); last = j + 2; j += 3
                            new = self.rewrite_path(segs, cur_mod(), cur_crate, relative_ok, file_label)
                            if new is not None:
                                edits.append((toks[sig[k]][2], toks[sig[last]][3], new))
                            k = last + 1; continue
            k += 1
        out = src
        for s, e, r in sorted(edits, reverse=True):
            out = out[:s] + r + out[e:]
        return out

    def reloc_rel_path(self, ctx, r):
        """path from module ctx (a kernel module) to a definition / module that moved into a relocated test module under the store root"""
        top = r.mod
        while top.parent is not None and not (top.key == top.file and top.file in self.plan.relocate): top = top.parent
        nm = self.plan.reloc_name.get(top.file)
        if not nm: return None
        target = ['os_store', 'component', nm] + r.mod.path()[len(top.path()):] + ([r.name] if r.kind == 'def' else []) + list(r.rest)
        here = ctx.path()
        k = 0
        while k < len(here) and k < len(target) - 1 and here[k] == target[k]: k += 1
        return '::'.join(['super'] * (len(here) - k) + target[k:]) if len(here) - k > 0 else '::'.join(['self'] + target[k:])

    def rewrite_path(self, segs, ctx, cur_crate, relative_ok, file_label, lead=False):
        head = segs[0]
        if self.kernel_fix:
            r = ns.resolve(ctx, segs)
            if r.kind in ('def', 'mod') and r.mod.file in self.plan.relocated_files: return self.reloc_rel_path(ctx, r)
            return None
        if head == 'self': return None
        if head == 'super' and relative_ok:
            n = 0
            while n < len(segs) and segs[n] == 'super': n += 1
            anc = ctx
            for _ in range(n):
                anc = anc.parent if anc is not None else None
            if anc is not None and self.crate_of_file(anc.file) == cur_crate and self.module_unchanged(anc, cur_crate):
                r = ns.resolve(ctx, segs)
                if r.kind in ('def', 'mod'):
                    loc = self.plan.location(r.mod, r.name, r.item) if r.kind == 'def' else self.plan.location(r.mod)
                    if loc and loc[0] == cur_crate: return None
                elif r.kind in ('extern', ns.EXTERN_GLOB):
                    return None
        r = ns.resolve(ctx, segs)
        text, info = self.post_path(r, cur_crate, file_label)
        if text is None and info == 'kernel':
            return self.kernel_path(segs)
        if text is None:
            self.rep.violations.append((self.cur_file, file_label, '::'.join(segs), info, str(r)[:140]))
            return None
        if info == 'extern':
            self.rep.externs[(cur_crate, text.split('::')[0])] += 1
            self.rep.extern_paths[(cur_crate, text)] += 1
        return text

    def upward(self, text, info, cur_crate):
        """does the resolved path leave the crate through a first-party crate that is not strictly below it in the rank order?"""
        if text is None or info not in ('cross', 'extern'): return False
        k = self.crate_in(text)
        import okcrates
        return (k.startswith('os-store-') or k == 'os-io') and k in okcrates.RANK and self.rank(k) >= self.rank(cur_crate)

    def crate_in(self, text):
        head = text.split('::')[0]
        return head[len('semio_framework_'):].replace('_', '-') if head.startswith('semio_framework_') else head

    def rank(self, key):
        import okcrates
        return okcrates.RANK.index(key) if key in okcrates.RANK else 99

    def kernel_rel_path(self, r):
        """path of a kernel definition / module as seen from a module that is a child of the store root"""
        if r.kind == 'def': names = r.mod.path() + [r.name]
        elif r.kind == 'mod': names = r.mod.path()
        else: return None
        names = names + list(r.rest)
        if names[:2] == ['os_store', 'component']: return '::'.join(['super'] + names[2:])
        return '::'.join(['crate'] + names)

    def kernel_path(self, segs):
        """a relocated test climbs one module less: the host module is no longer its parent"""
        if segs[0] != 'super': return None
        n = 0
        while n < len(segs) and segs[n] == 'super': n += 1
        if n < 2: return None
        return '::'.join(['super'] * (n - 1) + segs[n:])

    def module_unchanged(self, m, crate):
        return True

    def rewrite_use(self, tree_src, ctx, cur_crate, relative_ok, file_label, src, toks, sig, is_pub=False):
        body = tree_src.strip()
        if not body: return None
        leaves = U.parse(body)
        if self.kernel_fix:
            outl = []; ch = False
            for path, alias, glob in leaves:
                if path and (path[0] in ('crate', 'super', 'self') or path[0] in KERNEL_ALIASES or ctx.children.get(path[0])):
                    r = ns.resolve(ctx, list(path))
                    if r.kind in ('def', 'mod') and r.mod.file in self.plan.relocated_files:
                        np_ = self.reloc_rel_path(ctx, r)
                        if np_:
                            local = alias or path[-1]
                            outl.append((np_.split('::'), alias if alias else (None if np_.split('::')[-1] == path[-1] else local), glob)); ch = True; continue
                outl.append((path, alias, glob))
            if not ch: return None
            lines = U.emit(outl)
            if len(lines) == 1: return lines[0][len('use '):].rstrip(';')
            return [l[len('use '):].rstrip(';') for l in lines]
        changed = False
        out_leaves = []
        body_only = self.body_idents(src, toks, sig)
        used = body_only | self.descendant_idents(ctx)
        for path, alias, glob in leaves:
            head = path[0] if path else ''
            moved_child = False
            cm = ctx.children.get(head)
            if cm is not None and cm.key == cm.file:
                cc = self.plan.file_crate.get(cm.file)
                moved_child = cc is not None and (cc != cur_crate or not self._in_place)
            if not (head in ('crate', 'super') or head in KERNEL_ALIASES or moved_child):
                out_leaves.append((path, alias, glob)); continue
            segs = list(path)
            local = None if glob else (alias or (path[-1] if path[-1] != 'self' else path[-2]))
            if head == 'super' and relative_ok and self.use_relative_ok(segs, ctx, cur_crate, glob):
                out_leaves.append((path, alias, glob)); continue
            r = ns.resolve(ctx, segs)
            if glob:
                if r.kind == 'mod' and not r.rest:
                    expanded = self.explicitise_glob(r.mod, ctx, cur_crate, file_label, used, src + '\n' + self.__dict__.get('_desc_text', {}).get(ctx.file_module().file, ''), body_only)
                    if expanded is not None:
                        out_leaves.extend(expanded); changed = True; continue
                text, info = self.post_path(r, cur_crate, file_label)
                if text is None:
                    self.rep.violations.append((self.cur_file, file_label, 'use ' + '::'.join(segs) + '::*', info, str(r)[:140]))
                    out_leaves.append((path, alias, glob)); continue
                out_leaves.append((text.split('::'), alias, True)); changed = True; continue
            # module imports of a split-source module: the part that moved to this crate lives in the staging module
            if r.kind == 'mod' and not r.rest and local is not None and local not in used and local != '_' and not is_pub:
                changed = True; self.rep.dropped_imports.append((file_label, '::'.join(segs))); continue
            if r.kind == 'mod' and not r.rest:
                st = self.plan.staging.get((r.mod.file, cur_crate))
                if st is not None and r.mod.key == r.mod.file:
                    out_leaves.append((['crate', st.modname], local, False)); changed = True; continue
                other = [s for (f, c), s in self.plan.staging.items() if f == r.mod.file and c != cur_crate]
                if other and self.plan.file_crate.get(r.mod.file) is None:
                    s0 = other[0]
                    out_leaves.append(([ident_of(s0.crate), s0.modname], local, False)); changed = True; continue
            unused = local is not None and local not in used and local != '_' and not is_pub
            if unused and ((r.kind == 'def' and r.item is not None and r.item['kind'] != 'trait') or r.kind == 'mod'):
                changed = True; self.rep.dropped_imports.append((file_label, '::'.join(segs))); continue
            text, info = self.post_path(r, cur_crate, file_label, commit=False)
            if text is not None and self.upward(text, info, cur_crate) and is_pub and ctx.file == self.plan.root_file.get(cur_crate):
                changed = True; self.rep.shim_extra[ctx.file].append('pub use ' + text + (' as ' + alias if alias else '') + ';'); continue
            if text is not None and self.upward(text, info, cur_crate) and unused:
                changed = True; self.rep.dropped_imports.append((file_label, '::'.join(segs) + ' [upward]')); continue
            if text is not None and self.upward(text, info, cur_crate) and local is not None and not is_pub and local not in body_only:
                changed = True; self.rep.upward_via_desc.append((ctx.file, local)); self.rep.dropped_imports.append((file_label, '::'.join(segs) + ' [upward, descendants only]')); continue
            if text is not None: text, info = self.post_path(r, cur_crate, file_label)
            if text is None and info == 'kernel':
                kp = self.kernel_path(segs)
                if kp is not None:
                    out_leaves.append((kp.split('::'), alias, glob)); changed = True; continue
                out_leaves.append((path, alias, glob)); continue
            if text is None:
                if unused:
                    changed = True; self.rep.dropped_imports.append((file_label, '::'.join(segs))); continue
                self.rep.violations.append((self.cur_file, file_label, 'use ' + '::'.join(segs), info, str(r)[:140]))
                out_leaves.append((path, alias, glob)); continue
            if info == 'extern':
                self.rep.externs[(cur_crate, text.split('::')[0])] += 1
                self.rep.extern_paths[(cur_crate, text)] += 1
            np_ = text.split('::')
            if r.kind == 'mod' and not r.rest and len(np_) == 2 and np_[0] == 'crate' and ctx.file == self.plan.root_file.get(cur_crate) and np_[1] == local and alias is None:
                changed = True; continue
            if alias is None and np_[-1] != local and np_[-1] != 'crate':
                alias = local
            if np_ != list(path): changed = True
            out_leaves.append((np_, alias, glob))
        if not changed: return None
        if not out_leaves: return DELETE
        lines = U.emit(out_leaves)
        if len(lines) == 1: return lines[0][len('use '):].rstrip(';')
        return [l[len('use '):].rstrip(';') for l in lines]

    def descendant_idents(self, ctx):
        """identifiers of every module file mounted below ctx's file (children may rely on this module's imports through `use super::*`)"""
        fm = ctx.file_module()
        cache = self.__dict__.setdefault('_desc', {})
        if fm.file in cache: return cache[fm.file]
        ids = set(); stack = list(fm.children.values()); seen = set(); texts = []
        while stack:
            c = stack.pop()
            if c.file in seen and c.key == c.file: continue
            if c.file in self.plan.relocated_files: continue
            seen.add(c.file)
            if c.key == c.file or c.file != fm.file:
                try:
                    tx = open(c.file, encoding='utf8').read(); texts.append(tx)
                    ids |= {t[1] for t in tokenize(tx) if t[0] == 'id'}
                except Exception: pass
            stack.extend(c.children.values())
        cache[fm.file] = ids
        self.__dict__.setdefault('_desc_text', {})[fm.file] = '\n'.join(texts)
        return ids

    def stmt_prefix(self, src, toks, sig, k):
        """attributes and visibility written before the `use` keyword at sig[k], as text"""
        q = k - 1
        while q >= 0 and toks[sig[q]][1] not in (';', '{', '}'): q -= 1
        q += 1
        parts = []
        while q < k:
            tx = toks[sig[q]][1]
            if tx == '#':
                s = toks[sig[q]][2]; d = 0; e = q + 1
                while e < k:
                    if toks[sig[e]][1] == '[': d += 1
                    elif toks[sig[e]][1] == ']':
                        d -= 1
                        if d == 0: break
                    e += 1
                parts.append(src[s:toks[sig[e]][3]]); q = e + 1; continue
            if tx == 'pub':
                s = toks[sig[q]][2]; e = q
                if q + 1 < k and toks[sig[q + 1]][1] == '(':
                    d = 0; e = q + 1
                    while e < k:
                        if toks[sig[e]][1] == '(': d += 1
                        elif toks[sig[e]][1] == ')':
                            d -= 1
                            if d == 0: break
                        e += 1
                parts.append(src[s:toks[sig[e]][3]]); q = e + 1; continue
            q += 1
        return ''.join(x + ' ' for x in parts)

    def body_idents(self, src, toks, sig):
        """identifier tokens outside `use` statements"""
        ids = set(); n = len(sig); k = 0
        while k < n:
            t = toks[sig[k]]
            if t[0] == 'id' and t[1] == 'use':
                d = 0; e = k + 1
                while e < n:
                    x = toks[sig[e]][1]
                    if x == '{': d += 1
                    elif x == '}': d -= 1
                    elif x == ';' and d == 0: break
                    e += 1
                k = e + 1; continue
            if t[0] == 'id': ids.add(t[1])
            k += 1
        return ids

    def explicitise_glob(self, tmod, ctx, cur_crate, file_label, used, src, body_only=None):
        """names the file really uses out of a glob whose target module is not the unchanged parent -> explicit leaves"""
        local_defs = set(ctx.file_module().defs) | set(ctx.file_module().uses) | set(ctx.file_module().children)
        leaves = []; names = []
        for nm in sorted(used):
            if nm in local_defs or nm in ('self', 'super', 'crate', 'Self') or nm.startswith('_'): continue
            if not (nm[0].isupper() or nm in self._lower_hits(src, nm)): continue
            r = ns.lookup(tmod, nm)
            if r is None: continue
            if self.shim_heads and r.kind == 'mod' and not r.rest and self.shim_head_module(r.mod, cur_crate) and nm not in self._non_head_uses(src, nm): continue
            if r.kind == ns.EXTERN_GLOB:
                cand = ns.disambiguate(r)
                if not cand: continue
                text = '::'.join(cand[0]); info = 'extern'
            else:
                text, info = self.post_path(r, cur_crate, file_label, commit=False)
                if text is not None and info == 'cross' and body_only is not None and nm not in body_only and self.rank(self.crate_in(text)) >= self.rank(cur_crate):
                    self.rep.upward_via_desc.append((ctx.file, nm)); continue
                if text is not None: text, info = self.post_path(r, cur_crate, file_label)
            if text is None and info == 'kernel':
                kp = self.kernel_rel_path(r)
                if kp: text, info = kp, 'kernel-rel'
            if text is None and info == 'outside-batch' and body_only is not None and nm not in body_only:
                self.rep.upward_via_desc.append((ctx.file, nm)); continue
            if text is None:
                self.rep.violations.append((self.cur_file, file_label, 'glob-name ' + nm, info, str(r)[:120])); continue
            np_ = text.split('::')
            if info == 'extern': self.rep.externs[(cur_crate, np_[0])] += 1
            leaves.append((np_, None if np_[-1] == nm else nm, False)); names.append(nm)
        self.rep.glob_expanded.append((file_label, [n for n in names]))
        return leaves

    def _non_head_uses(self, src, nm):
        """{nm} when the identifier occurs anywhere but as the head of a `nm::...` path (a bare use of a module name)"""
        for m in re.finditer(r'(?<![\w.:])' + re.escape(nm) + r'\b(?!\s*::)', src):
            line_start = src.rfind('\n', 0, m.start()) + 1
            if src[line_start:m.start()].lstrip().startswith(('//', 'use ', 'pub use ')): continue
            return {nm}
        return set()

    def _lower_hits(self, src, nm):
        return {nm} if re.search(r'(?<![\w.])' + re.escape(nm) + r'\s*(\(|!|::)', src) else set()

    def _multi(self, lines):
        # several `use` statements are not expressible inside one statement: emit a brace group of all leaves
        parts = []
        for l in lines:
            parts.append(l[len('use '):].rstrip(';'))
        return '{' + ', '.join(parts) + '}'

    def use_relative_ok(self, segs, ctx, cur_crate, glob):
        n = 0
        while n < len(segs) and segs[n] == 'super': n += 1
        anc = ctx
        for _ in range(n): anc = anc.parent if anc is not None else None
        if anc is None or self.crate_of_file(anc.file) != cur_crate: return False
        r = ns.resolve(ctx, segs)
        if r.kind in ('def', 'mod'):
            loc = self.plan.location(r.mod, r.name, r.item) if r.kind == 'def' else self.plan.location(r.mod)
            return bool(loc and loc[0] == cur_crate)
        return r.kind in ('extern', ns.EXTERN_GLOB)
