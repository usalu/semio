# 🌲️ Stacked concrete forest — model derivation

## Sources read
- CAD play fixture `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🎮️play/🔣️.json`, model 3 `aec.building.structure.classic` (the CAD "structural representation" pane, `CadPaneId::StructureClassic`), objects 1–11:

| # | typology | wire / face |
|---|---|---|
| 9 | reinforcedconcretecolumn | (8.1, 2.3383, 0) → (8.1, 2.3383, 3) |
| 10 | reinforcedconcretecolumn | (2.7, 2.3383, 0) → (2.7, 2.3383, 3) |
| 8 | reinforcedconcreteinternalwall (spine) | (8.1, 2.3383, 3) → (2.7, 2.3383, 3) |
| 1 | cantilever | (8.1, 2.3383, 3) → (6.75, 4.6765, 3) |
| 2 | cantilever | (8.1, 2.3383, 3) → (9.45, 2.3383, 3) |
| 3 | cantilever | (2.7, 2.3383, 3) → (1.35, 0, 3) |
| 4 | cantilever | (2.7, 2.3383, 3) → (4.05, 0, 3) |
| 5 | cantilever | (8.1, 2.3383, 3) → (6.75, 0, 3) |
| 6 | cantilever | (2.7, 2.3383, 3) → (4.05, 4.6765, 3) |
| 7 | cantilever | (8.1, 2.3383, 3) → (9.45, 4.6765, 3) |
| 11 | onewayreinforcedconcreteslab | parallelogram (8.1,0) (10.8,4.6765) (2.7,4.6765) (0,0) at z 3 |

- Same fixture, model 1 `aec.building` (solid members) for section sizes: beams 0.30 wide × 0.45 deep (z 2.285–2.735), columns hexagonal, circumradius 0.30 m (x half-width 0.26 = apothem), slab 0.265 thick (z 2.735–3.0).
- puzzle3d `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio`, object `seed-left-001` vortices — the connection points:

| vortex | kind | point | direction |
|---|---|---|---|
| v0 | b-l | (4.05001, 4.676537, 3) | +y |
| v1 | b-l-m | (6.75001, 4.676537, 3) | +y |
| v2 | b-l | (9.45001, 4.676537, 3) | +y |
| v3 | b-s-m | (6.75001, 0, 3) | −y |
| v4 | b-s | (4.05001, 0, 3) | −y |
| v5 | b-s-m | (1.35001, 0, 3) | −y |
| v6 | b-s | (9.45001, 2.338269, 3) | (0.866, −0.5, 0) |
| v7 | c-b | (2.70001, 2.338269, 0) | −z |
| v8 | c-t | (2.70001, 2.338269, 3) | +z |
| v9 | c-b | (8.10001, 2.338269, 0) | −z |
| v10 | c-t | (8.10001, 2.338269, 3) | +z |

The seven beam vortices v0–v6 are exactly the seven CAD cantilever end points; v7–v10 are the column base/top points. Kind compatibility `c-b ↔ c-t` is the stacking joint: the upper piece is the lower piece translated by (0, 0, 3) so its `c-b` vortices coincide with the lower `c-t` vortices.

## FEM idealisation (nodes + frames only)
- Nodes: puzzle vortex coordinates verbatim (`lc1b/lc2b` = v7/v9, `lc1t/lc2t` = v8/v10, `lv0..lv6` = v0..v6; upper storey `uc1t/uc2t/uv0..uv6` lifted by +3). 20 nodes; the two column heads of the lower piece are shared with the upper columns (the c-b/c-t joint).
- Elements: 20 `frame`s — per piece 2 columns (`hex30`), 1 spine (`beam30x45`, member 8), 7 cantilevers (`beam30x45`, members 1–7 keyed by vortex index: b0 ← member 6, b1 ← 1, b2 ← 7, b3 ← 5, b4 ← 4, b5 ← 3, b6 ← 2).
- Sections: `hex30` A = 3√3/2·R² = 0.233827 m², I = 5√3/16·R⁴ = 0.00438425 m⁴ (both axes), J ≈ A⁴/(40·Ip) = 0.008523 m⁴. `beam30x45` A = 0.135, Iy = bh³/12 = 0.002278125 (strong, vertical bending: the engine's local z is up for horizontal members), Iz = hb³/12 = 0.0010125, J ≈ 0.196·b³h = 0.0023814.
- Material: C30/37, E 33 GPa, G 13.75 GPa, ν 0.2, ρ 2400.
- Slab: not meshed (line elements only); carried as tributary member UDLs. Slab area = 8.1 × 4.676537 = 37.88 m². Dead: 0.265·2400·9.81 = 6.239 kN/m² → 236.3 kN per storey over the 22.95 m of beams (6 × 2.7 + 1.35 + 5.4) = 10 298 N/m. Live 3 kN/m² → 4 952 N/m. Self weight on.
- Supports: both lower column bases fully fixed. Combinations ULS 1.35 G + 1.5 Q, SLS 1.0 G + 1.0 Q.

## Deformation (SLS, numpy oracle `🐍️frame-oracle.py`, confirmed by the Rust engine test)
- Column heads shorten: lc 0.16–0.19 mm, uc 0.24–0.28 mm.
- The cut is asymmetric (column 1 carries two −y cantilevers and one +y, column 2 two +y and one −y plus the short v6), so the heads lean apart in y: lower ±1.34 mm, upper ±4.94 mm; the deepest tips are v4 (−4.32 mm lower / −5.94 mm upper) and v1/v2/v5 close behind; tips v0/v3 on the light sides rise (+0.25 / +0.42 mm lower, +1.17 / +1.39 mm upper) because the head rotation lifts them more than their own sag.
- Every cantilever's sag beyond its head's rigid motion is exactly wL⁴/8EI: 1.628 mm for the 2.7 m arms, 0.102 mm for the 1.35 m arm (w = 10298 + 4952 + 3178 N/m).
- ULS peak vertical: −8.26 mm at uv4.
