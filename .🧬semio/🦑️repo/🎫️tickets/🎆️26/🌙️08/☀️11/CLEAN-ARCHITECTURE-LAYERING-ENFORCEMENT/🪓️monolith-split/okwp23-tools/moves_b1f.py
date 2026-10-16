# the member one-item wire family moves down into publication so that the BatchItemAuthority impl for its factory trait object has a local trait
import ns
S = ns.S
MOVES = [
    dict(src=S + '/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs', match=r'(pub struct MemberStoreOneItemWire\b|pub struct MemberStoreOneItemWireRequest\b|pub trait MemberStoreOneItemWirePreparationFactory\b)', dest='os-store-one-item-publication', count=3, reexport=True),
    dict(src=S + '/🦀️.rs', match=r'ArtifactStoreBatchItemAuthority<P, Mutation> for Arc<dyn MemberStoreOneItemWirePreparationFactory<P, Mutation>>', dest='os-store-one-item-publication', count=1),
]
