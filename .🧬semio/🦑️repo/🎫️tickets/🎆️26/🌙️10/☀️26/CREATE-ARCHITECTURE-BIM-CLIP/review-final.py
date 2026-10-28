"""🎞️ Inspect final standalone composition frames."""
from pathlib import Path
import av
root = Path(__file__).parent / "🗑️generated"
output = root / "review-final"
output.mkdir(exist_ok=True)
for video in sorted((root / "f/de/speaker/rendered").glob("BIM_*_640x360p1.mp4")):
    with av.open(str(video)) as container:
        frames = list(container.decode(video=0))
        for label, index in [("first",0),("middle",len(frames)//2),("last",len(frames)-1)]:
            file = output / (video.stem + "-" + label + ".png")
            frames[index].to_image().save(file)
        print(video.name, len(frames), float(container.duration / av.time_base))

from PIL import ImageChops
video = next((root / "f/de/speaker/rendered").glob("BIM_information_*_640x360p1.mp4"))
with av.open(str(video)) as container:
    frames = list(container.decode(video=0))
    print("first_last_pixel_identical", ImageChops.difference(frames[0].to_image(), frames[-1].to_image()).getbbox() is None)
    for index in (1,2,3,4):
        frames[index].to_image().save(output / ("opening-" + str(index) + ".png"))
