import importlib, sys
from pathlib import Path
import numpy as np
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
folder = Path(sys.argv[1])
group = S._place(folder, 0, 1.0, np.zeros(3), None, 1.0)
before = [m.get_fill_opacity() for m in group.submobjects]
group.set_opacity(1.0)
after = [m.get_fill_opacity() for m in group.submobjects]
changed = [i for i, (a, b) in enumerate(zip(before, after)) if a == 0 and b > 0]
print("unfilled paths that gained a fill:", len(changed), "of", before.count(0))
