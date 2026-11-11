import av
from pathlib import Path
from PIL import Image,ImageDraw
root=Path.cwd()
folder=root/'.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/rendered'
out=Path(__file__).parent/'🗑️generated'
for path in sorted(folder.glob('Optimization_*_speaker_en_480p15.mp4')):
 if path.stat().st_mtime < 1794000000 or not path.with_suffix('.render.json').exists(): continue
 with av.open(str(path)) as c:
  v=c.streams.video[0]
  duration=float(v.duration*v.time_base)
  sheet=Image.new('RGB',(854*3,510),'white')
  draw=ImageDraw.Draw(sheet)
  for i,fraction in enumerate((.25,.5,.75)):
   t=duration*fraction;c.seek(int(t/v.time_base),stream=v,backward=True)
   frame=next(f for f in c.decode(v) if f.time>=t)
   sheet.paste(frame.to_image().resize((854,480)),(i*854,30))
   draw.text((i*854+5,5),f'{path.stem} {t:.1f}s',fill='black')
  dest=out/(path.stem+'-fresh.png')
  sheet.save(dest)
  print(path.name,duration,flush=True)
