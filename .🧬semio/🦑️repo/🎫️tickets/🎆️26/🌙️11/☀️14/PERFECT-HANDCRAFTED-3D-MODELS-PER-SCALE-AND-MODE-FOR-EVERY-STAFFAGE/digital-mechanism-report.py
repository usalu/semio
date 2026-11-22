"""🔧 Record the installed wheel, track and scissor geometry and independent export results."""
from pathlib import Path
from hashlib import sha256
import sys,json,re
t=Path(__file__).resolve().parent;g=t/'🗑️generated';r=t.parents[6];notes={
'concrete-mixer-truck':('haul','The two steering axles have single tyres. The rear two driven axles have inner and outer tyres and distinct hubs on both sides. All twelve tyres carry their own structural and thermal surfaces.'),
'forklift':('haul','Four solid industrial tyres and their rims carry the mast and chassis at every scale. The hub retainers have individually chamfered wrench flats. The chassis and wheel-metal exchange area measured 14.921 m².'),
'excavator':('crawler','Both shoe belts retain their open track loop and eight lower rollers per side. The paired idler yoke cheeks have 2 mm radial clearance about their transverse pin. The adjuster rod enters its gland.'),
'scissor-lift':('link','All eight lifting arms have opposing 6 mm walls, three transverse bores and a 40.5 mm bore in each sleeve for a 40 mm pin. The physical and structural crossing links stay separate throughout the lift. The closed arm and sleeve area measured 14.681–14.727 m² in the four poses. Air reaches 13.907–13.946 m².'),
'motor-scooter':('haul','Both cast wheels retain their five windows. The fork, two controls, curved brake levers and hollow steering tube follow the steering joint. The grooved rubber grips have a measured component area of 0.05032 m² and 0.02990 m² exchanges with air in the eleven authored postures.'),
'electric-kick-scooter':('lacing','The rear wheel has five openings and two 14 × 32 × 9 mm bearing cartridges on the axle. The front wheel keeps its hub motor, separate from the rear disc brake.'),
'city-bicycle':('lacing','Each wheel has 36 individually positioned crossed spokes with hub elbows and through-bored nipples. The rear rack retains its bolted stays.'),
'cargo-bike':('lacing','The small front and larger rear wheels use separate flange widths and 36 crossed spokes each. The box beam, forked rear carrier and wheel metal retain their own geometry and thermal areas.')}
revisions={
'scissor-lift':('joint','The working platform and the extension deck retain their separate posts, perimeter rails and gate in the load-path view.'),
'motor-scooter':('joint','Two clamping halves, a bored steerer socket and twin pinch bolts join the 22 mm handlebar to the steering column.'),
'excavator':('cab','The actual curved bucket shell, twin cheeks, lip and five digging teeth transmit load to the pinned bucket bracket at all four scales.'),
'concrete-mixer-truck':('cab','The steel cab has punched apertures for the windscreen, both door lights and five cooling slats. The driver seat, passenger seat, instrument panel and steering wheel sit inside the cab.') }
rows=[]
for name,(study,detail) in notes.items():
 if name in revisions:
  latest,extra=revisions[name];rec=g/f'published-{latest}-{name}.json';src=next(r.glob('🎓*/*/*staffage/*catalog/*/'+name+'/*.json'))
  if rec.is_file() and json.loads(rec.read_bytes())['source']==sha256(src.read_bytes()).hexdigest():study,detail=latest,detail+' '+extra
 p=next(r.glob('🎓*/*/*staffage/*catalog/*/'+name+'/*.json'));h=sha256(p.read_bytes()).hexdigest();receipt=g/f'published-{study}-{name}.json'
 if not receipt.is_file() or json.loads(receipt.read_bytes()).get('source')!=h:continue
 data=next((json.loads(l)for l in (g/f'{study}-{name}-glb.log').read_text().splitlines() if l.startswith('{')),{});assert data.get('findings')==[] and data['scenes']==12
 rows.append('| '+name+' | '+detail+' | '+f"{data['bytes']:,}"+' | `'+h[:16]+'` |')
 single=next(t.glob('r3-*-'+name+'.md'),None)
 if single:
  before=single.read_text();marker='\n## November 21 running gear and moving joints\n';block=marker+'\n'+detail+' See the [measurements and installed revision](digital-running-gear.md).\n'
  rest=before.split(marker,1)[1] if marker in before else ''
  suffix=('\n## '+rest.split('\n## ',1)[1]) if '\n## ' in rest else ''
  single.write_text(before.split(marker,1)[0].rstrip()+'\n'+block+suffix)
(t/'digital-running-gear.md').write_text('# Running gear and moving joints\n\nEach installed catalog revision below passed all thirteen volume checks at each of the four scales, document and print validation, and independent geometry and GLB readback across twelve scenes. Wheel and linkage dimensions are in model space. The scissor and excavator regressions additionally probe their through-bores with Three.js.\n\n| Model | Installed geometry | Twelve-scene GLB bytes | SHA-256 prefix |\n| --- | --- | ---: | --- |\n'+'\n'.join(rows)+'\n')
