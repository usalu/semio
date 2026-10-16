#!/usr/bin/env python3
"""🏗️ Plan for the os-hub binary: splits every route module into its HubState-free part (T) and its HubState-dependent part (H) and mounts them in seven producer crates."""
import os, sys, collections, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve, itemgraph, ws as wsm
from helpers import *

B = P('🏗️bootstrap/')

# module folder -> (module name in the old root, T destination folder or None for the original file, H destination folder or None for the original file)
MODULES = collections.OrderedDict([
    ('🧰️http-support', ('support', None, None)),
    ('👥️presence', ('presence', None, None)),
    ('🛑️lifecycle', ('lifecycle', None, None)),
    ('🩺️readiness', ('readiness', None, '📡️probes')),
    ('🚀️startup', ('startup', None, None)),
    ('🚦️transport', ('transport', None, '⚖️limits')),
    ('🎟️socket-grants', ('socket_grants', None, '🔌️live')),
    ('🧭️open-plan', ('open_plan', None, None)),
    ('🛡️guard', ('guard', None, '🔐️sessions')),
    ('🔑️access-grants', ('access_grants', '🧱️types', None)),
    ('🪞️pair-blobs', ('pair_blobs', None, '📨️routes')),
    ('🛠️admin', ('admin', '🧱️runtime', None)),
    ('🌱️artifact-creation', ('artifact_creation', None, '📨️routes')),
    ('📣️directory-events', ('directory_events', '🧱️types', None)),
    ('📇️directory-commands', ('directory_commands', '🧱️types', None)),
    ('🔌️document-socket', ('document_socket', '🧱️types', None)),
    ('📌️check-in', ('check_in', '🧱️types', None)),
    ('🧩️assets', ('assets', '🧱️types', None)),
    ('💡️inference', ('inference', '🧱️types', None)),
    ('🪪️session-agent', ('session_agent', '🧱️types', None)),
    ('🧠️state', ('state', '🧱️seams', None)),
    ('🛣️routes', ('routes', None, None)),
])

CRATE_OF = {  # (module, layer) -> crate ident
    **{(m, 'T'): 'semio_hub_runtime' for m in ('🧰️http-support', '👥️presence', '🛑️lifecycle', '🩺️readiness', '🚀️startup', '🚦️transport')},
    **{(m, 'T'): 'semio_hub_ledgers' for m in ('🎟️socket-grants', '🧭️open-plan', '🛡️guard', '🔑️access-grants', '🪞️pair-blobs')},
    **{(m, 'T'): 'semio_hub_route_models' for m in ('🛠️admin', '🌱️artifact-creation', '📣️directory-events', '📇️directory-commands', '🔌️document-socket', '📌️check-in', '🧩️assets', '💡️inference', '🪪️session-agent', '🧠️state')},
    **{(m, 'H'): 'semio_hub_session_routes' for m in ('🛡️guard', '🚦️transport', '🩺️readiness', '🔑️access-grants', '🪪️session-agent', '📇️directory-commands')},
    **{(m, 'H'): 'semio_hub_document_routes' for m in ('🎟️socket-grants', '📣️directory-events', '🔌️document-socket', '📌️check-in', '🛠️admin', '🌱️artifact-creation', '💡️inference', '🧩️assets', '🪞️pair-blobs')},
    ('🧠️state', 'H'): 'semio_hub_state',
    ('🛣️routes', 'H'): 'semio_hub_routes',
}

ROOTS = collections.OrderedDict([
    ('semio_hub_runtime', B + '⚙️runtime/🦀️.rs'),
    ('semio_hub_ledgers', B + '📒️ledgers/🦀️.rs'),
    ('semio_hub_route_models', B + '🧱️models/🦀️.rs'),
    ('semio_hub_state', B + '🧠️state/🦀️.rs'),
    ('semio_hub_session_routes', B + '🛰️session-routes/🦀️.rs'),
    ('semio_hub_document_routes', B + '📨️document-routes/🦀️.rs'),
    ('semio_hub_routes', B + '🛣️routes/🦀️.rs'),
])

DOCS = {
    'semio_hub_runtime': '//! ⚙️ Hub runtime foundations: startup supervisors and controls, readiness, transport middleware, lifecycle and the shared HTTP support that carry no `HubState`.',
    'semio_hub_ledgers': '//! 📒️ Hub ledgers: socket grants, document open plans, the authentication guard types and the pair and access-grant vocabulary that carry no `HubState`.',
    'semio_hub_route_models': '//! 🧱️ Hub route models: the admin, artifact-creation, directory, document-socket, check-in and session vocabulary and the `HubState` test seams.',
    'semio_hub_session_routes': '//! 🛰️ Hub session routes: authentication guard, readiness probes, transport limits, access grants, agent sessions and directory commands over `HubState`.',
    'semio_hub_document_routes': '//! 📨️ Hub document routes: document sockets, directory events, check in, administration, artifact creation, inference and asset routes over `HubState`.',
}


def th_split(base_ws):
    """Return (T names, H names) per module folder from the item graph of the pristine bin (HubState-reachability)."""
    hcmod.FS = base_ws
    model, nodes, edges = itemgraph.build(P('🏗️bootstrap/🦀️.rs'), base_ws)
    rev = collections.defaultdict(set)
    for a, bs in edges.items():
        for b in bs:
            rev[b].add(a)
    hs = [k for k in nodes if k[2] == 'HubState']
    H = set(hs)
    st = list(hs)
    while st:
        v = st.pop()
        for u in rev.get(v, ()):
            if u not in H:
                H.add(u)
                st.append(u)
    out = collections.defaultdict(lambda: {'T': [], 'H': []})
    for k, nd in nodes.items():
        f, chain, name = k
        if not f.startswith(B) or chain:
            continue
        rel = f[len(B):]
        if '/🧪️' in rel or rel.startswith('🧪️'):
            continue
        mod = rel.split('/')[0]
        if mod not in MODULES:
            continue
        if name.startswith('<impl') or nd['kind'] in ('mod',):
            continue
        out[mod]['H' if k in H else 'T'].append(name)
    return out


def bin_structural(ws, E, base_ws):
    ag = B + '🔑️access-grants/🦀️.rs'
    standalone = P('🧪️tests/🔬️standalone/🦀️.rs')
    text = ws.read(standalone).rstrip('\n')
    head, _, rest = text.partition('\n')
    replace_once(ws, ag, '#[cfg(test)]\ninclude!("../../🧪️tests/🔬️standalone/🦀️.rs");', '#[cfg(test)]\n' + carve.rebase_paths(head + '\n' + rest, standalone, ag))
    ws.delete(standalone)
    st = B + '🚀️startup/🦀️.rs'
    replace_once(ws, st, '''

    fn catalog_progress(&self) -> Option<Arc<TrustedCatalogLoadProgressCellV1>> {
        Some(Arc::clone(&self.progress.catalog))
    }
}''', '''
}''')
    replace_once(ws, st, 'TrustedCatalogLoader::load_current(data_dir, providers, &context).await', 'TrustedCatalogLoader::load_current_observed(data_dir, providers, &progress.catalog, &context).await')
    th = th_split(base_ws)
    for mod, (modname, tdir, hdir) in MODULES.items():
        orig = B + mod + '/🦀️.rs'
        names = th[mod]
        if tdir and names['T']:
            dest = B + mod + '/' + tdir + '/🦀️.rs'
            create(ws, dest, f'//! 🧱️ The `{modname}` vocabulary that carries no `HubState`: types, ledgers and helpers the state and the routes build on.\n')
            E.move(orig, names['T'], dest)
            E.proxy[dest] = orig
        if hdir and names['H']:
            dest = B + mod + '/' + hdir + '/🦀️.rs'
            create(ws, dest, f'//! 🛣️ The `{modname}` handlers that act over `HubState`.\n')
            E.move(orig, names['H'], dest)
            E.proxy[dest] = orig
    # the directory event page source impl belongs with its trait
    E.move(B + '📣️directory-events/🦀️.rs', [], B + '📣️directory-events/🧱️types/🦀️.rs', with_impls=False, impl_idents=['HubDirectories'])
    add_state_parts(ws)
    # crate roots
    def mount(mod, layer):
        modname, tdir, hdir = MODULES[mod]
        sub = tdir if layer == 'T' else hdir
        has = th[mod][layer]
        if not has:
            return None
        rel = f'../{mod}/{sub}/🦀️.rs' if sub else f'../{mod}/🦀️.rs'
        return f'#[path = "{rel}"]\npub mod {modname};\n'
    for ident, root in ROOTS.items():
        if ident in ('semio_hub_state', 'semio_hub_routes'):
            continue
        text = DOCS[ident] + '\n\n'
        for (mod, layer), c in CRATE_OF.items():
            if c != ident:
                continue
            m = mount(mod, layer)
            if m:
                text += m + '\n'
        create(ws, root, text)
        E.proxy[root] = B + '🦀️.rs'
    # state crate: the HubState file is the root; its seams child lives in the models crate
    # routes crate: the router file is the root.
    return th


def bin_root_cleanup(ws, E):
    binroot = B + '🦀️.rs'
    mods = [v[0] for v in MODULES.values()] + ['test_artifact_root', 'tests', 'two_client_document']
    drop_mounts(ws, binroot, mods, glob_reexports=True)
    remove_items(ws, binroot, lambda it, toks: it.kind in ('use', 'extern_crate') and any(c.strip() == 'test' for c in it.cfg))


import re


def add_state_parts(ws):
    f = B + '🧠️state/🦀️.rs'
    src = ws.read(f)
    m = re.search(r'#\[derive\(Clone\)\]\npub\(crate\) struct HubState \{\n(.*?)\n\}\n', src, re.S)
    lines = m.group(1).split('\n')
    prod, seams = [], []
    pending = None
    for raw in lines:
        l = raw.strip()
        if not l or l.startswith('///'):
            continue
        if l.startswith('#[cfg('):
            pending = l
            continue
        if l.startswith('#['):
            continue
        mm = re.match(r'pub\(crate\) (\w+): (.*),$', l)
        if mm:
            (seams if pending else prod).append((pending, mm.group(1), mm.group(2)))
            pending = None
    parts = ['/// 🧩️ Every production field of [`HubState`]; the test seams are filled in by [`From`] only where a test build asks for them.', 'pub struct HubStateParts {']
    parts += [f'    pub {n}: {t},' for _, n, t in prod]
    parts.append('}')
    parts.append('')
    parts.append('impl From<HubStateParts> for HubState {')
    parts.append('    fn from(parts: HubStateParts) -> Self {')
    parts.append('        Self {')
    parts += [f'            {n}: parts.{n},' for _, n, t in prod]
    for attr, n, t in seams:
        parts.append(f'            {attr}')
        parts.append(f'            {n}: None,')
    parts.append('        }')
    parts.append('    }')
    parts.append('}')
    src = src[:m.end()] + '\n' + '\n'.join(parts) + '\n' + src[m.end():]
    import carve
    ws.write(f, carve.open_file_test_attrs(src))
    # bin root: the literal becomes the parts literal
    root = B + '🦀️.rs'
    r = ws.read(root)
    i = r.index('let state = HubState {')
    j = r.index('\n        };', i)
    lit = r[i:j + len('\n        };')]
    lit2 = lit.replace('let state = HubState {', 'let state: HubState = semio_hub_state::HubStateParts {', 1)
    lit2 = re.sub(r'\n\s*#\[cfg\(test\)\]\n\s*\w+: None,', '', lit2)
    lit2 = lit2.replace('\n        };', '\n        }\n        .into();')
    ws.write(root, r.replace(lit, lit2, 1))
