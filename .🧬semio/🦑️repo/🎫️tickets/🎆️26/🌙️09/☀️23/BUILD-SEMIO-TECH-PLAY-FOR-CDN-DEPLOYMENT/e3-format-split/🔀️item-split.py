#!/usr/bin/env python3
"""🔀️ Item-level layer split of one Rust source file by reachability from per-layer seed items.

Parses the file's top-level items (doc comments and attributes included, `//#region` markers tracked),
builds the identifier reference graph between them (an `impl X` block is owned by `X`), and assigns
every item to the layers whose seeds reach it: items reached from one layer only go to that layer,
items reached from several go to the `shared` layer of the spec, unreached items are reported.
`plan` prints the assignment, cross-layer impl blocks and the per-layer line totals; `emit` writes
one file per layer (items in source order, region markers re-opened per file, private items that are
used across files made `pub`) and prints the imports each file needs from the others.

usage: 🔀️item-split.py plan <file.rs> <spec.json>
       🔀️item-split.py emit <file.rs> <spec.json>
spec: {"layers": {layer: {"seeds": [item...], "file": path, "module": "crate::..", "header": "..."}},
       "shared": layer, "base": layer-that-wins-when-reached, "tops": [consumer layers that only claim what no other layer reaches], "pin": {item: layer}, "keep": layer-for-unreached}
"""
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

def strip(src):
    exec_ns = {}
    code = open(os.path.join(os.path.dirname(os.path.abspath(__file__)), '🕸️module-graph.py'), encoding='utf-8').read().split('\nPATH_RE')[0]
    exec(code, exec_ns)
    return exec_ns['strip'](src)

HEAD = re.compile(r'^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:(?:const|async|unsafe|extern\s+"C")\s+)*(fn|struct|enum|trait|type|const|static|mod|union|macro_rules!)\s*([A-Za-z_][A-Za-z0-9_]*)', re.S)
IMPL = re.compile(r'^\s*(?:unsafe\s+)?impl\s*(?:<(?:[^<>]|<[^<>]*>)*>)?\s*(.*?)\s*(?:where\b.*)?$', re.S)

class Item:
    def __init__(self, i, start, end, text, stripped, region):
        self.i, self.start, self.end, self.text, self.stripped, self.region = i, start, end, text, stripped, region
        body = stripped
        code = re.sub(r'^(\s*(#!?\[[^\]]*\]|///[^\n]*|//[^\n]*)\s*)*', '', body)
        m = HEAD.match(code)
        if m:
            self.kind, self.name = m.group(1), m.group(2)
        elif re.match(r'\s*(unsafe\s+)?impl\b', code):
            head = code.split('{', 1)[0]
            mm = IMPL.match(head); target = mm.group(1) if mm else head
            if ' for ' in ' ' + target + ' ':
                trait, ty = re.split(r'\s+for\s+', target, maxsplit=1)
            else:
                trait, ty = None, target
            tyname = re.findall(r'[A-Za-z_][A-Za-z0-9_]*', re.sub(r'<.*', '', ty.strip().lstrip('&').replace('dyn ', '')))
            self.kind, self.name = 'impl', f'impl#{i}'
            self.target = tyname[-1] if tyname else '?'; self.trait = trait
        elif re.match(r'\s*(pub(\s*\([^)]*\))?\s+)?use\b', code):
            self.kind, self.name = 'use', f'use#{i}'
        elif re.match(r'\s*[A-Za-z_][A-Za-z0-9_:]*!\s*[\(\{\[]', code):
            self.kind, self.name = 'macro', f'macro#{i}'
        else:
            self.kind, self.name = '?', f'item#{i}'
        self.idents = set(re.findall(r'\b[A-Za-z_][A-Za-z0-9_]*\b', code))
        self.paths = set(re.findall(r'\b([A-Za-z_][A-Za-z0-9_]*)\s*::', code))

def parse(path):
    src = open(path, encoding='utf-8').read(); s = strip(src); n = len(s)
    items = []; region = []; pos = 0
    while pos < n:
        p = re.compile(r'\s*').match(src, pos).end()
        if p >= n: break
        le = src.find('\n', p); le = n if le < 0 else le
        rm = re.match(r'//#(end)?region\b[ \t]*([^\n]*)', src[p:le])
        if rm:
            region = region[:-1] if rm.group(1) else region + [rm.group(2).strip()]
            pos = le; continue
        code = re.compile(r'\s*').match(s, p).end()
        if code >= n:
            if src[p:].strip(): items.append(Item(len(items), p, n, src[p:n], s[p:n], tuple(region)))
            break
        mk = re.search(r'(?m)^[ \t]*//#(end)?region\b', src[p:code])
        if mk:
            r = p + mk.start()
            if src[p:r].strip(): items.append(Item(len(items), p, r, src[p:r], s[p:r], tuple(region)))
            pos = r; continue
        j = code; depth = 0
        while j < n:
            c = s[j]
            if c in '{[(': depth += 1
            elif c in '}])':
                depth -= 1
                if depth == 0 and c == '}':
                    k = j + 1
                    rest = re.compile(r'[ \t]*').match(s, k).end()
                    if rest < n and s[rest] == ';': k = rest + 1
                    attr = re.match(r'(\s*#!?\[[^\]]*\])*\s*$', s[code:j + 1]) is not None and s[code] == '#' and s[j] == ']'
                    j = k; break
            elif c == ';' and depth == 0:
                j += 1; break
            j += 1
        items.append(Item(len(items), p, j, src[p:j], s[p:j], tuple(region)))
        pos = j
    return src, items

def analyse(items, spec):
    names = {}
    for it in items:
        if it.kind not in ('impl', 'use', 'macro', '?'): names.setdefault(it.name, []).append(it)
    owner = {}
    for it in items:
        if it.kind == 'impl' and it.target in names: owner[it.i] = names[it.target][0].i
    edges = collections.defaultdict(set)
    for it in items:
        for nm in it.idents:
            for t in names.get(nm, []):
                if t.i != it.i and (t.kind != 'mod' or nm in it.paths): edges[it.i].add(t.i)
        if it.i in owner: edges[owner[it.i]].add(it.i); edges[it.i].add(owner[it.i])
    reach = collections.defaultdict(set)
    for layer, cfg in spec['layers'].items():
        seeds = [t.i for nm in cfg.get('seeds', []) for t in names.get(nm, [])]
        missing = [nm for nm in cfg.get('seeds', []) if nm not in names]
        if missing: print(f'# WARN {layer}: unknown seeds {missing}', file=sys.stderr)
        stack = list(seeds); seen = set(seeds)
        while stack:
            x = stack.pop()
            for y in edges[x]:
                if y in seen: continue
                pin = spec.get('pin', {}).get(items[y].name)
                if pin and pin != layer and items[y].name not in cfg.get('seeds', []):
                    continue
                seen.add(y); stack.append(y)
        for x in seen: reach[x].add(layer)
    assign = {}
    for it in items:
        pin = spec.get('pin', {}).get(it.name)
        if pin: assign[it.i] = pin; continue
        ls = set(reach.get(it.i, set()))
        if ls - set(spec.get('tops', [])): ls -= set(spec.get('tops', []))
        if it.i in owner and assign.get(owner[it.i]): pass
        if spec.get('base') in ls: assign[it.i] = spec['base']
        elif len(ls) == 1: assign[it.i] = next(iter(ls))
        elif len(ls) > 1: assign[it.i] = spec['shared']
        else: assign[it.i] = spec.get('keep', '?')
    for it in items:
        if it.i in owner: assign[it.i] = assign[owner[it.i]]
    for it in items:
        if it.kind == 'use':
            mods = [t for nm in it.paths for t in names.get(nm, []) if t.kind == 'mod']
            if mods: assign[it.i] = assign[mods[0].i]
    for it in reversed(items):
        if it.kind == '?' and re.fullmatch(r'\s*', it.stripped) and it.i + 1 < len(items): assign[it.i] = assign[it.i + 1]
    return names, edges, assign

def plan(path, spec):
    src, items = parse(path)
    names, edges, assign = analyse(items, spec)
    tot = collections.Counter()
    for it in items: tot[assign[it.i]] += it.text.count('\n') + 1
    print('# lines per layer:', dict(tot))
    for layer in list(dict.fromkeys(list(spec['layers']) + [spec['shared'], spec.get('keep', '?')])):
        sel = [it for it in items if assign[it.i] == layer]
        print(f'\n## {layer} ({len(sel)} items)')
        for it in sel:
            tag = it.name if it.kind != 'impl' else f'impl {it.trait + " for " if it.trait else ""}{it.target}'
            print(f'  {it.kind:7} {tag}  [{"/".join(it.region)}] L{src[:it.start].count(chr(10)) + 1}')
    print('\n## cross-layer references (item -> item in other layer)')
    cross = collections.Counter()
    for it in items:
        for j in edges[it.i]:
            if assign[j] != assign[it.i] and items[j].kind != 'impl':
                cross[(assign[it.i], assign[j], items[j].name)] += 1
    for (a, b, nm), c in sorted(cross.items()): print(f'  {a} -> {b}: {nm}')

def emit(path, spec):
    src, items = parse(path)
    names, edges, assign = analyse(items, spec)
    layers = spec['layers']
    out = {l: [] for l in layers}
    needs = collections.defaultdict(set)
    publicize = set()
    for it in items:
        l = assign[it.i]
        for j in edges[it.i]:
            t = items[j]
            if assign[j] != l and t.kind not in ('impl', 'use', 'macro', '?'):
                needs[l].add((assign[j], t.name)); publicize.add(j)
    for it in items:
        l = assign[it.i]
        if l not in out: continue
        text = it.text
        if it.i in publicize:
            text = re.sub(r'^((?:\s*(?:#!?\[[^\]]*\]|///[^\n]*|//[^\n]*)\s*)*?\s*)(?:pub\s*\((?:crate|super)\)\s+)?((?:const\s+fn|fn|struct|enum|trait|type|const|static|union)\b)', r'\1pub \2', text, count=1)
        out[l].append((it.region, text))
    for l, cfg in layers.items():
        if not cfg.get('file'): continue
        body = []; cur = ()
        for region, text in out[l]:
            common = 0
            while common < min(len(cur), len(region)) and cur[common] == region[common]: common += 1
            for r in reversed(cur[common:]): body.append(f'//#endregion {r}')
            for r in region[common:]: body.append(f'//#region {r}')
            cur = region; body.append(text.strip('\n'))
        for r in reversed(cur): body.append(f'//#endregion {r}')
        imports = collections.defaultdict(list)
        for (other, nm) in sorted(needs[l]):
            if layers.get(other, {}).get('module'): imports[layers[other]['module']].append(nm)
        head = cfg.get('header', '')
        uses = ''.join(f'use {m}::{{{", ".join(sorted(set(v)))}}};\n' for m, v in sorted(imports.items()))
        with open(cfg['file'], 'w', encoding='utf-8') as f:
            f.write(head + uses + ('\n' if uses else '') + '\n'.join(body) + '\n')
        print(f'wrote {cfg["file"]}: {len(out[l])} items, imports {dict((k, len(v)) for k, v in imports.items())}')

if __name__ == '__main__':
    mode, path, specp = sys.argv[1], sys.argv[2], sys.argv[3]
    spec = json.load(open(specp))
    (plan if mode == 'plan' else emit)(path, spec)
