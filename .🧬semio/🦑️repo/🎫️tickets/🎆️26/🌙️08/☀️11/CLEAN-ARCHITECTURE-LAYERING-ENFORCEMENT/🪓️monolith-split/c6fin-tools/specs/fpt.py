from specs.lib import *
FT = PL + '/🧪️testing/🦀️.rs'
MISINDENTED = [197, 198, 199, 200, 201, 202, 203, 748, 1004, 1099, 1476, 1965, 2331]
def pre(text):
    lines = text.split('\n')
    for n in MISINDENTED:
        assert lines[n - 1] and not lines[n - 1].startswith(' '), n
        lines[n - 1] = '    ' + lines[n - 1]
    return '\n'.join(lines)
CH = [
    ch('🎟', 'fixture-dispatch-steps-and-document-loading', 'fixture_dispatch_steps_and_document_loading', [(20, 379)],
       'Fixture dispatch, steps and document loading: the fixture action dispatch, the mounted policy, retained turn and step context, the fixture step outcome, the document load helpers, the typing run, the maintenance pressure drain and the genesis children assertions.'),
    ch('🌳', 'fixture-tree-projection-and-scenes', 'fixture_tree_projection_and_scenes', [(381, 574)],
       'Fixture tree projection and scenes: the fixture tree limits, the node projection and retirement, the document input commit findings, the fixture scene decode and the built surface and carrier text.'),
    ch('🏗', 'registered-app-construction-and-typed-operation-settlement', 'registered_app_construction_and_typed_operation_settlement', [(576, 948)],
       'Registered app construction and typed operation settlement: the reserved action factories, the app constructors and registries, the store constructor settlement, the typed operation receipts and settlement, the extension invocations and the registered app close.'),
    ch('👯', 'paired-apps-history-and-convergence-laws', 'paired_apps_history_and_convergence_laws', [(950, 1094), (1534, 1770)],
       'Paired apps, history and convergence laws: the paired apps, the history verb settlement, the undo redo round trip, the declared actions bridge, the two instance convergence, the ingest idempotency and the transaction laws.'),
    ch('🔎', 'declared-verb-probing', 'declared_verb_probing', [(1096, 1530)],
       'Declared verb probing: the declared verb lanes, fingerprints, boot, state and dispatch, the registered app render, the addressed and agent previews and the declared verb probes with their honour assertion.'),
    ch('🔭', 'bounded-viewer-fixture-and-viewer-laws', 'bounded_viewer_fixture_and_viewer_laws', [(1777, 2132)],
       'Bounded viewer fixture and viewer laws: the bounded viewer fixture with its artifact viewer implementation, the editor command reduction and the viewer mutation, dialect and dependency assertions.'),
    ch('🧭', 'declaration-registration-laws', 'declaration_registration_laws', [(2135, 2198)],
       'Declaration registration laws: the declaration tree registration of every plugin, the atomic declaration registration and the derived subset declaration ids.'),
]
SPEC = dict(orig='fpt.rs', file=FT, children=[], wiring='block', external=mounted(FT, orig='fpt.rs'), pre=pre,
            expect_missing=['pub mod artifact_app_laws {', '}'],
            inner=dict(mod='artifact_app_laws', indent=4, folder='🧪️artifact-app-laws', nest=False, rebase_super=True,
                       doc='//! 🧪️ The generic test-harness helpers for `ArtifactApp` implementors, with the shared imports of the former inline module. See [`super`].',
                       children=CH))
