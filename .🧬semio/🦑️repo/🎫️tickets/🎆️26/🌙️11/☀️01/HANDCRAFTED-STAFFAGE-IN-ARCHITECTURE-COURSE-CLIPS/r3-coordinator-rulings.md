# Round 3 coordinator rulings

These rulings take precedence over `r2-coordinator-rulings.md` wherever the two conflict.

## Superseded r2 rulings

- **OI-04:** reversed. The demand practice clip's building hot-water and buffer store is a depicted building component. It uses the canonical `hot-water-cylinder` asset in the thermal view, and the store's charge state comes from the scenario. `energy_tank` stays a glyph only as the abstract source gauge ("Quelle"). Owner: W10.
- **OI-41:** stands. A thermometer drawn as a measuring annotation is a teaching instrument and stays a glyph, provided its reading equals the scene state. When a thermometer is the depicted subject, it becomes a cast. Owner: W14 and W15, per clip.

## Accepted and closed

These items are accepted as they are and closed:
- OI-21, OI-27, OI-54, OI-93, OI-117, OI-120, OI-122, OI-129
- OI-47: closed only once W08 delivers the seated and lying plan postures and W16 uses them in the Excel plan.

## Rulings R1–R4: answers to the inventory questions (r3-inv-report.md §5)

- **R1: building contexts.**
  - House and room shells, cut wall/slab layers and roof outlines are scene architecture or section diagrams that carry the lesson (layers, U-values). They stay scene-drawn.
  - Pipe runs are diagram connectors.
  - Any depicted building component that has a catalog asset must be a cast. This covers windows (tilt-window), radiators, blinds, doors, the store, pumps and valves.
  - `window_glyph` is in scope: W11 checks every call site and replaces it with a window cast unless it is a pure section mark.
- **R2: Nakagin furniture** moves into the catalog and is placed through the library (W11, OI-46).
- **R3: mixed scenes.**
  - In a thermal-mode scene, every cast uses the thermal view. Passive furniture (desk, table, chair, blackboard, crates) is shown at ambient or surface temperature.
  - Mixing dark-line casts into a thermal scene is not allowed. The only exception is a deliberate, legended transition between modes.
  - The tree in primary-energy Beat2 is a teaching icon for "renewable or biomass", not an object in the room. It becomes a canonical vegetation cast in the line view, inside a non-thermal panel with a legend (owners W09 and W15).
- **R4: the volume prop set** stays as it is. Its teaching intent is room uses with different air-change needs. Each prop must be thermal (R3).
- **Batches:** the dispatched batch split stays. Demand belongs to W15, not W16, despite the W01 proposal.

## Assigned

| OI | owner |
|---|---|
| OI-11 | W17. The 200 KiB budget applies per frame. W17 also states the per-cast total in the media plan. |
| OI-34 | W10 |
| OI-134 | W10 |
| OI-133 | W10 |
| OI-132 | W14 (heating conduction) |
| OI-131 | W17 |
| OI-91 | W18. One final full library vitest run. |
| OI-106 | Git state. Do not touch it. |
| OI-126 | Git state. Do not touch it. |
| OI-135 | W18 checks that no render output was lost. |
| OI-130 | Out of scope. |
