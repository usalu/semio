# 🗂️ Work Board: Staffage Plant Collection Wave

Wave launched 2026-11-22: 17 Opus 5.5 workers, 51 new vegetation assets (catalog grows from 16 to 67 plants). Brief: `📋️brief.md`.

| Worker | Theme | Assets | Status |
|---|---|---|---|
| PW-01 | Maples | norway-maple, field-maple, sycamore-maple | shipped by the finisher 2026-11-23: 3 of 3 valid (`validateAsset` 0, check ✓), rows written; volume findings left 10/8/9 (cross-scale), report `📝️PW-01.md` |
| PW-02 | Park trees | horse-chestnut, european-beech, common-ash | rebuilt by the finisher 2026-11-23 (extent fit): 3 of 3 valid, check ✓, rows rewritten; volume findings left 7/9/9 (cross-scale), report `📝️PW-02.md` |
| PW-03 | Flowering trees | japanese-cherry, saucer-magnolia, crab-apple | finished: 3 of 3 valid, `checkVolumes` 0, check ✓, rows match; report `📝️PW-03.md` |
| PW-04 | Urban street trees | ginkgo, honey-locust, sweetgum | finished: 3 of 3 valid, check ✓, rows match; `checkVolumes` 0/1/0 (honey-locust cross-scale 1:500); report `📝️PW-04.md` |
| PW-05 | Large broadleaves | tulip-tree, red-oak, black-locust | shipped by the finisher 2026-11-23: 3 of 3 valid, check ✓, rows written; `checkVolumes` 1/0/0 (tulip-tree cross-scale 1:500); report `📝️PW-05.md` |
| PW-06 | Small native trees | rowan, hawthorn, black-alder | shipped by the finisher 2026-11-23: 3 of 3 valid, check ✓, rows written; volume findings left 1/12/15; report `📝️PW-06.md` |
| PW-07 | Conifers I | norway-spruce, european-larch, atlas-cedar | reworked by the finisher 2026-11-23: 3 of 3 valid, check ✓, rows rewritten; volume findings 4/7/0 (were 8/14/1; cross-scale left); report `📝️PW-07.md` |
| PW-08 | Conifers II | italian-cypress, english-yew, dawn-redwood | shipped by the finisher 2026-11-23: 3 of 3 valid, check ✓, rows written; volume findings 0/7/8; report `📝️PW-08.md` |
| PW-09 | Mediterranean | olive-tree, fig-tree, stone-pine | 3 of 3 valid, check ✓, rows match; fig-tree rebuilt by the finisher 2026-11-23 from the changed template (volume findings 10, were 24), olive-tree 6 and stone-pine 11 unchanged; report `📝️PW-09.md` |
| PW-10 | Palms and tropical | windmill-palm, banana-plant, tree-fern | finished by the finisher 2026-11-22: shipped, proven (check CLI ✓, probe ✓), rows written; leaves are plates, tears and pinnae are strokes; report `📝️PW-10.md` |
| PW-11 | Desert | saguaro-cactus, prickly-pear, yucca | finished by the finisher 2026-11-22: shipped, proven (check CLI ✓, probe ✓), rows written; report `📝️PW-11.md` |
| PW-12 | Flowering shrubs | common-lilac, hydrangea, rhododendron | finished by the finisher 2026-11-22: lilac repaired, all three proven (check CLI ✓, probe ✓); report `📝️PW-12.md` |
| PW-13 | Formal shrubs and hedges | boxwood-ball, privet-hedge, cherry-laurel | finished by the finisher 2026-11-22: symbol notes added, all three proven (check CLI ✓, probe ✓); report `📝️PW-13.md` |
| PW-14 | Herbs and roses | lavender-bush, rosemary-bush, shrub-rose | finished by the finisher 2026-11-22: rose bark area corrected, all three proven (check CLI ✓, probe ✓); report `📝️PW-14.md` |
| PW-15 | Grasses and perennials | pampas-grass, common-reed, sunflower | finished by the second finisher 2026-11-23: every part remodelled as a mesh twin (body `polyhedron`, drawing its projected outline, no `slender`), all three proven on the shipped file (check CLI ✓, probe ✓, catalog-silhouette test 3 passed), rows rewritten; report `📝️PW-15.md` |
| PW-16 | Climbers and crops | wisteria-pergola, grapevine-row, maize-row | finished by the finisher 2026-11-22: all three repaired and proven (check CLI ✓, probe ✓); report `📝️PW-16.md` |
| PW-17 | Potted indoor plants | potted-monstera, fiddle-leaf-fig, snake-plant | finished by the finisher 2026-11-22: shipped, proven (check CLI ✓, probe ✓), rows written; report `📝️PW-17.md` |

## Coordinator integration (after the wave)

1. Merge `📥️manifest-rows/*.json` into `🗂️catalog/🔣️.json` in one write; add collections (e.g. conifers, flowering, mediterranean, desert, shrubs-and-hedges, grasses, climbers-and-crops, indoor-plants), bilingual.
2. Run the staffage test suite over the full catalog and the site build.
3. Delete `🗑️generated/` raw outputs, close the ticket with every file touched.
