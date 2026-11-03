import sys
try:
    import av; print("av", av.__version__)
except Exception as e: print("no av", e)
import imageio_ffmpeg
exe = imageio_ffmpeg.get_ffmpeg_exe(); print(exe)
import subprocess
r = subprocess.run([exe, "-version"], capture_output=True, text=True); print(r.returncode, r.stdout[:100], r.stderr[:300])
