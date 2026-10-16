# ✅️ WP4 static check, part two: component call sites in any text (arity, receiver mutability against the caller's access) and the field existence of `<recv>.<component>.<field>` paths.
import re, sys, os, glob, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from t1lib import *
from units import *
import verify_t1 as v1

def tables(root):
    files = sorted(glob.glob(root + '/📥️staging/*/*/🦀️.rs')) + glob.glob(root + '/*/🦀️.rs') + [root + '/🦀️.rs']
    fields = collections.defaultdict(set); methods = collections.defaultdict(dict); frees = {}
    for p in files:
        src, units = split_file(p)
        for u in units:
            if u.kind == 'struct' and u.name in STRUCTS:
                for i in range(u.code + 1, u.last):
                    m = re.match(r'^\s+pub ([A-Za-z_]\w*)\s*:', src.bl[i])
                    if m: fields[u.name].add(m.group(1))
            if u.kind == 'fn' and u.name in PLAN1 and PLAN1[u.name].get('recv') in ('free', None):
                b = '\n'.join(src.bl[u.code:u.last + 1]); sg = signature(b)
                if sg: frees[u.name] = [pp for pp in v1.sig_params(b, sg)]
            if u.kind == 'impl' and u.name in STRUCTS + ['ShellState'] and ' for ' not in u.head:
                for m in top_items(src, u.code + 1, u.last, 4):
                    if m['kind'] == 'fn':
                        nm = item_name(m['head'], 'fn')
                        b = '\n'.join(src.bl[m['first']:m['last'] + 1]); sg = signature(b)
                        if sg:
                            prm = v1.sig_params(b, sg)
                            rk = None
                            if sg['recv']: rk = 'mut' if 'mut' in sg['recv'] else ('ref' if '&' in sg['recv'] else 'value')
                            methods[u.name][nm] = dict(params=prm[1:] if sg['recv'] else prm, recv=rk)
    return fields, methods, frees

COMP_FIELDS = {v: k for k, v in FIELD_OF.items()}

def check_text(text, who, fields, methods, errs, recv_re=r'(?:[A-Za-z_]\w*(?:\(\))?\s*\.\s*)*shell|self|state'):
    b = blank(text)
    for m in re.finditer(r'(?<![\w])(?<![\w)\]]\.)((?:' + recv_re + r'))\s*\.\s*(' + '|'.join(COMP_FIELDS) + r')\s*\.\s*([A-Za-z_]\w*)', b):
        recv, comp, ident = m.group(1), m.group(2), m.group(3)
        S = COMP_FIELDS[comp]
        cm = re.match(r'\s*(::\s*<[^>]*>)?\s*\(', b[m.end():])
        if cm:
            lp = m.end() + cm.end() - 1; rp = find_matching(b, lp)
            n = v1.count_args(b[lp + 1:rp])
            if ident in methods[S]:
                want = len(methods[S][ident]['params'])
                if want != n: errs.append((who, f'{recv}.{comp}.{ident}() takes {want}, got {n}'))
            elif ident not in fields[S]:
                if ident not in v1.__dict__.get('STD', ()): pass
        else:
            if ident not in fields[S] and ident not in methods[S]:
                errs.append((who, f'{recv}.{comp}.{ident} unknown on {S}'))

def mutability(root, fields, methods, errs):
    """a `&self` or `&T` holder must not call a `&mut self` method of the component it holds"""
    files = sorted(glob.glob(root + '/📥️staging/*/*/🦀️.rs'))
    for p in files:
        src, units = split_file(p)
        for u in units:
            if u.kind != 'impl' or ' for ' in u.head or u.name not in STRUCTS: continue
            for m in top_items(src, u.code + 1, u.last, 4):
                if m['kind'] != 'fn': continue
                b = '\n'.join(src.bl[m['first']:m['last'] + 1]); sg = signature(b)
                if not sg: continue
                nm = item_name(m['head'], 'fn')
                held = {}
                if sg['recv']: held['self'] = ('mut' if 'mut' in sg['recv'] else 'ref', u.name)
                for prm in v1.sig_params(b, sg):
                    mm = re.match(r'([A-Za-z_]\w*)\s*:\s*&\s*(?:\'\w+\s+)?(mut\s+)?([A-Za-z_]\w*)\s*$', prm)
                    if mm and mm.group(3) in STRUCTS: held[mm.group(1)] = ('mut' if mm.group(2) else 'ref', mm.group(3))
                body = b[sg['cl'] + 1:]
                for hv, (kind, S) in held.items():
                    if kind == 'mut': continue
                    for mm in re.finditer(r'(?<![\w])(?<![\w)\]]\.)' + re.escape(hv) + r'\s*\.\s*([A-Za-z_]\w*)\s*\(', body):
                        d = methods[S].get(mm.group(1))
                        if d and d['recv'] == 'mut': errs.append((p.split('staging/')[1][:60], f'{nm}: shared {hv} calls &mut method {mm.group(1)}'))
                    # passing a shared holder as &mut argument is checked by arity mirror: callee param types
                for mm in re.finditer(r'(?<![\w])(?<![\w)\]]\.)([A-Za-z_]\w*)\s*\.\s*([A-Za-z_]\w*)\s*\(', body):
                    pass

if __name__ == '__main__':
    root = sys.argv[1]
    fields, methods, frees = tables(root)
    errs = []
    for p in sorted(glob.glob(root + '/📥️staging/*/*/🦀️.rs')) + sorted(glob.glob(root + '/*/🦀️.rs')) + [root + '/🦀️.rs']:
        src, units = split_file(p)
        who = p.replace(root + '/', '')[:70]
        for u in units:
            if u.kind == 'impl' and u.name == 'ShellState' and ' for ' not in u.head:
                for m in top_items(src, u.code + 1, u.last, 4):
                    check_text('\n'.join(src.lines[m['first']:m['last'] + 1]), who, fields, methods, errs, recv_re='self')
            elif u.kind == 'impl' and u.name == 'ShellState':
                check_text('\n'.join(src.lines[u.first:u.last + 1]), who, fields, methods, errs, recv_re='self')
            elif u.kind in ('fn', 'impl'):
                check_text('\n'.join(src.lines[u.first:u.last + 1]), who, fields, methods, errs, recv_re='state|shell')
    mutability(root, fields, methods, errs)
    if len(sys.argv) > 2:
        for p in sorted(glob.glob(sys.argv[2] + '/**/*.rs', recursive=True)):
            check_text(open(p, encoding='utf-8').read(), 'EXT ' + p.replace(sys.argv[2] + '/', '')[-70:], fields, methods, errs, recv_re=r'(?:[A-Za-z_]\w*(?:\(\))?\s*\.\s*)*shell')
    print(len(errs), 'findings')
    for e in errs[:60]: print(e)
