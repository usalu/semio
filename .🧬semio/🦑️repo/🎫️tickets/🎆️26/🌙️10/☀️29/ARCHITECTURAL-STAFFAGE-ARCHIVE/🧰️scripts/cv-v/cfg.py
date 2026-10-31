# region: (geometry, skyViewFactor, heatCapacityJM2K, lai, photosynthesis, reference)
# sunlit: calm, windy, storm, snow(wet-snow/ice/snow)
W = "wood-handbook-2021"; J = "jones-2014"
REG = {
"english-oak": {
  "trunk-bark": ("cylinder", .35, 80000, None, None, W, (.35, .35, .1, 0)),
  "limb-bark": ("cylinder", .45, 53000, None, None, W, (.25, .25, .1, 0)),
  "leaf-sun": ("vertical-plate", .7, 900, 1.5, "c3", J, (.9, .9, .5, 0)),
  "leaf-shade": ("vertical-plate", .3, 700, 1.2, "c3", J, (.08, .08, 0, 0))},
"lombardy-poplar": {
  "stem-bark": ("cylinder", .4, 70000, None, None, W, (.4, .4, .1, 0)),
  "branch-bark": ("cylinder", .3, 30000, None, None, W, (.15, .15, .05, 0)),
  "leaf-crown": ("vertical-plate", .5, 700, 17.3, "c3", J, (.5, .5, .25, 0))},
"multi-stem-birch": {
  "stem-bark": ("cylinder", .4, 70000, None, None, W, (.5, .5, .15, 0)),
  "branch-bark": ("cylinder", .3, 20000, None, None, W, (.3, .3, .1, 0)),
  "leaf-crown": ("vertical-plate", .55, 600, 1.9, "c3", J, (.6, .6, .3, 0))},
"scots-pine": {
  "bark-lower": ("cylinder", .4, 70000, None, None, W, (.45, .45, .15, 0)),
  "bark-upper": ("cylinder", .45, 50000, None, None, W, (.4, .4, .15, 0)),
  "limb-bark": ("cylinder", .5, 30000, None, None, W, (.3, .3, .1, 0)),
  "needles-sun": ("cylinder", .7, 1100, 1.2, "c3", J, (.9, .9, .5, 0)),
  "needles-shade": ("cylinder", .3, 1100, .8, "c3", J, (.1, .1, 0, 0))},
"date-palm": {
  "stem-bark": ("cylinder", .5, 60000, None, None, W, (.7, .7, .3, 0)),
  "fruit": ("sphere", .3, 20000, None, None, J, (.2, .2, .1, 0)),
  "fronds": ("vertical-plate", .75, 1300, 2.4, "c3", J, (.75, .75, .4, 0))},
"weeping-willow": {
  "trunk-bark": ("cylinder", .35, 80000, None, None, W, (.15, .15, .05, 0)),
  "limb-bark": ("cylinder", .4, 40000, None, None, W, (.3, .3, .1, 0)),
  "leaf-curtain": ("vertical-plate", .45, 600, 2.0, "c3", J, (.45, .45, .2, 0))},
"hornbeam-hedge": {
  "stem-bark": ("cylinder", .2, 13000, None, None, W, (.02, .02, 0, 0)),
  "leaf-face": ("vertical-plate", .5, 900, 7.3, "c3", J, (.5, .5, .25, 0)),
  "leaf-core": ("vertical-plate", .1, 900, 6.4, "c3", J, (.05, .05, 0, 0))},
"miscanthus-grass": {
  "blades": ("vertical-plate", .6, 600, .97, "c4", J, (.65, .65, .35, 0)),
  "plumes": ("cylinder", .8, 500, None, None, J, (.8, .8, .4, 0)),
  "culms": ("cylinder", .5, 6000, None, None, J, (.15, .15, .05, 0))},
}
WET = {"leaf": (0, 0, .4, .5), "bark": (0, 0, .4, .3)}
EMIT = {  # id suffix -> (region, diameter m, title en, title de)
"english-oak": [("sun", "leaf-sun", 2.4, "Transpiration of the sun leaves", "Transpiration der Sonnenblätter"), ("shade", "leaf-shade", 2.0, "Transpiration of the shade leaves", "Transpiration der Schattenblätter")],
"lombardy-poplar": [("crown", "leaf-crown", 1.4, "Transpiration of the crown", "Transpiration der Krone")],
"multi-stem-birch": [("crown", "leaf-crown", 1.6, "Transpiration of the crown", "Transpiration der Krone")],
"scots-pine": [("sun", "needles-sun", 2.2, "Transpiration of the sun needles", "Transpiration der Sonnennadeln"), ("shade", "needles-shade", 2.0, "Transpiration of the shade needles", "Transpiration der Schattennadeln")],
"date-palm": [("fronds", "fronds", 1.4, "Transpiration of the fronds", "Transpiration der Wedel")],
"weeping-willow": [("curtain", "leaf-curtain", 1.6, "Transpiration of the curtains and caps", "Transpiration der Vorhänge und Kappen")],
"hornbeam-hedge": [("face", "leaf-face", .6, "Transpiration of the hedge face", "Transpiration der Heckenaußenseite"), ("core", "leaf-core", .5, "Transpiration of the shaded hedge core", "Transpiration des beschatteten Heckenkerns")],
"miscanthus-grass": [("blades", "blades", .5, "Transpiration of the blades", "Transpiration der Blätter")],
}
PROJ = {"english-oak": 165, "lombardy-poplar": 13.9, "multi-stem-birch": 46.7, "scots-pine": 76.7, "date-palm": 37.9, "weeping-willow": 164.9, "hornbeam-hedge": 2.2, "miscanthus-grass": 3.1}
DIMS = {"english-oak": (18.2, 18.35), "lombardy-poplar": (26.0, 4.16), "multi-stem-birch": (13.0, 9.26), "scots-pine": (22.2, 12.15), "date-palm": (12.5, 6.87), "weeping-willow": (11.1, 16.23), "hornbeam-hedge": (1.88, 3.14), "miscanthus-grass": (1.75, 2.14)}
PLAN = {"english-oak": (9.175, 7.5, .45), "lombardy-poplar": (2.08, 2.2, .28), "multi-stem-birch": (4.63, 3.5, .15), "scots-pine": (6.08, 4.0, .3), "date-palm": (3.43, 3.5, .26), "weeping-willow": (8.1, 7.0, .5), "hornbeam-hedge": (1.57, .35, .05), "miscanthus-grass": (1.07, .9, .22)}
DEFAULT_SCALE = {"english-oak": 200, "lombardy-poplar": 200, "multi-stem-birch": 200, "scots-pine": 200, "date-palm": 100, "weeping-willow": 200, "hornbeam-hedge": 50, "miscanthus-grass": 50}

CROWN_BASE = {"english-oak": 4.2, "lombardy-poplar": 2.6, "multi-stem-birch": 4.4, "scots-pine": 12.4, "date-palm": 8.7, "weeping-willow": 0.3, "hornbeam-hedge": 0.0, "miscanthus-grass": 0.0}
