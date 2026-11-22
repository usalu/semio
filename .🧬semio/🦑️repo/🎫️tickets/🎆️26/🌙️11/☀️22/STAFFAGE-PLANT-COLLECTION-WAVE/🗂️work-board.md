# 🗂️ Work Board: Staffage Plant Collection Wave

Wave launched 2026-11-22: 17 Opus 5.5 workers, 51 new vegetation assets (catalog grows from 16 to 67 plants). Brief: `📋️brief.md`.

| Worker | Theme | Assets | Status |
|---|---|---|---|
| PW-01 | Maples | norway-maple, field-maple, sycamore-maple | running |
| PW-02 | Park trees | horse-chestnut, european-beech, common-ash | running |
| PW-03 | Flowering trees | japanese-cherry, saucer-magnolia, crab-apple | running |
| PW-04 | Urban street trees | ginkgo, honey-locust, sweetgum | running |
| PW-05 | Large broadleaves | tulip-tree, red-oak, black-locust | running |
| PW-06 | Small native trees | rowan, hawthorn, black-alder | running |
| PW-07 | Conifers I | norway-spruce, european-larch, atlas-cedar | running |
| PW-08 | Conifers II | italian-cypress, english-yew, dawn-redwood | running |
| PW-09 | Mediterranean | olive-tree, fig-tree, stone-pine | running |
| PW-10 | Palms and tropical | windmill-palm, banana-plant, tree-fern | running |
| PW-11 | Desert | saguaro-cactus, prickly-pear, yucca | running |
| PW-12 | Flowering shrubs | common-lilac, hydrangea, rhododendron | running |
| PW-13 | Formal shrubs and hedges | boxwood-ball, privet-hedge, cherry-laurel | running |
| PW-14 | Herbs and roses | lavender-bush, rosemary-bush, shrub-rose | running |
| PW-15 | Grasses and perennials | pampas-grass, common-reed, sunflower | running |
| PW-16 | Climbers and crops | wisteria-pergola, grapevine-row, maize-row | running |
| PW-17 | Potted indoor plants | potted-monstera, fiddle-leaf-fig, snake-plant | running |

## Coordinator integration (after the wave)

1. Merge `📥️manifest-rows/*.json` into `🗂️catalog/🔣️.json` in one write; add collections (e.g. conifers, flowering, mediterranean, desert, shrubs-and-hedges, grasses, climbers-and-crops, indoor-plants), bilingual.
2. Run the staffage test suite over the full catalog and the site build.
3. Delete `🗑️generated/` raw outputs, close the ticket with every file touched.
