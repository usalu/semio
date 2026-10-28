"""🎞️ Decode current physical navigation windows without rendering."""
from pathlib import Path
import hashlib,av,json
from PIL import Image,ImageDraw
t=Path(__file__).parent
m=json.loads((t/"🗑️generated/iv/chapter-measurements.json").read_text(encoding="utf-8"))
r=next(x for x in m["chapters"] if x["id"]=="EnergyBalance_07_TheorieVsPraxis")
p=Path(r["path"])
assert hashlib.sha256(p.read_bytes()).hexdigest()==r["sha256"]
out=t/"🗑️generated/inspection-cache/balance07-finalepoch";out.mkdir(parents=True,exist_ok=True)
captures=[]
for start,stop in [(0,r["clock"]["focus_frames"]),(r["frames"]-r["clock"]["restore_frames"],r["frames"])]:
 with av.open(str(p)) as c:
  stream=c.streams.video[0];stream.codec_context.thread_count=1
  c.seek(int((start/4)/stream.time_base),stream=stream,backward=True)
  for f in c.decode(stream):
   i=round(float(f.pts*f.time_base)*4)
   if i>=stop:break
   if i>=start:captures.append((i,f.to_image()))
assert len(captures)==r["clock"]["focus_frames"]+r["clock"]["restore_frames"]
for k in range(0,len(captures),9):
 items=captures[k:k+9];sheet=Image.new("RGB",(1440,296*((len(items)+2)//3)),"#222222");d=ImageDraw.Draw(sheet)
 for j,(i,im) in enumerate(items):
  x=j%3*480;y=j//3*296;sheet.paste(im,(x,y));d.text((x+5,y+275),"frame "+str(i)+" @ "+str(i/4)+"s",fill="white")
 sheet.save(out/("navigation-"+str(k//9+1)+".png"))
print("[DEBUG] "+str(len(captures))+" balance navigation frames captured")
