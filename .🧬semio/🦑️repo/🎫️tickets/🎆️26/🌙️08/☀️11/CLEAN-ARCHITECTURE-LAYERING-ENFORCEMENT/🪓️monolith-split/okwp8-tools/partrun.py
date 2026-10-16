import sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import part, unitgraph
src, its, names = part.load()
dep, attach = part.build_dep(its, names)
ext = set('DemoSnapshot ArtifactStore DemoMemberStoreProducer DemoDiff LossyDiff demo_closable_store_owners RetainedTextMutation RetainedTextSnapshot DemoOneItemPreparationFactory close_demo_artifact_store OriginalMemberCaller RetainedTestMembers retire_original_member_open OriginalEnvelopeCaller fixture_authoring_birth_demand fixture_authoring_member_owners batch_turn_grant close_durable_publication drain_channel_for_test mutation_envelope_at drive_test_report_replay DemoInitialSnapshotRetirementFactory DemoMutationRetirementFactory ExactDemoSnapshotRetirementFactory admit_demo_alias_retirement close_test_store complete_test_catalog drive_retirement_terminal assert_fixture_descriptor'.split())
tests = [r for r in its if r['test']]
total = sum(r['bytes'] for r in its)
print('total bytes', total, 'tests', len(tests))
k = int(sys.argv[1]) if len(sys.argv) > 1 else 6
# equal-bytes cut by cumulative bytes of items
cum = 0; cuts = []; target = total / k
for r in its:
    if r['test'] and cum >= target * (len(cuts) + 1) and len(cuts) < k - 1:
        cuts.append(r['idx'])
    cum += r['bytes']
print('cuts at lines', [its[c]['a'] for c in cuts])
assign, F = part.partition(its, names, dep, attach, cuts, ext)
sz = collections.Counter(); ln = collections.Counter(); cnt = collections.Counter()
for r in its:
    if r['idx'] in assign:
        sz[assign[r['idx']]] += r['bytes']; ln[assign[r['idx']]] += r['b'] - r['a'] + 1
        if r['test']: cnt[assign[r['idx']]] += 1
for key in sorted(sz, key=str): print(key, 'bytes', sz[key], 'lines', ln[key], 'tests', cnt[key])
