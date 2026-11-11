import av
from pathlib import Path
p = Path('.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/s/123/p/Beat4_Casting/uncached_00026.mp4')
with av.open(str(p)) as c:
 st=c.streams.video[0]
 for i,f in enumerate(c.decode(st)):
  if i in (0,5,10,15,20,25,30,35,40,45,50,55):
   f.to_image().save(Path(__file__).parent / '🗑️generated' / f'casting26-{i}.png')
 print(i+1, flush=True)
