# Teaching Intent of Course Morphs (haiku-teaching-audit, transcribed by coordinator)

Read-only audit at 2026-11-11. Counts are clusters, not grep lines. Energy was sampled (29 clusters), not exhaustive.

## Category counts

| Series | Clusters | Abstraction | Concretization | Continuation | Transformation | Absence | Decorative |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Grasshopper (capsule 19, tower 23, casting 19, anatomy 15, trees 1) | 77 | 27 | 19 | 13 | 14 | 3 | 1 |
| Excel (references 34, program 12, energy 22, anatomy 11, workbook 1) | 80 | 28 | 19 | 18 | 15 | 0 | 0 |
| Energy (sampled) | 29 | 8 | 3 | 7 | 11 | 0 | 0 |
| Optimization | 22 | 5 | 1 | 9 | 4 | 0 | 3 |
| BIM | 1 | 1 | 0 | 0 | 0 | 0 | 0 |
| Total | 209 | 69 | 42 | 47 | 44 | 3 | 4 |

Relations: most `morph`/`retext`/`Transform` are 1:1; about 30 scatter (n:n), 25 gather (n:1), 10 split (1:n), 2 unfold (1:n). "Absence" = an entity morphs into nothing (stray capsule, failure), which must be declarable (target `none`).

## Teaching-critical intermediates (final must keep the full morph)

1. Capsule cut ring → prism → hole (`🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py:1056–1058`).
2. Capsule front plane ring → plane token → front ring (1026–1027).
3. Capsule orient capsule → plane A → capsule B (1159–1161, `capsule_pairs`).
4. Capsule list item / stray warning (1202, 1210); wrap (1221); lift half values (940); rounded box → soft shell (986).
5. Casting failure chain (`🫗️casting/🐍️.py:842, 860`), rounding bands → line dot round-half-even (708, 713), world plane 90° turn (739), origin ghost (733).
6. Tower count gather (`🗼️tower/🐍️.py:282`), phantoms and cull (406, 416), per-tower totals (479, 488).
7. Excel relative/absolute/mixed reference moves (`📊️excel/🎬️clip/📍️references/🐍️.py:402, 495, 518, 537, 591, 632`), 3D sum shrink on deleted sheet (709, 785).
8. Excel countif/typo (`📋️program/🐍️.py:543–544, 719–720`), b-factor fix (`⚡️energy/🐍️.py:551–556`), U·A arrows and H_T gathers (483, 516, 526, 577).
9. Optimization sign flip (`🎯️optimization/…/📉️extrema/🐍️.py:712`), unit change (`⚖️index/🐍️.py:1206`), run4 → mean chip (`🧪️experiments/🐍️.py:819`), key dots → true line (883).
10. Energy natural ventilation opening chain and outdoor sign (`🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:449–506, 1541`), aeff (913–931), stack height 2.5 → 4.4 m (1143); mechanical WRG share 1500 W → 300 W (`⚙️mechanical/🐍️.py:627–638`); demand recap ring chains (`🔁️recap/🐍️.py:689–715, 1019–1031, 1127`), system-losses chain (`♨️system-losses/🐍️.py:691–1165`).
11. BIM information `ReplacementTransform(raw, dim.label)` (`🏗️bim/🎬️clip/📊️information/🐍️.py:639`).

Decorative: capsule recap legend (1237), optimization solver label swaps (519, 621, 768), `dip`. `ghost` carries meaning (casting 733, excel references 353/433, excel energy 483).

## Values and causal relations previews must keep

- Transmission U·A·Δθ: wall 209 m² × 0.24 ≈ 1000 W, roof 89 m² × 0.20 ≈ 360 W, windows 30 m² × 1.10 ≈ 660 W, Δθ 20 K, Φ_T ≈ 2300 W.
- Ventilation 220 m³/h × 0.34 Wh/m³K × 20 K ≈ 1500 W; total ≈ 3800 W; WRG η 0.8 leaves ≈ 300 W.
- Annual: Q_V 16 000 kWh/a, ≈ 3500 Kd, Q_sol ≈ 3000, Q_int ≈ 2000, Q_Gewinn ≈ 5000, η_h 0.90 (0.97 with mass), Q_nutz ≈ 4500, Q_h ≈ 11 500, system losses 600/900/360/700, Q_E ≈ 14 000 kWh/a.
- Solar 30 m² × 26 kWh/m² × g 0.6 × F_s 0.7 ≈ 328 kWh; internal gains 5 W/m².
- Purge 340 Wh vs tilted ≈ 830 Wh; tilted ≈ 8 % after 5 min; masonry 160 kg, 5 K → 190 Wh.
- Natural ventilation: night 18 °C out / 26 °C in; winter ΔT 20 K vs 8 K ≈ 3× pressure; night flush ≈ 4 K.
- Causal: opening width/duration → air change → loss; outdoor sign flips heating/cooling; smallest series opening governs; convection breeze overlay moves; BIM mesh loss (class/GlobalId lost) is a stated event.
- Staffage ids to keep in preview: `shared-person-winter`, `shared-radiator-front-winter`, `shared-radiator-thermostat-front`, `heating-internal-gains-*`, `shared-desk`, `shared-lamp-winter`.

## Contract gaps raised

1. `split` keeps the source visible for branches 2..n; `gather` fades the target in during the flight. Preview rule: n:1 sources move toward the target placement, target appears at f1; 1:n source moves toward union bounds, all targets appear at f1 (per-element windows only for lagged n:n). Final keeps the authored overlap.
2. Auto pairing (`pairs=None`) can mismatch parts (shell to dot); check energy scatters 483, 516, 589, tower 360/366/406/416, casting 669 and declare `pairs`/`recipe` where meaning depends on it.
3. Value text changes (2.5 → 2, Type A failure) must hit the authored endpoint exactly in preview.
4. "Absence" targets (morph into nothing) need declarative support.
