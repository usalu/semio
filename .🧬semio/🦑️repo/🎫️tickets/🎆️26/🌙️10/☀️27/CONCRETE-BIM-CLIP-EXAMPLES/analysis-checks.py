"""✅ Assertions on the analysis widgets against the model numbers (no pixels): counts, arrow lengths, classes, parts, anchors, crop.

Usage: python analysis-checks.py <en|de>
"""
import importlib
import os
import sys
from collections import Counter
from pathlib import Path

import numpy as np

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
language = sys.argv[1]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/fonts/analysis-{language}")
OUTPUT = TICKET / "🗑️generated/analysis"
manim = importlib.import_module("manim")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
module = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🧮️analysis.🐍️")
checks = []


def check(name, condition):
    checks.append((name, bool(condition)))
    print(("[DEBUG] ok   " if condition else "[DEBUG] FAIL ") + name, flush=True)


def render():
    class Checks(visuals.ClipScene):
        TITLE = {"en": "Checks", "de": "Prüfungen"}
        NARRATION = [("t", "t", "t")]

        def construct(self):
            fonts.apply_scene_style(self)
            a, t = model.translate_structure(), model.tributary()
            for projection in ("plan", "axon"):
                v = self.analysis_view(a, (0, 0, 0), 0.2, projection=projection)
                check(f"{projection}: {len(a['shells'])} shell parts", all(s["id"] in v.parts for s in a["shells"]))
                check(f"{projection}: {len(a['nodes'])} node parts", all(n["id"] in v.parts for n in a["nodes"]) and len(v.nodes) == len(a["nodes"]))
                check(f"{projection}: 25 member parts by tag and id", all(p["tag"] in v.parts and "M-" + p["tag"] in v.parts for p in model.building_geometry()["pilotis"]))
                check(f"{projection}: 25 supports", len(v.supports) == len(a["supports"]) == 25 and "B3.support" in v.parts and "F-B3.support" in v.parts)
                check(f"{projection}: slab tag addresses all shells", v.parts["SL-1"] is v.shells and len(v.shells) == 24)
                nodes = {n["id"]: n["position"] for n in a["nodes"]}
                check(f"{projection}: at(tag) is the member middle", np.allclose(v.at("B3"), (v.at(*nodes["F-B3"]) + v.at(*nodes["N-B3"])) / 2))
            plan = self.analysis_view(a, (0, 0, 0), 0.2)
            axon = self.analysis_view(a, (0, 0, 0), 0.2, projection="axon")
            top = np.array(next(n["position"] for n in a["nodes"] if n["id"] == "N-B3"))
            foot = np.array(next(n["position"] for n in a["nodes"] if n["id"] == "F-B3"))
            check("plan: top and foot nodes coincide on screen", np.allclose(plan.at(*top), plan.at(*foot)))
            check("axon: top node is above the foot node by 3.375 m x scale", np.isclose((axon.at(*top) - axon.at(*foot))[1], 0.2 * 3.375 * np.cos(np.radians(28))) and np.isclose((axon.at(*top) - axon.at(*foot))[0], 0))
            box = model.building_geometry()["box"]
            ref = self.axon_point(*top, (0, 0, 0), 0.2)
            check("axon projection equals villa_axon's axon_point", np.allclose(axon.at(*top), ref))
            check("window crop keeps only the north-west part", len(self.analysis_view(a, (0, 0, 0), 0.2, window=(0, 7.0, 0, 6.0)).shells) < 24)

            stale = self.analysis_view(a, (0, 0, 0), 0.2, stale=True)
            check("stale carries a revision badge with the source revision", len(stale.tag) > 0 and len(self.analysis_view(a, (0, 0, 0), 0.2).tag) == 0)

            unit = 0.004
            arrows = self.load_arrows([100.0, 220.0, -50.0], [(0, 0, 0)] * 3, unit)
            lengths = [np.linalg.norm(tip - tail) for tip, tail in zip(arrows.tips, arrows.tails)]
            check("arrow length = |value| x scale", np.allclose(lengths, [0.4, 0.88, 0.2]))
            check("negative value flips the arrow", arrows.tips[2][1] > arrows.tails[2][1] and arrows.tips[0][1] < arrows.tails[0][1])
            tipped = self.load_arrows([100.0], [(0, 1, 0)], unit, anchor="tip", direction=(0, 1))
            check("anchor tip puts the head on the point", np.allclose(tipped.tips[0], (0, 1, 0)) and np.isclose(tipped.tails[0][1], 0.6))
            row = {p["tag"]: p["reaction"] for p in t["pilotis"] if p["tag"] in ("C1", "C2", "C3")}
            v = self.analysis_view(a, (0, 0, 0), 0.2, projection="axon", loads=row)
            check("reaction arrows: one part per mapped piloti", all(k + ".load" in v.parts for k in row) and len(v.loads) == 3)
            q = self.analysis_view(a, (0, 0, 0), 0.2, projection="axon", loads=9.75)
            check("area load: one arrow per shell", len(q.loads) == 24)

            kinds = Counter(self.tributary_view(t, (0, 0, 0), 0.2).kinds.values())
            check("tributary classes: 4 corner, 6 cantilever, 6 edge, 9 interior", dict(kinds) == {"corner": 4, "cantilever": 6, "edge": 6, "interior": 9})
            tv = self.tributary_view(t, (0, 0, 0), 0.2, highlight=("corner",))
            check("tributary: 25 cell parts and class highlight", all(p["tag"] in tv.parts for p in t["pilotis"]) and len(tv.cells) == 25)
            check("tributary: at(tag) is the piloti position", np.allclose(tv.at("B3"), tv.view(10.75, 4.75, 3.375)))
            roof = model.tributary(level="roof")
            rv = self.tributary_view(roof, (0, 0, 0), 0.2, projection="axon")
            first = self.tributary_view(t, (0, 0, 0), 0.2, projection="axon")
            check("roof level comes from the tributary result and lifts the plane", np.isclose(rv.at("C3")[1] - first.at("C3")[1], 0.2 * (6.7 - 0.125 - 3.375) * np.cos(np.radians(28)), atol=0.02))
            mesh = model.mesh("B3", 8)
            mv = self.mesh_view(mesh, (0, 0, 0), 1.0, view="top")
            check("mesh top: stats equal the model counts", mv.stats == (mesh["vertex_count"], mesh["triangle_count"]))
            check("mesh top: only the cap fan is visible", len(mv.triangles) == 8)
            side = self.mesh_view(mesh, (0, 0, 0), 1.0, view="side")
            check("mesh side: front half of the wall (16 triangles / 2)", len(side.triangles) == 8)
            check("mesh segments re-tessellate", self.mesh_view(mesh, (0, 0, 0), 1.0, segments=16).stats == (34, 64))
            check("mesh by tag", self.mesh_view("B3", (0, 0, 0), 1.0, segments=12).stats == (26, 48))

    with manim.tempconfig({"pixel_width": 640, "pixel_height": 360, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False, "disable_caching": True,
                           "skip_animations": True, "media_dir": str(OUTPUT / f"media-checks-{language}"), "verbosity": "ERROR"}):
        Checks().render()


render()
failed = [name for name, ok in checks if not ok]
print(f"[DEBUG] {len(checks) - len(failed)} of {len(checks)} checks passed", flush=True)
sys.exit(1 if failed else 0)
