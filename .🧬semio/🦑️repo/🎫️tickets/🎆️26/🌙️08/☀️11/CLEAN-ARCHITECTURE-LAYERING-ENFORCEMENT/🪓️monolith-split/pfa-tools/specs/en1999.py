F = '✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs'
SPEC = dict(
    orig='en1999.rs.txt',
    file=F,
    children=[
        dict(folder='📏️parts', mod='parts', glob=False, ranges=[(71, 790)],
             doc='//! 📏️ EN 1999 clause parts: the German national annex parameters, the EN 1990 combination factors and the clause tables of EN 1999-1-1, 1-2 and 1-3.'),
        dict(folder='🧱️member-checks', mod='member_checks', ranges=[(792, 1595)],
             doc='//! 🧱️ EN 1999 member checks: the digests and the utilization helper behind `check_member`, which verifies one aluminium member against every clause.'),
        dict(folder='🔗️connection-checks', mod='connection_checks', ranges=[(1597, 1946)],
             doc='//! 🔗️ EN 1999 connection checks: `check_connection` verifies one aluminium connection against every clause.'),
        dict(folder='🔥️fire-fatigue-checks', mod='fire_fatigue_checks', ranges=[(1948, 2143)],
             doc='//! 🔥️ EN 1999 fire and fatigue checks: `check_fire` (EN 1999-1-2) and `check_fatigue` (EN 1999-1-3).'),
        dict(folder='🥫️thin-wall-checks', mod='thin_wall_checks', ranges=[(2145, 2589)],
             doc='//! 🥫️ EN 1999 thin-walled checks: `check_cold_formed` for cold-formed sheets and `check_shell` for shells.'),
        dict(folder='🧾️structure-evaluation', mod='structure_evaluation', ranges=[(2591, 2970)],
             doc='//! 🧾️ EN 1999 structure evaluation: the duplicate-id check and `evaluate_structure`, which runs every clause check over a whole snapshot.'),
    ],
    external=[],
)
