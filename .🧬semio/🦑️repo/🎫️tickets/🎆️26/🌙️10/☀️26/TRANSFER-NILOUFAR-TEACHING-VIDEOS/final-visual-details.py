"""🔍 Record final sample timestamps and verify adjacent authored build transitions."""
from pathlib import Path
import json
import subprocess
import av
from PIL import Image
TICKET=Path(__file__).resolve().parent
ROOT=TICKET/'🗑️generated/g/rendered'
OUT=TICKET/'🗑️generated/corrected-final-previews'
rows=[]
for movie in sorted(ROOT.glob('*.mp4')):
    info=json.loads(subprocess.check_output([str(TICKET/'🗑️generated/f/ffprobe.exe'),'-v','error','-show_format','-of','json',str(movie)],text=True,encoding='utf-8'))
    duration=float(info['format']['duration'])
    targets=[(.1,0),(.5,0),(.9,0)]
    if 'Physical' in movie.name or 'Ventilation' in movie.name: targets.append((.9,5))
    if 'EnergyBalance' in movie.name: targets.extend([(.1,5),(.5,5)])
    with av.open(str(movie)) as container:
        stream=container.streams.video[0]
        for fraction,offset in targets:
            target=duration*fraction+offset
            container.seek(int(target/stream.time_base),stream=stream)
            for frame in container.decode(stream):
                if frame.time>=target:
                    name=f'{movie.stem}-{fraction:.1f}-{offset}.png'
                    if offset or ('Physical' in movie.name and fraction==.9): frame.to_image().save(OUT/name)
                    rows.append({'movie':movie.name,'duration':duration,'fraction':fraction,'offset':offset,'target':target,'actual':float(frame.time),'image':name if offset or ('Physical' in movie.name and fraction==.9) else None})
                    break
            else: raise RuntimeError('No frame at '+str(target))
(OUT/'frames.json').write_text(json.dumps(rows,indent=2),encoding='utf-8')
print('[DEBUG] Recorded '+str(len(rows))+' decoded sample timestamps')
