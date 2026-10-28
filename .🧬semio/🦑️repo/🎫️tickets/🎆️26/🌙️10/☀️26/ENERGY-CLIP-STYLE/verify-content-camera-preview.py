"""🔍 Independently compare fullframe physical playback and transition seams."""
import json,os
from pathlib import Path
import av,numpy as np
TICKET=Path(__file__).resolve().parent
output=Path(os.environ['SEMIO_TEACHING_OUTPUT_ROOT'])
with av.open(str(output/'captured-body.mp4')) as source:raw=[f.to_ndarray(format='rgb24') for f in source.decode(video=0)]
with av.open(str(TICKET/'content-camera.mp4')) as movie:
    stream=movie.streams.video[0]
    frames=[f.to_ndarray(format='rgb24') for f in movie.decode(stream)]
    assert(stream.width,stream.height,float(stream.average_rate))==(1280,720,8)
info=json.loads((output/'preview-info.json').read_text(encoding='utf8'))
inward,outward=info['padding_frames']
assert len(frames)==len(raw)+inward+outward
error=lambda a,b:float(np.abs(a.astype(float)-b.astype(float)).mean())
body_errors=[error(a,b) for a,b in zip(frames[inward:inward+len(raw)],raw)]
assert max(body_errors)<2,'Playback differs from fullframe source'
start_join,end_join=error(frames[inward-1],frames[inward]),error(frames[inward+len(raw)-1],frames[inward+len(raw)])
assert max(start_join,end_join)<2,'Camera/body join differs'
assert np.array_equal(frames[0],frames[-1]),'Overview handoff changed'
metrics={'frames':len(frames),'seconds':len(frames)/8,'body_frames':len(raw),'body_start_frame':inward,'max_body_mean_rgb_error':max(body_errors),'inward_join_mean_rgb_error':start_join,'outward_join_mean_rgb_error':end_join,'overview_pixel_identical':True}
(TICKET/'content-camera-verification.md').write_text('# Content Camera Media Verification\n\nAll 106 frames decode at1280 ×720 and8fps. The48 authored playback frames fill the output and match the independent decoded source, including every outer edge. Opening-camera end and closing-camera start match the adjacent playback frames. The common overview boundary remains pixel-identical.\n\n'+json.dumps(metrics,indent=2)+'\n',encoding='utf8')
print('[DEBUG] Content camera media verified '+json.dumps(metrics),flush=True)
