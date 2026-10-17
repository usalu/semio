from specs.lib import *

CH = [
    ch('🔌', 'live-load-envelope', 'live_load_envelope', [(130, 234)],
       'Jack envelope wires, admission and the live load pump of the editor laws.'),
    ch('🖼', 'scene-rendering-and-text-edit', 'scene_rendering_and_text_edit', [(236, 479)],
       'Scene helpers, rendering, query runs, selection and text edits of the editor laws.'),
    ch('🔭', 'lod-catalogue-and-examples', 'lod_catalogue_and_examples', [(481, 703)],
       'Level of detail, the catalogue, the inspection panel, example loading, deletion, export and ports of the editor laws.', reexport=False),
    ch('🔎', 'query-ownership-and-examples', 'query_ownership_and_examples', [(705, 1003)],
       'Query ownership, the shared document query, window kind actions and the shipped queries of the editor laws.'),
    ch('🩹', 'rail-verb-laws', 'rail_verb_laws', [(1005, 1211)],
       'The rail verbs of the editor laws: patch nodes, undo and redo, select all and the text gesture verbs.'),
    ch('🤖', 'agent-lane-and-archive-laws', 'agent_lane_and_archive_laws', [(1213, 1283)],
       'The agent lane, the composed history laws and the foreign carrier archive of the editor laws.', reexport=False),
]

SPEC = dict(
    orig='tri.rs',
    file=TRI + '/🦀️.rs',
    children=CH,
    external=[],
    rebase_super=True,
)
