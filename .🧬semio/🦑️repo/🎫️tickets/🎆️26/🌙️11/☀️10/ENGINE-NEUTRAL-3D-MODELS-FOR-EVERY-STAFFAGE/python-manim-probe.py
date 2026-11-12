"""Frame facts of a rendered clip with PyAV in the course environment: size, frame count, how many frames differ from the first, and the share of non-background pixels of the middle frame. Usage: python python-manim-probe.py <video>"""
import json, sys, av, numpy as np
frames = []
with av.open(sys.argv[1]) as c:
    for f in c.decode(video=0):
        frames.append(f.to_ndarray(format="rgb24"))
first, mid = frames[0], frames[len(frames) // 2]
bg = np.median(mid.reshape(-1, 3), axis=0)
print(json.dumps({"frames": len(frames), "size": [mid.shape[1], mid.shape[0]], "framesDifferingFromFirst": int(sum(not np.array_equal(first, f) for f in frames)),
  "midFrameNonBackgroundShare": round(float((np.abs(mid.astype(int) - bg).sum(axis=2) > 24).mean()), 4), "midFrameStd": round(float(mid.std()), 2)}))
