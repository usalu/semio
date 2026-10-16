import sys, os, json, re, collections, itertools
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import part, units, mounts, unitgraph
R = units.R
HERE = os.path.dirname(os.path.abspath(__file__))
d = json.load(open(HERE + '/../🗑️generated/okwp8-closure-before.json'))
paths = [r['path'] for r in d['test native'] if r['kind'] != 'embedded']; ps = set(paths)
S = '🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/'
UNIT = S + '🧪️tests/🔬️unit/🦀️.rs'
EXT = set('DemoSnapshot ArtifactStore DemoMemberStoreProducer DemoDiff LossyDiff demo_closable_store_owners RetainedTextMutation RetainedTextSnapshot DemoOneItemPreparationFactory close_demo_artifact_store OriginalMemberCaller RetainedTestMembers retire_original_member_open OriginalEnvelopeCaller fixture_authoring_birth_demand fixture_authoring_member_owners batch_turn_grant close_durable_publication drain_channel_for_test mutation_envelope_at drive_test_report_replay DemoInitialSnapshotRetirementFactory DemoMutationRetirementFactory ExactDemoSnapshotRetirementFactory admit_demo_alias_retirement close_test_store complete_test_catalog drive_retirement_terminal assert_fixture_descriptor'.split())
SPECIAL_OWN = {  # units with a fixed companion
    'durable_group_tests': 'durable', 'composition_group_tests': 'durable',
    'factory_tests': 'member-open', 'dictionary_tests': 'member-open', 'history_tests': 'member-open', 'member_open_tests': 'member-open', 'receiving_tests': 'member-open', 'operation_snapshot_input_tests': 'member-open',
    'sync_tests': 'sync1', 'backbone_detach_refusal_tests': 'sync2',
}
SKIP = {'retained_codec_catalog', 'fixture_mutations', 'include'}
INC = re.compile(r'include_(?:str|bytes)!\(\s*(?:concat!\(\s*env!\("CARGO_MANIFEST_DIR"\)\s*,\s*)?"([^"]+)"')
KDIR = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust'
def unit_children():
    out = []
    for name, tgt, cfg in mounts.children(UNIT):
        if tgt in ps: out.append((name, tgt))
    return out
def rest_units():
    roots = []
    for name, tgt, cfg in mounts.children(S + '🦀️.rs'):
        if tgt in ps and tgt != UNIT: roots.append((name, tgt))
    roots += unit_children()
    res = []
    for name, tgt in roots:
        if name in SKIP: continue
        m = units.metrics(units.tree(tgt, ps))
        m['name'] = name; m['root'] = tgt
        res.append(m)
    return res
def chunk_metrics(its, assign, K):
    base = os.path.dirname(R + UNIT)
    out = []
    for c in range(K):
        lines = bytes_ = 0; emb = set()
        for r in its:
            if assign.get(r['idx']) == c:
                lines += r['b'] - r['a'] + 1; bytes_ += r['bytes']
                for m in INC.finditer(r['text']):
                    p = m.group(1)
                    if p.endswith('.rs'): continue
                    emb.add(os.path.normpath(KDIR + p) if p.startswith('/') else os.path.normpath(os.path.join(base, p)))
        eb = sum(os.path.getsize(e) for e in emb if os.path.exists(e))
        out.append(dict(lines=lines, bytes=bytes_ + eb, files=1 + len(emb)))
    return out
