import glob
import sys

import av
import numpy as np
from PIL import Image

G = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🗑️generated/lc3"
tag = sys.argv[1]
files = glob.glob(f"{G}/media-{tag}/videos/**/*.mp4", recursive=True)
files = [f for f in files if "partial_movie_files" not in f]
if not files:
    print("no video", tag)
    sys.exit(1)
best, best_score, best_i = None, -1.0, 0
with av.open(files[0]) as c:
    for i, fr in enumerate(c.decode(video=0)):
        if i % 8:
            continue
        a = fr.to_ndarray(format="gray").astype(np.int16)
        score = float((np.abs(np.diff(a, axis=1)) > 40).sum())
        if score >= best_score:
            best, best_score, best_i = fr.to_image(), score, i
best.save(f"{G}/{tag}.png")
print(tag, best_i, best.size)
