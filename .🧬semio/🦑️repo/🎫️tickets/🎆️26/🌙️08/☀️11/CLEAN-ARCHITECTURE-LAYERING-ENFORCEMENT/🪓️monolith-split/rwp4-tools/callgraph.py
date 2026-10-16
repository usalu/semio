# 🕸️ WP4 call graph: field touches, callees, receivers and re-homing fixed point for the 701 ShellState members.
import re, sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from assign import *

def load_components():
    comps = components()
    f2s = {}; order = []
    for c in comps:
        order.append(c['struct'])
        for f in c['fields']: f2s[f] = c['struct']
    s2c = {c['struct']: c['crate'] for c in comps}
    return comps, f2s, order, s2c

SELF_FIELD = re.compile(r'(?<![A-Za-z0-9_])self\s*\.\s*([A-Za-z_]\w*)')
SELF_CALL = re.compile(r'(?<![A-Za-z0-9_])(?:self|Self)\s*(?:\.|::)\s*([A-Za-z_]\w*)\s*(?:::<[^>]*>)?\(')
FREE_FNS = {'tutorial_capture_camera_pose','tutorial_apply_camera_pose','tutorial_capture_ui_snapshot','tutorial_apply_ui_snapshot','tutorial_apply_ui_change_to_shell','tutorial_resolve_gesture_point','render_tutorial_gesture_overlay_node','tutorial_recorder_sample','persist_ui_preferences'}
SIG_RECV = re.compile(r'fn\s+[A-Za-z_]\w*\s*(?:<[^>]*>)?\s*\(\s*(&\s*(?:\'\w+\s+)?mut\s+self|&\s*(?:\'\w+\s+)?self|mut\s+self|self)\b')

def member_info():
    home, files, items = analyse(rehomed=False)
    comps, f2s, order, s2c = load_components()
    mem = {}
    for it in items:
        if not it.is_member: continue
        txt = it.text
        sig_text = '\n'.join(it.f['src'].bl[it.code:it.last + 1])
        mm = re.search(r'\bfn\s+[A-Za-z_]\w*\s*(?:<[^>]*(?:<[^>]*>[^>]*)*>)?\s*\(\s*(&\s*(?:\'\w+\s+)?mut\s+self|&\s*(?:\'\w+\s+)?self|mut\s+self|self)\b', sig_text)
        first_fn = re.search(r'\bfn\s+[A-Za-z_]\w*', sig_text)
        recv = None
        if mm and first_fn and mm.start() == first_fn.start():
            r = re.sub(r'\s+', ' ', mm.group(1)); recv = 'mut' if 'mut self' in r and r.startswith('&') else ('ref' if r.startswith('&') else 'value')
        sig = sig_text[:400]
        touched = collections.Counter(f2s[x] for x in SELF_FIELD.findall(txt) if x in f2s)
        calls = collections.Counter(SELF_CALL.findall(txt))
        info = dict(name=it.name, crate=it.crate, file=it.f['slug'], recv=recv, touched=dict(touched), calls=dict(calls), lines=it.last - it.first + 1, unit=it, async_='async fn' in sig or 'ShellTurn' in sig)
        mem.setdefault(it.name, []).append(info)
    # free functions that take the whole ShellState (plan 7.6): graph nodes with the receiver variable `state`
    free_src = {}
    for f in files:
        d = f['slug']
        for u in f['units']:
            if u.kind == 'fn' and u.name in FREE_FNS and (d == '🎓️tutorial-runtime' or d == '🔑️storage-prefs-and-i18n'):
                free_src[u.name] = (f, u, u)
            if u.kind == 'impl' and u.name == 'UiPrefsSnapshot':
                src = f['src']
                free_src['UiPrefsSnapshot::capture'] = (f, u, u)
    fcrate = {'🎓️tutorial-runtime': 'shell-surfaces', '🔑️storage-prefs-and-i18n': 'shell-prefs'}
    for name, (f, u, _) in free_src.items():
        txt = '\n'.join(f['src'].bl[u.code:u.last + 1])
        touched = collections.Counter(f2s[x] for x in re.findall(r'(?<![\w.])state\s*\.\s*([A-Za-z_]\w*)', txt) if x in f2s)
        calls = collections.Counter(re.findall(r'(?<![\w.])state\s*\.\s*([A-Za-z_]\w*)\s*(?:::<[^>]*>)?\(', txt))
        u.text = txt
        mem[name] = [dict(name=name, crate=fcrate[f['slug']], file=f['slug'], recv='free', touched=dict(touched), calls=dict(calls), lines=u.last - u.first + 1, unit=u, async_=False, var='state')]
    free_names = set(free_src)
    for n, infos in list(mem.items()):
        for i in infos:
            txt = i['unit'].text if hasattr(i['unit'], 'text') else '\n'.join(i['unit'].f['src'].bl[i['unit'].code:i['unit'].last + 1])
            for fn in free_names:
                short = fn.split('::')[-1]
                pat = (r'UiPrefsSnapshot::capture\(' if '::' in fn else r'(?<![\w.:])' + re.escape(fn) + r'\s*\(\s*(?:&\s*mut\s+)?\*?(?:self|state)\b')
                if re.search(pat, txt) and fn != n: i['calls'][fn] = i['calls'].get(fn, 0) + 1
    return mem, f2s, order, s2c, comps

if __name__ == '__main__':
    mem, f2s, order, s2c, comps = member_info()
    names = set(mem)
    allf = set(f2s)
    # field accesses to names that are neither fields nor methods
    unk = collections.Counter()
    for n, infos in mem.items():
        for i in infos:
            for x in SELF_FIELD.findall(i['unit'].text):
                if x not in allf and x not in names: unk[x] += 1
    print('self.<x> unknown:', unk.most_common(15))
    # receiver kinds
    print(collections.Counter(i['recv'] for infos in mem.values() for i in infos))
    # homing consistency: callee crate rank > caller crate rank
    RK = {c: k for k, c in enumerate(RANK)}
    viol = []
    for n, infos in mem.items():
        for i in infos:
            for cal in i['calls']:
                for j in mem.get(cal, []):
                    if RK[j['crate']] > RK[i['crate']]: viol.append((n, i['crate'], cal, j['crate']))
    print('upward calls under Appendix B:', len(viol))
    for v in viol[:25]: print(v)
