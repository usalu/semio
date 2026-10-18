from specs.lib import *
CH = [
    ch('📋', 'ui-values-and-context-menus', 'ui_values_and_context_menus', [(141, 337)],
       'UI values and context menus of the layout play app: the boolean, number and list UI value builders, the layout element ids, the preflight issue lookup, the frame kinds, the authoring surface and the context menu items with their internal action.'),
    ch('🧵', 'retained-command-factories', 'retained_command_factories', [(409, 665)],
       'Retained command factories of the layout play app: the retained tool ids and contract, the window work record, the retained command job factory with its owned tool job factory implementation and the retained and export proof catalogues.'),
    ch('🎞', 'reserved-import-job', 'reserved_import_job', [(670, 968)],
       'Reserved import job of the layout play app: the import tool id and port, the import job with its interactive job and reserved job implementations and the export tool job builder.'),
    ch('🪧', 'window-engagement-and-labels', 'window_engagement_and_labels', [(971, 1043)],
       'Window engagement and labels of the layout play app: the window engagement record and the entity label with its excerpt helper.'),
    ch('🕹', 'layout-play-app', 'layout_play_app', [(1050, 1385)],
       'Layout play app: the editor app record, its default and constructor and the `ArtifactEditor` implementation with the dispatch, render, media and history hooks.'),
    ch('🧱', 'create-layout-app', 'layout_app_manifest', [(1389, 1560)],
       'Create layout app: the manifest stitch that builds the layout app definition from the per node definitions.'),
    ch('📤', 'export-tool-job', 'export_tool_job', [(1582, 1779)],
       'Export tool job of the layout play app: the export tool payload, the export tool job with its step, close and interactive job implementations and the export job factory.'),
    ch('🎥', 'media-export-job-and-factories', 'media_export_job_and_factories', [(1781, 1999)],
       'Media export job and factories of the layout play app: the media export job with its interactive job and reserved job implementations, the media export job factory and the owned tool job factory implementations of both export factories.'),
]
SPEC = dict(orig='ed.rs', file=LY + '/🦀️.rs', children=CH, keep_empty=('🔖️Interaction', '🪢️TaxonomyMounts'), external=mounted(LY + '/🦀️.rs', orig='ed.rs'))
