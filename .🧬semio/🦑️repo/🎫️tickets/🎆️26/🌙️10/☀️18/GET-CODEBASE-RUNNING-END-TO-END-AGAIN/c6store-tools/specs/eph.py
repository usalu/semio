from specs.lib import *
F = STORE + '/🫧️ephemeral/🦀️.rs'
CH = [
    ch('📤', 'ephemeral-one-item-publication', 'ephemeral_one_item_publication', [(16, 264)],
       'Ephemeral one item publication: the prepared publication, the base read and owner, the preparation request, trait and factory, the admission rejection and the ephemeral one item publication.'),
    ch('👥', 'presence-store-and-peers-publication', 'presence_store_and_peers_publication', [(267, 738)],
       'Presence store and peers publication: the presence store, the peers publication and commit, the presence retirement default and the presence store operations.'),
    ch('🌫', 'transient-and-interaction-stores', 'transient_and_interaction_stores', [(740, 1235)],
       'Transient and interaction stores: the transient store with its retirement and the interaction store with its default and operations.'),
]
SPEC = dict(orig='eph.rs', file=F, children=CH, wiring='block', rebase_self=True, external=mounted(F, orig='eph.rs'))
