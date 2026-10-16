# 📇️ Public surface parity of the sync module: top-level `pub` names of the pristine root against the public names of the three sync crate roots (explicit `pub use` lists included).
# usage: surface.py <old-root> <sync-root> <native-root> <wasm-root>    (texts of the carve output come from --dump json via SURFACE_JSON)
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools')
from okrs import items

def pub_names(text, nested=('native_actor', 'wasm_actor')):
    names = {}
    for it in items(text):
        k = it['kind']; t = it['text']
        code = re.sub(r'(?m)^\s*(//.*|#\[.*)$', '', t).strip()
        if not code.startswith('pub') or code.startswith('pub(') : continue
        if k in ('fn', 'struct', 'enum', 'trait', 'type', 'const', 'static', 'mod', 'union') and it['name']:
            names[it['name']] = k
        elif k == 'use':
            body = code[len('pub use'):].strip().rstrip(';')
            for leaf in re.findall(r'([A-Za-z_]\w*)(?:\s+as\s+([A-Za-z_]\w*))?\s*(?:,|\}|$)', body):
                names[leaf[1] or leaf[0]] = 'use'
        elif k is None and 'macro_rules!' in code:
            m = re.search(r'macro_rules!\s*(\w+)', code)
            if m and '#[macro_export]' in t: names[m.group(1)] = 'macro'
    return names

if __name__ == '__main__':
    old = pub_names(open(sys.argv[1], encoding='utf8').read())
    new = {}
    for p in sys.argv[2:]:
        new.update({k: (v, os.path.basename(os.path.dirname(p))) for k, v in pub_names(open(p, encoding='utf8').read()).items()})
    lost = sorted(set(old) - set(new)); gained = sorted(set(new) - set(old))
    print('old', len(old), 'new', len(new), 'lost', len(lost), 'gained', len(gained))
    print('LOST', lost); print('gained', gained)
