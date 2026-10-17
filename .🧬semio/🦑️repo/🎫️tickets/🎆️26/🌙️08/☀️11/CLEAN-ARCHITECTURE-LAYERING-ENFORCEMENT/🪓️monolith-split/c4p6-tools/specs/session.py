F = '✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️.rs'
SPEC = dict(
    orig='session.rs.txt',
    file=F,
    nest=True,
    children=[
        dict(folder='📋️clipboard', mod='clipboard', ranges=[(66, 440)],
             doc='//! 📋️ Clipboard jobs of the puzzle 5d editor session: the selection scan and the reserved copy and cut jobs. See [`super`].'),
        dict(folder='📌️paste', mod='paste', ranges=[(442, 760)],
             doc='//! 📌️ Paste job of the puzzle 5d editor session: the reserved paste job with its fixed-page state machine. See [`super`].'),
        dict(folder='📥️snapshot-import', mod='snapshot_import', ranges=[(761, 1454)],
             doc='//! 📥️ Import job of the puzzle 5d editor session: the reserved snapshot import job with its fixed-page state machine. See [`super`].'),
    ],
    external=[],
)
