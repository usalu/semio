"""🎬 Build the focused Animate writer fixture and verify its outputs with PyAV and Pillow."""

from pathlib import Path
import os
import subprocess
import sys

from PIL import Image
import av

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
outputs = ticket / "🗑️generated/animate-encoder"
outputs.mkdir(parents=True, exist_ok=True)
dependencies = Path(sys.argv[1]).resolve()
compiler = sys.argv[2]
writer = root / (
    "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/"
    "🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/✒️partial-movie-writer/🦀️.rs"
)
fixture = outputs / "writer-fixture.rs"
cache = writer.parents[1] / "💾️partial-movie-cache/🦀️.rs"
fixture.write_text((ticket / "animate-native-encoder.rs").read_text()
                   .replace("SEMIO_NATIVE_ANIMATE_WRITER", writer.as_posix())
                   .replace("SEMIO_NATIVE_ANIMATE_CACHE", cache.as_posix()))
suffix = ".exe" if os.name == "nt" else ""
probe = outputs / f"writer-fixture{suffix}"
unit = outputs / f"writer-unit{suffix}"
externs = []
for crate in ("semio_framework_pixels", "semio_framework_raster", "semio_s_artifact_stdio_gif", "semio_s_artifact_stdio_mp4",
              "semio_framework_async_macros"):
    candidates = [candidate for prefix in ("lib", "") for candidate in dependencies.glob(f"{prefix}{crate}-*")
                  if candidate.suffix in (".rlib", ".dylib", ".so", ".dll")]
    if len(candidates) != 1:
        raise ValueError(f"Expected one frozen archive for {crate}, got {candidates}")
    library = candidates[0]
    externs.extend(["--extern", f"{crate}={library}"])
    if library.suffix == ".rlib":
        externs.extend(["--extern", f"{crate}={library.with_suffix('.rmeta')}"])
hash_source = root / "🧰️framework/🔨️modules/🔏️hash/🦀️.rs"
hash_library = outputs / "libframework_hash.rlib"
subprocess.run([compiler, "--edition=2021", "--crate-name", "framework_hash", "--crate-type", "rlib", str(hash_source),
                "-o", str(hash_library)], check=True)
externs.extend(["--extern", f"framework_hash={hash_library}"])
base = [compiler, "--edition=2024", "-Zembed-metadata=no", "-Adead_code", "-L", f"dependency={dependencies}", *externs, str(fixture)]
subprocess.run([*base, "-o", str(probe)], check=True)
subprocess.run([str(probe), str(outputs)], check=True)

with av.open(str(outputs / "scene.mp4")) as movie:
    stream = movie.streams.video[0]
    assert (stream.codec_context.name, stream.width, stream.height, stream.average_rate) == ("h264", 16, 16, 15)
    frames = list(movie.decode(video=0))
assert [frame.key_frame for frame in frames] == [True, False, True, False]
with Image.open(outputs / "scene.gif") as gif:
    assert gif.n_frames == len(frames) == 4
    for index, frame in enumerate(frames):
        gif.seek(index)
        rgba = gif.convert("RGBA")
        decoded = frame.to_ndarray(format="rgba")
        expected = ((6, 26, 31, 255), (17, 35, 40, 255)) if index < 2 else ((233, 230, 215, 255), (218, 217, 204, 255))
        for (x, y), color in zip(((0, 0), (8, 8)), expected):
            assert rgba.getpixel((x, y)) == color, (index, rgba.getpixel((x, y)), color)
            actual = tuple(int(channel) for channel in decoded[y, x])
            assert max(abs(a - b) for a, b in zip(actual, color)) <= 4, (index, actual, color)
print("[DEBUG] Four independently decoded H.264 frames with correct keyframes and pixels; Pillow matched both GIF owners and chips exactly.")

try:
    with av.open(str(outputs / "raw-avc1.mp4")) as broken:
        old_frames = list(broken.decode(video=0))
except av.FFmpegError:
    old_frames = []
assert not old_frames
print("[DEBUG] PyAV decoded no pictures from the original raw RGBA / AVC1 layout.")

subprocess.run([*base, "--test", "-o", str(unit)], check=True)
subprocess.run([str(unit)], env={**os.environ, "TMPDIR": str(outputs), "TEMP": str(outputs), "TMP": str(outputs)}, check=True)
