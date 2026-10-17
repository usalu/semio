from . import _ext
F = '✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔨️modules/🏠️host/🦀️.rs'
SPEC = dict(
    orig='jack-host.rs.txt',
    file=F,
    wiring='each',
    children=[
        dict(folder='📸️snapshot-decode', mod='snapshot_decode',
             ranges=[(15, 189)],
             doc='//! 📸️ Jack snapshot decode: the owned field budget helpers and the retained snapshot decode authority.'),
        dict(folder='🔄️mutation-decode', mod='mutation_decode',
             ranges=[(191, 421)],
             doc='//! 🔄️ Jack mutation decode: the retained mutation decode authority, the rejected conflict authority and the owned field catalog.'),
        dict(folder='📑️snapshot-clone', mod='snapshot_clone',
             ranges=[(425, 751)],
             doc='//! 📑️ Jack snapshot clone: the retained snapshot clone authority and its step machine.'),
        dict(folder='🏗️initialization-authority', mod='initialization_authority',
             ranges=[(753, 1110)],
             doc='//! 🏗️ Jack store initialization authority: the phase machine and the authority that rebuilds the document store.'),
        dict(folder='📜️initialization-contract', mod='initialization_contract',
             ranges=[(1112, 1441)],
             doc='//! 📜️ Jack store initialization contract: the `ArtifactStoreInitializationAuthority` implementation and the document store initialization job.'),
    ],
    external=_ext.mounted(F),
)
