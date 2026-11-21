"""🌳 Form the supporting willow switches and keep the Scots pine's crown leader through its uppermost needle pad."""
from pathlib import Path
from copy import deepcopy
import json
import math
ticket=Path(__file__).resolve().parent
def willow(asset):
    switches={'w1':(.018,.17,-.12,.08),'w2':(.020,-.11,.08,.15),'w3':(.017,.13,.12,-.09),'w4':(.016,-.12,-.06,.13),'e1':(.017,-.16,.07,-.08),'e2':(.019,.12,-.11,.13),'e3':(.016,-.08,.10,-.15),'e4':(.015,.14,-.05,.07)}
    for scale,v in asset['volumes'].items():
        for name,(rad,sweep,turn,drift) in switches.items():
            member=next(s for s in v['structure']if s['id']=='member-curt-'+name)
            length=member['sections'][-1]['at'][0]
            wood=dict(id='switch-'+name,kind='loft',bone='curt-'+name,material='willow-bark',role='body',tone=.41,capStart=True,capEnd=True,sections=[],slender=True)
            profile=[(0,0,0,1),(.12,sweep*.46,turn*.18,.83),(.28,sweep,turn*.51,.64),(.48,sweep*.74,turn,.43),(.66,sweep*.30,turn*.71,.28),(.84,-sweep*.13,drift*.82,.15),(1,-sweep*.27,drift,.075)]
            for progress,y,z,taper in profile:
                wood['sections'].append(dict(at=[round(length*progress,6),round(y,6),round(z,6)],rx=round(rad*taper,6),ry=round(rad*taper*.93,6),rotation=[.70710678,0,.70710678,0]))
            keep={'50':range(7),'100':(0,1,2,3,5,6),'200':(0,2,3,5,6),'500':(0,2,3,6)}[scale]
            wood['sections']=[wood['sections'][i] for i in keep]
            v['body'].append(wood);v['thermal'].append(dict(id='th-'+wood['id'],of=wood['id'],region='limb-bark'))
            member.clear();member.update(id='member-curt-'+name,of=wood['id'],member=wood['bone'],part='member')
def pine(asset):
    coarse=asset['volumes']['500'];medium=asset['volumes']['200']
    for member in coarse['structure']:
        if not member['id'].startswith('member-pl-') or not member['id'].endswith('-axis'):continue
        key=member['id'][7:]
        if any(s['id']==key for s in coarse['body']):continue
        axis=next((s for s in medium['body'] if s['id']==key),None)
        if axis is None:continue
        coarse['body'].append(deepcopy(axis));coarse['thermal'].append(dict(id='th-'+key,of=key,region='limb-bark'))
        bone=member['bone'];name=member['id'];member.clear();member.update(id=name,of=key,part='member',member=bone)
if __name__=='__main__':
    for slug,author in [('weeping-willow',willow),('scots-pine',pine)]:
        path=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/'+slug+'/🔣️.json'))
        doc=json.loads(path.read_text());author(doc['asset'])
        (ticket/'🗑️generated'/('digital-'+slug+'-wood.json')).write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
