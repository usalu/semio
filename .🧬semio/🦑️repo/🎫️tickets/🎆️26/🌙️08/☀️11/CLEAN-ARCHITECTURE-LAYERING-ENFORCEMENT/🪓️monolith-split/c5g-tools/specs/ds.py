from specs.lib import *
DS = PL + '/📜️declaration/🪟️surface'
CH = [
    ch('✏', 'artifact-editor-trait', 'artifact_editor_trait', [(163, 664)],
       'Artifact editor trait: the editable surface contract `ArtifactEditor` with its snapshot, mutation, view and history hooks.'),
    ch('🧬', 'artifact-codec-table-and-replay', 'artifact_codec_table_and_replay', [(666, 800)],
       'Artifact codec table and replay: the server minted document id check, the genesis pair, the pair snapshot, op application, envelope replay and the version one codec table.'),
    ch('👁', 'artifact-viewer-trait', 'artifact_viewer_trait', [(802, 1180)],
       'Artifact viewer trait: the read-only surface contract `ArtifactViewer` with its snapshot, view and viewer hooks.'),
    ch('🖊', 'editor-app', 'editor_app', [(1182, 1542)],
       'Editor app: the app wrapper that turns an `ArtifactEditor` into an `ArtifactApp`.'),
    ch('📖', 'viewer-app', 'viewer_app', [(1544, 1842)],
       'Viewer app: the app wrapper that turns an `ArtifactViewer` into an `ArtifactApp`.'),
    ch('🏗', 'surface-builders', 'surface_builders', [(1844, 2093)],
       'Surface builders: the viewer and editor wrappers, their builders with the forwarded builder methods, the snapshot builder and the subset example tables.'),
]
SPEC = dict(orig='ds.rs', file=DS + '/🦀️.rs', children=CH, wiring='block', external=mounted(DS + '/🦀️.rs', orig='ds.rs'))
