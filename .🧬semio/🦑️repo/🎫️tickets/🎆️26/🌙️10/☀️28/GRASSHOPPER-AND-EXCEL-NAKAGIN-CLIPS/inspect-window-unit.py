"""🪟 Record actual fixed-pane, fan and closed inner-pane depth intervals for every physical family."""
import importlib
import json
from pathlib import Path
import sys
import numpy as np

root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(root))
M=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🐍️')
rows=[]
for family in M.capsule_families():
    normal=np.asarray(M.capsule_window('A',family).normal)
    row={'family':family,'depth_axis':normal.tolist()}
    for coverage in (0,.25,.5,.75,1):
        parts={p.key:p for p in M.capsule_window_unit('A',family,M.WindowUnitState(0,coverage,0)).parts}
        values={}
        for key in ('window-glazing','window-sash-glazing','window-paper-fan'):
            depth=np.asarray(parts[key].solid.vertices)@normal
            values[key]=[float(depth.min()),float(depth.max())]
        row[str(coverage)]=values
    rows.append(row)
output=Path(__file__).parent/'🗑️generated/window-unit-depths.json'
output.write_text(json.dumps(rows,ensure_ascii=False,indent=2),encoding='utf8')
print('[DEBUG] '+json.dumps(rows,ensure_ascii=True),flush=True)
