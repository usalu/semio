import av
from pathlib import Path
from PIL import Image,ImageDraw
root=Path.cwd()
cache=root/'.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/rendered'
out=Path(__file__).parent/'🗑️generated'
for stem,fraction in [('Optimization_Index_speaker_en_480p15',.25),('Grasshopper_tower_speaker_en_480p15',.5)]:
 path=cache/(stem+'.mp4')
 with av.open(str(path)) as c:
  v=c.streams.video[0]
  duration=float(v.duration*v.time_base)
  midpoint=duration*fraction
  canvas=Image.new('RGB',(854*3,510),'white')
  draw=ImageDraw.Draw(canvas)
  for i,delta in enumerate((-5,0,5)):
   point=midpoint+delta
   c.seek(int(point/v.time_base),stream=v,backward=True)
   frame=next(f for f in c.decode(v) if f.time>=point)
   canvas.paste(frame.to_image().resize((854,480)),(854*i,30))
   draw.text((854*i+5,5),f'{stem} {point:.1f}s',fill='black')
  canvas.save(out/(stem+'-adjacent.png'))
  print(stem,midpoint,flush=True)
