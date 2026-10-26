"""🧪 Inspect the media metadata rational type used by the installed PyAV."""
import av
from pathlib import Path
import sys
with av.open(sys.argv[1]) as container:
    rate = container.streams.video[0].average_rate
    print(type(rate), repr(rate), str(rate))
    print([name for name in dir(rate) if not name.startswith("_")])
