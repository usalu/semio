"""🧑 Checks the skater's exposed skin shells for folded geometry and binding."""
from pathlib import Path
import json
path=next(Path.cwd().glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/skateboarder/*'))
for scale,v in json.loads(path.read_text())['asset']['volumes'].items():
    face=next(s for s in v['thermal'] if s['id']=='th-face')
    assert all(b['at'][0]>a['at'][0] for a,b in zip(face['sections'],face['sections'][1:])),scale+' face folds through the neck'
    neck=next(s for s in v['thermal'] if s['id']=='th-neck')
    assert face['bone']==neck['bone']=='head' and face['region']==neck['region']=='face'
