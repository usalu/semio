import sys
from pathlib import Path
import av, numpy as np
root=Path.cwd()
sys.path.insert(0,str(root/'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🧰️r3-media'))
import lib
rt=lib.runtime()
job=next(j for j in rt.segment_plan() if j['segment']['scene']=='Beat2_EnergieImAlltag')
for lang in ('en','de'):
 try:
  print('NARRATION',lang,rt.source_narration(job['segment'],language=lang),flush=True)
  print('SPEECH_ROOT',lang,rt.source_speech_root(root/'🎓️teaching'/job['segment']['script'],language=lang),flush=True)
 except Exception as e: print('NARRATION_ERROR',lang,repr(e),flush=True)
clip=lib.base('en','speaker')/'s/2/v/Beat2_EnergieImAlltag.mp4'
with av.open(str(clip)) as c:
 print('STREAMS',[(s.type,s.codec_context.name) for s in c.streams],flush=True)
 for stream in c.streams.audio:
  total=0;peak=0.0;square=0.0
  for frame in c.decode(stream):
   a=frame.to_ndarray().astype('float64')
   peak=max(peak,float(np.max(np.abs(a))))
   square+=float(np.sum(a*a))
   total+=a.size
  print('AUDIO_SAMPLES',total,'PEAK',peak,'RMS',(square/total)**.5 if total else None,flush=True)
