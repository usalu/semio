"""🌡️ Separates the open face and neck surfaces of the skater at the mandible and beanie."""
from pathlib import Path
import json
from runpy import run_path
ticket=Path(__file__).resolve().parent
path=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/skateboarder/*'))
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
old=path.read_text()
text=old
offset=text.index('"volumes": ')
volumes=json.loads(text)['asset']['volumes']
face_rows=[
 (.029,-.1236,.02,.015),(.041,-.116,.041,.03),(.054,-.108,.053,.041),(.075,-.098,.061,.054),
 (.099,-.098,.067,.056),(.125,-.092,.069,.055),(.151,-.092,.071,.055)]
neck_rows=[(.01,-.023,.056,.052),(.03,-.024,.055,.052),(.051,-.027,.051,.047),(.069,-.03,.046,.04)]
for scale in ('50','100','200','500'):
    face=next(s for s in volumes[scale]['thermal'] if s['id']=='th-face')
    q=face['sections'][0]['rotation']
    def ring(row):
        x,y,w,d=row
        return dict(at=[x,y,0],rx=w,ry=d,rotation=q)
    face['sections']=[ring(face_rows[i]) for i in {'50':range(7),'100':(0,1,2,4,6),'200':(0,2,4,6),'500':(0,2,6)}[scale]]
    neck={**face,'id':'th-neck','sectionFaces':['-u','+u'],'sections':[ring(neck_rows[i]) for i in ([0,1,2,3] if scale=='50' else [0,2,3])]}
    i=text.index('{\n            "id": "th-face"',offset)
    end=sp['member_end'](text,i)
    repl=',\n          '.join(sp['dump'](p,5,'compact') for p in [face,neck])
    text=text[:i]+repl+text[end:]
    offset=i+len(repl)
assert json.loads(text)
assert path.read_text()==old
path.write_text(text)
