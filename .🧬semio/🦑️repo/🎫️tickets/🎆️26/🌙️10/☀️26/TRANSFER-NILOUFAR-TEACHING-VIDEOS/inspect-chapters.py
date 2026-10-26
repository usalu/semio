"""👀 Inspect completed chapter frames while independent renders continue."""
from pathlib import Path
import sys
import av
from PIL import Image, ImageDraw

root=Path(sys.argv[1])
output=Path(sys.argv[2])
clips=sorted(root.glob("*.mp4") if root.name=="rendered" else root.glob("[1-5]/v/*.mp4"))
rows=[]
for clip in clips:
    try:
        with av.open(str(clip)) as container:
            stream=container.streams.video[0]
            duration=float(stream.duration*stream.time_base)
            target=duration*(float(sys.argv[3]) if len(sys.argv)>3 else .5)
            container.seek(int(target/stream.time_base),stream=stream)
            frame=next(frame for frame in container.decode(stream) if frame.time>=target)
            rows.append((clip.stem,frame.to_image().resize((640,360))))
    except Exception:
        continue
sheet=Image.new("RGB",(640*3,390*((len(rows)+2)//3)),"#111111")
draw=ImageDraw.Draw(sheet)
for index,(name,image) in enumerate(rows):
    x,y=(index%3)*640,(index//3)*390
    draw.text((x+8,y+8),name,fill="white")
    sheet.paste(image,(x,y+30))
sheet.save(output,quality=90)
print(output,len(rows))
