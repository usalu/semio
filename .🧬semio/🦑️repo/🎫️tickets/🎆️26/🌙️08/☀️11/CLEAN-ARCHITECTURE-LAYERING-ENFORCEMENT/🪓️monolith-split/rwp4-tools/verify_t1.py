# ✅️ WP4 static check of the decomposed tree: every `self.x` / `<param>.x` / `<recv>.<component>.x` resolves to a field or method of the right component, with the right arity.
import re, sys, os, glob, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from t1lib import *
from units import *

def top_commas(b):
    d = 0; n = 0; seen = False
    for ch in b:
        if ch in '([{': d += 1
        elif ch in ')]}': d -= 1
        elif ch == ',' and d == 0: n += 1
        if not ch.isspace() and ch != ',': seen = True
    return n

def count_args(inner_b):
    if inner_b.strip() == '': return 0
    n = top_commas(inner_b) + 1
    if inner_b.rstrip().endswith(','): n -= 1
    return n

def sig_params(b, sig):
    inner = b[sig['op'] + 1:sig['cl']].replace('->', '__')
    parts = []; d = 0; cur = ''
    for ch in inner:
        if ch in '([{<': d += 1
        elif ch in ')]}>': d -= 1 if ch != '>' or True else 0
        if ch == ',' and d == 0: parts.append(cur); cur = ''
        else: cur += ch
    if cur.strip(): parts.append(cur)
    return [p.strip() for p in parts]

def main(root):
    files = sorted(glob.glob(root + '/📥️staging/*/*/🦀️.rs')) + glob.glob(root + '/*/🦀️.rs') + [root + '/🦀️.rs']
    fields = collections.defaultdict(set); methods = collections.defaultdict(dict)
    parsed = {}
    for p in files:
        src, units = split_file(p)
        parsed[p] = (src, units)
        for u in units:
            if u.kind == 'struct' and u.name in STRUCTS:
                for i in range(u.code + 1, u.last):
                    m = re.match(r'^\s+pub ([A-Za-z_]\w*)\s*:', src.bl[i])
                    if m: fields[u.name].add(m.group(1))
            if u.kind == 'impl' and u.name in STRUCTS + ['ShellState'] and not ' for ' in u.head:
                body_members = top_items(src, u.code + 1, u.last, 4)
                for m in body_members:
                    if m['kind'] == 'fn':
                        nm = item_name(m['head'], 'fn')
                        b = '\n'.join(src.bl[m['first']:m['last'] + 1])
                        sg = signature(b)
                        if sg: methods[u.name][nm] = (len(sig_params(b, sg)) - (1 if sg['recv'] else 0), sg['recv'])
    errs = []
    for p, (src, units) in parsed.items():
        for u in units:
            if u.kind != 'impl' or ' for ' in u.head or u.name not in STRUCTS + ['ShellState']: continue
            S = u.name
            for m in top_items(src, u.code + 1, u.last, 4):
                if m['kind'] != 'fn': continue
                nm = item_name(m['head'], 'fn')
                text_b = '\n'.join(src.bl[m['first']:m['last'] + 1])
                sg = signature(text_b)
                if not sg: continue
                params = {}
                for prm in sig_params(text_b, sg):
                    mm = re.match(r'([A-Za-z_]\w*)\s*:\s*&\s*(?:\'\w+\s+)?(?:mut\s+)?([A-Za-z_]\w*)\s*$', prm)
                    if mm and mm.group(2) in STRUCTS: params[mm.group(1)] = mm.group(2)
                body = text_b[sg['cl'] + 1:]
                def check(recv_struct, ident, after, where):
                    call = re.match(r'\s*(::\s*<[^>]*>)?\s*\(', after)
                    if call:
                        lp = after.index('(', 0); rp = find_matching(after, lp)
                        n = count_args(after[lp + 1:rp])
                        if ident in methods.get(recv_struct, {}):
                            want = methods[recv_struct][ident][0]
                            if want != n: errs.append((p, nm, f'{where}.{ident}() takes {want} args, got {n}'))
                        elif ident in fields.get(recv_struct, ()):
                            pass
                        else:
                            if ident not in ('clone','as_ref','as_mut','is_some','is_none','iter','len','get','insert','remove','push','contains_key','take','map','unwrap','cloned','is_empty','entry','values','keys','clear','contains','extend','retain','drain','iter_mut','get_mut','to_string','borrow','last','first','pop','as_deref','copied','unwrap_or','unwrap_or_default','to_owned','last_mut','values_mut','is_ok','replace','as_str','as_slice','push_back','pop_front','push_str','truncate','resize','swap','sort','sort_by','sort_by_key','position','any','all','find','filter','collect','sum','min','max','abs','join','split','trim','starts_with','ends_with','chars','bytes','lines','parse','into','from','default','new','with_capacity','capacity','reserve','is_empty','front','back','front_mut','back_mut','pop_back','binary_search','windows','chunks','concat','repeat','to_vec','into_iter','rev','zip','enumerate','skip','take_while','flat_map','filter_map','fold','for_each','sort_unstable','dedup','append','split_off','remove_entry','get_or_insert_with','get_or_insert','or_insert','or_insert_with','or_default','and_modify','is_none_or','is_some_and','ok','err','ok_or','ok_or_else','map_err','and_then','or_else','unwrap_or_else','expect','as_deref_mut','filter_map','saturating_sub','saturating_add','get_key_value','first_key_value','last_key_value','range','extend_from_slice','swap_remove','iter_mut','split_at','split_first','pop_first','pop_last','cancel','cancel_now','poll','try_recv','recv','send','lock','read','write','set','get_mut','abort','now','elapsed','as_millis'):
                                errs.append((p, nm, f'{where}.{ident} unknown on {recv_struct}'))
                    else:
                        if ident not in fields.get(recv_struct, ()) and ident not in methods.get(recv_struct, {}):
                            errs.append((p, nm, f'{where}.{ident} unknown field on {recv_struct}'))
                if S == 'ShellState':
                    for mm in re.finditer(r'(?<![\w])(?<![\w)\]]\.)self\s*\.\s*([A-Za-z_]\w*)(?:\.([A-Za-z_]\w*))?', body):
                        a, c = mm.group(1), mm.group(2)
                        tail = body[mm.end():]
                        if a in FIELD_OF.values():
                            Sx = [k for k, v in FIELD_OF.items() if v == a][0]
                            if c: check(Sx, c, tail, 'self.' + a)
                        else:
                            if a in methods['ShellState']:
                                n = count_args(tail[tail.index('(') + 1: find_matching(tail, tail.index('('))]) if re.match(r'\s*\(', tail) else None
                                if n is not None and methods['ShellState'][a][0] != n: errs.append((p, nm, f'self.{a}() takes {methods["ShellState"][a][0]}, got {n}'))
                            elif c is None or True:
                                if a not in ('clone',): errs.append((p, nm, f'self.{a} unknown on ShellState'))
                else:
                    for mm in re.finditer(r'(?<![\w])(?<![\w)\]]\.)self\s*\.\s*([A-Za-z_]\w*)', body):
                        check(S, mm.group(1), body[mm.end():], 'self')
                    for pn, st in params.items():
                        for mm in re.finditer(r'(?<![\w])(?<![\w)\]]\.)' + re.escape(pn) + r'\s*\.\s*([A-Za-z_]\w*)', body):
                            check(st, mm.group(1), body[mm.end():], pn)
    print('structs', {s: len(f) for s, f in fields.items()})
    print('impl methods', {s: len(m) for s, m in methods.items()})
    print(len(errs), 'findings')
    for e in errs[:80]: print(e[0].replace(root + '/', '')[:70], e[1], e[2])
    return errs

if __name__ == '__main__':
    main(sys.argv[1])
