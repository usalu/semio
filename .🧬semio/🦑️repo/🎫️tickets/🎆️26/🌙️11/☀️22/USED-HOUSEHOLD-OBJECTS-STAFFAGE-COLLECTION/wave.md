# Wave Ledger — Used Household Objects

State after the finishing pass of 2026-11-22 (evening). The wave ran rows 01–35 of `assign.md`; its session was cut off at about 17:30. Rows 36–40 (`used-ironing-board`, `used-clothes-airer`, `used-rocking-horse`, `used-kids-bicycle`, `used-balcony-furniture-set`) were dropped by the dev's order of 16:29 ("Just finish the ones that you have started.") and were never started: no candidate, no generator, no catalog file.

Every row below was measured by the finisher: full proof `r3-validate.ts --file … --volumes` and the product check CLI on the candidate (`finish-prove.sh`), catalog copy and manifest row against the candidate (`finish-reconcile.ts`: 35 of 35 equal, all members of `used-household-objects`), pose sheets opened (`finish-sheets.py`). Author reports: `reports/<slug>.md` (written by the finisher; no author report existed). Shared-code requests moved to the umbrella ticket's `requests.md`; `requests.md` here is the authors' original.

| # | slug | category | full proof | check | 2D unavailable | catalog = candidate, manifest row | finisher | open (asset level, stated in the report) |
|---|---|---|---|---|---|---|---|---|
| 01 | used-corner-sofa | furniture | VALID | ✓ | – | equal, equal | unchanged | No clip runs from `stored` to `in-use` (clips: `carry` stored to carried, `sit-down-three` from `in-use-empty`), so the feet sliding through the body only shows when a viewer blends the two poses directly (checked by the finisher in the clip keys); The two blankets and the tote lie beside the sofa in the home poses (seen on the pose sheet) |
| 02 | used-wingback-armchair | furniture | VALID | ✓ | – | equal, equal | unchanged | arm rolls approximate in tipped front/plan, wings straight slabs |
| 03 | used-recliner-chair | furniture | VALID | ✓ | – | equal, equal | unchanged | footrest single-link stand-in; blanket stays behind chair in use |
| 04 | used-coffee-table | furniture | VALID | ✓ | – | equal, equal | unchanged | rigid-body structure approximation for carried/in-use (requests.md) |
| 05 | used-tv-sideboard | furniture | VALID | ✓ | – | equal, equal | unchanged | Hinges and stays of the flaps are not modelled (author's ledger note); The coordinator's orientation question is settled: the schema defines `widthM` along world x (ahead, the real depth of an object drawn in side elevation) and `depthM` along z, so 0.428 / 1.80 is the convention, as for `used-wall-unit`, `used-upright-piano` and `used-kitchen-base-cabinet`; no defect |
| 06 | used-wall-unit | furniture | VALID | ✓ | – | equal, equal | unchanged | bar flap front 2D lacks routed line, knob square; stays keyed every 15° |
| 07 | used-double-bed | furniture | VALID | ✓ | – | equal, equal | unchanged | rigid-body solver splits carried mattress weight with stack floor; 1:500 print generalisations |
| 08 | used-hinged-wardrobe | furniture | VALID | ✓ | – | equal, equal | unchanged | blanket aside in use; 1:500 doors without gaps |
| 09 | used-chest-of-drawers | furniture | VALID | ✓ | – | equal, equal | unchanged | blanket in front in use |
| 10 | used-bedside-table | furniture | VALID | ✓ | 1:500 | equal, equal | unchanged | blanket reaction as upward load (rigid-body solver); marks offset in tilted carried plan |
| 11 | used-bunk-bed | furniture | VALID | ✓ | – | equal, equal | unchanged | The ladder's 6.06 kg act 0.31 m too far inwards laterally when set up: the bone mass has no lateral centre and the ladder bone sits at z -0.219 m for its twist in the bundle (measured by the finisher from the rig; request for `comZ` in the umbrella requests.md); Ladder hooks (4 mm steel) are not modelled (stated in the 2D decisions) |
| 12 | used-baby-cot | furniture | VALID | ✓ | 1:500 | equal, equal | unchanged | – |
| 13 | used-extendable-dining-table | furniture | VALID | ✓ | – | equal, equal | unchanged | leaf thermal area uses folded value in extended poses |
| 14 | used-bentwood-dining-chairs | furniture | VALID | ✓ | – | equal, equal | unchanged | in-use chairs all face one way (rig cannot turn them); carried chairs hang level |
| 15 | used-writing-desk | furniture | VALID | ✓ | – | equal, equal | unchanged | laptop and user loads only |
| 16 | used-shoe-cabinet | furniture | VALID | ✓ | – | equal, equal | unchanged | blanket massless; shoes loads only |
| 17 | used-coat-stand | furniture | VALID | ✓ | – | equal, equal | unchanged | rigid-body solver puts blanket weight on hands (requests.md); carried plan distorts curved feet; hang-coats clip has no motion; Checked by the finisher: the clip `hang-coats` has ten tracks (nine loads growing hook by hook and the exposed fraction of the hooks) and no bone motion, because the coats are loads, not geometry |
| 18 | used-kitchen-base-cabinet | furniture | VALID | ✓ | – | equal, equal | unchanged | joints 1.5 mm vs real 2-3; rigid-body solver puts blanket on grips; 1:200 silhouette artefact logged in requests.md |
| 19 | used-high-chair | furniture | VALID | ✓ | 1:500 | equal, equal | unchanged | floor tile mass wrongly on hands when carried; hands/toddler loads only |
| 20 | used-folding-bookcase-ladder | furniture | VALID | ✓ | – | equal, equal | unchanged | books/plants loads only; blanket and anchor massless |
| 21 | used-fridge-freezer | appliances | VALID | ✓ | – | equal, equal | unchanged | hand truck has no strap |
| 22 | used-washing-machine | appliances | VALID | ✓ | – | equal, equal | unchanged | hoses coiled in use; no door-open pose; spin drawn slow; rigid-body reactions shared with pallet |
| 23 | used-tumble-dryer | appliances | VALID | ✓ | 1:500 | equal, equal | unchanged | drum hidden thermal-only so open aperture looks empty |
| 24 | used-dishwasher | appliances | VALID | ✓ | 1:500 | equal, equal | unchanged | pallet 0.95 m behind in home poses |
| 25 | used-electric-cooker | appliances | VALID | ✓ | 1:500 | equal, equal | unchanged | hotplates share one temperature |
| 26 | used-microwave-oven | appliances | VALID | ✓ | 1:200, 1:500 | equal, equal | unchanged | cardboard lies beside it in use |
| 27 | used-crt-television | appliances | VALID | ✓ | – | equal, equal | unchanged | swivel joint unused by poses; Seen by the finisher: in `in-use` the set stands on its swivel stand directly on the floor, as its description says; a television bench is not part of the asset |
| 28 | used-floor-lamp | appliances | VALID | ✓ | – | equal, equal | unchanged | cardboard in front of lamp in use |
| 29 | used-oil-filled-radiator | appliances | VALID | ✓ | 1:500 | equal, equal | unchanged | envelope fins counted closed by thermal coverage; cord stays wound in use |
| 30 | used-sewing-machine | appliances | VALID | ✓ | 1:500 | equal, equal | table added, re-published | The folded blanket stays under the machine in use (as a damping mat) because it is a fixed storage aid on the table; Rigid-body solver: in `carried` the table and blanket share reactions with the hand (umbrella requests.md); Transitions between `stored` and `in-use` outside a clip move the pedal straight down through the table top (no clip does this) |
| 31 | used-upright-piano | objects | VALID | ✓ | – | equal, equal | unchanged | released strap rigid on edge in use; one strap only; Seen by the finisher: in the home poses the released strap lies as a rigid rectangular frame on the floor beside the dolly |
| 32 | used-bathtub | objects | VALID | ✓ | – | equal, equal | unchanged | shell inside/underside share one region; end view shows inside details over end wall; symmetric ends |
| 33 | used-washbasin-vanity | objects | VALID | ✓ | 1:500 | equal, equal | unchanged | mirror doors cannot open (vertical hinge twist flagged); light bar 0.6 kg low; Seen by the finisher: in the home poses the vanity hangs 0.136 m above the floor and the mirror cabinet at 1.06 m; both are wall-hung and the wall is surroundings, not part of the asset |
| 34 | used-wc-pan-cistern | objects | VALID | ✓ | 1:500 | equal, equal | unchanged | revolve lateralScale axis vs README (filed by coordinator in requests.md); pan 22.8 kg 4% over range |
| 35 | used-moving-boxes | objects | VALID | ✓ | 1:500 | equal, equal | unchanged | resting faces not outer surface (request); books loads only |

## History

- 15:45 rows 01–20 launched, rows 21–35 as slots freed; 16:15 the manifest row of 13 was lost to a concurrent manifest write and re-published; 16:29 rows 36–40 dropped; 17:04 the last author published; about 17:30 the session was cut off before reports, review pass and reconciliation.
- 20:23–21:15 finishing pass: 35 proofs re-run (35 VALID, 35 ✓), reconciliation 35/35, `used-sewing-machine` corrected (it floated at 0.75 m in use; it now stands on its own sewing table) and re-published, 35 pose contact sheets opened, 35 reports written.
