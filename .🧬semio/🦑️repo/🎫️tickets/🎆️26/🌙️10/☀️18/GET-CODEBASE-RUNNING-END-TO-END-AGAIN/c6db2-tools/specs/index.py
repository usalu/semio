from specs.lib import *
F = DB + '/🔢️index/🦀️.rs'
CH = [
    ch('📏', 'index-limits-kind-and-run-ids', 'index_limits_kind_and_run_ids', [(28, 173)],
       'Index limits, kind and run ids: the key, value and run limits, the index kind and the run id layout with its capacity and field accessors.'),
    ch('🧭', 'index-cursor-control-and-bytes', 'index_cursor_control_and_bytes', [(177, 461)],
       'Index cursor control and bytes: the cursor control, the retained index bytes with their rejection, the admission and the byte comparisons.'),
    ch('📼', 'run-entries-and-codec', 'run_entries_and_codec', [(463, 887)],
       'Run entries and codec: the run value, entry and entries, the run header and page reader, the varints and the run page encoders and decoders.'),
    ch('🔭', 'run-view-and-merge', 'run_view_and_merge', [(891, 1254)],
       'Run view and merge: the run range, view and picks, the run page views, the merge of run views and entries, the merge policy and the index stats.'),
    ch('🗂', 'index-handle', 'index_handle', [(1257, 1829)],
       'Index handle: the range step, the index handle over a storage with its run operations and the owned runs.'),
    ch('🧷', 'location-command-and-inverse-indexes', 'location_command_and_inverse_indexes', [(1833, 2036)],
       'Location, command and inverse indexes: the record location, the sequence location index, the command index and the inverse index.'),
    ch('👤', 'actor-frontier-touched-and-commit-indexes', 'actor_frontier_touched_and_commit_indexes', [(2040, 2368)],
       'Actor, frontier, touched region and commit indexes: the actor sequence index, the frontier index, the touched region index and the commit index.'),
    ch('🔎', 'text-blob-conflict-projection-and-preview-indexes', 'text_blob_conflict_projection_and_preview_indexes', [(2372, 2752)],
       'Full text, blob list, conflict, projection and preview indexes: the full text index, the blob list, the conflict index, the projection index and the preview index.'),
]
SPEC = dict(orig='index.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='index.rs'))
