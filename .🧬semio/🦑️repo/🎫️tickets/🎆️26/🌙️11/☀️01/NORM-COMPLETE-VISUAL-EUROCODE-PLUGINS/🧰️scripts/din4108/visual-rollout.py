import os, re, shutil

ART = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts'
S = '🏅️standards/🔖️1/🪆️subsets/✳️any'
SRC = f'{ART}/🪶️en1999/{S}'
DST = f'{ART}/🧱️din4108/{S}'
EW = '✏️editor/🎭️modes/✏️edit/🪟️windows'
VW = '👁️viewer/🎭️modes/👁️view/🪟️windows'

SWAPS = [('EN 1999', 'DIN 4108'), ('En1999', 'Din4108'), ('en1999', 'din4108'), ('EN1999', 'DIN4108'), ('EN-1999-Modell', 'DIN-4108-Modell'), ('🪶️din4108', '🧱️din4108'), ('EN-1999-', 'DIN-4108-')]
FIXTURES = [
    ('Din4108Snapshot::compliant_roof_purlin()', 'Din4108Snapshot::default()'),
    ('Din4108Snapshot::noncompliant_multi_fail()', 'Din4108Snapshot::failing_thin_insulation()'),
    ('Din4108Snapshot::empty()', 'Din4108Snapshot { zones: vec![], elements: vec![], thermal_bridges: vec![], ..Din4108Snapshot::default() }'),
    ('Din4108Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)', 'Din4108Snapshot::default()'),
]


def swap(text):
    for a, b in SWAPS + FIXTURES:
        text = text.replace(a, b)
    return text


def read(p):
    return open(p, encoding='utf8').read()


def write(p, text):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, 'w', encoding='utf8').write(text)


def copy(rel, patch=None):
    text = swap(read(f'{SRC}/{rel}'))
    if patch:
        text = patch(text)
    write(f'{DST}/{rel}', text)


def sub(text, old, new, what):
    if text.count(old) != 1:
        raise SystemExit(f'anchor not found or ambiguous: {what}')
    return text.replace(old, new)


def cut(text, start, end, what):
    i = text.index(start)
    j = text.index(end, i)
    return text[:i] + text[j:]


def editor_root(text):
    text = sub(text, '''pub const ENTITY_KIND_SCENE_KINDS: &[semio_s_plugin_norm_structural_scene::EntityKind] = &[
    semio_s_plugin_norm_structural_scene::EntityKind::Member,
    semio_s_plugin_norm_structural_scene::EntityKind::Plate,
    semio_s_plugin_norm_structural_scene::EntityKind::Shell,
    semio_s_plugin_norm_structural_scene::EntityKind::Fastener,
    semio_s_plugin_norm_structural_scene::EntityKind::Support,
    semio_s_plugin_norm_structural_scene::EntityKind::Load,
    semio_s_plugin_norm_structural_scene::EntityKind::Group,
];''', '''pub const ENTITY_KIND_SCENE_KINDS: &[semio_s_plugin_norm_structural_scene::EntityKind] = &[
    semio_s_plugin_norm_structural_scene::EntityKind::Solid,
    semio_s_plugin_norm_structural_scene::EntityKind::Member,
];''', 'scene kinds')
    text = cut(text, '/// 🎪️ The "full showcase example"', '#[derive(Default)]', 'showcase fns')
    text = re.sub(r'vec!\[semio_s_artifact_norm_din4108_examples::aluminium_roof_purlin::source\(\), semio_s_artifact_norm_din4108_examples::noncompliant_multi_fail::source\(\)\]\.into_iter\(\)\.chain\(\[full_showcase_example\(\), aluminium_showcase_example\(\)\]\)\.collect\(\)', 'vec![semio_s_artifact_norm_din4108_examples::demo::source(), semio_s_artifact_norm_din4108_examples::failing_thin_insulation::source()]', text)
    text = text.replace('catalogue_panel::render(Din4108PlayApp::examples(), view_state.locale, CONTROLLER_ID)', 'catalogue_panel::render(Din4108PlayApp::examples(), view_state.locale, CONTROLLER_ID, &semio_plugin::TreeWindows::for_body(view_state, catalogue_panel::BODY_CATALOGUE))')
    text = text.replace('semio_s_artifact_norm_din4108_mutations::resolve_edit', 'semio_s_artifact_norm_din4108_mutations::resolve')
    text = text.replace('Din4108Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)', 'Din4108Snapshot::default()')
    text = text.replace('semio_s_artifact_norm_din4108_model::Din4108Snapshot::default()', 'semio_s_artifact_norm_din4108_model::Din4108Snapshot::default()')
    text = text.replace('#[cfg(test)]\n#[path = "🧪️tests/📚️assets/🦀️.rs"]\nmod asset_laws;\n', '')
    return text


def main():
    copy('✏️editor/🦀️.rs', editor_root)
    copy('✏️editor/🎭️modes/✏️edit/🦀️.rs')
    copy('✏️editor/🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs')
    for window in ['✅️checks', '📈️diagrams', '🌲️outliner', '🧾️properties', '🧊️viewport']:
        copy(f'{EW}/{window}/🦀️.rs')
        copy(f'{EW}/{window}/🧪️tests/🔬️unit/🦀️.rs')
    for gone in ['📥️inputs', '📊️results']:
        shutil.rmtree(f'{DST}/{EW}/{gone}', ignore_errors=True)
    copy('✏️editor/🧮️incremental/🦀️.rs')
    for rel in ['✏️editor/🧮️incremental/🧪️tests/🔬️unit/🦀️.rs', '✏️editor/🧮️incremental/🧪️tests/🧭️ownership/🦀️.rs']:
        copy(rel)
    copy('✏️editor/🧪️tests/🔬️unit/🦀️.rs')
    copy('👁️viewer/🦀️.rs')
    copy('👁️viewer/🎭️modes/👁️view/🦀️.rs')
    copy('👁️viewer/🎭️modes/👁️view/🧪️tests/🔬️unit/🦀️.rs')
    for window in ['✅️checks', '📈️diagrams', '🧊️viewport']:
        copy(f'{VW}/{window}/🦀️.rs')
        copy(f'{VW}/{window}/🧪️tests/🔬️unit/🦀️.rs')
    shutil.rmtree(f'{DST}/{VW}/📊️report', ignore_errors=True)
    copy('👁️viewer/🧪️tests/🔬️unit/🦀️.rs')


main()
