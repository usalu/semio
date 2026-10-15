#!/usr/bin/env python3
import re, sys, os, json, subprocess, hashlib
from collections import Counter

ROOT = '/Users/ueli/Documents/semio'
PIN = '793f635'
TOOLS = os.path.dirname(os.path.abspath(__file__))
def canon(name):
    return name.split(' \u2014 ')[0].strip()


MARK = re.compile(r'^(\s*)//\s*#(end)?region\b[ \t]*(.*?)\s*$')


def head_files():
    return set(open(os.path.join(TOOLS, '..', '🗑️generated', 'regrestore-headfiles.txt'), 'rb').read().decode('utf-8').split('\n'))


HEAD = None


TEXTFIX = [('\xad\u0192\u00ac\u00aeRepl', '\U0001faa9\ufe0fRepl'), ('\xad\u0192\u00ba\u00acTests', '\U0001f9ea\ufe0fTests')]


def orig_text(path):
    t = orig_text_raw(path)
    for a, b in TEXTFIX:
        t = t.replace(a, b)
    return t


def orig_text_raw(path):
    r = subprocess.run(['git', '-C', ROOT, 'show', f'{PIN}:{path}'], capture_output=True)
    if r.returncode != 0:
        raise SystemExit(f'git show failed for {path}: {r.stderr.decode()}')
    return r.stdout.decode('utf-8')


def read(p):
    with open(p, encoding='utf-8', newline='') as f:
        return f.read()


class Orig:
    def __init__(self, text, label=''):
        self.label = label
        lines = text.split('\n')
        self.code = []
        self.stack_at = []
        self.inst = []
        self.anom = []
        self.mk = []
        st = []
        for n, ln in enumerate(lines):
            m = MARK.match(ln.rstrip('\r'))
            if m:
                if not m.group(2):
                    inst = dict(id=len(self.inst), name=canon(m.group(3)), open_raw=ln.strip(), close_raw=None,
                                parent=st[-1] if st else None, line=n + 1, first=None, last=None,
                                indent=m.group(1))
                    self.inst.append(inst)
                    self.mk.append(dict(kind='open', inst=inst['id'], after=len(self.code) - 1))
                    st.append(inst['id'])
                else:
                    nm = canon(m.group(3))
                    k = None
                    if nm == '' and st:
                        k = len(st) - 1
                    for q in range(len(st) - 1, -1, -1):
                        if k is None and self.inst[st[q]]['name'] == nm:
                            k = q
                            break
                    if k is None:
                        self.anom.append(f'stray close {nm!r} at O line {n + 1}')
                    else:
                        if k != len(st) - 1:
                            self.anom.append(f'close {nm!r} at {n + 1} skips open {[self.inst[x]["name"] for x in st[k + 1:]]}')
                        self.mk.append(dict(kind='close', inst=st[k], after=len(self.code) - 1))
                        self.inst[st[k]]['close_raw'] = ln.strip()
                        self.inst[st[k]]['close_line'] = n + 1
                        self.inst[st[k]]['close_after'] = len(self.code) - 1
                        del st[k:]
                continue
            self.code.append(ln.strip())
            self.stack_at.append(tuple(st))
            c = len(self.code) - 1
            for i in st:
                if self.inst[i]['first'] is None:
                    self.inst[i]['first'] = c
                self.inst[i]['last'] = c
        for i in st:
            self.anom.append(f'unclosed open {self.inst[i]["name"]!r} at O line {self.inst[i]["line"]}')
        self.implicit_close(st)
        for inst in self.inst:
            a, b = inst['first'], inst['last']
            inst['hollow'] = a is None or all(self.code[q] == '' for q in range(a, b + 1))
            if inst['close_raw'] is None:
                inst['close_raw'] = inst['open_raw'].replace('#region', '#endregion', 1)
                inst['synth_close'] = True
        self.count = Counter(self.code)
        self.pos = {t: j for j, t in enumerate(self.code) if self.count[t] == 1}

    def names(self):
        return {i['name'] for i in self.inst}

    def implicit_close(self, chain):
        k = len(chain)
        for idx, iid in enumerate(chain):
            U = self.inst[iid]
            if idx < k - 1:
                nxt = self.inst[chain[idx + 1]]
                open_mk = next(m for m in self.mk if m['kind'] == 'open' and m['inst'] == nxt['id'])
                end = open_mk['after']
                if all(self.code[q] == '' for q in range(U['first'] if U['first'] is not None else 0, end + 1)) or U['first'] is None:
                    continue
                nxt['parent'] = U['parent']
            else:
                kids = sorted([i for i in self.inst if i['parent'] == iid and i.get('close_after') is not None], key=lambda x: x['line'])
                if not kids:
                    continue
                end = kids[0]['close_after']
                for kd in kids[1:]:
                    if all(self.code[q] == '' for q in range(end + 1, kd['first'] if kd['first'] is not None else end + 1)):
                        end = kd['close_after']
                    else:
                        break
            for j in range(end + 1, len(self.code)):
                self.stack_at[j] = tuple(x for x in self.stack_at[j] if x != iid)
            U['last'] = end
            U['implicit_end'] = end

    def _anc(self, child, anc):
        p = self.inst[child]['parent']
        while p is not None:
            if p == anc:
                return True
            p = self.inst[p]['parent']
        return False


class Tfile:
    def __init__(self, path, text=None):
        self.path = path
        self.text = read(path) if text is None else text
        self.eol = '\r\n' if '\r\n' in self.text.split('\n', 1)[0] + '\r\n' and self.text.find('\r\n') >= 0 else '\n'
        self.lines = self.text.split('\n')
        self.marks = []
        self.code = []
        self.fidx = []
        for n, ln in enumerate(self.lines):
            m = MARK.match(ln.rstrip('\r'))
            if m:
                self.marks.append((n, 'close' if m.group(2) else 'open', canon(m.group(3))))
            else:
                self.code.append(ln.strip())
                self.fidx.append(n)
        self.mark_lines = [n for n, _, _ in self.marks]
        self.mark_set = set(self.mark_lines)
        self.names = {nm for _, _, nm in self.marks}

    def marker_between(self, f0, f1):
        for n in self.mark_lines:
            if f0 < n < f1:
                return True
        return False


def is_blank(t):
    return t == ''


MINLEN = 8


def runs_for(O, T, claimed_other=None):
    anchors = []
    for i, t in enumerate(T.code):
        if len(t) >= MINLEN and t in O.pos:
            anchors.append((i, O.pos[t]))
    runs = []
    cur = None
    for i, j in anchors:
        d = j - i
        if cur and cur['d'] == d and j > cur['j1']:
            cur['c1'] = i
            cur['j1'] = j
            cur['n'] += 1
            cur['len'] += len(T.code[i])
        else:
            cur = dict(d=d, c0=i, c1=i, j0=j, j1=j, n=1, len=len(T.code[i]))
            runs.append(cur)
    keep = [r for r in runs if r['n'] >= 2 or r['len'] >= 40]
    merged = []
    for r in keep:
        if merged and merged[-1]['d'] == r['d'] and r['c0'] > merged[-1]['c1']:
            merged[-1]['c1'] = r['c1']
            merged[-1]['j1'] = r['j1']
            merged[-1]['n'] += r['n']
        else:
            merged.append(dict(r))
    claimed = [None] * len(T.code)
    for r in merged:
        for c in range(r['c0'], r['c1'] + 1):
            claimed[c] = r
    for r in merged:
        while r['c1'] + 1 < len(T.code) and r['j1'] + 1 < len(O.code) and claimed[r['c1'] + 1] is None \
                and T.code[r['c1'] + 1] == O.code[r['j1'] + 1]:
            r['c1'] += 1
            r['j1'] += 1
            claimed[r['c1']] = r
        while r['c0'] - 1 >= 0 and r['j0'] - 1 >= 0 and claimed[r['c0'] - 1] is None \
                and T.code[r['c0'] - 1] == O.code[r['j0'] - 1]:
            r['c0'] -= 1
            r['j0'] -= 1
            claimed[r['c0']] = r
    return merged


def segments_for(O, T):
    runs = runs_for(O, T)
    segs = []
    for r in runs:
        d = r['d']
        j = r['j0']
        while j <= r['j1']:
            st = O.stack_at[j]
            k = j
            while k + 1 <= r['j1'] and O.stack_at[k + 1] == st:
                k += 1
            c0, c1 = j - d, k - d
            while c0 <= c1 and is_blank(T.code[c0]):
                c0 += 1
            while c1 >= c0 and is_blank(T.code[c1]):
                c1 -= 1
            if c0 <= c1:
                segs.append(dict(c0=c0, c1=c1, stack=st, j0=j, j1=k))
            j = k + 1
    segs.sort(key=lambda s: (s['c0'], s['c1']))
    out = []
    for s in segs:
        if out and s['c0'] <= out[-1]['c1']:
            if (s['c1'] - s['c0']) > (out[-1]['c1'] - out[-1]['c0']):
                out[-1] = s
            continue
        out.append(s)
    return out


def pair_markers(marks, lo=None, hi=None):
    st = []
    pairs = []
    strayc = []
    strayo = []
    for n, kind, name in marks:
        if lo is not None and (n < lo or n > hi):
            continue
        if kind == 'open':
            st.append((n, name))
            continue
        k = None
        if name == '':
            k = len(st) - 1 if st else None
        else:
            for q in range(len(st) - 1, -1, -1):
                if st[q][1] == name:
                    k = q
                    break
        if k is None or k < 0:
            strayc.append((n, name))
            continue
        for q in range(k + 1, len(st)):
            strayo.append((st[q][0], st[q][1], n))
        pairs.append((st[k][0], n, st[k][1]))
        del st[k:]
    for a_, nm in st:
        strayo.append((a_, nm, None))
    return pairs, strayc, strayo


def existing_pairs(T):
    pairs, strayc, strayo = pair_markers(T.marks)
    unmatched = {n for n, _ in strayc} | {n for n, _, _ in strayo}
    return pairs, unmatched


def desc_names(O, iid):
    out = set()
    for i in O.inst:
        p = i['parent']
        while p is not None:
            if p == iid:
                out.add(i['name'])
                break
            p = O.inst[p]['parent']
    return out


def ancestor_raws(O, nm, key):
    out = set()
    for i in O.inst:
        if i['name'] == nm:
            p = i['parent']
            while p is not None:
                out.add(O.inst[p][key].strip())
                p = O.inst[p]['parent']
    return out


def local_unmatched(T, lo, hi):
    pairs, strayc, strayo = pair_markers(T.marks, lo, hi)
    return sorted([n for n, _ in strayc] + [n for n, _, _ in strayo])


def plan_file(O, T, segs, skip_names):
    ops_close = {}
    ops_open = {}
    lang = 'rs' if T.path.endswith('.rs') else 'ts'
    grp, reliable, dstart, dend = gapinfo(T.code, lang)
    T.reliable = reliable
    dropped = []
    if reliable:
        pre = []
        for s in segs:
            c0, c1 = expand_segment(grp, s['c0'], s['c1'], len(T.code))
            if s['stack'] and O.inst[s['stack'][-1]]['indent'] == '':
                c0, c1 = snap_top_level(T.code, dstart, dend, c0, c1)
            if lang == 'rs':
                while c1 + 1 < len(T.code) and T.code[c1].startswith('#['):
                    c1 += 1
                while c0 - 1 >= 0 and (T.code[c0 - 1].startswith('#[') or T.code[c0 - 1].startswith('///')):
                    c0 -= 1
            while c1 + 1 < len(T.code) and is_comment_line(T.code[c1]):
                c1 += 1
            while c0 - 1 >= 0 and is_comment_line(T.code[c0 - 1]):
                c0 -= 1
            pre.append(dict(s, c0=c0, c1=c1, o0=s['c0'], o1=s['c1']))
        pre.sort(key=lambda z: (z['o0'], z['o1']))
        fixed = []
        seen = set()
        n_ = len(T.code)
        for k, s in enumerate(pre):
            c0, c1 = s['c0'], s['c1']
            if c0 > c1:
                continue
            base = min(dstart[c0], min(dend[c0:c1 + 1]))
            c = c0
            while c > 0 and dstart[c] > base:
                c -= 1
            if dstart[c] != base:
                dropped.append((s, 'unbalanced-'))
                continue
            c0 = c
            while c0 - 1 >= 0 and (is_comment_line(T.code[c0 - 1]) or (lang == 'rs' and T.code[c0 - 1].startswith('#['))):
                c0 -= 1
            if dend[c1] != base:
                c = c1
                while c >= c0 and dend[c] != base:
                    c -= 1
                while c >= c0 and lang == 'rs' and (T.code[c].startswith('#[') or T.code[c].startswith('///')):
                    c -= 1
                if c >= c0 and c1 - c <= max(40, (c1 - c0) // 4):
                    c1 = c
                else:
                    c = c1
                    while c + 1 < n_ and dend[c] > base:
                        c += 1
                    if dend[c] == base:
                        c1 = c
                    else:
                        dropped.append((s, 'unbalanced+'))
                        continue
            key = (c0, c1, tuple(s['stack']))
            if key in seen:
                continue
            seen.add(key)
            fixed.append(dict(s, c0=c0, c1=c1))
        segs = fixed
    T.dropped = dropped
    bare_ids = {i['id'] for i in O.inst if i['close_raw'].strip() in ('//#endregion', '// #endregion') and not i.get('synth_close')}
    bare_resolved = {}
    _p, _sc, _so = pair_markers(T.marks)
    for m, nm in _sc:
        if nm != '':
            continue
        pick = None
        for sg in reversed(segs):
            if T.fidx[sg['c0']] < m:
                inner = [i for i in sg['stack'] if i in bare_ids]
                if inner:
                    pick = O.inst[inner[-1]]['name']
                    break
        if pick:
            bare_resolved[m] = pick
    skip_names = set(skip_names) | set(bare_resolved.values())
    if not hasattr(O, '_desc'):
        O._desc = {}
    pairs, _ = existing_pairs(T)
    spans = []
    for s in segs:
        stack = [i for i in s['stack'] if O.inst[i]['name'] not in skip_names]
        fo, fc = T.fidx[s['c0']], T.fidx[s['c1']]
        dn = set()
        for i in stack:
            if i not in O._desc:
                O._desc[i] = desc_names(O, i)
            dn |= O._desc[i]
        changed = True
        minfo = {n_: (k_, nm_) for n_, k_, nm_ in T.marks}
        while changed and dn:
            changed = False
            k = fo - 1
            best = fo
            while k >= 0:
                if k in minfo:
                    if minfo[k][0] == 'open' and minfo[k][1] in dn:
                        best = k
                    else:
                        break
                elif T.lines[k].strip() != '':
                    break
                k -= 1
            if best != fo:
                fo = best
                changed = True
            k = fc + 1
            best = fc
            while k < len(T.lines):
                if k in minfo:
                    kd, nm_ = minfo[k]
                    if kd == 'close' and (nm_ in dn or (nm_ == '' and bare_resolved.get(k) in dn)):
                        best = k
                    else:
                        break
                elif T.lines[k].strip() != '':
                    break
                k += 1
            if best != fc:
                fc = best
                changed = True
            for a, b, nm in pairs:
                if nm in dn:
                    if a < fo <= b:
                        fo = a
                        changed = True
                    if a <= fc < b:
                        fc = b
                        changed = True
        cuts = [n_ for n_ in local_unmatched(T, fo, fc)
                if not (minfo[n_][1] in dn or (minfo[n_][1] == '' and bare_resolved.get(n_) in dn))]
        bounds = [fo - 1] + cuts + [fc + 1]
        for k in range(len(bounds) - 1):
            x, y = bounds[k] + 1, bounds[k + 1] - 1
            while x <= y and (T.lines[x].strip() == '' or (x in T.mark_set and x in cuts)):
                x += 1
            while y >= x and (T.lines[y].strip() == '' or (y in T.mark_set and y in cuts)):
                y -= 1
            if x <= y:
                spans.append(dict(fo=x, fc=y, stack=stack, full=list(s['stack'])))
    spans.sort(key=lambda z: (z['fo'], -z['fc']))
    clean = []
    for z in spans:
        while clean and z['fo'] <= clean[-1]['fc']:
            L = clean[-1]
            if z['fc'] <= L['fc'] and len(z['stack']) > len(L['stack']):
                clean.pop()
                if L['fo'] <= z['fo'] - 1:
                    clean.append(dict(L, fc=z['fo'] - 1))
                clean.append(z)
                if z['fc'] + 1 <= L['fc']:
                    clean.append(dict(L, fo=z['fc'] + 1))
                z = None
                break
            if z['fc'] <= L['fc']:
                z = None
                break
            z = dict(z, fo=L['fc'] + 1)
        if z is not None:
            clean.append(z)
    open_list = []
    added = []

    def close_from(k, after_f):
        for q in range(len(open_list) - 1, k - 1, -1):
            iid, ind = open_list[q]
            ops_close.setdefault(after_f + 1, []).append(ind + O.inst[iid]['close_raw'])
        del open_list[k:]

    prev = None
    for z in clean:
        if prev is not None and local_unmatched(T, prev['fc'] + 1, z['fo'] - 1):
            close_from(0, prev['fc'])
        desired = z['stack']
        k = 0
        while k < len(open_list) and k < len(desired) and open_list[k][0] == desired[k]:
            k += 1
        if prev is not None:
            close_from(k, prev['fc'])
        ind = re.match(r'\s*', T.lines[z['fo']]).group(0)
        for iid in desired[k:]:
            ops_open.setdefault(z['fo'], []).append(ind + O.inst[iid]['open_raw'])
            open_list.append((iid, ind))
            added.append(iid)
        prev = z
    if prev is not None:
        close_from(0, prev['fc'])
    fixes = []
    pairs_, strayc, strayo = pair_markers(T.marks)
    for n, nm, cb in strayo:
        cand = [z for z in clean if z['fc'] >= n and (cb is None or z['fc'] < cb)
                and any(O.inst[i]['name'] == nm for i in z['full'])]
        ind = re.match(r'\s*', T.lines[n]).group(0)
        close_raw = next((O.inst[i]['close_raw'] for i in range(len(O.inst)) if O.inst[i]['name'] == nm), '//#endregion ' + nm)
        if cand:
            g = max(z['fc'] for z in cand) + 1
        elif cb is not None:
            g = cb
        else:
            g = T.fidx[-1] + 1
        moved = True
        while moved:
            moved = False
            for a_, b_, _nm in pairs_:
                if a_ >= n and a_ < g <= b_:
                    g = b_ + 1
                    moved = True
        lst = ops_close.setdefault(g, [])
        anc_raw = ancestor_raws(O, nm, 'close_raw')
        pos = len(lst)
        for q, t in enumerate(lst):
            if t.strip() in anc_raw:
                pos = q
                break
        lst.insert(pos, ind + close_raw)
        fixes.append(('close-for-open', nm, n + 1, g))
    for m, nm in strayc:
        if nm in getattr(T, 'keep_stray', ()):
            fixes.append(('kept stray (unpaired in the original)', nm, m + 1, None))
            continue
        if nm == '':
            if m not in bare_resolved:
                fixes.append(('UNRESOLVED bare open-for-close', nm, m + 1, None))
                continue
            nm = bare_resolved[m]
        cand = [z for z in clean if z['fo'] < m and any(O.inst[i]['name'] == nm for i in z['full'])]
        ind = re.match(r'\s*', T.lines[m]).group(0)
        open_raw = next((O.inst[i]['open_raw'] for i in range(len(O.inst)) if O.inst[i]['name'] == nm), '//#region ' + nm)
        if cand:
            g = min(z['fo'] for z in cand)
            moved = True
            while moved:
                moved = False
                for a_, b_, _nm in pairs_:
                    if a_ < g <= b_ and b_ < m:
                        g = a_
                        moved = True
            lst = ops_open.setdefault(g, [])
            anc_raw = ancestor_raws(O, nm, 'open_raw')
            pos = 0
            for q, t in enumerate(lst):
                if t.strip() in anc_raw:
                    pos = q + 1
            lst.insert(pos, ind + open_raw)
            fixes.append(('open-for-close', nm, m + 1, g))
        else:
            g = None
            for c, ln_ in enumerate(T.code):
                if ln_ == '' or ln_.startswith(('//', '/**', '*', '#!', 'use ', 'import ', '#![')):
                    continue
                g = T.fidx[c]
                break
            if g is not None and g < m:
                ops_open.setdefault(g, []).insert(0, ind + open_raw)
                fixes.append(('open-for-close (first item)', nm, m + 1, g))
            else:
                fixes.append(('UNRESOLVED open-for-close', nm, m + 1, None))
    return ops_close, ops_open, added, fixes


def apply_ops(T, ops_close, ops_open):
    out = []
    n = len(T.lines)
    eol_char = '\r' if T.eol == '\r\n' else ''
    for g in range(n + 1):
        for ln in ops_close.get(g, []):
            out.append(ln + eol_char)
        for ln in ops_open.get(g, []):
            out.append(ln + eol_char)
        if g < n:
            out.append(T.lines[g])
    return '\n'.join(out)


def strip_markers(text):
    return '\n'.join(l for l in text.split('\n') if not MARK.match(l.rstrip('\r')))


def check_balance(text):
    st = []
    errs = []
    names = set()
    for n, ln in enumerate(text.split('\n')):
        m = MARK.match(ln.rstrip('\r'))
        if not m:
            continue
        nm = canon(m.group(3))
        names.add(nm)
        if not m.group(2):
            st.append((nm, n + 1))
        else:
            if st and (st[-1][0] == nm or nm == ''):
                st.pop()
            else:
                errs.append(f'line {n + 1}: endregion {nm!r} does not match top {st[-1] if st else None}')
    for nm, n in st:
        errs.append(f'line {n}: unclosed region {nm!r}')
    return names, errs


def tree_files(parent, folders, ext, include=None):
    global HEAD
    if HEAD is None:
        HEAD = head_files()
    files = []
    if parent and os.path.exists(os.path.join(ROOT, parent)):
        files.append(parent)
    for fo in folders:
        base = os.path.join(ROOT, fo)
        for dp, dn, fn in os.walk(base):
            dn[:] = [d for d in dn if d not in ('node_modules', 'target', '.git') and d != '🗑️generated']
            for f in fn:
                if not f.endswith(ext):
                    continue
                rel = os.path.relpath(os.path.join(dp, f), ROOT)
                if rel in HEAD or rel in files:
                    continue
                files.append(rel)
    if include:
        for i in include:
            if i not in files and os.path.exists(os.path.join(ROOT, i)):
                files.append(i)
    return sorted(files)


IMPORT_LEAD = ('import ', 'import{', 'export {', 'export type {', 'export * ', 'export type *', 'export{',
               'use ', 'pub use ', 'pub(crate) use ', 'pub(super) use ')


def gapinfo(codelines, lang):
    n = len(codelines)
    grp = [None] * n
    dstart = [0] * n
    dend = [0] * n
    stack = []
    mode = None
    depth_bc = 0
    raw_hashes = 0
    imp = False
    last = ''
    reliable = True
    groups = []
    for c in range(n):
        ln = codelines[c]
        st = ln.strip()
        top_block = (not stack) or (not stack[-1]['nb'])
        if mode is None and top_block and st.startswith(IMPORT_LEAD):
            imp = True
        top = stack[-1] if stack else None
        dstart[c] = len(stack)
        grp[c] = top if (top and top['nb']) else None
        if mode == 'str_ts':
            mode = None
        i = 0
        L = len(ln)
        while i < L:
            ch = ln[i]
            if mode == 'bc':
                if lang == 'rs' and ln.startswith('/*', i):
                    depth_bc += 1
                    i += 2
                    continue
                if ln.startswith('*/', i):
                    depth_bc -= 1
                    i += 2
                    if depth_bc == 0:
                        mode = None
                    continue
                i += 1
                continue
            if mode == 'str':
                if ch == '\\':
                    i += 2
                    continue
                if ch == '"':
                    mode = None
                    last = '"'
                i += 1
                continue
            if mode == 'tmpl':
                if ch == '\\':
                    i += 2
                    continue
                if ch == '`':
                    mode = None
                    last = '`'
                i += 1
                continue
            if mode == 'raw':
                if ch == '"' and ln.startswith('"' + '#' * raw_hashes, i):
                    i += 1 + raw_hashes
                    mode = None
                    continue
                i += 1
                continue
            if mode == 'q':
                if ch == '\\':
                    i += 2
                    continue
                if ch == q:
                    mode = None
                    last = q
                i += 1
                continue
            if lang != 'rs' and ch == '/' and not ln.startswith('//', i) and not ln.startswith('/*', i) and (last == '' or last in '(,=:[!&|?{;'):
                j = i + 1
                incls = False
                while j < L:
                    cj = ln[j]
                    if cj == '\\':
                        j += 2
                        continue
                    if cj == '[':
                        incls = True
                    elif cj == ']':
                        incls = False
                    elif cj == '/' and not incls:
                        break
                    j += 1
                if j < L:
                    j += 1
                    while j < L and ln[j].isalpha():
                        j += 1
                    i = j
                    last = 'r'
                    continue
            if ch == '/' and ln.startswith('//', i):
                break
            if ch == '/' and ln.startswith('/*', i):
                mode = 'bc'
                depth_bc = 1
                i += 2
                continue
            if lang == 'rs':
                if ch == 'r' and (i == 0 or not (ln[i - 1].isalnum() or ln[i - 1] == '_')):
                    m = re.match(r'b?r(#*)"', ln[i:])
                    if m:
                        raw_hashes = len(m.group(1))
                        mode = 'raw'
                        i += m.end()
                        continue
                if ch == 'b' and ln.startswith('br', i) and (i == 0 or not (ln[i - 1].isalnum() or ln[i - 1] == '_')):
                    m = re.match(r'br(#*)"', ln[i:])
                    if m:
                        raw_hashes = len(m.group(1))
                        mode = 'raw'
                        i += m.end()
                        continue
                if ch == '"':
                    mode = 'str'
                    i += 1
                    continue
                if ch == "'":
                    m = re.match(r"'(\\.[^']*|[^\\'])'", ln[i:])
                    if m:
                        i += m.end()
                        last = "'"
                        continue
                    i += 1
                    continue
            else:
                if ch in '"\'':
                    mode = 'q'
                    q = ch
                    i += 1
                    continue
                if ch == '`':
                    mode = 'tmpl'
                    i += 1
                    continue
            if ch in '([{':
                nb = True
                if ch == '{':
                    block_top = (not stack) or (not stack[-1]['nb'])
                    nb = (imp and block_top) or (last in ('=', ':', ',', '(', '[', '?', '|', '&'))
                g = dict(ch=ch, start=c, end=None, nb=nb)
                stack.append(g)
                groups.append(g)
            elif ch in ')]}':
                if not stack or {')': '(', ']': '[', '}': '{'}[ch] != stack[-1]['ch']:
                    reliable = False
                else:
                    g = stack.pop()
                    g['end'] = c
            elif ch == ';':
                if imp and ((not stack) or (not stack[-1]['nb'])):
                    imp = False
            if not ch.isspace():
                last = ch
            i += 1
        dend[c] = len(stack)
        if mode == 'q':
            mode = None
    if stack or mode in ('bc', 'str', 'raw', 'tmpl'):
        reliable = False
    return grp, reliable, dstart, dend


def is_comment_line(l):
    return l.startswith(('//', '/**', '/*', '*/', '* ')) or l == '*'


def snap_top_level(code, dstart, dend, c0, c1):
    n = len(code)
    c = c0
    while c > 0 and dstart[c] > 0:
        c -= 1
    if dstart[c] == 0 and c != c0:
        c0 = c
    while c0 - 1 >= 0 and code[c0 - 1] != '' and code[c0 - 1].lstrip().startswith(('#[', '///', '//', '/**', '*', '@')):
        c0 -= 1
    c = c1
    while c < n - 1 and dend[c] > 0:
        c += 1
    if dend[c] == 0:
        c1 = c
    return c0, c1


def expand_segment(grp, c0, c1, n):
    guard = 0
    while c0 < n and grp[c0] is not None and guard < 50:
        c0 = grp[c0]['start']
        guard += 1
    guard = 0
    while c1 + 1 < n and grp[c1 + 1] is not None and guard < 50:
        e = grp[c1 + 1]['end']
        if e is None:
            break
        c1 = e
        guard += 1
    return c0, c1


def prev_nonblank(O, j):
    while j >= 0 and O.code[j] == '':
        j -= 1
    return j


def hollow_plan(O, Ts_segs):
    res = {}
    hol = [i for i in O.inst if i['hollow']]
    if not hol:
        return res
    owner = {}
    for T, segs in Ts_segs:
        for s in segs:
            for j in range(s['j0'], s['j1'] + 1):
                if O.code[j] != '' and j not in owner:
                    owner[j] = (T, s)
    byan = {}
    for i in hol:
        open_mk = next(m for m in O.mk if m['kind'] == 'open' and m['inst'] == i['id'])
        jp = prev_nonblank(O, open_mk['after'])
        byan.setdefault(jp, []).append(i['id'])
    for jp, ids in byan.items():
        lines = []
        for m in O.mk:
            if m['inst'] in ids:
                inst = O.inst[m['inst']]
                lines.append((inst['open_raw'] if m['kind'] == 'open' else inst['close_raw'], inst))
        if jp < 0 or jp not in owner:
            res.setdefault(None, []).append((jp, [t for t, _ in lines], ids))
            continue
        T, s = owner[jp]
        c = s['c0'] + (jp - s['j0'])
        res.setdefault(T.path, []).append((c, [t for t, _ in lines], ids))
    return res
