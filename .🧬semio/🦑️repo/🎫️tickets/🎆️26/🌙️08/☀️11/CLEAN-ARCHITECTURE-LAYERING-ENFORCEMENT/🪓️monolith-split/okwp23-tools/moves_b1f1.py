import ns
S = ns.S
MOVES = [
    dict(src=S + '/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs', match=r'(pub struct MemberStoreOneItemWire\b|pub struct MemberStoreOneItemWireRequest\b|pub trait MemberStoreOneItemWirePreparationFactory\b)', dest='os-store-one-item-publication', count=3, reexport=True),
]
