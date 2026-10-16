#!/usr/bin/env python3
"""🧪️ Plan for the os-hub binary unit tests: the shared fixtures move to one testkit crate and the laws into five laws crates, each keeping the `tests::` law path."""
import os, sys, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bintests
from helpers import *

D = P('🧪️tests/🔬️bin-unit/')
BU = D + '🦀️.rs'

GROUPS = collections.OrderedDict([
    ('semio_hub_boot_laws', ('🚀️boot', None, 'Boot laws: startup, readiness, hostile routes, check in, inference and canonical pair routes over a real hub router.')),
    ('semio_hub_open_plan_laws', ('🧭️open-plan', 'document_open_plan_ledger_is_digest_only_bounded_single_use_revalidated_and_restart_scoped', 'Open-plan laws: document-open plans, execution targets, supersession, socket grants and scoped directory sockets.')),
    ('semio_hub_directory_route_laws', ('📇️directory', 'admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen', 'Directory route laws: administration, directory events and commands, space pages and presence.')),
    ('semio_hub_session_route_laws', ('🔐️session', 'credential_sign_in_reports_a_failing_instance_session_store', 'Session route laws: credential sign-in, agent delegation, origin and proxy posture, observability and shutdown.')),
    ('semio_hub_presence_route_laws', ('👥️presence', None, 'Presence route laws: presence normalization, lease, roster and liveness over real sockets.')),
    ('semio_hub_hostile_input_laws', ('🎲️hostile-input', None, 'Hostile input laws: the quick and long generative hostile-input suites and the held count lease.')),
])
TESTKIT = ('semio_hub_route_testkit', D + '🧰️testkit/🦀️.rs')
ROOTS = collections.OrderedDict([(TESTKIT[0], TESTKIT[1])] + [(k, D + v[0] + '/🏛️laws/🦀️.rs') for k, v in GROUPS.items()])
DOCS = {k: v[2] for k, v in GROUPS.items()}
DOCS[TESTKIT[0]] = 'Shared fixtures of the hub route laws: test states, servers, sockets, sessions and document-open plan authorities.'
PSEUDO_GROUP = 'semio_hub_hostile_input_laws'
PRESENCE_GROUP = 'semio_hub_presence_route_laws'
PRESENCE_RANGE = ('presence_normalization_matches_neutral_authority_and_no_effect_rejections', 'presence_join_expiry_and_leave_each_emit_one_record')
TWO_CLIENT = 'semio_hub_session_route_laws'


def assign():
    byname, deps, sizes, order, tests = bintests.full()
    pos = {k: i for i, k in enumerate(order)}
    ts = sorted(tests, key=lambda t: pos[t])
    idents = list(GROUPS)
    group_tests = {}
    explicit = [(i, GROUPS[i][1]) for i in idents if GROUPS[i][1]]
    start_idx = [0] + [ts.index(s) for _, s in explicit] + [len(ts)]
    main_idents = [i for i in idents if i not in (PSEUDO_GROUP, PRESENCE_GROUP)]
    for gi, ident in enumerate(main_idents):
        group_tests[ident] = ts[start_idx[gi]:start_idx[gi + 1]]
    lo, hi = ts.index(PRESENCE_RANGE[0]), ts.index(PRESENCE_RANGE[1])
    group_tests[PRESENCE_GROUP] = ts[lo:hi + 1]
    for ident in main_idents:
        group_tests[ident] = [t for t in group_tests[ident] if t not in group_tests[PRESENCE_GROUP]]
    group_tests[PSEUDO_GROUP] = [p for p in bintests.PSEUDO if p in byname]
    tset = set(tests)
    users = collections.defaultdict(set)
    for ident, g in group_tests.items():
        for t in g:
            for h in bintests.reach(deps, t, tests):
                users[h].add(ident)
            for h in deps.get(t, ()):
                if h not in tset:
                    users[h].add(ident)
    pseudo = set(bintests.PSEUDO)
    shared, own = [], collections.defaultdict(list)
    for h in order:
        if h in tset or h in pseudo:
            continue
        us = users.get(h, set())
        if len(us) > 1:
            shared.append(h)
        elif len(us) == 1:
            own[next(iter(us))].append(h)
        else:
            own[idents[0]].append(h)
    return group_tests, shared, own, byname, sizes


def bin_tests(ws, E, base_ws):
    group_tests, shared, own, byname, sizes = assign()
    create(ws, TESTKIT[1], f'//! 🧰️ {DOCS[TESTKIT[0]]}\n')
    E.move(BU, shared, TESTKIT[1])
    E.proxy[TESTKIT[1]] = BU
    for ident, (dirname, _, doc) in GROUPS.items():
        root = ROOTS[ident]
        unit = D + dirname + '/🔬️unit/🦀️.rs'
        extra = ''
        if ident == TWO_CLIENT:
            extra = '\n#[cfg(test)]\n#[path = "../../../🤝️two-client-document/🦀️.rs"]\nmod two_client_document;\n'
        create(ws, root, f'//! 🏛️ {doc}\n\n#[cfg(test)]\n#[path = "../🔬️unit/🦀️.rs"]\nmod tests;\n' + extra)
        create(ws, unit, '')
        names = group_tests[ident] + own[ident]
        E.move(BU, names, unit)
        E.proxy[unit] = BU
        E.proxy[root] = BU
    rest = ws.read(BU)
    its, _ = __import__('carve').parse_items(rest)
    left = [i.name for i in its if i.kind not in ('use', 'none')]
    if left:
        raise RuntimeError(f'bin-unit items not distributed: {left}')
    ws.delete(BU)


ALIAS = re.compile(r'\bsemio_framework_os_kernel::')


def postprocess(ws):
    """The retired `extern crate directory as semio_framework_os_kernel` alias lived in the bin root's test build; the laws crates name the dependency `directory`."""
    for p in list(ws.cur):
        if p.startswith(D) and p.endswith('.rs'):
            s = ws.cur[p]
            t = ALIAS.sub('directory::', s)
            t = runtime_includes(ws, p, t)
            if t != s:
                ws.write(p, t)


INCLUDE = re.compile(r'include_str!\(\s*"([^"]+)"\s*\)')
THRESHOLD = 6000


def runtime_includes(ws, file, text):
    """Production sources and large fixtures a law only reads are read when the law runs: embedded, they would count toward the laws crate."""
    def swap(m):
        line_start = text.rfind('\n', 0, m.start()) + 1
        head = text[line_start:m.start()]
        if re.match(r'\s*(pub(\([^)]*\))?\s+)?(const|static)\b', head) or ' const ' in head:
            return m.group(0)
        target = os.path.normpath(os.path.join(os.path.dirname(file), m.group(1)))
        if not os.path.exists(target) or not (target.endswith('.rs') or os.path.getsize(target) > THRESHOLD):
            return m.group(0)
        return 'semio_hub_testing::hub_source("%s")' % os.path.relpath(target, HUB.rstrip('/'))
    return INCLUDE.sub(swap, text)


def postprocess_files(ws, files):
    for p in files:
        s = ws.read(p)
        t = runtime_includes(ws, p, s)
        if t != s:
            ws.write(p, t)
