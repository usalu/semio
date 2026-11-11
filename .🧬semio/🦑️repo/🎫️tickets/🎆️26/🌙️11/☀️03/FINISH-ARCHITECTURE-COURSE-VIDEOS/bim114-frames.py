"""🖼️ Extract completed current BIM 114 partial-frame previews with PyAV."""
import sys
from pathlib import Path
import av
from PIL import Image
root=Path.cwd()
folder=root/".🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/s/114/p/Beat1_DataToInformation"
output=Path(__file__).parent/"🗑️generated"
for number in map(int,sys.argv[1:]):
    path=folder/f"uncached_{number:05d}.mp4"
    with av.open(str(path)) as container:
        frames=list(container.decode(video=0))
        if not frames:raise ValueError(f"No frame in {path.name}")
        for label,frame in (("middle",frames[len(frames)//2]),("end",frames[-1])):
            frame.to_image().save(output/f"bim114-partial-{number:05d}-{label}.png")
        print("[DEBUG]",path.name,"frames",len(frames),flush=True)
