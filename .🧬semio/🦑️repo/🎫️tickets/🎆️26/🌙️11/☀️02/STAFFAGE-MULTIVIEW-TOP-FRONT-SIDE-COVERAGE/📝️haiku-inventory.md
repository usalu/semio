# 📝️ haiku-inventory: Catalog Inventory (relayed by the coordinator)

This is a read-only inventory built from Grep counts on `🗂️catalog`, dated 2026-11-02. The per-asset tables (dimensions, bands, poses, clips, emitters, loads, supports, symbols) were returned in chat. The coverage matrix produced by sonnet-validation replaces them as the executable record.

## Totals

- **Assets:** 149 in 9 categories. Humans 28, animals 14, vegetation 16, objects 15, furniture 12, appliances 24, machinery 16, vehicles 9, components 15.
- **Bands:** 596 (149 × 4 presets: 1:50, 1:100, 1:200, 1:500). Every `denominator` is one of the presets.
  - 534 are authored, and all of them have elevation, front and plan geometry.
  - 62 are unavailable, in 54 assets. 54 of these are at 1:500 and 8 at 1:200. All 62 carry `limitingFeature`.
- **Unavailable at 1:200:** red-squirrel, hedgehog, expansion-valve, thermostatic-radiator-valve, room-thermostat, parent-carrying-toddler, skateboarder, kneeling-gardener.
- **Unavailable at 1:500, by category:** appliances 24/24, machinery 8/16, animals 7/14, objects 4/15, components 4/15, humans 3/28, vehicles 2/9, vegetation 1/16, furniture 1/12.
- **Alternatives:** 111 in total: 100 `scale`, 7 `symbol`, 4 `asset`.
- **Symbols:** 53 in 46 assets. **None has a front view**, and 7 lack a plan view. `walking-direction-marker` (walking-commuter) has an elevation only.
- **Sections:** 16, in 4 assets: bookshelf, curtain-wall-bay, masonry-wall-with-lintel and sliding-door.
- **Supports:** 92 assets have supports, 57 have none.
- **Loads, emitters and poses:**
  - Loads: 88, in 61 assets.
  - Emitter anchors: 373, in 106 assets.
  - Pose (`ground`) keys: 921 across all 149 assets.
- **Clips:** 353. Every asset has at least two.
- **Authored transitions:** none. Transitions are runtime queries between two poses.

## Gaps

- **G1. Unavailable bands.** 62 bands; each needs an honest cause. A follow-up audit is assigned to haiku-inventory.
- **G2. Forbidden asset substitution.**
  - At 1:500, hedgehog → red-squirrel, which is itself unavailable at 1:500 and points on to domestic-cat. The domestic cat is also unavailable at 1:500.
  - red-squirrel → domestic-cat (unavailable).
  - Each step substitutes a different animal, and the chain still ends with nothing drawable.
- **G3. Symbol views.** No symbol has a front view, and at least 7 lack a plan view.
- **G4. Wrong name in the brief.** The real gap is `SymbolRepresentation.geometry`. The `Alternative` type has no geometry of its own.
- **G5. No transitions authored.** Coverage of transitions must come from evaluator sampling of pose pairs.
- **G6. Pose geometry.** Band geometry is drawn in the rest pose, and poses come from the rig. Manifest pose lists are not complete: walking-commuter has 4 listed against 8 ground keys, labrador 3 against 19.
- **G7. Few sections.** Only 4 assets have sections.
- **G8. Declared ranges beyond the presets.** `validRange` reaches maxDenominator 1000 on 91 bands, and minDenominator up to 351 (bar-stool). These are declared supported ranges, so the matrix samples the range ends as well as the presets.
- **G9. Not extracted:** thermal regions per asset, shape counts per view, and massKg.
- **G10. JSON indentation is inconsistent** (vehicles use 1 space). This is cosmetic.
