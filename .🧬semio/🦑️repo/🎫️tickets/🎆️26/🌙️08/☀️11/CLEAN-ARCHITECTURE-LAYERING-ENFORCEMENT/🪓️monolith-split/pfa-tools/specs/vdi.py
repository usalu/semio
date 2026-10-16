F = '✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs'
V = '✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/'
SPEC = dict(
    orig='vdi.rs.txt',
    file=F,
    children=[
        dict(folder='🧰️helpers', mod='helpers', ranges=[(80, 389), (570, 589)],
             doc='//! 🧰️ VDI 3805 compliance helpers: part labels, localized copy, subject paths, edition profile lookup, per-sheet attribute extraction and the remedies shared by the sheet checks.'),
        dict(folder='🧱️sheet-checks', mod='sheet_checks', ranges=[(391, 568), (591, 1486)],
             doc='//! 🧱️ VDI 3805 sheet checks: attribute and record synchronisation, the part 1 structure check and the per-sheet product checks for valves, radiators, pumps and heat generators.'),
        dict(folder='📚️document-checks', mod='document_checks', ranges=[(1488, 2751)],
             doc='//! 📚️ VDI 3805 document checks: catalog integrity, historical sheets, edition profiles, operative sheet coverage and `evaluate`, which assembles the compliance report.'),
        dict(folder='📏️parts', mod='parts', ranges=[(2753, 3020)],
             doc='//! 📏️ VDI 3805 sheet parts: the `define_vdi_part!` macro, part 1 and the generated part modules 2 to 100.'),
    ],
    external=[
        V + '🧪️tests/🔬️unit/🦀️.rs',
        V + '🧪️tests/🔬️compliance-report/🦀️.rs',
        V + '🧾outline/🦀️.rs',
        V + '🧾outline/🧪️tests/🔬️unit/🦀️.rs',
    ],
)
