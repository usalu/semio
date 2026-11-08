"""Rolls the EN 1999 visual editor and viewer vertical onto DIN EN 16798 (one-shot, not idempotent): copies the editor root, windows, edit mode, incremental module and the viewer shims with the naming swapped, then restores the DIN-specific parts."""
import os, re, shutil

ART = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts'
S = '🏅️standards/🔖️1/🪆️subsets/✳️any'
SRC = f'{ART}/🪶️en1999/{S}'
DST = f'{ART}/🌬️din16798/{S}'
EW = '✏️editor/🎭️modes/✏️edit/🪟️windows'
VW = '👁️viewer/🎭️modes/👁️view/🪟️windows'

SWAPS = [
    ('🪶️en1999', '🌬️din16798'), ('En1999Family', 'DinEn16798Family'), ('EN-1999-Modell', 'DIN-EN-16798-Modell'),
    ('En1999', 'Din16798'), ('en1999', 'din16798'), ('EN 1999', 'DIN EN 16798'), ('EN1999', 'DIN16798'),
]


def swap(text):
    for a, b in SWAPS:
        text = text.replace(a, b)
    return text


def read(p):
    return open(p, encoding='utf8').read()


def write(p, text):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, 'w', encoding='utf8').write(text)


def copy(rel, edit=None):
    text = swap(read(f'{SRC}/{rel}'))
    write(f'{DST}/{rel}', edit(text) if edit else text)


for old in ('📥️inputs', '📊️results'):
    shutil.rmtree(f'{DST}/{EW}/{old}', ignore_errors=True)
shutil.rmtree(f'{DST}/{VW}/📊️report', ignore_errors=True)

for window in ('✅️checks', '📈️diagrams', '🌲️outliner', '🧾️properties', '🧊️viewport'):
    for leaf in ('🦀️.rs', '🧪️tests/🔬️unit/🦀️.rs'):
        if window == '🧊️viewport' and leaf == '🦀️.rs':
            continue
        copy(f'{EW}/{window}/{leaf}')
copy('✏️editor/🎭️modes/✏️edit/🦀️.rs')
copy('✏️editor/🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs')
for window in ('✅️checks', '📈️diagrams', '🧊️viewport'):
    for leaf in ('🦀️.rs', '🧪️tests/🔬️unit/🦀️.rs'):
        copy(f'{VW}/{window}/{leaf}')
copy('👁️viewer/🎭️modes/👁️view/🦀️.rs')
copy('👁️viewer/🎭️modes/👁️view/🧪️tests/🔬️unit/🦀️.rs')
copy('✏️editor/🧮️incremental/🦀️.rs', lambda t: t.replace('use semio_s_artifact_norm_din16798_checks::Din16798Units;', 'use semio_s_artifact_norm_din16798_checks::Din16798Units;'))
copy('✏️editor/🧮️incremental/🧪️tests/🧭️ownership/🦀️.rs')

diagrams_test = f'{DST}/{EW}/📈️diagrams/🧪️tests/🔬️unit/🦀️.rs'
t = read(diagrams_test)
t = t.replace('renders_the_buckling_diagram_of_the_default_member', 'renders_the_adaptive_comfort_diagram_of_the_default_room')
t = re.sub(r'assert!\(rendered\.contains\("Flexural buckling curves"\).*?\);', 'assert!(rendered.contains("Adaptive comfort") || rendered.contains("Adaptiver Komfort"), "the diagrams body must show the adaptive comfort diagram: {rendered}");', t)
write(diagrams_test, t)
vt = f'{DST}/{EW}/🧊️viewport/🧪️tests/🔬️unit/🦀️.rs'
write(vt, read(vt) if os.path.exists(vt) else '')

units = swap(read(f'{SRC}/✏️editor/🧮️incremental/🧪️tests/🔬️unit/🦀️.rs'))
units = re.sub(r'pub\(super\) fn documents\(\).*?\n}\n', 'pub(super) fn documents() -> Vec<Din16798Snapshot> {\n    vec![Din16798Snapshot::compliant_office(), Din16798Snapshot::residential_method3(), Din16798Snapshot::noncompliant_office(), Din16798Snapshot { zones: vec![], vent_systems: vec![], ..Din16798Snapshot::compliant_office() }]\n}\n', units, flags=re.S)
units = units[:units.index('#[test]\nfn a_row_field_edit')]
units += '''#[test]
fn a_zone_field_edit_leaves_the_non_row_units_clean() {
    use semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource;
    let units = <DinEn16798Family as UnitSource>::units(&documents().remove(0));
    let reads = |id: &str| units.iter().find(|unit| unit.id.0 == id).map(|unit| unit.reads.clone()).unwrap_or_else(|| panic!("the unit {id}"));
    let global = reads("global:document");
    assert!(global.covers("annex") && global.covers("zones.0.id") && global.covers("ventSystems.0.id"));
    assert!(!global.overlaps("zones.0.co2Ppm") && !global.overlaps("ventSystems.0.sfpWM3S"), "a design value edit leaves the integrity unit clean");
    let envelope = reads("part:envelope");
    assert!(envelope.covers("envelopeN50HInv") && !envelope.overlaps("zones.0.co2Ppm"));
}
'''
write(f'{DST}/✏️editor/🧮️incremental/🧪️tests/🔬️unit/🦀️.rs', units.replace('use crate::Din16798Family;', 'use crate::DinEn16798Family;').replace('Din16798Family', 'DinEn16798Family').replace('DinDinEn16798Family', 'DinEn16798Family'))

# ---- editor root
root = swap(read(f'{SRC}/✏️editor/🦀️.rs'))
old_root = read(f'{DST}/✏️editor/🦀️.rs')

def cut(text, start, end, repl):
    i = text.index(start)
    j = text.index(end, i)
    return text[:i] + repl + text[j:]

root = cut(root, '/// 🎪️ The "full showcase example"', '#[derive(Default)]\npub struct Din16798PlayApp;', '')
m = re.search(r'    fn examples\(\) -> Vec<semio_plugin::ExampleSource> \{.*?\n    \}\n', old_root, re.S)
root = re.sub(r'    fn examples\(\) -> Vec<semio_plugin::ExampleSource> \{.*?\n    \}\n', lambda _: m.group(0), root, count=1, flags=re.S)
m = re.search(r'    fn reference_birth\(\).*?\}\)\}\n', old_root, re.S)
root = re.sub(r'    fn reference_birth\(\).*?\}\)\}\n', lambda _: m.group(0), root, count=1, flags=re.S)
root = re.sub(r'pub const ENTITY_KIND_SCENE_KINDS: &\[semio_s_plugin_norm_structural_scene::EntityKind\] = &\[.*?\];', 'pub const ENTITY_KIND_SCENE_KINDS: &[semio_s_plugin_norm_structural_scene::EntityKind] = &[semio_s_plugin_norm_structural_scene::EntityKind::Solid];', root, count=1, flags=re.S)
root = root.replace('catalogue_panel::render(Din16798PlayApp::examples(), view_state.locale, CONTROLLER_ID)', 'catalogue_panel::render(Din16798PlayApp::examples(), view_state.locale, CONTROLLER_ID, &semio_plugin::TreeWindows::for_body(view_state, catalogue_panel::BODY_CATALOGUE))')
root = root.replace('semio_s_artifact_norm_din16798_model::Din16798Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)', 'semio_s_artifact_norm_din16798_model::Din16798Snapshot::default()')
root = re.sub(r'semio_s_artifact_norm_din16798_mutations::resolve\b(?!_)', 'semio_s_artifact_norm_din16798_mutations::resolve_edit', root)
root = root.replace('#[cfg(test)]\n#[path = "🧪️tests/📚️assets/🦀️.rs"]\nmod asset_laws;\n', '')
write(f'{DST}/✏️editor/🦀️.rs', root)

# ---- viewer root
viewer = swap(read(f'{SRC}/👁️viewer/🦀️.rs'))
viewer = viewer.replace('semio_s_artifact_norm_din16798_model::Din16798Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)', 'semio_s_artifact_norm_din16798_model::Din16798Snapshot::default()')
write(f'{DST}/👁️viewer/🦀️.rs', viewer)
print('rolled')
