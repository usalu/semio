import sys; sys.path.insert(0, '.')
from lib6 import *
import touched6, t2plan, t2run, rw6
GONE = ['🧵️instance/🚪️ingress', '🧵️instance/📨️dispatch', '🧵️instance/🛠️tool-machine', '🧵️instance/🤝️group', '🧵️instance/🔄️full-operation', '🧵️instance/⏳️operation-progress', '🪜️close-ladder', '🪜️envelope-ladder', '🪜️maintenance-ladder', '🔁️document-replacement/🦀️.rs', '🎟️lease/🦀️.rs', '🏪️stores/🦀️.rs', '🧵️instance/🧾️typed-operations', '🧵️instance/🧷️command-routes', '🧵️instance/🖼️render', '🧵️instance/🔌️plugin-app-impl', '🧵️instance/🦀️.rs', '🎮️command/📥️ingress', '"🚪️ingress/', '"📨️dispatch/', '"🤝️group/']

def match(s, i, o, c):
    d = 0; q = None
    while i < len(s):
        ch = s[i]
        if q:
            if ch == '\\': i += 2; continue
            if ch == q: q = None
        elif ch in '"\'`': q = ch
        elif ch == o: d += 1
        elif ch == c:
            d -= 1
            if d == 0: return i
        i += 1
    return -1

def fix_text(t, w):
    n = 0
    # (a) arrays of band paths: [...].map(...).join(...)
    out = []; i = 0
    while True:
        j = t.find('[', i)
        if j < 0: out.append(t[i:]); break
        if not re.match(r'\[\s*["\'`][^"\'`\n]{0,160}(?:🧵️instance|🚪️ingress)', t[j:j + 220]):
            out.append(t[i:j + 1]); i = j + 1; continue
        e = match(t, j, '[', ']')
        inner = t[j:e + 1] if e > 0 else ''
        if e > 0 and '🧵️instance/' in inner and inner.count('.rs') >= 2 or (e > 0 and '"🚪️ingress/🦀️.rs"' in inner and '"🦀️.rs"' in inner):
            k = e + 1
            m = re.match(r'\s*\.map\(', t[k:])
            if m:
                pe = match(t, k + m.end() - 1, '(', ')')
                k2 = pe + 1
                m2 = re.match(r'\s*\.join\(', t[k2:])
                if m2:
                    pj = match(t, k2 + m2.end() - 1, '(', ')')
                    out.append(t[i:j] + 'familyText()'); i = pj + 1; n += 1; continue
        out.append(t[i:j + 1]); i = j + 1
    t = ''.join(out)
    # (b) single reads of removed family files
    res = []; i = 0
    while True:
        j = t.find('readFileSync(', i)
        if j < 0: res.append(t[i:]); break
        e = match(t, j + len('readFileSync'), '(', ')')
        call = t[j:e + 1]
        # the argument holds the path; second argument may be the encoding
        if e > 0 and any(g in call for g in GONE) and '🧵️instance/📜' not in call and '📥️staging' not in call:
            res.append(t[i:j] + 'familyText()'); i = e + 1; n += 1
        else:
            res.append(t[i:j + len('readFileSync(')]); i = j + len('readFileSync(')
    t = ''.join(res)
    # (c) Rust code fragments inside string literals: component paths of fields and converted method calls
    def lit(m):
        s = m.group(0); body = s[1:-1]
        if 'self' not in body: return s
        new = rw6.rewrite(body, w, 'composer', recv='self')
        return s[0] + new + s[-1] if new != body else s
    t2 = re.sub(r'"(?:[^"\\\n]|\\.)*"', lit, t) if not os.environ.get('NOC') else t
    return t2, n

def run(files, write=False):
    model, methods, table, known = t2plan.build_world()
    w = rw6.World(methods, t2run.build_comp_generics()); w.known_composer = known
    changed = []
    for rel in files:
        p = ROOT + '/' + rel
        t = open(p, encoding='utf-8').read()
        import time; _t=time.time(); t2, n = fix_text(t, w); print('[DEBUG] fix', round(time.time()-_t,1), len(t), rel[-60:], flush=True)
        if t2 != t:
            if 'familyText()' in t2 and 'instance-family' not in t2:
                depth = os.path.dirname(os.path.relpath(p, PLUG)).count('/') + 1
                imp = 'import { familyText } from "' + '../' * depth + '🧷️companions/🧯️test-support/🧵️instance-family/🟦️.ts";\n'
                last = [m for m in re.finditer(r'(?m)^import[^\n]*\n', t2)]
                pos = last[-1].end() if last else 0
                t2 = t2[:pos] + imp + t2[pos:]
            changed.append((rel, n))
            if write: open(p, 'w', encoding='utf-8').write(t2)
    return changed

if __name__ == '__main__':
    files = [f for f in (l.strip() for l in open(SCR + '/pwp6-ts-subset.txt', encoding='utf-8')) if f] + ['🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎬️media-slots/🎟️collect/🧪️tests/🟦️.ts', '🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/📸️typed-snapshot/🧪️tests/🟦️.ts']
    ch = run(files, write='--write' in sys.argv)
    for rel, n in ch: print(n, rel.split('/')[-3] + '/' + rel.split('/')[-2])
    if '--write' in sys.argv:
        touched6.merge(modified=[r.replace('🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/', '') for r, n in ch if r.startswith('🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/')])
