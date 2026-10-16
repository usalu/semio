"""🗺️ Master plan of WP8: unit-file labels, bin membership. Writes 🗑️generated/okwp8-plan.json."""
import sys, os, json, copy, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import part, part3, packing as P, units
HERE = os.path.dirname(os.path.abspath(__file__))
FIXED = {
    'durable': ['durable_group_tests', 'composition_group_tests', 'outbound_announcement_tests', 'deferred_reprojection_tests'],
    'member-open': ['factory_tests', 'dictionary_tests', 'history_tests', 'member_open_tests', 'receiving_tests', 'operation_snapshot_input_tests'],
    'sync-1': ['sync_tests'],
    'sync-2': ['supersede_replay_tests', 'tool_transaction_tests', 'replay_retirement_tests'],
}
GROUPS = [['reader_tests', 'canonical_edit_borrowed_tests', 'canonical_edit_tests']]
CAP = dict(bytes=214000, lines=3300, files=26)
def make():
    src, its, names = part.load()
    dep0, attach = part.build_dep(its, names)
    cuts = json.load(open(HERE + '/../🗑️generated/okwp8-cuts6b.json'))['cuts']
    dep = copy.deepcopy(dep0)
    assign, F, ft, c, tl = part3.compute(its, names, dep, attach, 6, P.EXT, fixed_cuts=cuts)
    cm = P.chunk_metrics(its, assign, 6)
    ru = P.rest_units()
    fixed_names = set(sum(FIXED.values(), []))
    rest = [u for u in ru if u['name'] not in fixed_names and u['name'] != 'sqlite_snapshot_space_history_baseline']
    sqlite = [u for u in ru if u['name'] == 'sqlite_snapshot_space_history_baseline'][0]
    rest.append(dict(name='fixture_descriptors', files=19, lines=140, bytes=7677 + 5 * 1500, root=None))
    for grp in GROUPS:
        members = [u for u in rest if u['name'] in grp]
        rest = [u for u in rest if u['name'] not in grp]
        rest.append(dict(name='+'.join(grp), files=sum(u['files'] for u in members), lines=sum(u['lines'] for u in members), bytes=sum(u['bytes'] for u in members), root=None, parts=grp))
    import random
    def attempt(order):
        bins = {}
        for i in range(1, 11):
            bins['tests-%02d' % i] = dict(units=[], bytes=0, lines=0, files=0, chunk=None)
        for k in range(6):
            b = bins['tests-%02d' % (k + 1)]
            b['chunk'] = k; b['bytes'] += cm[k]['bytes']; b['lines'] += cm[k]['lines']; b['files'] += cm[k]['files']
        b7 = bins['tests-07']; b7['units'].append(sqlite['name']); b7['bytes'] += sqlite['bytes']; b7['lines'] += sqlite['lines']; b7['files'] += sqlite['files']
        for u in order:
            cands = [key for key, b in bins.items() if b['bytes'] + u['bytes'] <= CAP['bytes'] and b['lines'] + u['lines'] <= CAP['lines'] and b['files'] + u['files'] <= CAP['files']]
            if not cands: return None
            key = min(cands, key=lambda k: bins[k]['bytes'] + random.random() * 20000)
            b = bins[key]; b['units'] += u.get('parts', [u['name']]); b['bytes'] += u['bytes']; b['lines'] += u['lines']; b['files'] += u['files']
        return bins
    random.seed(7)
    base = sorted(rest, key=lambda u: -u['bytes'])
    bins = None
    for t in range(2000):
        order = list(base) if t == 0 else sorted(rest, key=lambda u: -u['bytes'] * (0.7 + 0.6 * random.random()))
        bins = attempt(order)
        if bins: break
    if bins is None: raise SystemExit('no packing found')
    for key, names_ in FIXED.items():
        bins[key] = dict(units=list(names_), chunk=None, bytes=sum(u['bytes'] for u in ru if u['name'] in names_), lines=sum(u['lines'] for u in ru if u['name'] in names_), files=sum(u['files'] for u in ru if u['name'] in names_))
    bins['member-open']['units'] += ['catalog_include']
    plan = dict(cuts=cuts, labels={str(k): v for k, v in assign.items()}, bins=bins, F=sorted(F), tl={str(k): v for k, v in tl.items()})
    json.dump(plan, open(HERE + '/../🗑️generated/okwp8-plan.json', 'w'), indent=1)
    return plan, its
if __name__ == '__main__':
    plan, its = make()
    for k, b in plan['bins'].items(): print(k, b['bytes'], b['lines'], b['files'], b['chunk'], b['units'])
