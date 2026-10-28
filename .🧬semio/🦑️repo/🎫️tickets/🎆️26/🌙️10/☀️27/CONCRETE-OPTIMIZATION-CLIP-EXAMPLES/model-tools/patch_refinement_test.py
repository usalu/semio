from pathlib import Path
path = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧪️tests/🐍️.py")
text = path.read_text(encoding="utf-8")
old = '''    def test_refinement_is_an_off_grid_design_that_dominates_a_sampled_front_member(self):
        refinement = self.result["refinement"]
'''
new = '''    def test_refinement_is_an_off_grid_design_that_dominates_a_sampled_front_member(self):
        import numpy as np

        refinement = self.result["refinement"]
        dense = np.array(self.api.surface()["electricity"])
        i, j = np.unravel_index(int(np.argmin(dense)), dense.shape)
        surface = self.api.surface()
        optimum = {**surface["design"], "wwr": surface["wwr"][i], "overhang": surface["overhang"][j]}
        self.assertEqual(refinement["design"], optimum)
        self.assertEqual(refinement["design"], surface["minimum"]["design"])
        self.assertAlmostEqual(refinement["electricity"], dense.min())
        for key in ("orientation", "insulation"):
            self.assertEqual(refinement["design"][key], self.api.load_study()["surface"]["design"][key])
        costs = np.array([item["cost"] for item in self.result["designs"]])
        energies = np.array([item["electricity"] for item in self.result["designs"]])
        beaten = costs > refinement["cost"]
        beaten &= energies > refinement["electricity"]
        self.assertEqual({item["id"] for item, flag in zip(self.result["designs"], beaten) if flag and item["id"] in self.result["front"]}, set(refinement["dominates"]))
'''
assert text.count(old) == 1
path.write_text(text.replace(old, new), encoding="utf-8")
