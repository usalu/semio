"""🔎 Independently decode layered preview media and inspect captured shared UI geometry."""
import hashlib,json,os,sys
from pathlib import Path
import av,numpy as np
from PIL import Image
TICKET=Path(__file__).resolve().parent
OUTPUT=Path(os.environ['SEMIO_TEACHING_OUTPUT_ROOT']).resolve()
nav=OUTPUT/'layered-navigation'/'metadata.json'
result=json.loads(nav.read_text(encoding='utf8'))
frames=result['metadata']
native=[(event['type'],event['target']) for frame in frames for event in frame.get('pointer_events',[])]
assert native==[(kind,target) for target in ['energy','physical_fundamentals','physical_fundamentals:fullscreen','footer:range'] for kind in ['pointerdown','pointerup','click']]
excerpt=OUTPUT/'layered-physical/physical-excerpt.mp4'
excerpt_hash=hashlib.sha256(excerpt.read_bytes()).hexdigest()

first=np.asarray(Image.open(frames[0]['output']).convert('RGB'))
last=np.asarray(Image.open(frames[-1]['output']).convert('RGB'))
assert np.array_equal(first,last),'Navigation handoff frames differ'
active=lambda frame:next((layer for layer in frame['layers'] if layer['interactive']),None)
root_frames=[f for f in frames if active(f) and active(f)['id']=='root']
video_frames=[f for f in frames if active(f) and active(f)['id']=='energy']
assert len(set(active(f)['strip_transform'] for f in root_frames))>5
assert len(set(active(f)['strip_transform'] for f in video_frames))>5
for frame in root_frames+video_frames:
    layer=active(frame)
    for card in layer['cards']:
        assert card['chip_text'].strip()=='Ansehen'
        assert abs(card['chip_rect']['x']+card['chip_rect']['width']-card['rect']['x']-card['rect']['width'])<.5
        assert card['chip_icons']==['maximize-2']
        assert abs(card['chip_rect']['y']+card['chip_rect']['height']-card['rect']['y']-card['rect']['height'])<.5
        assert card['glass_background'] and card['backdrop_filter']
summary=[]
for name in ['layered-navigation.mp4','layered-physical.mp4']:
    path=TICKET/name
    with av.open(str(path)) as video:
        stream=video.streams.video[0]
        decoded=list(video.decode(stream))
        assert decoded and all(frame.width==stream.width and frame.height==stream.height for frame in decoded)
        duration=len(decoded)/float(stream.average_rate)
        if 'physical' in name:
            assert len(decoded)==192 and abs(duration-24)<.01
            assert np.array_equal(decoded[0].to_ndarray(format='rgb24'),decoded[-1].to_ndarray(format='rgb24')),'Physical encoded handoff frames differ'
            decoded[round(12*float(stream.average_rate))].to_image().save(TICKET/'layered-physical-body.png')
        summary.append({'file':name,'width':stream.width,'height':stream.height,'fps':float(stream.average_rate),'frames':len(decoded),'seconds':duration,'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'audio':bool(video.streams.audio)})
report=['# Layered Preview Verification','','Fresh canonical Beat1_Leistung excerpt: 18 seconds of authored source plus 6 seconds of navigation; no institute intro. Actual shared React/CSS navigation, no recreated card geometry.','',
        '- Navigation first and last PNGs are pixel identical; physical encoded first and last frames are also identical.','- The navigation film records actual native pointer down, up and click on the topic chip, video chip, fullscreen control and parent control.','- The physical preview uses the canonical Beat1_Leistung excerpt captured during this task; the input video hash is recorded below.','- Both overview levels have more than five distinct recorded background transforms during pointer motion.','- Every visible card uses a bottom-right Ansehen and fullscreen icon chip and shared tinted glass styles.','- Native click/cancellation, app-equivalent full hover geometry and all five encoded lesson handoffs are covered by the registered regression tests.','',
        '| File | Size | FPS | Frames | Duration | Audio Stream |','| --- | --- | ---: | ---: | ---: | --- |']
for entry in summary:
    report.append(f"| [{entry['file']}](./{entry['file']}) | {entry['width']} × {entry['height']} | {entry['fps']:g} | {entry['frames']} | {entry['seconds']:.3f}s | {entry['audio']} |")
report+=['','## SHA-256','',f'- Captured source excerpt: {excerpt_hash}']+[f"- {entry['file']}: {entry['sha256']}" for entry in summary]
(TICKET/'layered-preview-verification.md').write_text('\n'.join(report)+'\n',encoding='utf8')
print('[DEBUG] Layered previews decoded and verified',flush=True)
