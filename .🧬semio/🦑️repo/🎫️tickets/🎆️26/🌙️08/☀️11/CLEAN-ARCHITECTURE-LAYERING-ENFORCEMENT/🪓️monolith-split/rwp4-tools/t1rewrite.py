# 🔧️ WP4 T1 rewriting of one function text: a ShellState member / a free fn taking ShellState becomes a component method or free fn with component parameters, or stays on the composer with component paths.
import re, sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from t1lib import *

CALL_AFTER = re.compile(r'\s*(::\s*<[^>]*>)?\s*\(')
FREE_TIER1 = {n for n in TIER1 if PLAN1[n].get('recv') == 'free'}
FREE_ALL = {n for n, v in PLAN1.items() if v.get('recv') == 'free' or n in ('tutorial_capture_ui_snapshot','tutorial_apply_ui_snapshot','tutorial_apply_ui_change_to_shell','render_tutorial_gesture_overlay_node','tutorial_recorder_sample')}
FREE_PAT = {}
for n in FREE_ALL:
    FREE_PAT[n] = re.compile(r'UiPrefsSnapshot::capture\(' if '::' in n else r'(?<![\w.:])' + re.escape(n) + r'\s*\(')

def comp_exprs(params, ctx, names, S, var, report, who, need):
    """expressions for the components `params` [(struct, mut)] in the caller context"""
    out = []
    for k, mut in params:
        if ctx == 'tier1':
            if k == S: out.append('self')
            else:
                if k not in need: report.append((who, f'free callee needs {k} not in needs'))
                out.append(names[k])
        else:
            out.append(('&mut ' if mut else '&') + f'{var}.{FIELD_OF[k]}')
    return out

def rewrite_fn(raw, name, mode, S, report, var='self', free_def=False):
    """mode 'tier1' (S = component struct or None for a free fn) or 'usecase'"""
    b = blank(raw)
    sig = signature(b)
    if sig is None:
        report.append((name, 'no signature')); return raw
    idents = body_idents_of(b)
    info = PLAN1.get(name)
    names = {s: pname(s, idents) for s in STRUCTS}
    edits = []
    def insert_before(pos_text_end, text, lead_comma=True):
        edits.append((pos_text_end, pos_text_end, text))
    def ins_args(lp, extras):
        rp = find_matching(b, lp)
        args = b[lp + 1:rp]
        if not extras: return
        ex = ', '.join(extras)
        if args.strip() == '': edits.append((lp + 1, lp + 1, ex))
        else:
            e = lp + 1 + len(args.rstrip())
            edits.append((e, e, (' ' if b[e - 1] == ',' else ', ') + ex))
    need = set()
    if mode == 'tier1':
        assert info and info['tier1'], (name, 'not tier1')
        need = {s for s, m in info['params']}
        if free_def:
            m = re.search(r'\b' + var + r'\s*:\s*&\s*(?:mut\s+)?ShellState', b[sig['op']:sig['cl'] + 1])
            assert m, (name, 'no state param')
            ps = extra_params(info['params'], None, names)
            edits.append((sig['op'] + m.start(), sig['op'] + m.end(), ', '.join(ps)))
        else:
            ex = extra_params(info['params'], sig['life'], names)
            if ex:
                inner = b[sig['op'] + 1:sig['cl']]
                e = sig['op'] + 1 + len(inner.rstrip())
                if inner.strip() == '': edits.append((sig['op'] + 1, sig['op'] + 1, ', '.join(ex)))
                else: edits.append((e, e, (' ' if b[e - 1] == ',' else ', ') + ', '.join(ex)))
        S_ = S
    else:
        S_ = None
    body0 = sig['cl'] + 1
    vre = re.escape(var)
    for m in re.finditer(r'(?<![\w])(?<![\w)\]]\.)' + vre + r'\s*\.\s*([A-Za-z_]\w*)', b[body0:]):
        s0 = body0 + m.start(); x = m.group(1); after = body0 + m.end(); vend = s0 + len(var)
        cm = CALL_AFTER.match(b, after)
        if cm:
            lp = cm.end() - 1
            if x in TIER1 and PLAN1[x].get('recv') != 'free':
                cinfo = PLAN1[x]; H = cinfo['struct']
                if mode == 'tier1':
                    recv = 'self' if H == S_ else names[H]
                    if H != S_ and H not in need: report.append((name, f'callee {x} home {H} not in needs'))
                    edits.append((s0, vend, recv))
                    cex = []
                    for k, mm in cinfo['params']:
                        if k == S_: report.append((name, f'callee {x} needs own struct'))
                        elif k not in need: report.append((name, f'callee {x} needs {k} not in needs'))
                        cex.append(names[k])
                else:
                    edits.append((s0, vend, f'{var}.{FIELD_OF[H]}'))
                    cex = [('&mut ' if mm else '&') + f'{var}.{FIELD_OF[k]}' for k, mm in cinfo['params']]
                ins_args(lp, cex)
            else:
                if mode == 'tier1' and x not in ('clone', 'to_owned', 'borrow', 'as_ref', 'as_mut', 'iter', 'len'):
                    report.append((name, f'tier-1 calls non-tier-1 method {var}.{x}()'))
        elif x in F2S:
            F = F2S[x]
            if mode == 'tier1':
                if F != S_:
                    if F not in need: report.append((name, f'field {x} of {F} not in needs'))
                    edits.append((s0, vend, names[F]))
            else:
                edits.append((s0, vend, f'{var}.{FIELD_OF[F]}'))
        else:
            report.append((name, f'unknown {var}.{x}'))
    for m in re.finditer(r'(?<![\w])Self::([A-Za-z_]\w*)', b[body0:]):
        s0 = body0 + m.start(); x = m.group(1)
        if x in TIER1 and PLAN1[x].get('recv') != 'free':
            H = PLAN1[x]['struct']
            if PLAN1[x]['params']: report.append((name, f'Self::{x} assoc with params'))
            if H is None: edits.append((s0, s0 + 6, ''))
            else: edits.append((s0, s0 + 4, H))
        elif mode == 'tier1':
            report.append((name, f'tier-1 uses Self::{x}'))
    # free-function calls taking the whole state as first argument
    for g, rx in FREE_PAT.items():
        if g == name: continue
        for m in rx.finditer(b, body0):
            lp = m.end() - 1
            rp = find_matching(b, lp)
            mm = re.match(r'\s*(?:' + vre + r')\b(\s*,)?', b[lp + 1:rp]) if True else None
            if not mm:
                report.append((name, f'free call {g} without {var} first'))
                continue
            if g in FREE_TIER1:
                params = PLAN1[g]['params']
                exprs = comp_exprs(params, mode, names, S_, var, report, name, need)
                a0 = lp + 1 + (len(mm.group(0)) - len(mm.group(1) or ''))
                st = lp + 1 + mm.group(0).index(var)
                edits.append((st, st + len(var), ', '.join(exprs)))
    if mode == 'tier1' and not free_def:
        for m in re.finditer(r'(?<![\w.:])self(?![\w])(?!\s*\.)', b[body0:]):
            s0 = body0 + m.start()
            ctx = b[max(0, s0 - 60):s0 + 1]
            if any(rx.search(ctx + '(') for rx in ()): continue
            if re.search(r'(' + '|'.join(re.escape(g) for g in FREE_TIER1) + r')\s*\(\s*$', b[max(0, s0 - 60):s0]) or re.search(r'UiPrefsSnapshot::capture\(\s*$', b[max(0, s0 - 40):s0]): continue
            report.append((name, 'bare self at %d: %s' % (s0, raw[max(0, s0 - 30):s0 + 30].replace('\n', ' '))))
    out = raw
    for s, e, t in sorted(edits, key=lambda x: (x[0], x[1]), reverse=True):
        out = out[:s] + t + out[e:]
    return out

def rewrite_member(raw, name, mode, S, report):
    return rewrite_fn(raw, name, mode, S, report, 'self', False)
