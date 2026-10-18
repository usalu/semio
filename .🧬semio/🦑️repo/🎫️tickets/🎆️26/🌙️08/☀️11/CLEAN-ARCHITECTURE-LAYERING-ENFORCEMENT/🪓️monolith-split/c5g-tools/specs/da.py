from specs.lib import *
DA = PL + '/📜️declaration/🧩️app'
CH = [
    ch('🎛', 'artifact-app-trait', 'artifact_app_trait', [(144, 871)],
       'Artifact app trait: the typed per-app author surface `ArtifactApp` with its snapshot and mutation types and every defaulted hook.'),
    ch('🗂', 'natural-file-and-media-artifacts', 'natural_file_and_media_artifacts', [(873, 1112)],
       'Natural file and media artifacts: the natural file codec, decode steps and cursors, the mounted natural file actions, the media artifact descriptor, errors and operation failures, the document load fault codes and the media fault helpers.'),
    ch('🛡', 'viewer-actions-and-transaction-outcomes', 'viewer_actions_and_transaction_outcomes', [(1115, 1167)],
       'Viewer actions and transaction outcomes: the transaction prepare outcome, the ephemeral snapshot and the reserved viewer and time-travel action ids.'),
]
SPEC = dict(orig='da.rs', file=DA + '/🦀️.rs', children=CH, wiring='block', external=mounted(DA + '/🦀️.rs', orig='da.rs'))
