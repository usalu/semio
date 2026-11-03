import importlib
import numpy as np

v = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
cast = "cooling-heating-comparison-cooler"
unit = v.fitted_staffage(cast, width=0.9, height=1.5, rate=0.0)
unit.shift(np.array([-3.15, 0.0, 0.0]))
margin = 0.05
metres = unit.height / (v.staffage_height(cast) + 2 * margin)
lift = -unit.get_bottom()[1] - margin * metres
print("metres per unit", round(metres, 4), "lift", round(lift, 4), "expected", round(0.062609 * metres, 4))
