"""🎨 Theme table of the fixtures-crate modules: item -> module (folder emoji + kebab)."""
import re
FE = '️'
THEMES = [  # (key, folder, rust module name, doc, predicate regexes on name)
    ('snapshot', '🧮' + FE + 'snapshot', 'snapshot', 'Demo and retained-text snapshots, their diffs and decode counters.'),
    ('retirement', '♻' + FE + 'retirement', 'retirement', 'Retirement doubles of the demo snapshot and mutation owners.'),
    ('store', '🗄' + FE + 'store', 'store', 'The test store wrapper, its member factory impls, the demo member store producer and the close helpers.'),
    ('preparation', '🏭' + FE + 'preparation', 'preparation', 'One-item preparation doubles and the published-root witnesses.'),
    ('operations', '🧵' + FE + 'operations', 'operations', 'Operation text, binary and canonical JSON images of the fixture mutation families.'),
    ('callers', '🧾' + FE + 'callers', 'callers', 'Original-caller doubles, member fixtures and envelope field decoders.'),
    ('envelopes', '📨' + FE + 'envelopes', 'envelopes', 'Envelope builders and one-item grants shared by the replay tests.'),
]
NAME_THEME = [
    (r'^(DemoSnapshot|DemoDiff|DemoFieldChange|LossyDiff|MEMBER_SNAPSHOT_DECODE_COUNT|RetainedText\w*|SetRetainedText|assert_fixture_descriptor)$', 'snapshot'),
    (r'^(DemoSnapshotRetirementFactory|ExactDemo\w*|DemoInitialSnapshotRetirementFactory|DemoMutationRetirementFactory|retire_demo_envelope|admit_demo_alias_retirement|drive_retirement_terminal|DemoPublishedRoot\w*|demo_snapshot_frame_bytes)$', 'retirement'),
    (r'^(DemoOneItem\w*)$', 'preparation'),
    (r'^(FixtureOperation\w*|fixture_canonical_json)$', 'operations'),
    (r'^(OriginalEnvelopeCaller|OriginalMemberCaller|TestEnvelopeFieldDecoder|artifact_envelope_decode_test_cursor|envelope_field_close_grant|drive_test_envelope_field_return|RetainedTestMembers|member_dialect_fixture|close_member_dialect_fixture|retire_original_member_open|demo_child_dialect)$', 'callers'),
    (r'^(mutation_envelope_at|foreign_mutation_envelope|owned_test_envelope|one_item_test_policy|batch_turn_grant|fresh_demo_store)$', 'envelopes'),
]
def theme_of(r):
    nm = r['name']
    if r['kind'] == 'impl':
        st = r.get('selfty'); tr = r.get('trait')
        if st in ('DemoMutation', 'SeverityMutation', 'ValidatedMutation', 'TimestampedMutation', 'LossyMutation') and tr in ('OpText', 'OpBinary', 'ArtifactCanonicalJson', 'FixtureOperation'): return 'operations'
        nm = st
    if r['kind'] is None:
        nm = r.get('macro')
        if nm == 'fixture_canonical_json': return 'operations'
        return 'store'
    for rx, th in NAME_THEME:
        if nm and re.match(rx, nm): return th
    return 'store'

THEME_CRATE = {'snapshot': 'A', 'operations': 'A', 'retirement': 'B', 'store': 'B', 'preparation': 'B', 'callers': 'B', 'envelopes': 'B'}
