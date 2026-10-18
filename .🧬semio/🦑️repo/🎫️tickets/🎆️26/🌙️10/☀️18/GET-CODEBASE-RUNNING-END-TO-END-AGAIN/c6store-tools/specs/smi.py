from specs.lib import *
F = STORE + '/🧩️composition/👥️space-member/📥️lifted/🏪️store/🦀️.rs'
CH = [
    ch('🔗', 'member-link-resolution-and-blob-store', 'member_link_resolution_and_blob_store', [(64, 220)],
       'Member link resolution and blob store: the content id, the artifact refs, the link state and resolver, the member directory, the no blob store, the member link resolver and the blob store.'),
    ch('👥', 'space-member-trait', 'space_member_trait', [(223, 431)],
       'Space member trait: the space member contract of a document store inside a space, its identity, publication, checkpoint, history and wire operations.'),
    ch('🏪', 'space-member-for-artifact-store', 'space_member_for_artifact_store', [(433, 910)],
       'Space member for artifact store: the space member implementation of the artifact store over its one item publication, snapshot reads, checkpoints, undo, redo, wire dispatch and pack operations.'),
    ch('🏭', 'member-factory-visitors-and-no-members', 'member_factory_visitors_and_no_members', [(912, 1195)],
       'Member factory, visitors and no members: the member factory, the member store visitors, the member visit and the empty member set with its space member, factory and visit.'),
]
SPEC = dict(orig='smi.rs', file=F, children=CH, wiring='block', rebase_self=True, external=crate_ext(STORE + '/🧩️composition/👥️space-member/🦀️.rs', F, orig='smi.rs', children=CH))
