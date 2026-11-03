"""🚗 The one en1991 index selector the lane template cannot generate: `change-accidental-assumed-force` addresses the impact block through its accidental case; it is rewritten from its id-addressed sibling `change-accidental-vehicle-mass`.

`python -I en1991_accidental_force.py [--apply]`; stage `🗑️generated/kind-convention-v2/stage/en1991-accidental`.
"""
import os, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from migrate_ids import *


def main():
    stage = Stage(TICKET + "/🗑️generated/kind-convention-v2/stage/en1991-accidental")
    art = Artifact("en1991", stage)
    sibling = kind_folder(art.mut, "change-accidental-vehicle-mass")
    folder = kind_folder(art.mut, "change-accidental-assumed-force")
    swap = lambda text: text.replace("change_accidental_vehicle_mass", "change_accidental_assumed_force").replace("ChangeAccidentalVehicleMass", "ChangeAccidentalAssumedForce").replace("new_vehicle_mass", "new_assumed_force").replace("vehicle_mass", "assumed_force").replace("change-accidental-vehicle-mass", "change-accidental-assumed-force")
    diff = swap(rd(sibling + "/🔺️diff/🦀️.rs"))
    inverse = swap(rd(sibling + "/↩️inverse/🦀️.rs"))
    if "index" in rd(folder + "/🦀️.rs"):
        renames = [("index", "id", "accidentalCase")]
        labels = {"index": {"en": "Accidental case", "de": "Außergewöhnlicher Fall"}}
        report = dict(set=0, fixtures=0)
        finish(art, stage, folder, "change-accidental-assumed-force", diff, inverse, renames, labels, ["accidentalCases"], ["index"], report, "set")
        rules = stage.read(art.rules_path)
        old = 'kind: "change-accidental-assumed-force", selectors: &[SelectorKey::Index("index"), SelectorKey::Implied]'
        stage.write(art.rules_path, rules.replace(old, old.replace('Index("index")', 'Id("id")'), 1))
        print(report)
    if "--apply" in sys.argv:
        print(stage.apply())


main()
