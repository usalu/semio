"""🎞️ Decode current physical navigation windows without rendering."""
from pathlib import Path
import hashlib,av
from PIL import Image,ImageDraw
t=Path(__file__).parent;p=t/"inspection-clip-preview-physical_fundamentals.mp4"
assert hashlib.sha256(p.read_bytes()).hexdigest()=="692a87119b8b65402e74c513f4eec4be3b5fb98185e0b857673e7a45d28fb4ce"
out=t/"🗑️generated/inspection-cache/physical-chapter";out.mkdir(parents=True,exist_ok=True)
captures=[]
for start,stop in [(0,26),(4103,4118)]:
 with av.open(str(p)) as c:
  stream=c.streams.video[0];stream.codec_context.thread_count=1
  c.seek(int((start/4)/stream.time_base),stream=stream,backward=True)
  for f in c.decode(stream):
   i=round(float(f.pts*f.time_base)*4)
   if i>=stop:break
   if i>=start:captures.append((i,f.to_image()))
assert len(captures)==41
for k in range(0,len(captures),9):
 items=captures[k:k+9];sheet=Image.new("RGB",(1440,296*((len(items)+2)//3)),"#222222");d=ImageDraw.Draw(sheet)
 for j,(i,im) in enumerate(items):
  x=j%3*480;y=j//3*296;sheet.paste(im,(x,y));d.text((x+5,y+275),"frame "+str(i)+" @ "+str(i/4)+"s",fill="white")
 sheet.save(out/("navigation-"+str(k//9+1)+".png"))
print("[DEBUG] 41 physical navigation frames captured")
