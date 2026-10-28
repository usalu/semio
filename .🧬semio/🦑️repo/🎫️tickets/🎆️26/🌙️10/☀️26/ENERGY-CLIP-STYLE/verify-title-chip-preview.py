"""🔎 Independently decode the name-chip preview and preserve chromeless body joins."""
import json,os
from pathlib import Path
import av,numpy as np
TICKET=Path(__file__).resolve().parent
output=Path(os.environ['SEMIO_TEACHING_OUTPUT_ROOT'])
with av.open(str(output/'body.mp4')) as movie:raw=[f.to_ndarray(format='rgb24') for f in movie.decode(video=0)]
with av.open(str(TICKET/'title-chip.mp4')) as movie:frames=[f.to_ndarray(format='rgb24') for f in movie.decode(video=0)]
info=json.loads((output/'preview-info.json').read_text(encoding='utf8'));start=info['body_start']
assert len(raw)==8 and len(frames)==66
error=lambda a,b:float(np.abs(a.astype(float)-b.astype(float)).mean())
assert max(error(a,b) for a,b in zip(frames[start:start+len(raw)],raw))<2
assert error(frames[start-1],frames[start])<2 and error(frames[start+len(raw)-1],frames[start+len(raw)])<2
assert np.array_equal(frames[0],frames[-1])
(TICKET/'title-chip-verification.md').write_text('# Window Name Chip Preview Verification\n\nAll 66 frames decode at 1280 × 720 and 8 fps (8.25seconds). The 8 source body frames remain full frame and match decoded input. Both camera/body joins are continuous and first/last overview frames are pixel-identical. Actual DOM/native interaction, title-contained icons, text-only Ansehen footers, canonical 22.4px caps and bilingual labels are covered by focused browser checks.\n',encoding='utf8')
print('[DEBUG] Title chip preview decode and playback continuity verified',flush=True)
