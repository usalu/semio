# Storyboard: grasshopper_tower, `Beat3_Tower`

File: `A/🦗️grasshopper/🎬️clip/🗼️tower/🐍️.py`. Series 🦗️grasshopper, topic 🗼️tower. Length target 4.5 to 5 minutes, 20 clauses.

## Learning goals

1. A tower is not drawn but generated: core, planes, capsules, capital and podium are five parts of one definition, each in its own coloured canvas group.
2. The core is a parametric extrusion (plan side, storey height, storey count) and the storey lines come from the same numbers.
3. Planes are the interface between core and capsule: a Series of elevations crossed with the facade directions gives one plane per storey and facade (X along the facade, Y outward, Z up); Orient moves a capsule from its base plane onto each of them, and the count follows `storeys x facades x 2`.
4. Parts get the effort that matters for the subject: capsule and core are parametric, the capital is a small parametric part, the podium is only an extruded massing because it is context.
5. Replaceability (Metabolism): swapping one capsule type updates every capsule of that type.

## Misconceptions addressed (research-grasshopper, research-nakagin)

- "Every capsule is modelled by hand": one capsule, many planes, one Orient.
- "Planes are just points": a plane carries orientation; with the wrong X axis the capsule faces the wrong way (preview of the casting video).
- "More storeys means more modelling": the slider changes the count, no wire changes.
- "Everything needs the same level of detail": the podium stays an extrusion, and we say why.
- "Item 0 and item 1 on a facade are the same capsule": two items per branch sit side by side along X.
- Open data: the historical count is 140 (Wikipedia), the teaching model keeps the same number from the definition, never typed in the scene. Say "teaching model" once (in `recap` or `counts`).

## Screen layout

Left 55 percent: Grasshopper canvas (real component names, groups Core, Planes, Capsules, Capital, Podium as coloured group frames with the same colour as the drawn part). Right 45 percent: Rhino-like viewport (`gh_viewport`) with the Nakagin axonometric. Caption band at the bottom, at most two lines. Group colours: Core = concrete grey-blue, Planes = green (axes colour family), Capsules = warm orange, Capital = violet, Podium = neutral sand.

## Clauses

| # | Key | English | German | On screen (tool / building) | Value and its drawing | Animation |
|---|---|---|---|---|---|---|
| 1 | `hook` | One capsule is easy. How do 140 become a tower? | Eine Kapsel ist einfach. Wie werden aus ihr 140 ein Turm? | Empty canvas / grey Nakagin axon with one capsule highlighted among all | count 140 from the definition, all capsules lit | other capsules fade out, one stays |
| 2 | `parts` | The tower splits into five parts: core, planes, capsules, capital, podium. Each gets its own group. | Der Turm besteht aus fünf Teilen: Kern, Ebenen, Kapseln, Kopf, Sockel. Jeder bekommt eine eigene Gruppe. | five empty coloured groups appear on the canvas / tower axon tinted in the same five colours | colour = part | groups and tinted parts pop in pairwise |
| 3 | `core` | The core is a square extrusion: plan side, storey height and storey count are three sliders. | Der Kern ist eine quadratische Extrusion: Seite, Geschosshöhe und Geschossanzahl sind drei Slider. | Core group: 3 Number Sliders, Rectangle, Extrude / one core axon | plan side (m) dimension on the plan, storey height (m) on one storey, storeys as lines | wires draw, extrusion grows |
| 4 | `shaft` | The result is the shaft: one box with a line on every storey. | Das Ergebnis ist der Schacht: ein Quader mit einer Linie auf jedem Geschoss. | Panel with storey count, Extrude output / core with storey lines | storey count = number of lines, lines from the engine | lines draw one by one |
| 5 | `series` | A Series of elevations gives one height per storey: 0, 2.75, 5.5 and so on. | Eine Series liefert eine Höhe pro Geschoss: 0, 2,75, 5,5 und so weiter. | Series component (start, step, count) + Panel list / horizontal storey marks on the core | list of elevations (m) as ticks on the core | ticks pop up |
| 6 | `facades` | The four facade directions come from one more list. Elevations times facades: one plane per storey and facade. | Die vier Fassadenrichtungen kommen aus einer weiteren Liste. Höhen mal Fassaden: eine Ebene pro Geschoss und Fassade. | Construct Plane + cross of two lists, Panel `{storey;facade}` / plan view of the core with four arrows | plane count = storeys x facades | four facade planes rotate in around the core |
| 7 | `axes` | Each plane has axes: X runs along the facade, Y points outward, Z points up. | Jede Ebene hat Achsen: X läuft entlang der Fassade, Y zeigt nach außen, Z nach oben. | Param Viewer for one plane / one `plane_axes` on a facade, red X, green Y, blue Z | plane origin and axes | axes grow from the origin |
| 8 | `items` | On each facade two capsules sit side by side: item 0 and item 1 along X. | Auf jeder Fassade sitzen zwei Kapseln nebeneinander: Item 0 und Item 1 entlang X. | List Item / one facade plane with two origins labelled `[0]`, `[1]` | two offsets along X (m) | two markers slide to their spots |
| 9 | `orient` | Orient moves the capsule from its base plane onto each target plane. | Orient setzt die Kapsel von ihrer Basisebene auf jede Zielebene. | Orient component, wires from capsule geometry, source plane, target planes / capsule travels from the origin onto one plane | source plane and target plane drawn as axes | capsule moves and turns |
| 10 | `types` | A type pattern picks A, B, C or D for every plane. The wire is the same, only the list differs. | Ein Typmuster wählt A, B, C oder D für jede Ebene. Die Verbindung bleibt, nur die Liste ändert sich. | Value List / List Item + Panel with the pattern / capsules tinted by their type | type key per plane | capsules on the storey fill in by type colour |
| 11 | `count` | Now the storeys slider: capsule count equals storeys times facades times two. | Nun der Storeys-Slider: Kapselanzahl gleich Geschosse mal Fassaden mal zwei. | Slider + Panel `count` with formula / core grows with capsules | count live (ValueTracker), storeys 1 to maximum, capsule per storey drawn | slider sweeps, capsules and counter follow |
| 12 | `nowire` | No wire changes. The definition does not know how tall the tower is. | Keine Verbindung ändert sich. Die Definition weiß nicht, wie hoch der Turm ist. | highlight of all wires unchanged / tower with more storeys | wires shown constant | wires glow while the tower grows |
| 13 | `capital` | The capital is its own small parametric part: height and chamfer are two sliders. | Der Kopf ist ein eigenes kleines Parametermodell: Höhe und Fase sind zwei Slider. | Capital group: 2 sliders + Extrude + taper / capital axon on one core | capital height (m) and chamfer (m) as dimension lines | slider moves, chamfer changes live |
| 14 | `ontop` | It sits on top of each core, at the elevation the storey slider gives. | Er sitzt auf jedem Kern, genau auf der Höhe, die der Storey-Slider vorgibt. | wire from storeys to capital move vector / capital rises with the core when storeys change | capital base elevation (m) | capital follows the slider |
| 15 | `podium` | The lower building is only an extruded massing: one footprint, one height. | Das Sockelgebäude ist nur ein extrudierter Baukörper: ein Grundriss, eine Höhe. | Podium group: Rectangle, Extrude, 2 sliders / podium box with footprint and height | footprint (m x m) and height (m) | box extrudes |
| 16 | `why` | It stays simple on purpose: it is context, not the subject. Detail belongs where the design idea is. | Es bleibt bewusst einfach: Es ist Kontext, nicht das Thema. Detail gehört dorthin, wo die Entwurfsidee liegt. | podium highlighted next to the detailed capsule group / the same two next to each other | none (comparison of effort) | podium dims, capsules light |
| 17 | `towers` | The same definition builds tower A and tower B: only the storey count differs. | Dieselbe Definition baut Turm A und Turm B: nur die Geschossanzahl unterscheidet sich. | tower A and B inputs side by side / both towers on the podium | storeys A and B (from model) | second tower grows next to the first |
| 18 | `total` | Together: the historical count of 140 capsules, read from the definition, not typed. | Zusammen: die historische Zahl von 140 Kapseln, aus der Definition gelesen, nicht eingetippt. | Panel with total / all capsules lit, counter | total = engine count, per tower counts | counter ticks up while capsules light |
| 19 | `swap` | Metabolism: swap one capsule type and every capsule of that type updates. | Metabolismus: Tausche einen Kapseltyp, und jede Kapsel dieses Typs wird aktualisiert. | change of the type geometry input (e.g. window diameter slider) / all capsules of that type change at once | window diameter (m) on all type B capsules | one slider, many capsules change |
| 20 | `recap` | Recap: core, planes, capsules, capital, podium. This is the teaching model of the Nakagin Capsule Tower. | Zusammenfassung: Kern, Ebenen, Kapseln, Kopf, Sockel. So sieht das Lehrmodell des Nakagin Capsule Tower aus. | final canvas with the five groups, legend en/de / tower | five colours | groups pulse one after another |

## Legend of terms (clause 20, both languages)

Core / Kern, Plane / Ebene, Capsule / Kapsel, Capital / Kopf, Podium / Sockel, Series, Orient, Group / Gruppe, Storey / Geschoss, Facade / Fassade.

## Open points to resolve against the real model on disk

- Exact slider domains, the cross-product components and the `tower` canvas object ids come from `🧱️model/🔣️.json` (GH-ENGINE) and are checked when api.md sections 1 to 4 are ready.
- `storeys x facades x 2` must match the engine count for the generic definition; the historical 140 is the assembled sum over towers A and B of the occupied capsule slots, as the NAKAGIN model derives it.
