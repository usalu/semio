import os, sys, json
T = os.path.dirname(os.path.abspath(__file__))
V = '️'
FILES = [l.rstrip('\n') for l in open(os.path.join(T, 'files.txt'), encoding='utf-8') if l.strip()]
TAGS = ['layoutdiff', 'picking', 'playapp', 'cadunit', 'inferences', 'compliance', 'appsurface', 'en1990']


def ch(emoji, kebab, ranges, doc, **kw):
    return dict(folder=emoji + V + kebab, mod=kebab.replace('-', '_'), ranges=ranges, doc='//! %s%s %s See [`super`].' % (emoji, V, doc), **kw)


def lift(emoji, kebab, mod, doc, **kw):
    return dict(folder=emoji + V + kebab, mod=mod, doc='//! %s%s %s See [`super`].' % (emoji, V, doc), **kw)


def ext(i, *rel):
    d = os.path.dirname(FILES[i - 1])
    return [os.path.normpath(os.path.join(d, r)) for r in rel]


TABLE = {
 'layoutdiff': dict(i=1, children=[
    ch('🧱', 'grid-text-and-guide-deltas', [(66, 247)], 'Grid, text style run and page guide deltas of the layout diff: the grid patch, the data-field presence deltas and the keyed row deltas of the text and guide lists.'),
    ch('🗂', 'data-entries-delta', [(249, 352)], 'Data entries delta of the layout diff: the keyed data entry rows and their removal, insertion and relocation records.'),
    ch('🖼', 'page-frames-delta', [(354, 478)], 'Page frames delta of the layout diff: the keyed frame rows of a page and their delta algebra.'),
    ch('🩹', 'row-patches', [(480, 865)], 'Row patches of the layout diff: the paragraph, character, parent page, spread, page and grid patches and their row patch implementations.'),
    ch('📍', 'positional-algebra', [(869, 976)], 'Positional algebra of the layout diff: the positional list delta trait, its apply, absorb and inverse operations and the payload row helpers.'),
    ch('🧮', 'diff-apply-and-algebra', [(980, 1170)], 'Apply and algebra of the layout diff: the diff absorption, the mutation diff application and the diff algebra implementation.'),
    ch('🧩', 'patchable-frames', [(1177, 1487)], 'Patchable rows of the layout diff: the patchable style, page and story rows, the frame field patch and the frame selection helpers.'),
 ], external=['@dir/🧪️tests/🔬️unit/🦀️.rs', '@dir/../../🚪️io/💾️binary/🔺️diff/🦀️.rs', '@dir/../../🚪️io/📝️text/🔺️diff/🦀️.rs']),
 'picking': dict(i=2, children=[
    ch('🏷', 'pick-vocabulary', [(28, 317)], 'Vocabulary of the CAD pick engine: the pick kinds, targets, toggles, kernel kind mapping and hover keys.'),
    ch('🪣', 'geometry-buckets-and-curves', [(321, 703)], 'Geometry buckets and curve sampling of the CAD pick engine: the anchor rows, the per-kind buckets and the arc, circle, ellipse and NURBS sample points.'),
    ch('🎯', 'pick-targets-and-indices', [(707, 917)], 'Pick target construction of the CAD pick engine: the object and typology indices and the pick target creation, filtering and events.'),
    ch('🎚', 'visibility-and-toggles', [(921, 1142)], 'Visibility and toggles of the CAD pick engine: the entity flags, kind, typology and primitive toggles and the scene visibility.'),
    ch('👁', 'reveal-and-selection-merge', [(1146, 1274)], 'Reveal and selection merging of the CAD pick engine: the revealed objects, the render targets and the modifier-mode selection merges.'),
    ch('🎨', 'target-style', [(1278, 1382)], 'Target styling of the CAD pick engine: the scene palette and the per-target material style.'),
 ], external=['@dir/🧪️tests/🔬️unit/🦀️.rs']),
 'playapp': dict(i=3, keep_empty=['🧪️Tests'], children=[
    ch('⌨', 'command-from-action', [(173, 283)], 'Action decoding of the CAD play app: the flag value reader and the command from action mapping.'),
    ch('🧵', 'retained-command-tools', [(339, 613)], 'Retained command tools of the CAD play app: the tool ids, publication contracts, retained reduce and the command job factory.'),
    ch('📤', 'media-export-job', [(617, 776)], 'Media job of the CAD play app: the interactive export and import job and its command work.'),
    ch('🔧', 'config-store-preparation', [(779, 901)], 'Config store preparation of the CAD play app: the bounded admission of config mutations.'),
    ch('🗄', 'artifact-store-preparation', [(905, 1049)], 'Artifact store preparation of the CAD play app: the bounded admission of snapshot mutations.'),
    ch('🪟', 'editor-and-interaction-scope', [(1053, 1436)], 'Editor implementation of the CAD play app: the world body keys, the interaction scope and the artifact editor trait implementation.'),
    ch('🧭', 'app-manifest', [(1440, 1664)], 'Manifest of the CAD play app: the Dislocate utility, the interaction definition and the app assembly.'),
 ], external=['@dir/🌉️wasm/🦀️.rs', '@dir/📚️examples/🎬️demo-session/🦀️.rs', '@dir/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs']),
 'cadunit': dict(i=4, children=[
    ch('🧰', 'command-fixtures-and-retained-lanes', [(171, 538)], 'Command fixtures and retained lanes of the CAD editor unit tests: the every-command fixture, the example admission, the retained factory proofs and the wire round trips.'),
    ch('🌲', 'forest-mesh-and-schema', [(540, 813)], 'Forest mesh and schema tests of the CAD editor unit tests: the optional field rows, the per-object meshes, the forest reference geometry and the schema checks.'),
    ch('🖼', 'rendered-scene-and-manifest', [(816, 1077)], 'Rendered scene and manifest tests of the CAD editor unit tests: the world scene per pane, the utility, the manifest stitching and the reference verbs.'),
    ch('👁', 'view-model', [(1079, 1274)], 'View model tests of the CAD editor unit tests: the gumball, camera, utility, context menu and sun measure behaviour.'),
    ch('⚙', 'operations-and-file-effects', [(1278, 1412)], 'Operation tests of the CAD editor unit tests: the composed panes, object operations, the derived transformations and the file effects.'),
    ch('🎮', 'engagement-and-import', [(1415, 1600)], 'Engagement and import tests of the CAD editor unit tests: the interaction sessions, window transient state and the spatial and OBJ imports.'),
    ch('⏪', 'history-and-convergence', [(1603, 1858)], 'History, convergence and object child lane tests of the CAD editor unit tests: undo and redo, the backbone convergence and the rendered instance lane.'),
    ch('🧩', 'contributions', [(1862, 2037)], 'Contribution tests of the CAD editor unit tests: the computer packs and the demonstrator packs admitted by the retained command.'),
    ch('🎚', 'engagement-commit-and-render-probes', [(2041, 2202)], 'Engagement submit, commit and probe tests of the CAD editor unit tests: the state confirm, the committing event, the interaction scope, the render cost probe and the solid export.'),
    ch('🧬', 'geometry-media-and-owners', [(8, 76), (2205, 2531)], 'Geometry media and owner tests of the CAD editor unit tests: the imported typed owners, the intrinsic and owned geometry media, the pricing and the parent operation pack.'),
 ], external=['@dir/../📦️member-authority/🦀️.rs', '@dir/../🧰️support/🦀️.rs']),
 'inferences': dict(i=5, expect_missing=['mod derive_transformation {', 'mod construct_query {', 'mod scene_compute {', '}', '}', '}', 'include!("🧪️tests/🔬️derive-transformation-unit/🦀️.rs");', 'include!("🧪️tests/🔬️construct-query-unit/🦀️.rs");'], expect_extra=['include!("../🧪️tests/🔬️derive-transformation-unit/🦀️.rs");', 'include!("../🧪️tests/🔬️construct-query-unit/🦀️.rs");'], children=[], lift=[
    lift('🔄', 'derive-transformation', 'derive_transformation', 'Derive transformation of the cad inference: the classification rules, face analytics, solid construction and derive engine.',
         patch=[('    include!("🧪️tests/🔬️derive-transformation-unit/🦀️.rs");'.lstrip(), 'include!("../🧪️tests/🔬️derive-transformation-unit/🦀️.rs");')]),
    lift('🔍', 'construct-query', 'construct_query', 'Construct query of the cad inference: the read-only graph adapter over one geometry pane.',
         patch=[('    include!("🧪️tests/🔬️construct-query-unit/🦀️.rs");'.lstrip(), 'include!("../🧪️tests/🔬️construct-query-unit/🦀️.rs");')]),
    lift('🖥', 'scene-compute', 'scene_compute', 'Scene compute of the cad inference: the derived scene, mesh and contribution compute over the snapshot.'),
 ], external=['@dir/🧪️tests/🔬️unit/🦀️.rs', '@dir/📦bounds/🦀️.rs', '@dir/../../🚪️io/💾️binary/💡️inferences/🦀️.rs', '@dir/../../🚪️io/📝️text/💡️inferences/🦀️.rs']),
 'compliance': dict(i=6, children=[
    ch('📐', 'quantities-clauses-and-text', [(18, 132)], 'Quantities, clauses and localized text of the norm compliance core.'),
    ch('✅', 'check-results', [(136, 519)], 'Check results of the norm compliance core: the status, remedies, check builder, part verdicts and the check report.'),
    ch('📜', 'annex-and-tables', [(523, 616)], 'National annex selection and table lookups of the norm compliance core.'),
    ch('🏗', 'design-situation-and-shared', [(620, 790)], 'Design situations and shared classifications of the norm compliance core: the consequence classes, imposed categories, limit states, climate zones and occupancies.'),
    ch('🏛', 'norm-error-and-family', [(793, 957)], 'Norm errors and families of the norm compliance core: the error type, the family ids and trait, the report cache and the norm host.'),
    ch('🔤', 'op-text-and-pack-codec', [(966, 1064)], 'Operation text and pack codec of the norm compliance core: the field escaping, the artifact record trait and the snapshot dsl and pack codec.'),
 ], external=['@dir/🧪️tests/🔬️child-identity-oracle/🦀️.rs', '@dir/🧪️tests/🔬️unit/🦀️.rs']),
 'appsurface': dict(i=7, children=[
    ch('🛡', 'store-ownership-macros', [(88, 156)], 'Exported store ownership macros of the norm app surface: the exact bounded store owners and the read-only viewer twin every norm editor installs.'),
    ch('🏷', 'ids-chrome-and-labels', [(159, 304)], 'Ids, window layout and labels of the norm app surface.'),
    ch('🧭', 'value-path', [(308, 557)], 'Value paths of the norm app surface: the path parser and the get, set, insert and remove operations on a document value.'),
    ch('🔖', 'field-meta', [(559, 675)], 'Field metadata of the norm app surface: the field choices and the wildcard-aware meta resolution.'),
    ch('✏', 'edit-rules', [(677, 946)], 'Edit rules of the norm app surface: the edit kinds, selector keys, edit rules and the set, insert, remove and remedy handlers.'),
    ch('🌳', 'report-tree', [(950, 1176)], 'Report tree rendering of the norm app surface: the roster example, the text helpers, the check tree and the report render.'),
    ch('📝', 'document-editors', [(1178, 1390)], 'Document editors of the norm app surface: the object, array and value editors and the summary.'),
    ch('📚', 'catalogue-and-inspection', [(1392, 1605)], 'Catalogue and inspection of the norm app surface: the catalogue table and the inspection render.'),
    ch('🪟', 'manifest-and-media', [(1609, 1741)], 'Manifest and media of the norm app surface: the mode, window and panel definitions, the artifact kind spec and the media export and import.'),
    ch('⌨', 'command-args-and-dispatch', [(1745, 1825)], 'Command arguments and dispatch of the norm app surface: the argument readers and the dispatch helpers.'),
    ch('🏭', 'command-and-tool-job-macros', [(1827, 1931), (2239, 2302)], 'Exported command and tool job macros of the norm app surface: the action to typed command bridge and the owned tool job factory every norm app declares.'),
    ch('🧵', 'retained-commands', [(1935, 2237), (2304, 2347)], 'Retained commands of the norm app surface: the publication contracts, the retained editor trait, the evaluate work and the tool job builder.'),
 ], external=['@dir/🧪️tests/🔬️retained-disposition-oracle/🦀️.rs', '@dir/🧪️tests/🔬️unit/🦀️.rs']),
 'en1990': dict(i=8, children=[
    ch('🧰', 'report-helpers', [(101, 188)], 'Report helpers of the EN 1990 compliance report: the copy, quantity and subject helpers and the action set per member.'),
    ch('🏗', 'ultimate-limit-state', [(190, 355), (513, 584)], 'Ultimate limit state checks of the EN 1990 compliance report: the structural, equilibrium, accidental, seismic, geotechnical and stability combinations.'),
    ch('📏', 'serviceability-and-fatigue', [(357, 511)], 'Serviceability and fatigue checks of the EN 1990 compliance report: the action effects, deflection, vibration and fatigue combinations.'),
    ch('🌍', 'working-life-and-bridge-annexes', [(586, 826)], 'Design working life and bridge annex checks of the EN 1990 compliance report: the working life, annex B and the A2 bridge serviceability.'),
    ch('🧮', 'partial-factor-sets', [(828, 896)], 'Partial factor sets of the EN 1990 compliance report: the K_FI and geotechnical set C factors.'),
    ch('📋', 'compliance-evaluation', [(1063, 1112)], 'Evaluation entry point of the EN 1990 compliance report: the member loop that gathers every check into one report.'),
    ch('⛓', 'referential-integrity', [(898, 1061)], 'Referential integrity of the EN 1990 compliance report: the action and member ids, duplicate and dangling references and unassessable members.'),
 ], external=['@dir/🧪️tests/🔬️unit/🦀️.rs', '@dir/🧪️tests/🔬️compliance-report/🦀️.rs', '@dir/🧾outline/🦀️.rs']),
}


OVERRIDES = {
    'layoutdiff': {'🧮️diff-apply-and-algebra': dict(reexport=False)},
    'playapp': {'🪟️editor-and-interaction-scope': dict(reexport=False)},
    'cadunit': {'*': dict(reexport=False), '🧰️command-fixtures-and-retained-lanes': dict(reexport=True)},
    'compliance': {'🏗️design-situation-and-shared': dict(glob=False)},
    'appsurface': {'🧭️value-path': dict(glob=False), '🛡️store-ownership-macros': dict(glob=False), '🏭️command-and-tool-job-macros': dict(glob=False)},
}


def main():
    os.makedirs(os.path.join(T, 'specs'), exist_ok=True)
    for tag in TAGS:
        t = TABLE[tag]
        f = FILES[t['i'] - 1]
        d = os.path.dirname(f)
        externals = []
        for e in t.get('external', []):
            if e.startswith('@dir/'):
                externals.append(os.path.normpath(os.path.join(d, e[5:])))
            else:
                externals.append(e)
        for c in t['children']:
            ov = OVERRIDES.get(tag, {})
            c.update(ov.get('*', {}))
            c.update(ov.get(c['folder'], {}))
        sp = dict(orig='%d.rs' % t['i'], file=f, nest=False, children=t['children'], external=externals)
        for k in ('lift', 'keep_empty', 'expect_missing', 'expect_extra'):
            if t.get(k):
                sp[k] = t[k]
        with open(os.path.join(T, 'specs', tag + '.py'), 'w', encoding='utf-8') as fh:
            fh.write('import json\nSPEC = json.loads(%r)\n' % json.dumps(sp, ensure_ascii=False))
        print('spec', tag, len(t['children']) + len(t.get('lift', [])), 'children')


main()
