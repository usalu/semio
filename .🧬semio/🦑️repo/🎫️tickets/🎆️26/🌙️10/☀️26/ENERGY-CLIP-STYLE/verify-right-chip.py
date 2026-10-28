"""🔎 Decode the focused preview and verify fused right-chip geometry and native events."""
import json,os,sys
from pathlib import Path
import av,numpy as np
from PIL import Image
TICKET=Path(__file__).resolve().parent
output=Path(os.environ['SEMIO_TEACHING_OUTPUT_ROOT'])
frames=json.loads((output/'metadata.json').read_text(encoding='utf8'))
for frame in frames:
    for level in frame['browser']['layers']:
        if not level['interactive']:continue
        for card in level['cards']:
            assert abs(card['chip_rect']['x']+card['chip_rect']['width']-card['rect']['x']-card['rect']['width'])<.5
            assert card['chip_icons']==['maximize-2'] and card['chip_text'].strip()=='Ansehen'
    for window in frame['browser']['windows']:
        assert window['control_icons']==['minimize-2'] and window['control_text'].strip()=='Wiederherstellen' and window['control_count']==1
native=[(event['type'],event['target']) for frame in frames for event in frame['browser'].get('pointer_events',[])]
for target in ['energy','physical_fundamentals','physical_fundamentals:fullscreen','footer:range']:
    assert [kind for kind,identity in native if identity==target]==['pointerdown','pointerup','click']
with Image.open(frames[0]['path']) as first,Image.open(frames[-1]['path']) as last:assert np.array_equal(np.asarray(first),np.asarray(last))
with av.open(str(TICKET/'right-chip.mp4')) as movie:
    stream=movie.streams.video[0];count=sum(1 for _ in movie.decode(stream));assert count==56 and(stream.width,stream.height)==(1600,900)
(TICKET/'right-chip-verification.md').write_text('# Right Chip Preview Verification\n\nThe 7-second preview contains 56 decoded frames at 1600 × 900 and 8 fps. All active cards have their sole Ansehen + maximize icon chip aligned with the right edge. Expanded windows have one Restore/Wiederherstellen + minimize icon button in their right cap. Native down/up/click traces target the topic chip, video chip, expanded restore chip and parent footer control. First and last preview PNGs are identical.\n',encoding='utf8')
print('[DEBUG] Right-side fused-chip preview verified',flush=True)
