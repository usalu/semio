"""⏱️ Sum video seconds of partial movie files below directories: python render-time-sum.py <dir> ..."""
import sys
from pathlib import Path

import av

for argument in sys.argv[1:]:
    seconds, frames, files = 0.0, 0, 0
    for path in sorted(Path(argument).rglob("uncached_*.mp4")):
        with av.open(str(path)) as container:
            stream = container.streams.video[0]
            count = stream.frames or sum(1 for _ in container.decode(stream))
            frames += count
            seconds += count / float(stream.average_rate)
            files += 1
    print(argument.replace("\\", "/")[-40:], files, "files", frames, "frames", round(seconds, 2), "s")
