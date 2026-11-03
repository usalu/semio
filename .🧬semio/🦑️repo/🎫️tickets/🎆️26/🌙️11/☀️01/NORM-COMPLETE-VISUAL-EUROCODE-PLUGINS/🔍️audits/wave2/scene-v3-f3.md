# scene v3 F3 (en1994 slip arrows)
- New `🎬️scene/🧷️slip-arrows/🦀️.rs`: up to 12 arrows per composite beam at stud stations, on the interface (z = 0), direction = sign of the slip (slab slips toward the support halves' ends, antisymmetric), length = |δ|/6 mm * 0.8 stud pitches (min 0.1 pitch), utilisation binding ratio = |δ|/δ_uk (status via `scaled`), layer `slip` (EN/DE), EN/DE names and details.
- `📈️diagrams`: `slip_inputs(f) -> (peak kN/m, P_Rk kN)` and `SLIP_CAPACITY_MM` extracted from `slip_distribution`, which now uses them (same values; the 2D diagram with table twin already existed).
- Hooks: `mod slip_arrows` + `slip_arrows::project` in scene `🦀️.rs`; `LAYER_SLIP` and the 13th layer row in `🧰️kit`; legend title now mentions slip.
- Test `slip_arrows_follow_the_slip_distribution_and_the_capacity` (arrow length rule 6 mm -> 0.8 pitch, 3 mm -> 0.4, 0 -> 0.1; antisymmetric signs; values equal `slip_at`; ratio = |δ|/6).
- Unverified (nothing compiled): `binding` signature use, sign assumption of the default document (first > 0, last < 0), golden re-bless needed. Placements law test left to parent.
