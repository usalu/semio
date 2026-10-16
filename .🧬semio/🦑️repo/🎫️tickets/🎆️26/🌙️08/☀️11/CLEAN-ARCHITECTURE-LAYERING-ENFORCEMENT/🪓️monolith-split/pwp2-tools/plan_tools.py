#!/usr/bin/env python3
# plan_tools.py -- reproduces the PLAN-PLUGIN-CRATES measurements. Python 3.9+, stdlib only, read-only on the repo.
#   LIST=closure.json bun closure.ts   (Appendix F: canonical gate counter; writes the per-file listing)
#   python3 plan_tools.py census              # item graph, SCCs, crate sizes
#   python3 plan_tools.py moves               # fixed-order min-cost item moves (section 5.2)
#   python3 plan_tools.py home                # VcsArtifactApp method homing (section 7)
import json, re, sys, collections, os
ROOT = os.environ.get('SEMIO_ROOT', '/Users/ueli/Documents/semio/')
P = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/'
CLOSURE = json.load(open(os.environ.get('CLOSURE', os.path.dirname(os.path.abspath(__file__)) + '/../🗑️generated/pwp0-closure.json')))   # {"lib native":[{path,bytes,lines}], "lib wasm32-wasip2":[...]}

def strip(t):
    o = []; i = 0; n = len(t)
    while i < n:
        c = t[i]
        if t.startswith('//', i):
            j = t.find('\n', i); i = n if j < 0 else j; continue
        if t.startswith('/*', i):
            d = 1; i += 2
            while i < n and d:
                if t.startswith('/*', i): d += 1; i += 2
                elif t.startswith('*/', i): d -= 1; i += 2
                else:
                    if t[i] == '\n': o.append('\n')
                    i += 1
            continue
        m = re.match(r'(?:br|rb|r|cr)(#*)"', t[i:i + 12]) if c in 'rbc' and (i == 0 or not (t[i - 1].isalnum() or t[i - 1] == '_')) else None
        if m:
            e = '"' + m.group(1); j = t.find(e, i + m.end()); j = n if j < 0 else j + len(e)
            o.append('""' + '\n' * t[i:j].count('\n')); i = j; continue
        if c == '"':
            j = i + 1
            while j < n and t[j] != '"': j += 2 if t[j] == '\\' else 1
            o.append('""' + '\n' * t[i:j].count('\n')); i = j + 1; continue
        if c == "'":
            m = re.match(r"'(?:\\.[^']*|[^\\'])'", t[i:i + 12])
            if m: o.append("''"); i += m.end(); continue
            m = re.match(r"'[A-Za-z_]\w*", t[i:])
            if m: o.append(' '); i += m.end(); continue
        o.append(c); i += 1
    return ''.join(o)

HEAD = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:(?:async|const|unsafe|extern\s+"[^"]*")\s+)*(fn|struct|enum|union|trait|type|static|const|mod|impl|macro_rules!|thread_local!|use)(?:\b|(?<=!))\s*([A-Za-z_<][\w:<>, ]*)?')
IDENT = re.compile(r'[A-Za-z_]\w*')
GENERIC = set('Cell Inner Node Frame Cursor Source Slot Lane Phase Reserve Admission Vacant Step Ready Pending Closed Open Failed Disabled Enabled Idle Running Done Active new default from into fmt len get set push pop run step next poll drop clone eq hash cmp as_ref self Self super crate std Some None Ok Err Result Option String Vec Box Arc Rc RefCell Mutex HashMap HashSet BTreeMap BTreeSet VecDeque u8 u16 u32 u64 u128 usize i32 i64 bool str f32 f64 mod pub fn let mut const static ref use impl struct enum trait type where for if else match return async await move dyn in as loop while break continue unsafe extern'.split())

def skip_angle(s, i):
    d = 0
    while i < len(s):
        if s[i] == '<': d += 1
        elif s[i] == '>' and not (i > 0 and s[i - 1] == '-'):
            d -= 1
            if d == 0: return i + 1
        i += 1
    return i

def impl_target(head):
    m = re.search(r'\bimpl\s*', head); i = m.end(); s = head
    if i < len(s) and s[i] == '<': i = skip_angle(s, i)
    rest = re.split(r'\bwhere\b|\{', s[i:].strip())[0]
    mm = re.search(r'\bfor\b', rest)
    tr, ty = (rest[:mm.start()].strip(), rest[mm.end():].strip()) if mm else (None, rest)
    tid = IDENT.findall(re.sub(r'<.*', '', re.sub(r'^[&\s]*', '', ty)))
    trid = IDENT.findall(re.sub(r'<.*', '', tr)) if tr else []
    return (tid[-1] if tid else None), (trid[-1] if trid else None)

def outline(path):
    raw = open(path, encoding='utf-8').read(); L = strip(raw).split('\n'); R = raw.split('\n'); n = len(L)
    depth = 0; starts = []
    for l in L: starts.append(depth); depth += l.count('{') - l.count('}')
    ln = 0; items = []
    while ln < n:
        l = L[ln]; m = HEAD.match(l) if starts[ln] == 0 and l and not l[0].isspace() and l[0] not in '}#' else None
        if m:
            d = 0; seen = False; j = ln
            while j < n:
                d += L[j].count('{') - L[j].count('}'); seen = seen or '{' in L[j]
                if (seen and d <= 0) or (not seen and L[j].rstrip().endswith(';')): break
                j += 1
            s = ln
            while s > 0 and R[s - 1].lstrip().startswith(('///', '#[', '//!')): s -= 1
            if m.group(1) != 'use': items.append(dict(s=s + 1, e=j + 1, kind=m.group(1), head=l.strip(), name=(m.group(2) or '').strip()))
            ln = j + 1
        else: ln += 1
    return items, L, R

def tokens(txt):
    out = set()
    for m in IDENT.finditer(txt):
        st = m.start(); w = m.group(0)
        if re.search(r'(?<![A-Za-z0-9_])[A-Z]\w*::$', txt[max(0, st - 48):st]): continue
        pre = txt[max(0, st - 3):st].rstrip()
        if pre.endswith('.') and not pre.endswith('..'): continue
        post = txt[m.end():m.end() + 3].lstrip()
        if post.startswith(':') and not post.startswith('::'): continue
        if w[0].islower() or w[0] == '_':
            if not (post.startswith(('(', '::', '!', '<')) or txt[max(0, st - 2):st].endswith('::')): continue
        out.add(w)
    return out

# ---- crate map (prefix -> crate, longest prefix wins); TOP-* are the instance family, builder, facade root
MAP = {
 '🪟️window/🫧️transient/🔁️document-replacement/': 'TOP-instance', '🎮️command/🎟️lease/': 'TOP-instance', '🎮️command/📥️ingress/': 'TOP-instance', '🪆️child/👁️read/🧪️test-view/': 'TOP-instance',
 '♻️cancellation/': 'operation', '⏳️operation-progress/': 'operation', '🧵️task/': 'operation', '🌱️initialization/': 'operation', '📬️completion/': 'operation',
 '🚪️io/': 'abi', '🧬️schema/': 'abi', '🚪️lifetime/': 'runtime-instances', '🕹️interaction/': 'interaction', '🖱️context-menu/': 'window-kits',
 '🪟️window/🎚️config/': 'window-config', '🪟️window/': 'window-transient',
 '🫧️transient/': 'decl-views', '🎚️config/': 'decl-views', '📝️draft/': 'decl-views', '👥️presence/': 'decl-views',
 '📜️declaration/👁️view/': 'decl-views', '📜️declaration/🪆️child/': 'decl-views', '📜️declaration/🎮️command/': 'decl-views', '📜️declaration/🛠️tool-machine/': 'decl-views', '📜️declaration/⏯️tool-run/': 'decl-views', '📜️declaration/🎞️media/': 'decl-views',
 '📜️declaration/📨️emission/': 'decl-emission', '📜️declaration/👷️job/': 'decl-app', '📜️declaration/🧩️app/': 'decl-app',
 '📜️declaration/🪟️surface/': 'decl-surface', '📜️declaration/📥️natural-file/': 'decl-surface',
 '📜️declaration/📇️registration/': 'decl-registry', '📜️declaration/🌳️tree/': 'decl-registry', '📜️declaration/🦀️.rs': 'decl-registry',
 '🪪️identity/': 'identity', '💡️inference/': 'extension', '🧩️extension/': 'extension', '🛂️describe/': 'extension',
 '🧩️composition/📬️publication/🤝️group/🪟️mounted/': 'publication-mounted', '🧩️composition/📬️publication/': 'composition-publication', '🧩️composition/': 'composition-emission',
 '🪆️child/': 'child', '🗃️document/': 'document-media', '🎞️media/': 'document-media', '🎮️command/': 'command', '👷️job/': 'job', '🧵️retained-command/': 'retained-command', '👥️peer/': 'peer',
 '🌐host/': 'actor-host', '📡️backbone/': 'actor-host', '⚛️reactor/🧵️executor/': 'actor-host',
 '🪟️window-kits/🌳️tree/': 'window-kits-rich', '🪟️window-kits/📊️table/': 'window-kits-rich', '🪟️window-kits/🌍️world3d/': 'window-kits-rich', '🪟️window-kits/': 'window-kits',
 '⏪️time-travel/🧵️instance/': 'TOP-tt', '⏪️time-travel/': 'time-travel', '⏯️tool-run/🧵️instance/': 'TOP-tr', '⏯️tool-run/': 'tool-run', '🛠️tool-machine/': 'tool-run',
 '⚛️reactor/🔌️app/': 'reactor-app', '⚛️reactor/🚪️lifetime/': 'reactor-lifecycle', '⚛️reactor/📨️pending/': 'reactor-lifecycle',
 '⚛️reactor/🪟️surfaces/': 'reactor-surfaces', '⚛️reactor/📥️cold-pair/': 'reactor-surfaces', '⚛️reactor/📸️checkpoint/': 'reactor-surfaces',
 '⚛️reactor/🩹️patches/': 'reactor-patches', '⚛️reactor/💼️jobs/': 'reactor-jobs', '⚛️reactor/🔄️turn/': 'reactor-turn', '⚛️reactor/🎟️context/': 'reactor-context',
 '⚛️reactor/🏃️runtime/': 'runtime-ops', '⚛️reactor/🦀️.rs': 'reactor-core',
 '♻️lifecycle/': 'TOP-instance', '🧵️instance/': 'TOP-instance', '🪜️': 'TOP-ladders',
 '🏗️builder/🧩️app/': 'app-builder', '🏗️builder/🔌️program/': 'plugin-program', '🏗️builder/': 'TOP-builder', '🦀️.rs': 'TOP-root', '⚖️authoring-law/': 'TOP-root', '📦️packages/': 'TOP-root',
}
RT = '⚛️reactor/🏃️runtime/🦀️.rs'; TT = '⏪️time-travel/🦀️.rs'; RC = '⚛️reactor/🦀️.rs'
def file_crate(f):
    best = None
    for k, v in MAP.items():
        if f.startswith(k) and (best is None or len(k) > len(best[0])): best = (k, v)
    return best[1] if best else 'UNMAPPED:' + f
def item_crate(f, s):
    if f == RT:   # function families of the 5,465-line runtime file (start line of the item)
        return ('reactor-lifecycle' if s < 600 else 'runtime-instances' if s < 1881 else 'runtime-ops' if s < 3838 else 'runtime-exchange' if s < 4650
                else 'TOP-root' if s < 5189 else 'extension' if s < 5588 else 'runtime-ops')
    if f == TT:   # regions of the 3,444-line time-travel file
        return 'command' if 1773 <= s <= 2277 else 'time-travel-panel' if (1140 <= s <= 1771 or s >= 2524) else 'time-travel'
    if f == RC and 1236 <= s <= 1829: return 'reactor-wit-bridge'
    return file_crate(f)

# fixed rank order of the lower crates (low -> high), found by the search of section 5.2
ORDER = ["abi", "actor-host", "identity", "interaction", "operation", "window-config", "window-transient", "window-kits", "window-kits-rich", "decl-views", "decl-emission", "decl-app", "app-builder", "decl-surface", "child", "composition-emission", "composition-publication", "document-media", "job", "command", "peer", "publication-mounted", "retained-command", "reactor-app", "decl-registry", "plugin-program", "reactor-lifecycle", "runtime-instances", "runtime-ops", "extension", "reactor-patches", "reactor-surfaces", "reactor-core", "reactor-jobs", "runtime-exchange", "reactor-turn", "reactor-wit-bridge", "reactor-context", "time-travel-panel", "tool-run", "time-travel"]
SUBSYSTEM = {c: s for s, cs in {'F': 'abi actor-host identity interaction operation', 'W': 'window-config window-transient window-kits window-kits-rich', 'D': 'decl-views decl-emission decl-app decl-surface app-builder',
             'O': 'child composition-emission composition-publication publication-mounted document-media job command peer retained-command',
             'R': 'reactor-app decl-registry plugin-program reactor-lifecycle runtime-instances runtime-ops extension reactor-patches reactor-surfaces reactor-core reactor-jobs runtime-exchange reactor-turn reactor-wit-bridge reactor-context',
             'T': 'time-travel-panel tool-run time-travel'}.items() for c in cs.split()}

def tarjan(nodes, adj):
    sys.setrecursionlimit(100000)
    idx = {}; low = {}; st = []; on = set(); out = []; c = [0]
    def sc(v):
        idx[v] = low[v] = c[0]; c[0] += 1; st.append(v); on.add(v)
        for w in adj.get(v, ()):
            if w not in idx: sc(w); low[v] = min(low[v], low[w])
            elif w in on: low[v] = min(low[v], idx[w])
        if low[v] == idx[v]:
            comp = []
            while True:
                w = st.pop(); on.discard(w); comp.append(w)
                if w == v: break
            out.append(comp)
    for v in nodes:
        if v not in idx: sc(v)
    return out

def build():
    files = sorted({x['path'] for k in ('lib native', 'lib wasm32-wasip2') for x in CLOSURE[k] if x['path'].endswith('.rs')})
    items = []; text = {}
    for p in files:
        its, L, R = outline(ROOT + p); f = p.replace(P, ''); text[f] = (L, R)
        for it in its:
            it['file'] = f; k = it['s'] - 1; at = []
            while k < len(R) and R[k].lstrip().startswith(('#[', '///', '//!')): at.append(R[k]); k += 1
            atx = ' '.join(at)
            it['cfgtest'] = any(re.search(r'\btest\b', x) and not re.search(r'not\(\s*test', x) for x in re.findall(r'cfg\((.*?)\)\]', atx)) or '#[test]' in atx
            body = '\n'.join(L[it['s'] - 1:it['e']]); it['toks'] = tokens(body)
            if it['kind'] == 'impl': it['target'], it['trait'] = impl_target(it['head']); it['defname'] = None
            else:
                nm = re.match(r'[A-Za-z_]\w*', it['name']) if it['name'] else None; it['defname'] = nm.group(0) if nm else None
            items.append(it)
    for i, it in enumerate(items): it['id'] = i
    defs = collections.defaultdict(list)
    for it in items:
        d = it['defname']
        if d and d not in ('_', 'tests') and not re.fullmatch(r'[a-z][a-z0-9]{0,12}', d) and it['kind'] not in ('impl', 'use', 'mod') and not it['cfgtest']: defs[d].append(it['id'])
    owner = {}
    for it in items:
        if it['kind'] == 'impl' and it.get('target') and not it.get('trait'):
            d = [j for j in defs.get(it['target'], []) if items[j]['kind'] in ('struct', 'enum', 'union', 'type', 'trait')]
            if len(d) == 1: owner[it['id']] = d[0]
    return files, items, defs, owner

def graph(items, defs, owner):
    unit = lambda i: owner.get(i, i)
    size = collections.Counter(); adj = collections.defaultdict(set); crate_of = {}
    for it in items:
        f = it['file']
        if '/🧪️tests/' in f or f.startswith('🧪️') or it['cfgtest']: continue
        u = unit(it['id']); size[u] += it['e'] - it['s'] + 1
    for u in size: crate_of[u] = item_crate(items[u]['file'], items[u]['s'])
    for it in items:
        u = unit(it['id'])
        if u not in size: continue
        for t in it['toks']:
            if t in GENERIC or t == (it['defname'] or it.get('target')): continue
            ds = defs.get(t)
            if not ds or len(ds) > 3: continue
            for j in ds:
                v = unit(j)
                if v != u and v in size: adj[u].add(v)
    return size, adj, crate_of

def simulate(size, adj, crate_of, items, order=ORDER):
    low = [u for u in size if not crate_of[u].startswith('TOP')]; lows = set(low)
    ladj = {u: {v for v in adj.get(u, ()) if v in lows} for u in low}
    sccs = tarjan(low, ladj); node = {u: k for k, c in enumerate(sccs) for u in c}
    members = collections.defaultdict(list)
    for u in low: members[node[u]].append(u)
    nl = {k: sum(size[u] for u in m) for k, m in members.items()}
    ncr = {}
    for k, m in members.items():
        c = collections.Counter()
        for u in m: c[crate_of[u]] += size[u]
        ncr[k] = c.most_common(1)[0][0]
    rank = {c: i for i, c in enumerate(order)}
    nadj = collections.defaultdict(set); rn = collections.defaultdict(set)
    for u in low:
        for v in ladj[u]:
            a, b = node[u], node[v]
            if a != b: nadj[a].add(b); rn[b].add(a)
    mult = lambda c1, c2: 1.0 if SUBSYSTEM.get(c1) == SUBSYSTEM.get(c2) else 2.0
    moves = []
    while True:
        be = [(a, b) for a, bs in nadj.items() for b in bs if rank[ncr[b]] > rank[ncr[a]]]
        if not be: break
        best = None
        for a, b in be:
            ra, rb = rank[ncr[a]], rank[ncr[b]]
            D = set(); st = [b]
            while st:
                x = st.pop()
                if x in D or rank[ncr[x]] <= ra: continue
                D.add(x); st.extend(nadj.get(x, ()))
            Up = set(); st = [a]
            while st:
                x = st.pop()
                if x in Up or rank[ncr[x]] >= rb: continue
                Up.add(x); st.extend(rn.get(x, ()))
            cd = sum(nl[x] * mult(ncr[x], ncr[a]) for x in D); cu = sum(nl[x] * mult(ncr[x], ncr[b]) for x in Up)
            opt = (cd, 'down', D, ncr[a]) if cd <= cu else (cu, 'up', Up, ncr[b])
            if best is None or opt[0] < best[0]: best = opt
        _, kind, S, dest = best
        for x in S:
            moves.append((nl[x], items[members[x][0]]['defname'] or items[members[x][0]]['head'][:40], ncr[x], dest, kind)); ncr[x] = dest
    final = {u: ncr[node[u]] for u in low}
    return final, moves, sccs

def census():
    files, items, defs, owner = build(); size, adj, crate_of = graph(items, defs, owner)
    print(len(files), 'lib files;', len(items), 'items;', len(size), 'production units;', sum(size.values()), 'unit lines')
    sc = sorted((c for c in tarjan(sorted(size), adj) if len(c) > 1), key=lambda c: -sum(size[x] for x in c))
    print('unit SCCs > 1:', [(len(c), sum(size[x] for x in c)) for c in sc[:8]])
    low = [u for u in size if not crate_of[u].startswith('TOP')]
    final, moves, _ = simulate(size, adj, crate_of, items)
    lines = collections.Counter()
    for u, c in final.items(): lines[c] += size[u]
    print('moves', len(moves), 'lines', sum(m[0] for m in moves))
    for c in ORDER: print(f'{c:24s} {lines[c]:6d}')

def moves_cmd():
    files, items, defs, owner = build(); size, adj, crate_of = graph(items, defs, owner)
    final, moves, _ = simulate(size, adj, crate_of, items)
    agg = collections.defaultdict(list)
    for l, n, f, t, k in moves: agg[(f, t)].append((l, n))
    for (f, t), v in sorted(agg.items(), key=lambda kv: -sum(x[0] for x in kv[1])):
        v.sort(reverse=True); print(f'{sum(x[0] for x in v):5d} {f} -> {t}: ' + ', '.join(f'{n[:36]}({l})' for l, n in v[:6]))

RULES = [(r'^app$|^mounted_policy$', 'core'), (r'^constructor_', 'constructor'),
 (r'^(store|config_store|draft_store|presence_store|transient_store|interaction_store)$|_one_item_factory$|local_root_retirement_factory$|^unsupported_publication_contracts$|^window_config_store$|^window_transient_store$|^retired_window_transient_stores$', 'stores'),
 (r'^close_', 'close'), (r'^maintenance_', 'maintenance'), (r'^peer_|^own_color$|^presence_peer_retirements$|^pending_presence$|^presence_marked_keys$', 'roster'), (r'action_ingress', 'ingress'),
 (r'^(children|private_child_groups|child_|followed_parent_generation|pending_child_pins|composition)', 'children'),
 (r'local_interaction|^interaction_|snapshot_read_return_stage|^config_snapshot_read_returns$', 'interaction'),
 (r'^command_|^log_generation$|^history_|^shell_|^next_command_seq$|^supersedes$|^cache$|^pending_command_prune$', 'history'),
 (r'^pending_reserved$|^reserved_commit|^pending_configuration|^pending_render|^pending_document_window_admission$|^dispatch_report$|^last_emit_wire$', 'pending'),
 (r'^media_|^segmented_|^snapshot_retirements$|^current_media|^pending_media_import', 'media'),
 (r'^envelope_|^store_replacement|^import_staging$|^document_archive|^document_snapshot_read_returns$', 'ingest'),
 (r'^host_event|^delivering_host_event$|^invocation_kind$', 'hostevent'), (r'^pending_transaction', 'txn'),
 (r'^tool_runs$', 'toolrun'), (r'^time_travel$', 'timetravel'), (r'^tool_machines$', 'toolmachine'),
 (r'^typed_|^tool_|^latest_wins|^bounded_tool|^framework_tool|^app_tool|^instance_operation_owner$|^live_runtime_instance_id$|^agent_lane_preview$|^pending_tool_lease|^operation_progress_retired$|^registry$|^test_tool_clock$', 'tools')]
COMP_ORDER = ['core', 'constructor', 'stores', 'roster', 'ingress', 'interaction', 'history', 'children', 'pending', 'tools', 'media', 'ingest', 'hostevent', 'txn', 'toolmachine', 'timetravel', 'toolrun', 'maintenance', 'close']

def home():
    files, items, defs, owner = build()
    va = [it for it in items if it['defname'] == 'VcsArtifactApp'][0]
    body = '\n'.join(text_of(items, va)); inner = body[body.index('{') + 1:body.rindex('}')]
    cur = ''; d = 0; parts = []; prev = ''
    for ch in inner:
        if ch in '<([{': d += 1
        elif ch in '>)]}' and not (ch == '>' and prev in '-='): d -= 1
        prev = ch
        if ch == ',' and d == 0: parts.append(cur); cur = ''
        else: cur += ch
    fields = []
    for p in parts + [cur]:
        m = re.match(r'(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:', re.sub(r'#\[[^\]]*\]', '', p).strip())
        if m: fields.append(m.group(1))
    comp = {f: next((c for pat, c in RULES if re.search(pat, f)), 'UNASSIGNED') for f in fields}
    methods = []
    for it in items:
        if it['kind'] == 'impl' and it.get('target') == 'VcsArtifactApp':
            seg = text_of(items, it); depth = 0; cur = None
            for k, l in enumerate(seg):
                before = depth
                m = re.match(r'\s*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+)', l)
                if before == 1 and m:
                    if cur: cur['end'] = k
                    cur = dict(name=m.group(1), start=k, file=it['file'], trait=it.get('trait')); methods.append(cur)
                depth = before + l.count('{') - l.count('}')
                if k == 0 and '{' in l: depth = max(depth, 1)
            if cur: cur['end'] = len(seg)
            for m in methods:
                if 'text' not in m and m['file'] == it['file']: m['text'] = '\n'.join(seg[m['start']:m.get('end', len(seg))])
    fn = set(fields); by = collections.defaultdict(list)
    for m in methods: by[m['name']].append(m)
    uniq = {n: v[0] for n, v in by.items() if len(v) == 1}; rk = {c: i for i, c in enumerate(COMP_ORDER)}; USE = len(COMP_ORDER)
    for m in methods:
        m['lines'] = m['text'].count('\n') + 1; m['fields'] = sorted(set(re.findall(r'\bself\.(\w+)', m['text'])) & fn)
        m['calls'] = sorted(set(re.findall(r'\b(?:self|Self)(?:\.|::)(\w+)\s*(?:\(|::<)', m['text'])))
        cs = {comp[f] for f in m['fields']}; base = max([rk[c] for c in cs], default=-1)
        if len(cs - {'core'}) >= 4: base = USE
        if m.get('trait') == 'PluginApp': base = USE + 1
        m['rank'] = base
    ch = True
    while ch:
        ch = False
        for m in methods:
            for cn in m['calls']:
                t = uniq.get(cn)
                if t and t is not m and t['rank'] > m['rank']: m['rank'] = t['rank']; ch = True
    tot = collections.Counter()
    for m in methods: tot[COMP_ORDER[m['rank']] if 0 <= m['rank'] < USE else ('free' if m['rank'] < 0 else 'usecase' if m['rank'] == USE else 'pluginapp')] += m['lines']
    print(len(fields), 'fields', len(methods), 'methods', sum(tot.values()), 'lines'); print(dict(tot.most_common()))
    dist = collections.Counter(min(len({comp[f] for f in m['fields']}), 4) for m in methods)
    print('components touched per method (0,1,2,3,4+):', [dist[k] for k in range(5)])
    if '--dump' in sys.argv:
        for m in methods:
            r = m['rank']; h = COMP_ORDER[r] if 0 <= r < USE else ('free' if r < 0 else 'usecase' if r == USE else 'pluginapp')
            print('\t'.join([m['file'], m['name'], str(m['lines']), h, ','.join(sorted({comp[f] for f in m['fields']}))]))
def text_of(items, it):
    return globals()['_TEXT'][it['file']][0][it['s'] - 1:it['e']]

if __name__ == '__main__':
    cmd = sys.argv[1] if len(sys.argv) > 1 else 'census'
    if cmd == 'home':
        _files, _items, _d, _o = build()
        _TEXT = {}
        for p in _files: L, R = outline(ROOT + p)[1:]; _TEXT[p.replace(P, '')] = (L, R)
    {'census': census, 'moves': moves_cmd, 'home': home}[cmd]()
