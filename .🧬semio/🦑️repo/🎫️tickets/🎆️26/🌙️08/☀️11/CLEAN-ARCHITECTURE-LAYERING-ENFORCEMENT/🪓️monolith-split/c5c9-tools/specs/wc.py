from specs.lib import *
WC = OD + '/🔌️plugin/🪟️window/🎚️config'
CH = [
    ch('📐', 'owner-contract-and-mutation', 'owner_contract_and_mutation', [(37, 165)],
       'Owner contract and mutation of the window config: the window config owner trait and its bounded factories, the mutation, the rejected emission and the snapshot.'),
    ch('🔏', 'pack-authority-and-publication', 'pack_authority_and_publication', [(167, 300)],
       'Pack, authority and publication of the window config: the pack, the authority, the erased publication trait and the typed publication with its drop.'),
    ch('🗄', 'store-owners', 'store_owners', [(302, 654)],
       'Store owners of the window config: the store and preview owner types, the partition, the erased store owner trait, the direct ingress and the typed store owner.'),
    ch('🧭', 'owner-registry', 'owner_registry', [(656, 1013)],
       'Owner registry of the window config: the pack load turns, the owner registry, the controlled owner and window actor demands and the registry operations.'),
]
SPEC = dict(orig='wc.rs', file=WC + '/🦀️.rs', children=CH, external=mounted(WC + '/🦀️.rs', orig='wc.rs'))
