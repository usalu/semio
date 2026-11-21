"""🍃 Model each attached ivy blade as a tapered lamina with its original basal sinus, lateral lobes and apical point."""
import runpy
from pathlib import Path
ticket=Path(__file__).parent
botany=runpy.run_path(str(ticket/'living-botanical-shells.py'))
edit=runpy.run_path(str(ticket/'living-shape-rows.py'))['edit']
def lamina(scale,mode,s):
    if scale not in ('50','100') or mode!='body' or s.get('material')!='ivy-leaf' or s.get('kind')!='extrude' or not s['id'].startswith('leaf-'):return False
    profile=s['profile'];root=profile[0];tip=profile[len(profile)//2]
    centre=[root[i]+.34*(tip[i]-root[i]) for i in (0,1)]
    half=(s['to']-s['from'])/2
    middle=s['from']+half
    margin=[]
    for j,(x,y) in enumerate(profile):
        i=min(j,len(profile)-j)
        z=(0,.18,.32,.16,-.11,.12,.04,-.18)[i] if len(profile)==14 else (0,.28,.14,.03,-.18)[i]
        margin.append([x,y,middle+half*z])
    turns=[(b[0]-a[0])*(centre[1]-a[1])-(b[1]-a[1])*(centre[0]-a[0]) for a,b in zip(profile,profile[1:]+profile[:1])]
    if not all(t<0 for t in turns) and not all(t>0 for t in turns):raise ValueError((s['id'],turns))
    botany['mesh'](s,[[[*centre,s['from']]],margin,[[*centre,s['to']]]])
    return True
for path in (next(botany['root'].glob('*/ivy-on-trellis/*.json')),ticket/'volumes/vegetation/ivy-on-trellis.json'):edit(path,lamina)
