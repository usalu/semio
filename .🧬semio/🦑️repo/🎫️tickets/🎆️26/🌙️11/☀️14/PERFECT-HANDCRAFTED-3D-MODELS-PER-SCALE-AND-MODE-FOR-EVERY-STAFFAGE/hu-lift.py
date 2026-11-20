"""🏋 Reconstructs the worker's squat from the planted boots, forward pelvic tilt and the box's two hand holes."""
from pathlib import Path
from runpy import run_path
import json
import r3_worker_carrying_box_ik as ik
import r3_worker_carrying_box_kit as kit
ticket=Path(__file__).resolve().parent
path=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/worker-carrying-box/*'))
old=path.read_text()
a=json.loads(old)['asset']
pose=next(p for p in a['poses'] if p['id']=='squat-grip')
bones=pose['bones']
bones['torso']['angleDeg']=-28
for side in ('l','r'):bones['thigh-'+side]['angleDeg']=110
rest={b:dict(angleDeg=o['angleDeg']) for b,o in bones.items() if not b.startswith(('upper-arm','forearm','hand','box'))}
toe=kit.grounded(a,rest,ik.FEET)['toe-l']
length=next(b['lengthM'] for b in a['rig']['bones'] if b['id']=='toe-l')
tip=toe['o'][0]+length*toe['u'][0]
bones.update(ik.solve_arms(a,rest,(round(tip+.17,3),.235,.217),14,3))
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
i=old.index('"bones": {',old.index('"id": "squat-grip"',old.index('"poses": [')))+len('"bones": ')
j=sp['member_end'](old,i)
text=old[:i]+sp['dump'](bones,4,'compact')+old[j:]
assert json.loads(text)['asset']==a
assert path.read_text()==old
path.write_text(text)
