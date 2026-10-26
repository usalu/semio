"""👀 Inspect the last complete original Physical Fundamentals partial."""

from pathlib import Path
import av

generated = Path(__file__).resolve().parent / "🗑️generated"
partials = sorted((generated / "videos/1/p/PhysicalFundamentals_FullSection").glob("*.mp4"), key=lambda path: path.stat().st_mtime)
for partial in reversed(partials[:-1]):
    try:
        with av.open(partial) as video:
            frame = next(video.decode(video=0))
            frame.to_image().save(generated / "progress-pf.png")
        print("[DEBUG]", partial.name)
        break
    except (av.error.FFmpegError, StopIteration):
        continue
