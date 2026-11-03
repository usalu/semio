import sys
sys.path.insert(0, ".")
from spec import *

class R:
    def __init__(self, struct, coll, item, slug, en, de, group, existing, ins_emoji="➕", rem_emoji="➖", upd_emoji="✏️"):
        self.struct, self.coll, self.item, self.slug, self.en, self.de, self.group = struct, coll, item, slug, en, de, group
        self.existing = existing
        self.emoji = {"insert": ins_emoji, "remove": rem_emoji, "update": upd_emoji}
        self.pn = "En1997" + pascal(coll)
        self.camel_coll = camel(coll)

ROWS = [
    R("SoilLayer", "layers", "layer", "layer", "soil layer", "Bodenschicht", "ground", {"insert", "remove"}, "➕️", "➖️"),
    R("SpreadFoundation", "footings", "footing", "footing", "footing", "Fundament", "foundations", {"insert", "remove"}),
    R("Pile", "piles", "pile", "pile", "pile", "Pfahl", "foundations", {"insert", "remove"}, "📥", "📤"),
    R("RetainingWall", "retaining_walls", "retaining_wall", "retaining-wall", "retaining wall", "Stützwand", "foundations", set()),
    R("Slope", "slopes", "slope", "slope", "slope", "Böschung", "ground", set()),
    R("UpliftCase", "uplift_cases", "uplift_case", "uplift-case", "uplift case", "Aufschwimmfall", "foundations", set()),
    R("Anchor", "anchors", "anchor", "anchor", "ground anchor", "Verpressanker", "foundations", set()),
    R("EmbeddedWall", "embedded_walls", "embedded_wall", "embedded-wall", "embedded wall", "Eingespannte Wand", "foundations", set()),
    R("PileGroup", "pile_groups", "pile_group", "pile-group", "pile group", "Pfahlgruppe", "foundations", set()),
    R("LabTest", "lab_tests", "lab_test", "lab-test", "laboratory test", "Laborversuch", "ground", set()),
    R("LayerWater", "layer_waters", "layer_water", "layer-water", "layer water regime", "Wasserverhältnis der Schicht", "ground", set()),
]
VERBS = ["insert", "remove", "update"]
MUT = S + "/🧬️schema/🧬️mutations"

def kind(r, v): return f"{v}-{r.slug}"
def mod(r, v): return f"{v}_{r.slug.replace('-', '_')}"
def struct(r, v): return v.title() + pascal(r.slug.replace("-", "_"))
def dirname(r, v): return f"{r.emoji[v]}{kind(r, v)}"
def newkinds():
    return [(r, v) for r in ROWS for v in VERBS if v not in r.existing]
def allkinds():
    return [(r, v) for r in ROWS for v in VERBS]
