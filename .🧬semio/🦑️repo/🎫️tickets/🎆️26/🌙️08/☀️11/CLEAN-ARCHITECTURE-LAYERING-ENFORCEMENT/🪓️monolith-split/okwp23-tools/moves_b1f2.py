import ns
S = ns.S
MOVES = [
    dict(src=S + '/🦀️.rs', match=r'ArtifactStoreBatchItemAuthority<P, Mutation> for Arc<dyn MemberStoreOneItemWirePreparationFactory<P, Mutation>>', dest='os-store-one-item-publication', count=1,
         fixups={'semio_framework_os_store_one_item_batch::lifted_one_item_publication': 'crate::lifted_one_item_owners'}),
]
