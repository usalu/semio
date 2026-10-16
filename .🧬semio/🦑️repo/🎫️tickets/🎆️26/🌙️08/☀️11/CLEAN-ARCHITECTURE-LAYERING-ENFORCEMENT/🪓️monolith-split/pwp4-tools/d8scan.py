# 🔬️ D8 definition scan: closure of the reactor-tasks seed over the item graph, with the rank constraints of Appendix H.
import json, collections
from model4 import *
files, items, defs, owner, owner2, text, final1 = load()
# dedupe synthetic statics already seen as plain statics
seen = {(i['file'], i['defname']) for i in items if i['kind'] == 'static' and 'body' not in i and i['id'] < 0}
size, adj, crate_of = PT.graph(items, defs, owner2)
final, moves, _ = PT.simulate(size, adj, crate_of, items)
unit = lambda i: owner2.get(i, i)
# statics are items with their own unit; assign by file rule unless table says otherwise
STATIC_CRATE = {
 'PATCHES': 'reactor-patches', 'PRESENCE': 'reactor-core', 'REGISTRY': 'actor-host', 'REACTOR_EXECUTOR': 'actor-host', 'TASK_EXECUTOR': 'actor-host',
 'INSTANCE_METADATA': 'reactor-core', 'PACKAGE_ASSETS': 'reactor-core', 'ARMED_TIMERS': 'reactor-core', 'TASK_RECORDS': 'reactor-core', 'TASK_RESUMES': 'reactor-core', 'JOB_RENDER_BINDINGS': 'reactor-core',
 'REACTOR_CLOSES': 'reactor-lifecycle', 'REACTOR_CLOSE_CURSOR': 'reactor-lifecycle', 'PENDING_PATCHES': 'reactor-lifecycle',
 'COLD_PAIR_INGRESS': 'reactor-surfaces', 'COLD_PAIR_DOCUMENT_LOAD': 'reactor-surfaces', 'RESTORED_DOCUMENT_LOADS': 'reactor-surfaces',
 'KIND_REGISTRY': 'reactor-jobs', 'KIND_REGISTRY_EPOCH': 'reactor-jobs', 'REGISTERED_KIND_COUNT': 'reactor-jobs', 'JOBS': 'reactor-jobs', 'STAGED_PAGES': 'reactor-wit-bridge', 'COMMAND_INGRESS': 'reactor-turn', 'PROCESS_POOL': 'reactor-turn',
}
crate = {}
for it in items:
    u = unit(it['id'])
    if u in final: crate[u] = final[u]
for it in items:
    if it['kind'] == 'static' and it['defname'] in STATIC_CRATE and WP4F(it['file']): crate[it['id']] = STATIC_CRATE[it['defname']]
# graph with statics included
size, adj, _ = PT.graph(items, defs, owner2)
byname = lambda f, n: [it for it in items if it['file'] == f and (it['defname'] == n)]
RC, RT, TURN = '⚛️reactor/🦀️.rs', '⚛️reactor/🏃️runtime/🦀️.rs', '⚛️reactor/🔄️turn/🦀️.rs'
seed_names = ['TaskRecord', 'TaskRecordRegistry', 'TASK_RECORDS', 'TaskResumeOutcome', 'PendingResume', 'FixedResumeQueue', 'TASK_RESUMES', 'instance_task_quota', 'spawn_task', 'encode_mutation_lane', 'queue_extension_invocation', 'host_for_instance', 'REACTOR_TASK_SLOTS', 'REACTOR_TASK_KEY_BYTES', 'REACTOR_TASK_LABEL_BYTES', 'REACTOR_TASK_RESTART_BYTES', 'REACTOR_RESUME_SLOTS', 'REACTOR_RESUME_BYTES', 'REACTOR_RESUME_ACTOR_BYTES']
S = set()
for n in seed_names:
    r = byname(RC, n)
    assert len(r) >= 1, n
    for it in r: S.add(unit(it['id']))
T = 'reactor-tasks'
RANK[T] = CR['reactor-tasks']['rank']
def dep_crate(v): return crate.get(v)
report = []
changed = True
while changed:
    changed = False
    for u in list(S):
        for v in adj.get(u, ()):
            if v in S: continue
            cv = dep_crate(v)
            if cv is None: continue
            if RANK.get(cv, 999) > RANK[T] and items[v]['file'] and WP4F(items[v]['file']):
                S.add(v); changed = True; report.append((items[u]['defname'] or items[u]['head'][:30], '->', items[v]['file'].split('/')[-2], items[v]['defname'] or items[v]['head'][:30], cv))
print('closure size', len(S), 'lines', sum(size.get(u, 0) for u in S))
for r in report: print('  pulled:', r)
print('--- members')
for u in sorted(S, key=lambda x: (items[x]['file'], items[x]['s'])):
    print(' ', items[u]['file'].replace('⚛️reactor/', ''), items[u]['s'], items[u]['e'], items[u]['kind'], items[u]['defname'] or items[u]['head'][:30], 'was', crate.get(u))
print('--- dependencies of the closure on items OUTSIDE it (crate, rank)')
out = collections.Counter()
for u in S:
    for v in adj.get(u, ()):
        if v in S: continue
        cv = dep_crate(v)
        if cv is None: continue
        out[(cv, RANK.get(cv, '?'), items[v]['defname'] or items[v]['head'][:30])] += 1
for k, v in sorted(out.items(), key=lambda kv: str(kv[0][1])): print(' ', k, v)
print('--- users of the closure from OUTSIDE (crate, rank) with rank < tasks')
for it in items:
    uu = unit(it['id'])
    if uu in S or uu not in size: continue
    for v in adj.get(uu, ()):
        if v in S and RANK.get(dep_crate(uu) or '', 999) < RANK[T]:
            print('  LOWER USER', items[uu]['file'], items[uu]['s'], items[uu]['defname'], dep_crate(uu), '->', items[v]['defname'])
