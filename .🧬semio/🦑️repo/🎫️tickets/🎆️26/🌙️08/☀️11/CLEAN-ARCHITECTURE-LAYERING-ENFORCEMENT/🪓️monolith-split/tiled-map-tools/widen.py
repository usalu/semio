import os, re, sys, json

TM = '/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map'
FILES = {
    '🦀️.rs': (),
    '🧭️projection/🦀️.rs': ('projection',),
    '🧩️tiles/🦀️.rs': ('tiles',),
    '🧱️vector-tiles/🦀️.rs': ('vector_tiles',),
    '🧱️vector-tiles/📦️mvt-decoder/🦀️.rs': ('vector_tiles', 'mvt_decoder'),
    '🌍️gis-policy/🦀️.rs': ('gis_policy',),
    '🌍️gis-policy/🎨️vector-style/🦀️.rs': ('gis_policy', 'vector_style'),
    '🪢️map-state/🦀️.rs': ('map_state',),
    '🏛️map-host/🦀️.rs': ('map_host',),
    '🏛️map-host/📷️camera-and-picking/🦀️.rs': ('map_host', 'camera_and_picking'),
    '🏛️map-host/🖼️host-scene/🦀️.rs': ('map_host', 'host_scene'),
    '🖱️interaction-and-labels/🦀️.rs': ('interaction_and_labels',),
    '🌐️wasm-session/🦀️.rs': ('wasm_session',),
    '🧪️tests/🔬️unit/🦀️.rs': ('tests',),
    '🧪️tests/🔬️vector-tiles-decode-geometry/🦀️.rs': ('vector_tiles', 'mvt_decoder'),
}
APPLY = '--apply' in sys.argv
ROOT_GROUP = ['🌍️gis-policy/🦀️.rs', '🪢️map-state/🦀️.rs', '🏛️map-host/🦀️.rs', '🏛️map-host/📷️camera-and-picking/🦀️.rs', '🏛️map-host/🖼️host-scene/🦀️.rs', '🖱️interaction-and-labels/🦀️.rs', '🌐️wasm-session/🦀️.rs']
VT_GROUP = ['🧱️vector-tiles/🦀️.rs', '🧱️vector-tiles/📦️mvt-decoder/🦀️.rs', '🌍️gis-policy/🎨️vector-style/🦀️.rs', '🧪️tests/🔬️vector-tiles-decode-geometry/🦀️.rs']
DEFS = ROOT_GROUP + ['🧱️vector-tiles/📦️mvt-decoder/🦀️.rs', '🌍️gis-policy/🎨️vector-style/🦀️.rs']
EXCLUDE = set(json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'exclude.json'), encoding='utf-8'))) if os.path.exists(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'exclude.json')) else set()


def strip(src):
    out = []
    i = 0
    n = len(src)
    while i < n:
        c = src[i]
        if src.startswith('//', i):
            j = src.find('\n', i)
            j = n if j < 0 else j
            out.append(' ' * (j - i))
            i = j
        elif src.startswith('/*', i):
            j = src.find('*/', i + 2)
            j = n if j < 0 else j + 2
            out.append(''.join(ch if ch == '\n' else ' ' for ch in src[i:j]))
            i = j
        elif c == 'r' and re.match(r'r#*"', src[i:i + 8]) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == '_')):
            m = re.match(r'r(#*)"', src[i:])
            hashes = m.group(1)
            end = '"' + hashes
            j = src.find(end, i + len(m.group(0)))
            j = n if j < 0 else j + len(end)
            out.append(''.join(ch if ch == '\n' else ' ' for ch in src[i:j]))
            i = j
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == '\\' else 1
            j = min(n, j + 1)
            out.append(''.join(ch if ch == '\n' else ' ' for ch in src[i:j]))
            i = j
        elif c == "'":
            m = re.match(r"'(\\.[^']*|[^\\'])'", src[i:i + 12])
            if m:
                out.append(' ' * len(m.group(0)))
                i += len(m.group(0))
            else:
                out.append(c)
                i += 1
        else:
            out.append(c)
            i += 1
    return ''.join(out)


texts = {}
stripped = {}
for rel in FILES:
    texts[rel] = open(TM + '/' + rel, encoding='utf-8').read()
    stripped[rel] = strip(texts[rel])

VIS = r'(?:pub(?:\([^)]*\))?\s+)?'
MODS = r'(?:(?:async|unsafe|const|extern\s+"[^"]*")\s+)*'
TOP = re.compile(r'^(\s*)(' + VIS + r')(' + MODS + r')(fn|struct|enum|trait|type|static|const|mod|union)\s+(\w+)')
IMPL = re.compile(r'^\s*(?:unsafe\s+)?impl\b([^{]*)\{?')
FIELD = re.compile(r'^(\s*)(' + VIS + r')(\w+)\s*:(?!:)')
METHOD = re.compile(r'^(\s*)(' + VIS + r')(' + MODS + r')(fn|const)\s+(\w+)')


def candidates(rel):
    """returns list of (lineno, kind, name, owner) for private declarations"""
    out = []
    ctx = []
    depth = 0
    pending = None
    for ln, line in enumerate(stripped[rel].split('\n')):
        top_ctx = ctx[-1] if ctx else None
        at_depth = depth
        in_block = None
        if top_ctx and top_ctx[2] == at_depth:
            in_block = top_ctx
        opener = None
        if at_depth == 0 or (in_block and in_block[0] == 'mod'):
            m = TOP.match(line)
            if m and 'use ' not in line[:m.start(4)]:
                vis, kind, name = m.group(2), m.group(4), m.group(5)
                if not vis.strip() and not (kind == 'mod' and line.rstrip().endswith(';') and False):
                    out.append((ln, kind, name, None, m.start(1) + len(m.group(1))))
                if kind in ('struct', 'enum', 'trait', 'mod', 'union') and '{' in line[m.end():]:
                    opener = (kind, name)
            else:
                m2 = IMPL.match(line)
                if m2 and (at_depth == 0):
                    head = m2.group(1)
                    if ' for ' in head + ' ':
                        opener = ('impl_trait', None)
                    else:
                        tm = re.search(r'(\w+)\s*(?:<[^>]*>)?\s*$', head.strip())
                        opener = ('impl', tm.group(1) if tm else None)
        elif in_block:
            kind = in_block[0]
            if kind == 'struct':
                m = FIELD.match(line)
                if m and not line.strip().startswith('#'):
                    if not m.group(2).strip():
                        out.append((ln, 'field', m.group(3), in_block[1], len(m.group(1))))
            elif kind == 'impl':
                m = METHOD.match(line)
                if m and not m.group(2).strip():
                    out.append((ln, 'method' if m.group(4) == 'fn' else 'assoc', m.group(5), in_block[1], len(m.group(1))))
        opens = line.count('{')
        closes = line.count('}')
        if opener and opens > closes - 0:
            ctx.append((opener[0], opener[1], at_depth + 1))
        depth += opens - closes
        while ctx and depth < ctx[-1][2]:
            ctx.pop()
    return out


def lca(paths):
    first = paths[0]
    n = len(first)
    for p in paths[1:]:
        k = 0
        while k < min(n, len(p)) and first[k] == p[k]:
            k += 1
        n = k
    return first[:n]


def uses(name, kind, owner, rel):
    own = FILES[rel]
    res = []
    for other, mp in FILES.items():
        if other == rel:
            continue
        if rel in VT_GROUP and other not in VT_GROUP:
            continue
        txt = stripped[other]
        if kind in ('method',):
            pat = r'(?:\.|::)' + re.escape(name) + r'\b'
        elif kind == 'assoc':
            pat = r'::' + re.escape(name) + r'\b'
        elif kind == 'field':
            pat = r'\.' + re.escape(name) + r'\b(?!\s*\()'
            if not re.search(pat, txt):
                if owner and re.search(r'\b' + re.escape(owner) + r'\s*\{', txt) and re.search(r'\b' + re.escape(name) + r'\b', txt):
                    res.append(mp)
                continue
        else:
            pat = r'\b' + re.escape(name) + r'\b'
        if re.search(pat, txt):
            res.append(mp)
    return res


edits = {}
report = []
for rel, mp in FILES.items():
    if rel not in DEFS:
        continue
    for ln, kind, name, owner, col in candidates(rel):
        if '%s::%s::%s' % (rel.split('/')[-2] if '/' in rel else 'root', owner, name) in EXCLUDE:
            continue
        users = uses(name, kind, owner, rel)
        ext = [u for u in users if u[:len(mp)] != mp]
        if not ext:
            continue
        r = lca([mp] + ext)
        if r == mp:
            continue
        if len(r) == len(mp) - 1:
            vis = 'pub(super)'
        else:
            vis = 'pub(in crate::tiled_map' + ''.join('::' + s for s in r) + ')'
        edits.setdefault(rel, []).append((ln, col, vis, kind, name, owner))
        report.append((rel, ln + 1, kind, name, owner, vis, ['/'.join(u) or 'root' for u in ext]))

for r in report:
    print(r[0].split('/')[-2] if '/' in r[0] else 'root', r[1], r[2], r[3], r[4], r[5], r[6])
print('total widened', len(report))

if APPLY:
    for rel, es in edits.items():
        lines = texts[rel].split('\n')
        for ln, col, vis, kind, name, owner in sorted(es, key=lambda e: (e[0], -e[1]), reverse=True):
            l = lines[ln]
            lines[ln] = l[:col] + vis + ' ' + l[col:]
        open(TM + '/' + rel, 'w', encoding='utf-8').write('\n'.join(lines))
    json.dump(report, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'widened.json'), 'w'), ensure_ascii=False, indent=1)
    print('applied')
