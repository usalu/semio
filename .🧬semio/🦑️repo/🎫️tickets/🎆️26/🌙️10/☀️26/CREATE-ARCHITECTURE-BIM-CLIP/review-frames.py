"""🖼️ Extract representative BIM frames for ticket visual audit."""
from pathlib import Path
import av
root = Path(__file__).parent / "🗑️generated"
output = root / "review"
output.mkdir(exist_ok=True)
for video in sorted((root / "r/de/speaker/s").glob("*/v/Beat*.mp4")):
    with av.open(str(video)) as container:
        target = float(container.duration / av.time_base) * 0.70
        for frame in container.decode(video=0):
            if frame.time >= target:
                file = output / (video.stem + ".png")
                frame.to_image().save(file)
                print(file)
                break
