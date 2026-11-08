import os, shutil, sys

ART = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts'
S = '🏅️standards/🔖️1/🪆️subsets/✳️any'
SRC = f'{ART}/🧩️en1994/{S}'
DST = f'{ART}/⚡️din18599/{S}'
EW = '🎭️modes/✏️edit/🪟️windows'
VW = '🎭️modes/👁️view/🪟️windows'

SUBS = [
    ('🧩️en1994', '⚡️din18599'),
    ('EN-1994-Modell', 'DIN-V-18599-Modell'),
    ('EN 1994 model', 'DIN V 18599 model'),
    ('EN 1994', 'DIN V 18599'),
    ('En1994Family', 'DinV18599Family'),
    ('En1994', 'Din18599'),
    ('EN1994', 'DIN18599'),
    ('en1994', 'din18599'),
]


def swap(text):
    for a, b in SUBS:
        text = text.replace(a, b)
    text = text.replace('Din18599Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)', 'Din18599Snapshot::default()')
    return text


def rd(p):
    return open(p, encoding='utf8').read()


def wr(p, t):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, 'w', encoding='utf8').write(t)


def copy_swapped(rel, patch=None):
    t = swap(rd(f'{SRC}/{rel}'))
    if patch:
        t = patch(t)
    wr(f'{DST}/{rel}', t)


def must(t, old, new):
    assert old in t, old[:80]
    return t.replace(old, new, 1)


def cut(t, start, end):
    a = t.index(start)
    b = t.index(end, a)
    return t[:a] + t[b:]


for w in ('📥️inputs', '📊️results'):
    shutil.rmtree(f'{DST}/✏️editor/{EW}/{w}', ignore_errors=True)
shutil.rmtree(f'{DST}/👁️viewer/{VW}/📊️report', ignore_errors=True)

DESC = {
    'diagrams': ("buckling curves, LTB, M–N interaction, classification, S–N and fire curves of the selected check's entity", "monthly heating and cooling balance per zone, useful-final-primary energy flow and U-values of the selected check's entity"),
    'viewport': ('the projected steel scene with utilisation colouring, supports, loads and joints', 'the thermal zones and envelope elements as boxes and panels with utilisation colouring'),
}


def windows(base_rel, names, extra=None):
    for n in names:
        rel = f'{base_rel}/{n}'
        for dirpath, _, files in os.walk(f'{SRC}/{rel}'):
            for f in files:
                if f.endswith('.ts') or f.startswith('📌'):
                    continue
                r = os.path.relpath(os.path.join(dirpath, f), SRC)
                copy_swapped(r, extra)


def patch_window(t):
    for k, (a, b) in DESC.items():
        t = t.replace(a, b)
    t = t.replace('renders_the_buckling_diagram_of_the_default_member', 'renders_the_monthly_balance_of_the_default_zone')
    t = t.replace('rendered.contains("Flexural buckling curves") || rendered.contains("Knicklinien")', 'rendered.contains("Monthly balance") || rendered.contains("Monatsbilanz")')
    t = t.replace('the diagrams body must show the buckling diagram', 'the diagrams body must show the monthly balance')
    t = t.replace('!rendered.contains("\\"members\\"")', '!rendered.contains("\\"zones\\"")')
    t = t.replace('semio_s_artifact_norm_din18599_checks::case_catalogue::Din18599Cases', 'X')
    t = t.replace('render_inspector_with_cases::<crate::DinV18599Family, X>', 'render_inspector::<crate::DinV18599Family>')
    t = t.replace('render_inspector_with_cases', 'render_inspector')
    t = t.replace('the typed unit-aware inspector with the entity outliner, focused on the selected check\'s entity', 'the typed unit-aware inspector focused on the selected check\'s entity')
    return t


ED = f'✏️editor/{EW}'
windows(ED, ['✅️checks', '📈️diagrams', '🌲️outliner', '🧾️properties', '🧊️viewport'], patch_window)
for rel in (f'✏️editor/🎭️modes/✏️edit/🦀️.rs', f'✏️editor/🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs'):
    copy_swapped(rel)

vp = f'{DST}/{ED}/🧊️viewport/🦀️.rs'
t = rd(vp)
t = cut(t, '/// 🧱️ The scene projected from scratch', '/// 🕹️ The drag handles')
t = t.replace('/// 🕹️ The drag handles', '''/// 🧱️ The scene projected from scratch (no retained fragments): the baseline of the incremental-rebuild gate of the latency bench.
pub fn full_scene_of(document: &Din18599Snapshot, report: &CheckReport) -> StructuralScene {
    let revision = semio_s_plugin_norm_structural_scene::projection_kit::revision_of(document, &[], report);
    semio_s_artifact_norm_din18599_scene::project(&semio_s_artifact_norm_din18599_scene::SceneInput { document, report, revision })
}

/// 🕹️ The drag handles''', 1)
t = t.replace('semio_s_artifact_norm_din18599_scene::handles(document, &placements_of(document), selection)', 'semio_s_artifact_norm_din18599_scene::handles(document, selection)')
t = t.replace('use semio_s_plugin_norm_structural_scene::projection_kit::placements_of;\n', '')
a = t.index('/// ➕ The insert palette')
b = t.index('/// 🧊️ Projects the document')
t = t[:a] + '''/// ➕ The insert palette: one tool per entity kind of this standard.
pub fn insert_specs() -> Vec<InsertSpec> {
    semio_s_artifact_norm_din18599_scene::ENTITY_KINDS.iter().map(|kind| InsertSpec { id: kind.id.to_string(), collection: kind.collection.to_string(), kind: kind.scene_kind, label_en: kind.label_en.to_string(), label_de: kind.label_de.to_string() }).collect()
}

''' + t[b:]
t = t.replace('use semio_s_plugin_norm_structural_scene::{EntityKind, EntityRef, Handle, StructuralScene};', 'use semio_s_plugin_norm_structural_scene::{EntityRef, Handle, StructuralScene};')
wr(vp, t)

VI = f'👁️viewer/{VW}'
windows(f'👁️viewer/{VW}', ['✅️checks', '📈️diagrams', '🧊️viewport'], patch_window)
for rel in ('👁️viewer/🎭️modes/👁️view/🦀️.rs', '👁️viewer/🎭️modes/👁️view/🧪️tests/🔬️unit/🦀️.rs', '👁️viewer/🧪️tests/🔬️unit/🦀️.rs'):
    copy_swapped(rel)

ir = lambda t: t
def patch_viewer(t):
    return t

copy_swapped('👁️viewer/🦀️.rs', lambda t: t.replace('semio_s_artifact_norm_din18599_model::Din18599Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)', 'semio_s_artifact_norm_din18599_model::Din18599Snapshot::default()'))

def patch_root(t):
    t = cut(t, '/// 🎪️ The "full showcase example"', '#[derive(Default)]\npub struct Din18599PlayApp;')
    t = must(t, 'vec![semio_s_artifact_norm_din18599_examples::composite_floor_beam::source(), semio_s_artifact_norm_din18599_examples::composite_floor_beam_failing::source(), semio_s_artifact_norm_din18599_examples::composite_bridge_girder::source()].into_iter().chain([full_showcase_example(), composite_showcase_example()]).collect()',
             'vec![semio_s_artifact_norm_din18599_examples::demo::source(), semio_s_artifact_norm_din18599_examples::compliant_detached::source(), semio_s_artifact_norm_din18599_examples::noncompliant_detached::source(), semio_s_artifact_norm_din18599_examples::compliant_two_zone::source(), semio_s_artifact_norm_din18599_examples::cooled_office::source()]')
    a = t.index('pub const ENTITY_KIND_SCENE_KINDS')
    b = t.index('];', a) + 2
    t = t[:a] + 'pub const ENTITY_KIND_SCENE_KINDS: &[semio_s_plugin_norm_structural_scene::EntityKind] = &[semio_s_plugin_norm_structural_scene::EntityKind::Solid];' + t[b:]
    t = t.replace('The scene entity kinds DIN V 18599 projects', 'The scene entity kinds DIN V 18599 projects')
    t = t.replace('semio_s_artifact_norm_din18599_model::Din18599Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)', 'semio_s_artifact_norm_din18599_model::Din18599Snapshot::default()')
    t = must(t, '    fn interaction_topology(', '''    type Members = semio_s_artifact_stdio_semio::SemioMembers;

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_s_artifact_norm_din18599_model::din18599_child_restore_projection(snapshot)
    }

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, semio_framework_value::ValueError> {
        Ok(semio_s_artifact_norm_din18599_model::genesis_din18599_child_pack(snapshot, slot, child_id))
    }

    fn interaction_topology(''')
    t = t.replace('semio_s_artifact_norm_din18599_io::text::snapshot::decode_din18599_snapshot_json', 'semio_s_artifact_norm_din18599_io::text::snapshot::decode_din18599_snapshot_json')
    t = must(t, '''        catalogue_panel::BODY_CATALOGUE => catalogue_panel::render(Din18599PlayApp::examples(), view_state.locale, CONTROLLER_ID).and_then(|panel| semio_s_plugin_norm_app_surface::with_case_browser::<semio_s_artifact_norm_din18599_checks::case_catalogue::Din18599Cases>(panel, &host.document().cases, view_state.locale, CONTROLLER_ID, &semio_plugin::TreeWindows::for_body(view_state, catalogue_panel::BODY_CATALOGUE))),''',
             '''        catalogue_panel::BODY_CATALOGUE => catalogue_panel::render(Din18599PlayApp::examples(), view_state.locale, CONTROLLER_ID, &semio_plugin::TreeWindows::for_body(view_state, catalogue_panel::BODY_CATALOGUE)),''')
    t = must(t, 'resolve = insert_item::resolve_edit', 'resolve = semio_s_artifact_norm_din18599_mutations::resolve_edit')
    t = must(t, '''#[cfg(test)]
#[path = "🧪️tests/🗂️case-edits/🦀️.rs"]
mod case_edit_tests;
#[cfg(test)]
#[path = "🧪️tests/📚️assets/🦀️.rs"]
mod asset_laws;
''', '')
    t = t.replace('NormFamilyId::Din18599', 'NormFamilyId::DinV18599')
    t = must(t, '<DinV18599Family as semio_s_plugin_norm_compliance::IncrementalNormFamily>::evaluate_incrementally(document)', 'semio_s_artifact_norm_din18599_inferences::evaluate(document)')
    t = t.replace('#[path = "🧮️incremental/🦀️.rs"]\npub mod incremental;\n', '')
    return t

copy_swapped('✏️editor/🦀️.rs', patch_root)

shutil.move(f'{DST}/✏️editor/🎭️modes/✏️edit/🏷️field-meta', f'{DST}/✏️editor/🏷️field-meta') if os.path.isdir(f'{DST}/✏️editor/🎭️modes/✏️edit/🏷️field-meta') else None

for w in ('inputs', 'results'):
    pass

def toml(rel, add_after, lines):
    p = f'{DST}/{rel}'
    t = rd(p)
    for ln in lines:
        if ln not in t:
            t = t.replace(add_after, add_after + ln + '\n', 1)
    wr(p, t)

toml('✏️editor/📦️packages/🦀️rust/Cargo.toml', 'semio-s-artifact-norm-din18599-mutations = { workspace = true }\n', ['semio-s-artifact-norm-din18599-scene = { workspace = true }', 'semio-s-plugin-norm-structural-scene = { workspace = true }', 'semio-framework-pack-json = { workspace = true }', 'semio-framework-diagnostic = { workspace = true }'])
toml('👁️viewer/📦️packages/🦀️rust/Cargo.toml', 'semio-s-artifact-norm-din18599-model = { workspace = true }\n', ['semio-s-artifact-norm-din18599-scene = { workspace = true }', 'semio-s-artifact-norm-din18599-inferences = { workspace = true }', 'semio-s-artifact-norm-din18599-mutations = { workspace = true }', 'semio-s-plugin-norm-compliance = { workspace = true }', 'semio-s-plugin-norm-structural-scene = { workspace = true }', 'semio-framework-diagnostic = { workspace = true }', 'semio-framework-dsl-record-derive = { workspace = true }', 'semio-framework = { workspace = true }'])
print('ported')
