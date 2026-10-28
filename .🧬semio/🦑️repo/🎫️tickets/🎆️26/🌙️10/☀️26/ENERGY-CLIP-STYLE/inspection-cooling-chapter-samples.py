"""🎞️ Sample already published chapter navigation without rendering."""
from pathlib import Path
import hashlib,av
from PIL import Image,ImageDraw
t=Path(__file__).parent
p=t/"inspection-chapter-preview-Cooling_01_HeatingVsCooling.mp4"
assert hashlib.sha256(p.read_bytes()).hexdigest()=="1617e422de827567bd682d3f9bf8767f5795265ff65b41aecba00e9b066b1b5b"
out=t/"🗑️generated/inspection-cache/cooling-chapter";out.mkdir(parents=True,exist_ok=True)
indices=list(range(14))+list(range(513,525));captures=[]
with av.open(str(p)) as c:
 s=c.streams.video[0];s.codec_context.thread_count=1
 for i,f in enumerate(c.decode(s)):
  if i in indices:captures.append((i,f.to_image()))
for k in range(0,len(captures),9):
 items=captures[k:k+9];sheet=Image.new("RGB",(1440,296*((len(items)+2)//3)),"#222222");d=ImageDraw.Draw(sheet)
 for j,(i,im) in enumerate(items):
  x=j%3*480;y=j//3*296;sheet.paste(im,(x,y));d.text((x+5,y+275),"frame "+str(i)+" @ "+str(i/4)+"s",fill="white")
 sheet.save(out/("navigation-"+str(k//9+1)+".png"))
print("[DEBUG] captured "+str(len(captures))+" existing navigation frames")
