V = '️'
F = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🦀️.rs'
def ch(emoji, kebab, mod, ranges, doc, **kw):
    return dict(folder=emoji + V + kebab, mod=mod, ranges=ranges, doc='//! %s%s %s See [`super`].' % (emoji, V, doc), **kw)
SPEC = dict(orig='decl-view.rs', file=F, children=[
    ch('🔭', 'artifact-view', 'artifact_view', [(73, 104), (214, 355)], 'The read-only view of an app document, the pending view of a poll and the materialized snapshot, history and undo reads of the view.'),
    ch('🪪', 'operation-contexts', 'operation_contexts', [(106, 204)], 'The durable authority captured at command admission, the fixed render authority of a retained background job, the mounted snapshot read preparation and the publication revisions.'),
    ch('🔍', 'lane-views', 'lane_views', [(357, 504)], 'The read views of the config, draft, presence and interaction lanes.'),
    ch('🕳', 'empty-lanes', 'empty_lanes', [(506, 905)], 'The uninhabited lanes for apps without config, draft, presence or transient state: their empty diffs, snapshots, packs, mutations and operation codecs.'),
    ch('🫂', 'peer-presence', 'peer_presence', [(907, 1006)], 'The ephemeral interaction hover map, the last-adopted presence slice of each peer, its bounded root, the mark a peer leaves on a target and the cancellation action.'),
])
