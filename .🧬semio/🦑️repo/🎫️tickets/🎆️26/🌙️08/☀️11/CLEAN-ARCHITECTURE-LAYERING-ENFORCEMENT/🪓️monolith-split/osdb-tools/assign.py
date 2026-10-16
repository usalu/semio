"""🎯 Item-to-crate assignment for the four split modules, and test item assignment.

Produces ASSIGN (fid -> {item index: lib}) and TESTS (fid -> {item index: [libs]}).
"""
import collections
import json
import os
import re

from lib import *
from model import parse_items
from rs import strip, idents
from sources import load_all
from graph import load as gload, edges as gedges
from scc2 import sccs

CUT1 = 2933
CUT2 = 4900


def node_name(it):
    return it.self_type if it.kind == 'impl' else it.name


def wal_assign(srcs):
    fid = '📝️wal/🦀️.rs'
    ov = {}
    out = {}
    for i, it in enumerate(srcs[fid].items):
        if (it.kind == 'use' and not it.vis.startswith('pub')) or (it.kind == 'mod' and srcs[fid]._is_decl(it)):
            continue
        out[i] = 'db_wal_log' if it.header + 1 <= 2451 else 'db_wal'
    return {fid: out}


def artifact_assign(srcs):
    fid = '🗿️artifact/🦀️.rs'
    ov = {'MAX_RECENT_TOUCHES': 'db_artifact_contract', 'ArtifactMessage': 'db_artifact_contract', 'wire_err': 'db_artifact_contract'}
    out = {}
    for i, it in enumerate(srcs[fid].items):
        if (it.kind == 'use' and not it.vis.startswith('pub')) or (it.kind == 'mod' and srcs[fid]._is_decl(it)):
            continue
        nm = node_name(it)
        h = it.header + 1
        if nm in ov:
            out[i] = ov[nm]
        elif h <= 427:
            out[i] = 'db_artifact_contract'
        elif h <= 1047:
            out[i] = 'db_artifact_contract'
        elif h <= 3096:
            out[i] = 'db_artifact_engine'
        elif h <= 4669:
            out[i] = 'db_artifact_history'
        else:
            out[i] = 'db_artifact'
    return {fid: out}


def engine_assign(srcs):
    fid = '⚙️engine/🦀️.rs'
    ov = {'DATABASE_CAPABILITY_OPEN_LIVE': 'db_engine_capability_open', 'DATABASE_CATALOG_READ_LIVE': 'db_engine_catalog_read',
          'DATABASE_CATALOG_BOOTSTRAP_LIVE': 'db_engine_catalog_bootstrap', 'DATABASE_CREATE_CATALOG_LIVE': 'db_engine_catalog_create',
          'HistoryView': 'db_engine_handle', 'DatabaseCreateCatalogToken': 'db_engine_base', 'ArtifactHandleLease': 'db_engine_handle',
          'ArtifactCompactionFuture': 'db_engine_handle', 'ArtifactHandleJournalSinkV1': 'db_engine_handle',
          'DATABASE_DOCUMENT_MOUNT_WAITERS': 'db_engine_base', 'DATABASE_DOCUMENT_MOUNT_WORK_LIVE': 'db_engine_base',
          'database_document_mount_work_live_slots': 'db_engine_base'}
    out = {}
    for i, it in enumerate(srcs[fid].items):
        if (it.kind == 'use' and not it.vis.startswith('pub')) or (it.kind == 'mod' and srcs[fid]._is_decl(it)):
            continue
        nm = node_name(it)
        h = it.header + 1
        if it.kind == 'use' and h == 4396:
            out[i] = 'db_engine_handle'
            continue
        if nm in ov:
            g = ov[nm]
        elif nm.startswith('DatabaseDocumentMount'):
            g = 'db_engine_base'
        elif h <= 1333:
            g = 'db_engine_capability_open' if h >= 53 else 'db_engine'
        elif h <= 2639:
            g = 'db_engine_catalog_read'
        elif h <= 4040:
            g = 'db_engine_catalog_bootstrap'
        elif h <= 4638:
            g = 'db_engine_base'
        elif h <= 6718:
            g = 'db_engine_catalog_create'
        elif h <= 8247:
            g = 'db_engine'
        else:
            g = 'db_engine_handle'
        out[i] = g
    return {fid: out}


def storage_assign(srcs):
    core = '🗄️storage/🦀️.rs'
    wr = '🗄️storage/🔐️writer/🦀️.rs'
    rl = '🗄️storage/🔐️writer/🔔️release/🦀️.rs'

    def region(fid, it):
        if fid != core:
            return 'kernel'
        h = it.header + 1
        if h < 5688:
            return 'kernel'
        if h < 5889:
            return 'contract'
        if h < 6567:
            return 'dispatch'
        if h < 7698:
            return 'memory'
        if h < 8961:
            return 'fs'
        return None

    files = {'core': f'{PRISTINE}/{core}', 'writer': f'{PRISTINE}/{wr}', 'release': f'{PRISTINE}/{rl}'}
    nodes = gload(files)
    K = {}
    for k, n in nodes.items():
        gs = {region({'core': core, 'writer': wr, 'release': rl}[f], it) for f, it in n.parts if it.kind != 'impl'} or {region({'core': core, 'writer': wr, 'release': rl}[n.parts[0][0]], n.parts[0][1])}
        if 'kernel' in gs and not k.startswith('tests@'):
            K[k] = n
    g = gedges(K)
    comps = sccs(g)
    cum = 0
    lay = {}
    for c in comps:
        n = sum(K[x].lines for x in c)
        grp = 'db_io_core' if cum + n <= CUT1 else ('db_io_registry' if cum + n <= CUT2 else 'db_io_driver')
        for x in c:
            lay[x] = grp
        cum += n
    rank = {'db_io_core': 0, 'db_io_registry': 1, 'db_io_driver': 2}
    # contract-region nodes (the storage traits) and aliases sink to the lowest group their dependencies allow
    contract_nodes = [k for k, n in nodes.items() if k not in K and any(region({'core': core, 'writer': wr, 'release': rl}[f], it) == 'contract' for f, it in n.parts)]
    for k, n in nodes.items():
        if k in K and ('type' in n.kinds) and 'impl' not in n.kinds and not k.startswith('<'):
            deps = [lay[w] for w in g.get(k, ()) if w in lay]
            if deps:
                lay[k] = max(deps, key=lambda x: rank[x])
    cg = gedges({k: nodes[k] for k in contract_nodes + list(K)})
    contract_lay = {}
    for k in contract_nodes:
        deps = [lay[w] for w in cg.get(k, ()) if w in lay]
        contract_lay[k] = max(deps, key=lambda x: rank[x]) if deps else 'db_io_core'
    # anonymous nodes (const _ asserts, impls for foreign types) follow the highest group they reference
    for k, n in K.items():
        if k.startswith('<'):
            refs = [lay[w] for w in g[k] if w in lay]
            lay[k] = max(refs, key=lambda x: rank[x]) if refs else 'db_io_driver'
    out = {core: {}, wr: {}, rl: {}}
    for fid in (core, wr, rl):
        s = srcs[fid]
        for i, it in enumerate(s.items):
            if it.kind == 'mod' and it.name == 'claimed_backend':
                out[fid][i] = 'db_io_core'
                continue
            if (it.kind == 'use' and not it.vis.startswith('pub')) or (it.kind == 'mod' and s._is_decl(it)):
                continue
            r = region(fid, it)
            nm = node_name(it)
            if it.kind == 'use' and 'writer' in s.lines[it.header]:
                continue
            if it.kind == 'mod' and nm == 'claimed_backend':
                out[fid][i] = 'db_io_core'
                continue
            if it.kind == 'mod' and nm in ('tests', 'fence_conformance') and fid != core:
                continue
            if r == 'kernel':
                key = nm
                if it.kind == 'macro_call':
                    key = None
                if not nm or nm == '_':
                    key = f'<{it.kind}@{ {core: "core", wr: "writer", rl: "release"}[fid] }:{it.header + 1}>'
                lib = lay.get(key)
                if lib is None:
                    lib = 'db_io_driver'
                out[fid][i] = lib
            elif r == 'contract':
                ck = nm if nm and nm != '_' else None
                out[fid][i] = contract_lay.get(ck, 'db_io_core')
            elif r == 'dispatch':
                out[fid][i] = 'db_storage'
            elif r == 'memory':
                out[fid][i] = 'db_storage_memory'
            elif r == 'fs':
                out[fid][i] = 'db_storage_fs'
    return out


ORDERS = {
    'db_wal': ['db_wal_log', 'db_wal'],
    'db_artifact': ['db_artifact_history', 'db_artifact_contract', 'db_artifact_engine', 'db_artifact'],
    'db_engine': ['db_engine_base', 'db_engine_capability_open', 'db_engine_catalog_read', 'db_engine_catalog_bootstrap', 'db_engine_catalog_create', 'db_engine_handle', 'db_engine'],
    'db_storage': ['db_io_core', 'db_io_registry', 'db_io_driver', 'db_storage_memory', 'db_storage_fs', 'db_storage'],
    'db_fault_testing': ['db_storage', 'db_fault_testing'],
}
TEST_FILES = {
    'db_fault_testing': ['🧪️tests/🧯️fault-storage-laws/🦀️.rs'],
    'db_wal': ['📝️wal/🧪️tests/🔬️unit/🦀️.rs', '📝️wal/🧪️tests/🔬️retained/🦀️.rs'],
    'db_artifact': ['🗿️artifact/🧪️tests/🔬️unit/🦀️.rs', '🗿️artifact/🧪️tests/⚔️concurrent-write/🦀️.rs'],
    'db_engine': ['⚙️engine/🧪️tests/🔬️unit/🦀️.rs', '⚙️engine/🧪️tests/⏱️throughput/🦀️.rs'],
    'db_storage': ['🗄️storage/🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs', '🗄️storage/🧪️tests/🔬️unit/🦀️.rs', '🗄️storage/🔐️writer/🧪️tests/🔬️unit/🦀️.rs',
                   '🗄️storage/🔐️writer/🧪️tests/🔬️fence-conformance/🦀️.rs', '🗄️storage/🔐️writer/🔔️release/🧪️tests/🔬️unit/🦀️.rs'],
}


BALANCED_UP_ITEMS = {'committed_durable_group_decision_accepts_only_one_exact_event_transaction', 'committed_durable_group_recovery_consumes_wal_witness_and_returns_exact_three_stores_on_pre_mutation_rejection'}
BALANCED_UP = {'🗿️artifact/🧪️tests/⚔️concurrent-write/🦀️.rs'}


def fault_assign(srcs):
    fid = '🧪️tests/🧯️fault-storage/🦀️.rs'
    out = {}
    for i, it in enumerate(srcs[fid].items):
        if (it.kind == 'use' and not it.vis.startswith('pub')) or (it.kind == 'mod' and srcs[fid]._is_decl(it)):
            continue
        nm = node_name(it)
        out[i] = 'db_storage' if nm in ('FaultScript', 'FaultStorage') else 'db_fault_testing'
    return {fid: out}


def assign_all(srcs):
    A = {}
    for f in (wal_assign, artifact_assign, engine_assign, storage_assign, fault_assign):
        A.update(f(srcs))
    by_module = {}
    for fid, m in A.items():
        for i, lib in m.items():
            it = srcs[fid].items[i]
            if it.kind not in ('impl', 'use') and it.name:
                by_module.setdefault(srcs[fid].module, {})[it.name] = lib
    for fid, m in A.items():
        for i in list(m):
            it = srcs[fid].items[i]
            if it.kind == 'impl' and it.self_type in by_module.get(srcs[fid].module, {}):
                m[i] = by_module[srcs[fid].module][it.self_type]
    return A


def test_assign(srcs, A):
    """For each split module: test item -> list of libs (own group plus helper copies)."""
    out = {}
    for mod, tfiles in TEST_FILES.items():
        order = ORDERS[mod]
        rank = {g: i for i, g in enumerate(order)}
        # subject node -> lib
        home = {}
        if mod == 'db_storage':
            from scope import top_defs, module_path
            for backend in ('db_storage_sqlite', 'db_storage_postgres', 'db_storage_neo4j', 'db_storage_driver_runtime'):
                home.setdefault(backend, 'db_storage')
                for nm in top_defs(module_path(backend, PRISTINE)):
                    home.setdefault(nm, 'db_storage')
        for fid, m in A.items():
            if srcs[fid].module != mod:
                continue
            for i, lib in m.items():
                it = srcs[fid].items[i]
                if it.kind == 'impl':
                    continue
                if it.kind == 'use':
                    s_ = srcs[fid]
                    decl = '\n'.join(s_.lines[it.header:it.end + 1])
                    mm = re.match(r'(?:pub(?:\([^)]*\))?\s+)?use[ \t\n]+(.*?);\s*$', decl, re.S)
                    if mm:
                        for p_, a_, g_ in parse_use(re.sub(r'\s+', ' ', mm.group(1)).strip()):
                            if not g_ and p_[0] not in ('crate', 'super'):
                                home[a_ or p_[-1]] = lib
                    continue
                if it.name:
                    home[it.name] = lib
        titems = {}
        for tf in tfiles:
            s = srcs[tf]
            sl = s.stripped.split('\n')
            for i, it in enumerate(s.items):
                if it.kind == 'use':
                    continue
                txt = re.sub(r'(?<!\.)\.\s*[A-Za-z_][A-Za-z0-9_]*', '.', '\n'.join(sl[it.start:it.end + 1]))
                titems[(tf, i)] = (it, idents(txt))
        tname = {}
        for (tf, i), (it, ids) in titems.items():
            if it.kind != 'impl' and it.name:
                tname[(tf, it.name)] = (tf, i)
        merged = {}
        for key, (it, ids) in titems.items():
            tgt = key
            if it.kind == 'impl' and (key[0], it.self_type) in tname:
                tgt = tname[(key[0], it.self_type)]
            merged.setdefault(tgt, set()).update(ids)
        direct = {}
        for key, (it, ids) in titems.items():
            if it.kind == 'impl' and (key[0], it.self_type) in tname:
                continue
            allids = merged[key]
            libs = {home[x] for x in allids if x in home}
            helpers = {tname[(key[0], x)] for x in allids if (key[0], x) in tname and tname[(key[0], x)] != key}
            direct[key] = (libs, helpers)

        def closure(key, seen=None):
            seen = seen if seen is not None else set()
            if key in seen:
                return set(), set()
            seen.add(key)
            libs, helpers = direct[key]
            L = set(libs)
            H = set(helpers)
            for h in helpers:
                l2, h2 = closure(h, seen)
                L |= l2
                H |= h2
            return L, H

        emit = collections.defaultdict(set)
        for key, (it, ids) in titems.items():
            if key not in direct:
                continue
            is_test = any(re.search(r'\basync_test\b|\btest\b', a) and 'cfg(test)' not in a.replace(' ', '') or re.search(r'\basync_test\b', a) for a in it.attrs)
            L, H = closure(key)
            if it.kind == 'impl' and (key[0], it.self_type) in tname:
                continue
            if is_test or it.kind in ('mod', 'impl'):
                g = max(L, key=lambda x: rank[x]) if L else order[-1]
                if it.name in BALANCED_UP_ITEMS:
                    g = order[-1]
                emit[key].add(g)
                for h in H:
                    emit[h].add(g)
        for key, (it, ids) in titems.items():
            if it.kind == 'impl' and (key[0], it.self_type) in tname:
                tgt = tname[(key[0], it.self_type)]
                emit[key] |= emit[tgt]
        for key, libs in emit.items():
            if key[0] in BALANCED_UP:
                libs = {order[-1]}
            out.setdefault(key[0], {})[key[1]] = sorted(libs, key=lambda x: rank[x])
    return out


if __name__ == '__main__':
    srcs = load_all()
    A = assign_all(srcs)
    size = collections.Counter()
    for fid, m in A.items():
        for i, lib in m.items():
            it = srcs[fid].items[i]
            size[lib] += it.end - it.start + 1
    print(dict(size))
    T = test_assign(srcs, A)
    tsize = collections.Counter()
    for fid, m in T.items():
        for i, libs in m.items():
            it = srcs[fid].items[i]
            for l in libs:
                tsize[l] += it.end - it.start + 1
    print(dict(tsize))
