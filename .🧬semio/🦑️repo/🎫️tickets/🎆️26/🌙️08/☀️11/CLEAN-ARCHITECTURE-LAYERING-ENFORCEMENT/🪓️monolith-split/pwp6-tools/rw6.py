import sys; sys.path.insert(0, '.')
from sig6 import *
from comps6 import COMPONENTS, field_table, COMP_RANK, PLAN2FIELD

class MInfo:
    def __init__(self, name, cf=None, params=(), recv=None, is_async=False, trait=None, vis='', file=None):
        self.name, self.cf, self.params, self.recv, self.is_async, self.trait, self.vis, self.file = name, cf, tuple(params), recv, is_async, trait, vis, file
    def __repr__(self): return 'MInfo(%s cf=%s params=%s recv=%s)' % (self.name, self.cf, self.params, self.recv)

class World:
    def __init__(self, methods, comp_generics):
        ft = field_table()
        self.field_cf = {n: d['cf'] for n, d in ft.items()}
        self.cf_fields = collections.defaultdict(set)
        for n, cf in self.field_cf.items():
            if cf: self.cf_fields[cf].add(n)
        self.methods = methods                 # name -> MInfo (inherent preferred)
        self.comp_generics = comp_generics     # cf -> ['A', 'M']
        self.comp_type = {cf: v[0] for cf, v in COMPONENTS.items()}
        self.flags = []
    def flag(self, msg): self.flags.append(msg)

def comp_path(w, cf):
    g = w.comp_generics[cf]
    return w.comp_type[cf] + ('::<' + ', '.join(g) + '>' if g else '')

def param_ref_composer(recv, p, mut):
    if p == 'app': return ('&mut ' if mut else '&') + recv + '.app'
    if p == 'mounted_policy': return '&' + recv + '.mounted_policy'
    return ('&mut ' if mut else '&') + recv + '.' + p

def rewrite(raw, w, mode, own=None, params=(), recv='self', gens=(), selfpath=True):
    """mode 'composer': recv is a VcsArtifactApp place; mode 'component': recv=self is the component `own`, `params` are lower-component variables"""
    M = mask(raw)
    w.selfpath = selfpath
    return _rw(raw, M, 0, len(raw), w, mode, own, set(params), recv, set(gens))

def _find_call_args(M, raw, p):
    """p = index just after the callee name; returns (turbofish_text, open_paren_idx) or None when not a call"""
    q = p
    while q < len(M) and M[q].isspace(): q += 1
    turbo = ''
    if M.startswith('::', q):
        r = q + 2
        while M[r].isspace(): r += 1
        if M[r] == '<':
            e = match_angle(M, r)
            turbo = raw[q:e + 1]; q = e + 1
            while q < len(M) and M[q].isspace(): q += 1
    if q < len(M) and M[q] == '(': return turbo, q
    return None

def _rw(raw, M, lo, hi, w, mode, own, params, recv, gens):
    rx_recv = re.compile(r'(?<![\w])(?<![^.]\.)' + re.escape(recv) + r'\s*\.\s*(\w+)')
    rx_self = re.compile(r'(?<![\w:])(?<![^.]\.)(?:Self|VcsArtifactApp)\s*::\s*(\w+)')
    out = []; i = lo
    while i < hi:
        m1 = rx_recv.search(M, i, hi); m2 = rx_self.search(M, i, hi) if (w.selfpath or True) else None
        if m2 is not None and not w.selfpath and not M[m2.start():m2.end()].startswith('VcsArtifactApp'): m2 = None
        cands = [m for m in (m1, m2) if m]
        if not cands: out.append(raw[i:hi]); break
        m = min(cands, key=lambda x: x.start())
        out.append(raw[i:m.start()])
        name = m.group(1); is_recv = m is m1
        call = _find_call_args(M, raw, m.end())
        if is_recv and call is None:
            out.append(_field(raw, w, mode, own, params, recv, name, m, out)); i = m.end(); continue
        if call is None:
            info = w.methods.get(name)
            txt_ = raw[m.start():m.end()]
            if info is not None and info.cf and not is_recv and info.recv is None:
                out.append(comp_path(w, info.cf) + '::' + name)
            elif info is not None and info.cf:
                w.flag('path-value use of converted method %s' % name); out.append(txt_)
            elif info is not None and mode == 'component' and not is_recv:
                w.flag('component body names composer-level Self::%s as a value' % name); out.append(txt_)
            else: out.append(txt_)
            i = m.end(); continue
        turbo, op = call
        cp = match_close(M, op, '(', ')')
        info = w.methods.get(name)
        args = [(a, b) for a, b in split_args(M, raw, op + 1, cp)]
        if (not is_recv) and info is not None and info.recv is not None:
            # UFCS `Self::m(self, rest..)`: the first argument is the receiver
            first = raw[args[0][0]:args[0][1]].strip() if args else ''
            if first in ('self', '&mut self', '&self') or first == recv:
                args = args[1:]; is_recv = True
            else:
                w.flag('UFCS call Self::%s(%s...) with a foreign receiver' % (name, first[:30]))
        if info is None or info.cf is None:
            # not converted: keep the call, but its arguments still need rewriting
            new_args = [_rw(raw, M, a, b, w, mode, own, params, recv, gens) for a, b in args]
            if is_recv:
                if info is None and name not in w.known_composer: w.flag('unknown callee %s.%s' % (recv, name))
                if mode == 'component': w.flag('component body calls composer-level method %s' % name)
                head = raw[m.start():m.start() + (m.end() - m.start())]
                out.append(head + turbo + '(' + ','.join(new_args) + ')')
            else:
                if info is not None and info.recv is None and info.cf is None: pass
                out.append(raw[m.start():m.end()] + turbo + '(' + ','.join(new_args) + ')')
            i = cp + 1; continue
        new_args = [_rw(raw, M, a, b, w, mode, own, params, recv, gens).strip() for a, b in args]
        mut = info.recv in ('mut', 'val')
        pre = []
        for p in info.params:
            if mode == 'composer': pre.append(param_ref_composer(recv, p, mut))
            else:
                if p not in params and p not in ('app', 'mounted_policy'):
                    w.flag('component body lacks param %s for %s' % (p, name))
                if p == 'app' and 'app' not in params: w.flag('component body lacks core `app` for %s' % name)
                pre.append(p)
        callee_ref = None
        if info.recv is None:
            callee_ref = comp_path(w, info.cf) + '::' + name
        elif mode == 'composer': callee_ref = recv + '.' + info.cf + '.' + name
        else: callee_ref = ('self' if info.cf == own else info.cf) + '.' + name
        if mode == 'composer':
            hoist = bool(pre) and any(re.search(r'(?<![\w.])' + re.escape(recv) + r'\b', a) for a in new_args)
        else:
            vars_ = sorted(set(params) | {'self'})
            hoist = bool(pre) and any(re.search(r'(?<![\w.])(?:' + '|'.join(map(re.escape, vars_)) + r')\b', a) for a in new_args)
        if hoist:
            lets = ' '.join('let __a%d = %s;' % (k, a) for k, a in enumerate(new_args))
            call_txt = callee_ref + turbo + '(' + ', '.join(pre + ['__a%d' % k for k in range(len(new_args))]) + ')'
            out.append('({ ' + lets + ' ' + call_txt + ' })')
        else:
            out.append(callee_ref + turbo + '(' + ', '.join(pre + new_args) + ')')
        i = cp + 1
    return ''.join(out)

def _field(raw, w, mode, own, params, recv, name, m, out):
    cf = w.field_cf.get(name, 'MISSING')
    txt = raw[m.start():m.end()]
    if cf == 'MISSING':
        return txt                                          # not a field of the composer: a method value or an unrelated member
    if mode == 'composer':
        if cf is None: return txt
        return re.sub(r'^(' + re.escape(recv) + r'\s*\.\s*)', lambda mm: mm.group(1) + cf + '.', txt)
    # component mode
    if cf is None:
        if name == 'app':
            prev = ''.join(out)[-8:]
            if 'app' not in params: w.flag('component body reads self.app without a core param')
            return '*app' if re.search(r'&\s*(mut\s+)?$', prev) else 'app'
        if name == 'mounted_policy':
            if 'mounted_policy' not in params: w.flag('component body reads self.mounted_policy without a param')
            return 'mounted_policy'
        w.flag('component body reads composer-kept field %s' % name)
        return txt
    if cf == own: return txt
    if cf not in params:
        w.flag('component body reads field %s of component %s that is not a parameter' % (name, cf))
        return txt
    return cf + '.' + name
