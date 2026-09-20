# EnergyBalance Manim academic review

Goal: `🎯r2602🎯updateddocs🎯updateduserdocs🎯updatedtutorials`

Repo MCP was unavailable (`repo://goals`, `ticket_open`). Work logged here.

Read all EnergyBalance chapter scenes plus `full_energy_balance_video.py`, `README.md`, intro.

Applied (2026-09-20):

- Q_h named Heizwärmebedarf / Nutzenergie, not Verbrauch or a generic Nutzwärmebedarf.
- Envelope crossings in the annual balance are energy, not instantaneous Wärmestrom.
- ΔU_WB = 0,05 is the pauschal with Wärmebrückennachweis, not “typical” for all buildings.
- Solar term uses irradiation (kWh/m²), not irradiance.
- η_h is gain/loss-dependent, not a fixed 0,8.
- Unused summer gains are unused in the heating balance, not “lost”.
- f_p = 1,8 for grid electricity is GEG Anlage 4 (verified 2026); not the largest factor in the table.
- Closing chain is the gas path; the heat pump is the other letter.

Must-fix:

1. Q_h called consumption / Nutzwärmebedarf.
2. Heat flow vs annual energy on the envelope.
3. η_h ≈ 0,8 as a constant; solar “irradiance”.
4. Summer surplus “verfällt”.
5. Netzstrom “größter Faktor” (KWK Verdrängungsstrommix is 2,8).

Ticket close: repo MCP `ticket_close` unavailable. Sources patched 2026-09-20. Worked-example numbers unchanged (f_p,Strom = 1,8 still GEG Anlage 4). VO not re-synthed.
