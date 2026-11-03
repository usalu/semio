import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os
import shutil

ART = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts'
S = '🏅️standards/🔖️1/🪆️subsets/✳️any'
SRC = f'{ART}/🔩️en1993/{S}'
DST = f'{ART}/🪨️en1996/{S}'
EDIT_WINDOWS = '🎭️modes/✏️edit/🪟️windows'
VIEW_WINDOWS = '🎭️modes/👁️view/🪟️windows'

SWAPS = [
    ('EN 1993', 'EN 1996'), ('En1993', 'En1996'), ('en1993', 'en1996'), ('EN1993', 'EN1996'),
    ('the projected steel scene with utilisation colouring, supports, loads and joints', 'the projected masonry scene with utilisation zones, openings, floors, loads and yield-line patterns'),
    ("buckling curves, LTB, M–N interaction, classification, S–N and fire curves of the selected check's entity", "eccentricity, Φ, N_Ed against N_Rd, slenderness, f_k, shear, lateral α and capacity, fire thickness and M–N diagrams of the selected check's wall"),
    ('"members"', '"walls"'),
    ('Flexural buckling curves', 'Eccentricity along the height'),
    ('Knicklinien', 'Ausmitte über die Höhe'),
    ('renders_the_buckling_diagram_of_the_default_member', 'renders_the_eccentricity_diagram_of_the_default_wall'),
    ('the diagrams body must show the buckling diagram', 'the diagrams body must show the eccentricity diagram'),
    ('noncompliant_overloaded_frame', 'noncompliant_multi_fail'),
]


def swap(text):
    for a, b in SWAPS:
        text = text.replace(a, b)
    return text


def copy_tree(src, dst):
    for root, _, files in os.walk(src):
        for name in files:
            if name == '📌️.empty.md':
                continue
            path = os.path.join(root, name)
            target = os.path.join(dst, os.path.relpath(path, src))
            os.makedirs(os.path.dirname(target), exist_ok=True)
            text = open(path, encoding='utf8').read()
            open(target, 'w', encoding='utf8').write(swap(text))


def twin(en_icon, window, body, what, kind='edit'):
    n = window.capitalize()
    return f'''/** {en_icon} EN 1996 {'editor' if kind == 'edit' else 'viewer'} — {window} window: typed twin of `🦀️.rs`, {what}. */

export interface En1996{n}ViewModel {{
  windowKindId: "norm-en1996-{window}";
  bodyKey: "norm.en1996.{'play' if kind == 'edit' else 'view'}.{window}";
}}

export const EN1996_{'EDIT' if kind == 'edit' else 'VIEW'}_{window.upper()}_WINDOW_KIND_ID = "norm-en1996-{window}" as const;
export const EN1996_{'EDIT' if kind == 'edit' else 'VIEW'}_{window.upper()}_BODY_KEY = "norm.en1996.{'play' if kind == 'edit' else 'view'}.{window}" as const;
'''


WHAT = {
    'viewport': 'the projected masonry scene on the `world-3d` surface',
    'diagrams': 'the eccentricity, reduction factor, capacity, slenderness, strength, shear, lateral, fire and M–N diagrams with table twins',
    'properties': 'the typed unit-aware inspector with the entity outliner',
    'checks': 'every computed check with utilisation bars, clause links and remedies',
}
ICON = {'viewport': '🧊️', 'diagrams': '📈️', 'properties': '🧾️', 'checks': '✅️'}
DIRS = {'viewport': '🧊️viewport', 'diagrams': '📈️diagrams', 'properties': '🧾️properties', 'checks': '✅️checks'}

for old in ('📥️inputs', '📊️results'):
    shutil.rmtree(f'{DST}/✏️editor/{EDIT_WINDOWS}/{old}', ignore_errors=True)
shutil.rmtree(f'{DST}/👁️viewer/{VIEW_WINDOWS}/📊️report', ignore_errors=True)

for w, d in DIRS.items():
    copy_tree(f'{SRC}/✏️editor/{EDIT_WINDOWS}/{d}', f'{DST}/✏️editor/{EDIT_WINDOWS}/{d}')
    open(f'{DST}/✏️editor/{EDIT_WINDOWS}/{d}/🟦️.ts', 'w', encoding='utf8').write(twin(ICON[w], w, w, WHAT[w]))
for w in ('viewport', 'diagrams', 'checks'):
    d = DIRS[w]
    copy_tree(f'{SRC}/👁️viewer/{VIEW_WINDOWS}/{d}', f'{DST}/👁️viewer/{VIEW_WINDOWS}/{d}')
    open(f'{DST}/👁️viewer/{VIEW_WINDOWS}/{d}/🟦️.ts', 'w', encoding='utf8').write(twin(ICON[w], w, w, WHAT[w], 'view'))

for rel in (f'✏️editor/🎭️modes/✏️edit/🦀️.rs', f'✏️editor/🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs', f'👁️viewer/🎭️modes/👁️view/🦀️.rs', f'👁️viewer/🎭️modes/👁️view/🧪️tests/🔬️unit/🦀️.rs'):
    os.makedirs(os.path.dirname(f'{DST}/{rel}'), exist_ok=True)
    open(f'{DST}/{rel}', 'w', encoding='utf8').write(swap(open(f'{SRC}/{rel}', encoding='utf8').read()))

ts = f'{DST}/✏️editor/🟦️.ts'
text = open(ts, encoding='utf8').read()
text = text.replace('export * as inputsWindow from "./🎭️modes/✏️edit/🪟️windows/📥️inputs/🟦️";\nexport * as resultsWindow from "./🎭️modes/✏️edit/🪟️windows/📊️results/🟦️";\n', ''.join(f'export * as {w}Window from "./🎭️modes/✏️edit/🪟️windows/{DIRS[w]}/🟦️";\n' for w in ('viewport', 'diagrams', 'properties', 'checks')))
open(ts, 'w', encoding='utf8').write(text)

vts = f'{DST}/👁️viewer/🟦️.ts'
text = open(vts, encoding='utf8').read()
print(text[-400:])

root = f'{DST}/✏️editor/🦀️.rs'
t = open(root, encoding='utf8').read()


def rep(old, new, count=1):
    global t
    assert old in t, old
    t = t.replace(old, new, count)


rep('use crate::modes::edit::windows::{inputs, results};', 'use crate::modes::edit::windows::{checks, diagrams, properties, viewport};')
rep('//#region ðï¸Constants', '''/// 🏷️ The scene entity kinds EN 1996 projects: one interaction granularity each.
pub const ENTITY_KIND_SCENE_KINDS: &[semio_s_plugin_norm_structural_scene::EntityKind] = &[
    semio_s_plugin_norm_structural_scene::EntityKind::Group,
    semio_s_plugin_norm_structural_scene::EntityKind::Solid,
    semio_s_plugin_norm_structural_scene::EntityKind::Plate,
    semio_s_plugin_norm_structural_scene::EntityKind::Surface,
    semio_s_plugin_norm_structural_scene::EntityKind::Fastener,
    semio_s_plugin_norm_structural_scene::EntityKind::Support,
    semio_s_plugin_norm_structural_scene::EntityKind::Load,
];

//#region ðï¸Constants''') if '//#region ðï¸Constants' in t else None
rep('registry.register::<results::ResultsWindowConfigOwner>()', 'registry.register::<checks::ChecksWindowConfigOwner>()')
rep('''        match body_key {
            inputs::BODY_INPUTS => inputs::render(doc.snapshot, view_state.locale, CONTROLLER_ID, &semio_framework_plugin::TreeWindows::for_body(view_state, inputs::BODY_INPUTS)),
            results::BODY_RESULTS => results::render(&host, &semio_framework_plugin::TreeWindows::for_body(view_state, results::BODY_RESULTS), view_state.locale, Some(CONTROLLER_ID)),
''', '''        let selected = semio_s_plugin_norm_results_config::current::<checks::ChecksWindowConfigOwner>(cfg).selected_check_index;
        let focus = selected.and_then(|index| host.report().checks.get(index as usize)).map(|check| check.subject.entity_id.clone()).filter(|id| !id.is_empty());
        let worst = host.report().checks.iter().filter(|check| check.status != semio_s_plugin_norm_compliance::CheckStatus::NotApplicable && !check.subject.entity_id.is_empty()).max_by(|a, b| a.utilization.total_cmp(&b.utilization)).map(|check| check.subject.entity_id.as_str());
        match body_key {
            properties::BODY_PROPERTIES => properties::render(&host, focus.as_deref(), view_state.locale, CONTROLLER_ID, &semio_framework_plugin::TreeWindows::for_body(view_state, properties::BODY_PROPERTIES)),
            viewport::BODY_VIEWPORT => viewport::render(doc.snapshot, host.report(), focus.as_deref(), view_state.locale),
            diagrams::BODY_DIAGRAMS => diagrams::render(doc.snapshot, host.report(), focus.as_deref().or(worst), &semio_framework_plugin::TreeWindows::for_body(view_state, diagrams::BODY_DIAGRAMS), view_state.locale),
            checks::BODY_CHECKS => checks::render(host.report(), selected, &semio_framework_plugin::TreeWindows::for_body(view_state, checks::BODY_CHECKS), view_state.locale, Some(CONTROLLER_ID)),
''')
rep('inspection_panel::render(&host, semio_s_plugin_norm_results_config::current::<results::ResultsWindowConfigOwner>(cfg).selected_check_index, view_state.locale', 'inspection_panel::render(&host, selected, view_state.locale')
rep('type ResultsWindowConfigOwner = results::ResultsWindowConfigOwner;', 'type ResultsWindowConfigOwner = checks::ChecksWindowConfigOwner;')
rep('''            .window_kind_def(inputs::definition())
            .window_kind_def(results::definition())
''', '''            .interaction(semio_s_plugin_norm_app_surface::viewport_interaction_definition(VARIANT, LocalizedLabel::native("EN 1996 model", "EN-1996-Modell"), ENTITY_KIND_SCENE_KINDS))
            .window_kind_def(properties::definition())
            .window_kind_def(viewport::definition())
            .window_kind_def(diagrams::definition())
            .window_kind_def(checks::definition())
''')
t = t.replace('refreshes the results window', 'refreshes the checks window').replace('aktualisiert das Ergebnisfenster', 'aktualisiert das Nachweisfenster').replace('in the results list', 'in the checks list').replace('in der Ergebnisliste', 'in der Nachweisliste')
open(root, 'w', encoding='utf8').write(t)

tests = f'{DST}/✏️editor/🧪️tests/🔬️unit/🦀️.rs'
u = open(tests, encoding='utf8').read()
u = u.replace('results::WINDOW_RESULTS', 'checks::WINDOW_CHECKS')
u = u.replace('[inputs::BODY_INPUTS, results::BODY_RESULTS,', '[properties::BODY_PROPERTIES, viewport::BODY_VIEWPORT, diagrams::BODY_DIAGRAMS, checks::BODY_CHECKS,')
open(tests, 'w', encoding='utf8').write(u)

cargo = f'{DST}/✏️editor/📦️packages/🦀️rust/Cargo.toml'
c = open(cargo, encoding='utf8').read()
c = c.replace('semio-s-artifact-norm-en1996-schema = { workspace = true }\n', 'semio-s-artifact-norm-en1996-scene = { workspace = true }\nsemio-s-artifact-norm-en1996-schema = { workspace = true }\n', 1)
if 'structural-scene' not in c:
    c = c.replace('semio-s-plugin-norm-app-surface = { workspace = true }\n', 'semio-s-plugin-norm-app-surface = { workspace = true }\nsemio-s-plugin-norm-structural-scene = { workspace = true }\n', 1)
open(cargo, 'w', encoding='utf8').write(c)

vroot = f'{DST}/👁️viewer/🦀️.rs'
t = open(vroot, encoding='utf8').read()
rep('use crate::modes::view::windows::report;', 'use crate::modes::view::windows::{checks, diagrams, viewport};')
rep('            report::BODY_KEY => report::render(doc.snapshot, view_state.locale).map(semio_framework_plugin::built_to_component_tree),\n', '''            viewport::BODY_VIEWPORT => viewport::render(doc.snapshot, view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            diagrams::BODY_DIAGRAMS => diagrams::render(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, diagrams::BODY_DIAGRAMS), view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            checks::BODY_CHECKS => checks::render(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, checks::BODY_CHECKS), view_state.locale).map(semio_framework_plugin::built_to_component_tree),
''')
rep('        .window_kind_def(report::definition())\n', '        .window_kind_def(viewport::definition())\n        .window_kind_def(checks::definition())\n        .window_kind_def(diagrams::definition())\n')
open(vroot, 'w', encoding='utf8').write(t)

vc = f'{DST}/👁️viewer/📦️packages/🦀️rust/Cargo.toml'
c = open(vc, encoding='utf8').read()
c = c.replace('semio-s-artifact-norm-en1996-mutations = { workspace = true }\n', 'semio-s-artifact-norm-en1996-mutations = { workspace = true }\nsemio-s-artifact-norm-en1996-scene = { workspace = true }\n', 1)
if 'structural-scene' not in c:
    c = c.replace('semio-s-plugin-norm-app-surface = { workspace = true }\n', 'semio-s-plugin-norm-app-surface = { workspace = true }\nsemio-s-plugin-norm-structural-scene = { workspace = true }\n', 1)
open(vc, 'w', encoding='utf8').write(c)

vt = open(vts, encoding='utf8').read()
vt = vt.replace('export * from "./🎭️modes/👁️view/🪟️windows/📊️report/🟦️";\n', ''.join(f'export * as {w}Window from "./🎭️modes/👁️view/🪟️windows/{DIRS[w]}/🟦️";\n' for w in ('viewport', 'diagrams', 'checks')))
open(vts, 'w', encoding='utf8').write(vt)
print('done')
