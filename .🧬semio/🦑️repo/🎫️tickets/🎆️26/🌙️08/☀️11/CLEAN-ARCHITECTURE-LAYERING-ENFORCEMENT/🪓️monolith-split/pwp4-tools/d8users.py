# 🔬️ D8 definition scan on the final layout: who names the task-lane statics and the closure types, grouped by crate and rank; every user must sit at or above reactor-tasks.
import collections
exec(open('dag4.py').read().split("bad = []")[0])
NAMES = ['TASK_RECORDS', 'TASK_RESUMES', 'INSTANCE_METADATA', 'REGISTRY', 'REACTOR_EXECUTOR', 'TASK_EXECUTOR', 'JOB_RENDER_BINDINGS', 'ReactorFixedSlots', 'spawn_task', 'queue_extension_invocation', 'host_for_instance', 'cancel_instance_tasks_step', 'instance_task_quota', 'InstanceMetadataRegistry', 'TaskRecordRegistry', 'FixedResumeQueue']
rev = collections.defaultdict(set)
for u, vs in adj.items():
    for v in vs: rev[v].add(u)
T = 'reactor-tasks'
print(f"reactor-tasks rank {RANK[T]}")
low = 0
for n in NAMES:
    ids = [j for j in defs.get(n, []) if items[j]['kind'] != 'impl']
    for j in ids:
        v = unit(j); owner_c = crate.get(v)
        users = collections.Counter(crate.get(u) for u in rev[v] if crate.get(u) and crate.get(u) != owner_c)
        lower = {c: k for c, k in users.items() if RANK.get(c, 999) < RANK[T]}
        low += sum(lower.values())
        print(f"  {n:30s} owner {owner_c:16s} users {dict(users)}" + (f"  LOWER USERS {lower}" if lower else ''))
print('users below reactor-tasks:', low)
