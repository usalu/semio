import os, sys, json
T = os.path.dirname(os.path.abspath(__file__))
V = '️'
P = '✏️s/🔌️plugins/'
WFC = P + '🀄️wfc/🗿️artifacts/'
PRO = P + '🌀️procedural/🗿️artifacts/'
SUB = '/🏅️standards/🔖️1/🪆️subsets/✳️any'
FILES = dict(
    wfcbitmap=WFC + '🖼️bitmap' + SUB + '/✏️editor/🦀️.rs',
    wfc3d=WFC + '🧊️3d' + SUB + '/✏️editor/🦀️.rs',
    wfchost=WFC + '🧊️3d/🔨️modules/🏠️host/💡️inferences/🦀️.rs',
    gen2d=PRO + '🌀️generation2d' + SUB + '/✏️editor/🦀️.rs',
    gen2dunit=PRO + '🌀️generation2d' + SUB + '/✏️editor/🧪️tests/🔬️unit/🦀️.rs',
    value=PRO + '🧊️generation3d' + SUB + '/🚪️io/📝️text/🔣️value/🦀️.rs',
    preview=PRO + '🧊️generation3d' + SUB + '/🧵️preview-eval/🦀️.rs',
    poly=P + '➗️mathematical/🗿️artifacts/➗️equation' + SUB + '/🧬️schema/💡️inferences/📈️polynomial-internals/🦀️.rs',
)

def ch(emoji, kebab, mod, ranges, doc, **kw):
    return dict(folder=emoji + V + kebab, mod=mod, ranges=[list(r) for r in ranges], doc='//! %s%s %s See [`super`].' % (emoji, V, doc), **kw)

TABLE = {
 'wfcbitmap': dict(external=[WFC + '🖼️bitmap' + SUB + '/✏️editor/🧪️tests/🔬️unit/🦀️.rs'], children=[
    ch('🎛', 'command-and-action-bridge', 'command_and_action_bridge', [(76, 337)], 'Command surface of the WFC bitmap editor: the command vocabulary with its wire ids and tool ids and the argument bridge that turns declared action arguments into commands.'),
    ch('🧵', 'retained-commands', 'retained_commands', [(341, 525)], 'Retained command route of the WFC bitmap editor: the bounded publication contracts, the command work and the job factories.'),
    ch('📬', 'store-preparation', 'store_preparation', [(529, 676)], 'Store preparation of the WFC bitmap editor: the one-item publication factory and its preparation.'),
    ch('📝', 'artifact-editor', 'artifact_editor', [(680, 1061)], 'Artifact editor of the WFC bitmap editor: the editor type, its `ArtifactEditor` implementation and its helpers.'),
 ]),
 'wfc3d': dict(external=[WFC + '🧊️3d' + SUB + '/✏️editor/🧪️tests/🔬️unit/🦀️.rs'], children=[
    ch('🎛', 'command-vocabulary', 'command_vocabulary', [(62, 179)], 'Command vocabulary of the WFC 3D editor: the command enum, its wire ids and the tool and example constants.'),
    ch('🧵', 'retained-commands', 'retained_commands', [(183, 391)], 'Retained command route of the WFC 3D editor: the bounded publication contracts, the command work and the job factories.'),
    ch('🔗', 'graph-view-and-editor', 'graph_view_and_editor', [(395, 800)], 'Graph view and artifact editor of the WFC 3D editor: the slot graph view, the editor type with its `ArtifactEditor` implementation, command emission and the body renderer.'),
    ch('🧰', 'examples-and-graph-edit', 'examples_and_graph_edit', [(802, 941)], 'Examples and graph editing of the WFC 3D editor: the example snapshots and the wire and slot graph edit gestures.'),
 ]),
 'wfchost': dict(external=[WFC + '🧊️3d/🔨️modules/🏠️host/💡️inferences/🧪️tests/🔬️unit/🦀️.rs'], children=[
    ch('📜', 'contract-and-request', 'contract_and_request', [(8, 164)], 'Contract and request of the WFC 3D inference: the published ids and schemas, the request, the stage vocabulary and the job state.'),
    ch('🧮', 'job-core', 'job_core', [(166, 550)], 'Job core of the WFC 3D inference: construction, the solve steps and the run, lent and child-end vocabulary.'),
    ch('🔁', 'job-lifecycle', 'job_lifecycle', [(552, 862)], 'Job lifecycle of the WFC 3D inference: the checkpoint, progress and publication members and the `InteractiveJob` implementation.'),
    ch('🏭', 'factory-and-solve', 'factory_and_solve', [(864, 1029)], 'Factory and inferred fields of the WFC 3D inference: the job factory, the descriptor, the solve entry points and the inferred solve, contradiction and entropy fields.'),
 ]),
 'gen2d': dict(keep_empty=['🧪️UnitTests'], patch_parent=[['use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};\n#![allow(clippy::result_large_err)]\n', '#![allow(clippy::result_large_err)]\nuse semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};\n']], external=[
        PRO + '🌀️generation2d' + SUB + '/✏️editor/🧪️tests/🔬️generation2d-config-preparation-laws/🦀️.rs',
        PRO + '🌀️generation2d' + SUB + '/✏️editor/📚️examples/🎬️demo-session/🦀️.rs',
        PRO + '🌀️generation2d' + SUB + '/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs',
        PRO + '🌀️generation2d' + SUB + '/✏️editor/🌉️wasm/🦀️.rs',
        PRO + '🌀️generation2d' + SUB + '/✏️editor/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs',
    ], children=[
    ch('🛰', 'owner-and-io', 'owner_and_io', [(127, 301)], 'Instance operation owner and media I/O of the generation2d editor: the flow session close, the command categorisation, the owner with its retirement and maintenance steps and the typed media ports.'),
    ch('🧵', 'retained-commands', 'retained_commands', [(341, 763)], 'Retained command route of the generation2d editor: the bounded, preview and flow-eval tool ids, the three command works and the bounded job factory.'),
    ch('🎁', 'contributions-route', 'contributions_route', [(768, 957)], 'Contributions route of the generation2d editor: the contributions contract, its work and job factory and the factory proofs.'),
    ch('📬', 'store-preparation', 'store_preparation', [(962, 1293)], 'Store preparation of the generation2d editor: the artifact and config one-item publication factories with their admission and preparation.'),
    ch('🎞', 'reserved-import', 'reserved_import', [(1298, 1606)], 'Reserved import job of the generation2d editor: the media import job with its `InteractiveJob` implementation.'),
    ch('🕹', 'play-app-editor', 'play_app_editor', [(1610, 2137)], 'Play app of the generation2d editor: the play app type and its `ArtifactEditor` implementation.'),
    ch('🖱', 'graph-interaction-and-render', 'graph_interaction_and_render', [(2345, 2432)], 'Graph interaction and window bodies of the generation2d editor: the resolved interaction marks and the single body-key render.'),
 ]),
 'value': dict(external=[PRO + '🧊️generation3d' + SUB + '/🚪️io/📝️text/🔣️value/🧪️tests/🦀️.rs'], children=[
    ch('🧊', 'snapshot-and-catalogue-values', 'snapshot_and_catalogue_values', [(15, 573)], 'Snapshot and catalogue values of generation3d: the artifact, preview camera, localized text, the widget catalogue records, the snapshot, the diff patches and the inference, topology and geometry records.'),
    ch('🧬', 'mutation-delta-and-canonical-values', 'mutation_delta_and_canonical_values', [(577, 1278)], 'Mutation, delta and canonical values of generation3d: the mutation enum with its payload records, the widget, synapse and generation deltas, the typed widget input literals and the canonical JSON trees.'),
 ]),
 'preview': dict(external=[PRO + '🧊️generation3d' + SUB + '/🧵️preview-eval/' + x for x in ('⏯️tool-run/🦀️.rs', '📥️tessellation/🦀️.rs', '🧪️tests/🔬️unit/🦀️.rs')], children=[
    ch('🪟', 'window-addressing', 'window_addressing', [(52, 306)], 'Window addressing of the preview evaluation: the flow-eval hop payloads, the hop effects and the owing of attached preview windows.'),
    ch('🧊', 'preview-geometry', 'preview_geometry', [(310, 882)], 'Preview geometry of the preview evaluation: the channel items, the mesh roles, the fit, the marker meshes and the retained mesh handles.'),
    ch('⏱', 'tick-and-resolve', 'tick_and_resolve', [(886, 1141)], 'Tick and resolve of the preview evaluation: the evaluation tick, the invocation resolution and the release hops.'),
    ch('📈', 'status-json', 'status_json', [(1146, 1444)], 'Status of the preview evaluation: the status, scene and progress JSON.'),
 ]),
}

FOLDERS = {
    'poly': ['📏️univariate', '🕸️multivariate', '🧮️finite', '🔍️factor', '🌱️roots', '🔢️algebraic'],
    'gen2dunit': ['🧰️production-fixtures-and-maintenance-law', '🎛️command-surface-laws', '📜️manifest-and-cross-cutting-laws', '📏️surface-budget-and-context-menu-laws', '🔌️port-laws', '📇️window-action-archive-and-genesis-laws'],
}

if __name__ == '__main__':
    for k, fl in FOLDERS.items():
        sp = dict(orig='%s.rs.txt' % k, file=FILES[k], nest=False, children=[dict(folder=f) for f in fl], external=[])
        with open(os.path.join(T, 'specs', k + '.py'), 'w', encoding='utf-8') as fh:
            fh.write('SPEC = %s\n' % json.dumps(sp, ensure_ascii=False, indent=1).replace('true', 'True').replace('false', 'False'))
        print('wrote', k)
    for k, v in TABLE.items():
        sp = dict(orig='%s.rs.txt' % k, file=FILES[k], nest=False, children=v['children'], external=v['external'])
        if v.get('keep_empty'):
            sp['keep_empty'] = v['keep_empty']
        if v.get('patch_parent'):
            sp['patch_parent'] = v['patch_parent']
        with open(os.path.join(T, 'specs', k + '.py'), 'w', encoding='utf-8') as fh:
            fh.write('SPEC = %s\n' % json.dumps(sp, ensure_ascii=False, indent=1).replace('true', 'True').replace('false', 'False'))
        print('wrote', k)
