#!/usr/bin/env python3
"""⚙️ Re-homes engine tests whose subject sits above their module (lane P1, slice S1a).

- `🧠️precompute` tests that build BESTEST geometry and the `🧪️sim` design-day window test move into `🏛️bestest`'s tests
  (bestest owns the case models; precompute and sim stay below it).
- `🎬️scene`'s BESTEST 600 test builds the case through `crate::bestest` instead of the artifact's example module.
- `🏛️bestest`'s EnergyPlus comparison and committed-fixture/asset laws read the artifact's fixture tree and DSL assets,
  so they move to the artifact's examples tests (`📚️examples/🧪️tests/🔬️bestest-catalog/🦀️.rs`).
Anchors are asserted unique; refuses to run twice.
"""
import sys
E = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/"
ANY = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/"
def rd(p): return open(p, encoding="utf-8").read()
def wr(p, s): open(p, "w", encoding="utf-8").write(s)
def take(s, start, end):
    assert s.count(start) == 1, start[:80]; i = s.index(start); j = s.index(end, i) + len(end); return s[:i] + s[j:], s[i:j]
pre_p, sim_p, scene_p, best_p = (E + x + "/🧪️tests/🔬️unit/🦀️.rs" for x in ("🧠️precompute", "🧪️sim", "🎬️scene", "🏛️bestest"))
pre, sim, scene, best = rd(pre_p), rd(sim_p), rd(scene_p), rd(best_p)
if "//#region 🧠️PrecomputeOnCases" in best: sys.exit("already applied")
pre, moved_pre = take(pre, "/// 🐛️ Floor-area defect: the zone floor area", "    assert!(participation.iter().all(|p| *p > 0.0));\n}\n")
sim, moved_sim = take(sim, "#[test]\nfn a_window_row_carries_transmitted_solar_on_a_sunny_design_day() {", "the sun-exposed opaque surfaces absorb solar on their outside face too\");\n}\n")
pre = pre.rstrip("\n") + "\n"; sim = sim.replace("\n\n\n", "\n\n")
a = "    let model = crate::examples::bestest_600::model();"; assert scene.count(a) == 1
scene = scene.replace(a, "    let model = crate::bestest::model(\"600\").expect(\"ANSI/ASHRAE 140 §5.2 case 600 is registered\");")
best, eplus = take(best, "//#region 🔮️EnergyPlusComparison", "//#endregion 🔮️EnergyPlusComparison\n")
best, fixtures = take(best, "//#region 🧫️Fixtures", "//#endregion 🧫️Fixtures\n")
best = best.rstrip("\n") + "\n\n//#region 🧠️PrecomputeOnCases\n" + moved_pre + "//#endregion 🧠️PrecomputeOnCases\n\n//#region ☀️DesignDaySolar\n" + moved_sim + "//#endregion ☀️DesignDaySolar\n"
a = "use super::*;\n"; assert best.count(a) == 1
best = best.replace(a, "use super::*;\nuse crate::geometry::surface_area_m2;\nuse crate::kernel::SimulationEnvironment;\nuse crate::precompute::{enclosure_factors, mean_radiant_participation, EnclosureRadiation, PrecomputedModel};\nuse crate::sim::Engine;\n", 1)
catalog = ("//! 🏛️ The bundled ANSI/ASHRAE 140 §5.2 catalogue against this subset's committed data: the EnergyPlus comparison over\n"
           "//! the committed Denver TMY year, and the laws that keep the committed case fixtures and example DSL assets equal to\n"
           "//! what `crate::bestest` builds today.\n\n"
           "use crate::bestest::{is_free_float, model, model_json, project, run, CASES};\n"
           "use crate::model::Model;\n\n" + eplus + "\n" + fixtures)
import os
os.makedirs(ANY + "📚️examples/🧪️tests/🔬️bestest-catalog", exist_ok=True)
wr(ANY + "📚️examples/🧪️tests/🔬️bestest-catalog/🦀️.rs", catalog)
wr(pre_p, pre); wr(sim_p, sim); wr(scene_p, scene); wr(best_p, best)
print("ok")
