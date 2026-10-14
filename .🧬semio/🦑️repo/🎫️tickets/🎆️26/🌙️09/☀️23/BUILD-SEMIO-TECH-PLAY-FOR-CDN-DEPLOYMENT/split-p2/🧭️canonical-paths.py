#!/usr/bin/env python3
"""🧭️ Rewrites every path that reaches an item through a re-export barrel to the item's owning module.

A barrel is a module path whose names are re-exports (`pub use x::*`, a root `pub use`, an `engine`
facade, an alias module). Each barrel has an ordered search list of owning modules; a name is resolved
to the first module of that list that defines it (definitions come from the crate's module tree,
including `mod component; pub use component::*;` file modules and one level of inline-module glob
re-exports). `use <barrel>::{..}` groups are expanded per owner, `use <barrel> as alias;` aliases are
inlined. Paths are rewritten in the crate's own files (`crate::`) and in external files (`<crate>::`).

usage: 🧭️canonical-paths.py <config.json> [--write] [--external <file-list>]
config: {"root": crate-root.rs, "crate": "semio_s_artifact_x", "barrels": {"engine": [module...], "": [module...]},
         "aliases": {"semio_s_artifact_stdio_contract": ["apply_mutation", ...]}, "defs": {module: [macro-generated names]}}
  modules are written without `crate::`; "" is the crate root itself (its own items are kept).
"""
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
ns = {}
exec(open(os.path.join(HERE, '🕸️module-graph.py'), encoding='utf-8').read().split('\ndef main():')[0], ns)
Crate, strip = ns['Crate'], ns['strip']

ITEM = re.compile(r'(?m)^[ \t]*pub(?:\([^)]*\))?\s+(?:(?:const|async|unsafe)\s+)*(?:fn|struct|enum|trait|type|const|static|mod|union)\s+([A-Za-z_][A-Za-z0-9_]*)')
MACRO = re.compile(r'(?m)^[ \t]*macro_rules!\s*([A-Za-z_][A-Za-z0-9_]*)')
PUBUSE = re.compile(r'(?m)^[ \t]*pub\s+use\s+([^;]+);')

def definitions(crate):
    defs = collections.defaultdict(set)
    for f, mp in crate.files.items():
        mod = tuple(x for x in mp if x != 'component')
        s = crate.text[f]
        depth_ok = []
        level = 0; inline = []
        for m in re.finditer(r'(?m)^([ \t]*)pub(?:\([^)]*\))?\s+mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{', s):
            inline.append((m.group(2), m.start()))
        body_top = s
        for m in ITEM.finditer(s):
            indent = len(m.group(0)) - len(m.group(0).lstrip())
            if indent == 0: defs[mod].add(m.group(1))
        for m in PUBUSE.finditer(s):
            indent = len(m.group(0)) - len(m.group(0).lstrip())
            if indent: continue
            target = m.group(1).strip()
            g = re.match(r'^([A-Za-z_][A-Za-z0-9_]*)::\*$', target)
            if g:
                child = g.group(1)
                cm = re.search(r'(?ms)^pub mod ' + child + r'\s*\{(.*?)^\}', s)
                if cm:
                    for mm in ITEM.finditer(cm.group(1)): defs[mod].add(mm.group(1))
                continue
            for part in re.split(r',', re.sub(r'.*\{|\}', '', target)):
                part = part.strip()
                if part: defs[mod].add(re.split(r'\s+as\s+', part)[-1].split('::')[-1])
        for mm in MACRO.finditer(s): defs[mod].add(mm.group(1))
    for mp in list(crate.mods):
        norm = tuple(x for x in mp if x != 'component')
        if norm: defs[norm[:-1]].add(norm[-1])
    return defs

def main():
    cfg = json.load(open(sys.argv[1])); write = '--write' in sys.argv
    crate = Crate(cfg['root']); defs = definitions(crate)
    barrels = {tuple(k.split('::')) if k else (): [tuple(m.split('::')) if m else () for m in v] for k, v in cfg['barrels'].items()}
    own_root = set(re.findall(r'(?m)^pub (?:fn|const|static|struct|enum|type|mod) ([A-Za-z_][A-Za-z0-9_]*)', crate.text[crate.root]))
    for b in barrels:
        if len(b) == 1: own_root.discard(b[0])
    for m, names in cfg.get('defs', {}).items(): defs[tuple(m.split('::'))].update(names)
    aliases = {n: k for k, v in cfg.get('aliases', {}).items() for n in v}
    unresolved = collections.Counter()

    def resolve(barrel, name):
        if barrel == () and name in own_root: return None
        if name in aliases and barrel == (): return ('@' + aliases[name],)
        hits = [m for m in barrels[barrel] if name in defs.get(m, set())]
        if not hits:
            unresolved[('::'.join(barrel), name)] += 1; return None
        return hits[0]

    def render(prefix, mod):
        if mod and mod[0].startswith('@'): return mod[0][1:]
        return '::'.join([prefix] + list(mod))

    seg = r'[A-Za-z_][A-Za-z0-9_]*'
    def rewrite(text, prefixes):
        changes = 0
        for prefix in prefixes:
            pat = re.compile(r'(?<![A-Za-z0-9_:])' + re.escape(prefix) + r'((?:\s*::\s*' + seg + r')*)(\s*::\s*\{)?')
            def group_use(m_use):
                return m_use
            out = []; pos = 0
            for m in pat.finditer(text):
                segs = [x for x in re.split(r'\s*::\s*', m.group(1)) if x]
                if not segs and not m.group(2): continue
                best = None
                for k in range(len(segs), -1, -1):
                    if tuple(segs[:k]) in barrels: best = k; break
                if best is None or best >= len(segs):
                    if best is not None and m.group(2):
                        out.append(text[pos:m.start()]); start = m.end()
                        d = 1; j = start
                        while d:
                            if text[j] == '{': d += 1
                            elif text[j] == '}': d -= 1
                            j += 1
                        inner = text[start:j - 1]
                        parts = []; cur = ''; dd = 0
                        for ch in inner:
                            if ch == '{': dd += 1
                            if ch == '}': dd -= 1
                            if ch == ',' and dd == 0: parts.append(cur); cur = ''
                            else: cur += ch
                        if cur.strip(): parts.append(cur)
                        barrel = tuple(segs[:best]); groups = collections.OrderedDict()
                        for part in parts:
                            part = part.strip()
                            if not part: continue
                            head = re.split(r'\s*::\s*|\s+as\s+|\s*\{', part)[0]
                            mod = resolve(barrel, head) if head != 'self' else None
                            key = render(prefix, mod) if mod is not None else render(prefix, barrel)
                            groups.setdefault(key, []).append(part)
                        semi = text.find(';', j)
                        tail = text[j:semi + 1]
                        lead_m = re.search(r'(?:pub(?:\([^)]*\))?\s+)?use\s*$', text[:m.start()])
                        if lead_m and len(groups) > 1:
                            out[-1] = out[-1][:len(out[-1]) - len(lead_m.group(0))]
                            indent = re.search(r'[ \t]*$', out[-1]).group(0)
                            out.append(('\n' + indent).join(f'{lead_m.group(0).strip()} {k}::{{{", ".join(v)}}}' + tail.strip() for k, v in groups.items()))
                            pos = semi + 1; changes += 1; continue
                        if len(groups) > 1:
                            out.append(text[m.start():j]); pos = j
                            continue
                        k, v = next(iter(groups.items()))
                        out.append(f'{k}::{{{", ".join(v)}}}'); pos = j; changes += 1
                        continue
                    continue
                barrel = tuple(segs[:best]); name = segs[best]
                mod = resolve(barrel, name)
                if mod is None: continue
                new = render(prefix, mod) + '::' + '::'.join(segs[best:])
                out.append(text[pos:m.start()]); out.append(new + (m.group(2) or '')); pos = m.end(); changes += 1
            out.append(text[pos:]); text = ''.join(out)
        return text, changes

    def inline_aliases(text):
        for m in list(re.finditer(r'(?m)^[ \t]*use\s+(crate(?:::' + seg + r')+)\s+as\s+(' + seg + r')\s*;\n', text)):
            path, alias = m.group(1), m.group(2)
            segs = tuple(path.split('::')[1:])
            if segs in barrels:
                text = text.replace(m.group(0), '')
                text = re.sub(r'(?<![A-Za-z0-9_:])' + alias + r'::', path + '::', text)
        return text

    files = [f for f in crate.files]
    total = 0
    for f in files:
        t = open(f, encoding='utf-8').read()
        t2 = inline_aliases(t)
        t2, n = rewrite(t2, ['crate'])
        if t2 != t:
            total += 1
            if write: open(f, 'w', encoding='utf-8').write(t2)
    ext = []
    if '--external' in sys.argv:
        ext = [l.strip() for l in open(sys.argv[sys.argv.index('--external') + 1]) if l.strip()]
    for f in ext:
        t = open(f, encoding='utf-8').read()
        t2, n = rewrite(t, [cfg['crate']])
        if t2 != t:
            total += 1
            if write: open(f, 'w', encoding='utf-8').write(t2)
    print(f'files changed: {total}')
    for (b, n), c in sorted(unresolved.items()): print(f'UNRESOLVED {b or "<root>"}::{n} x{c}')

main()
