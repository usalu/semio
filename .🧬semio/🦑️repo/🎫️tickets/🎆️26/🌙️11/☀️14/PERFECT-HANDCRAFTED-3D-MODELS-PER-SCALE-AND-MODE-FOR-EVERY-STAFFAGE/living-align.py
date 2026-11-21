"""🐾 Restore the pig's lateral bone frames and coarse thermal anatomy of sheep, goat and pigeon."""
import copy
import json
import sys
from pathlib import Path

catalog = next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog'))
for slug in sys.argv[1:]:
    path = next(catalog.glob(f'*/{slug}/*.json'))
    raw = path.read_text()
    asset = json.loads(raw)['asset']
    volumes = asset['volumes']
    if slug == 'pig':
        first = next(s for s in volumes['50']['body'] if s['id'] == 'near-gaskin')
        if first['sections'][0]['at'][2] < -0.075:
            offsets = {}
            for bone in asset['rig']['bones']:
                offsets[bone['id']] = offsets.get(bone['parent'], 0) + bone.get('z', 0)
            for volume in volumes.values():
                for mode in ('body', 'thermal', 'structure'):
                    for s in volume[mode]:
                        z = offsets.get(s.get('bone'), 0)
                        if not z:
                            continue
                        if 'sections' in s and not s.get('transform'):
                            for sec in s['sections']:
                                sec['at'][2] = round(sec['at'][2] - z, 6)
                        else:
                            pos = s.setdefault('transform', {}).setdefault('translation', [0, 0, 0])
                            pos[2] = round(pos[2] - z, 6)
    if slug in ('sheep','goat','pig'):
        detailed = {s['id']:s for s in volumes['100']['body']}
        for scale in (('200', '500') if slug=='goat' else ('500',)):
            thermal = volumes[scale]['thermal']
            for i,s in enumerate(thermal):
                part = s['id'].removeprefix('th-')
                if s.get('region') != 'hooves' or part not in detailed:
                    continue
                horn = copy.deepcopy(detailed[part])
                if scale == '500':
                    sections = horn['sections']
                    horn['sections'] = [sections[0], sections[1], sections[-1]]
                thermal[i] = {**horn,'id':s['id'],'region':'hooves'}
    if slug == 'pigeon':
        detailed = {s['id']:s for s in volumes['100']['body']}
        for scale in ('200','500'):
            thermal = volumes[scale]['thermal']
            for name in ('beak','cere','eye'):
                position = next((i for i,s in enumerate(thermal) if s['id']==f'th-{name}-head'),None)
                if position is None:
                    continue
                parts=[]
                for part in (('eye','eye-f') if name=='eye' else (name,)):
                    shape = copy.deepcopy(detailed[part])
                    if scale=='500' and 'sections' in shape:
                        secs=shape['sections']
                        shape['sections']=[secs[0],secs[len(secs)//2],secs[-1]] if len(secs)>3 else secs
                    parts.append({**shape,'id':'th-'+part,'region':name})
                thermal[position:position+1] = parts
    start=raw.index('{',raw.index('"volumes":'))
    old,n=json.JSONDecoder().raw_decode(raw[start:])
    if old != volumes:
        replacement=json.dumps(volumes,ensure_ascii=False,indent=2).replace('\n','\n    ')
        if path.read_text()!=raw:
            raise SystemExit(f'{slug} changed concurrently')
        path.write_text(raw[:start]+replacement+raw[start+n:])
        print('[DEBUG] updated',slug)

