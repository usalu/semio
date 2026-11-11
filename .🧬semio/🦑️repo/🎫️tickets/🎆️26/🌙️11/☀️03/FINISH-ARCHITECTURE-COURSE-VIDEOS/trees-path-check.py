import av
from pathlib import Path
root=Path('.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/s/124/p/Beat5_DataTrees')
out=Path(__file__).parent/'🗑️generated'
for n in range(20,21):
 with av.open(str(root / f'uncached_{n:05}.mp4')) as c:
  frames=list(c.decode(video=0))
  for label,i in [('first',0),('mid',len(frames)//2),('last',len(frames)-1)]: frames[i].to_image().save(out / f'trees-{n}-{label}.png')
  print(n,len(frames),flush=True)
