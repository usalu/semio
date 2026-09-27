"""🔮️ AV1 third-party oracle: ffprobe (stream parameters, frame count, duration) + ffmpeg (decoded pixels) over every MP4 a
twin of the raster video tier wrote for `🧫️fixtures/🔣️.json`. Usage: python3 av1-oracle.py <mp4 dir> [<fixture>]"""
import json, os, subprocess, sys

OVERLAY = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-overlay"
FIXTURE = os.path.join(OVERLAY, "🧰️framework/🔨️modules/🖌️raster/🎥️video/🧫️fixtures/🔣️.json")


def probe(path: str) -> dict:
    out = subprocess.run(["ffprobe", "-v", "error", "-count_frames", "-show_streams", "-show_format", "-of", "json", path], capture_output=True, text=True)
    if out.returncode != 0:
        raise AssertionError(f"ffprobe refused {path}: {out.stderr.strip()}")
    return json.loads(out.stdout)


def decode_rgb(path: str, width: int, height: int) -> list:
    out = subprocess.run(["ffmpeg", "-v", "error", "-i", path, "-f", "rawvideo", "-pix_fmt", "rgb24", "-"], capture_output=True)
    if out.returncode != 0:
        raise AssertionError(f"ffmpeg decode refused {path}: {out.stderr.decode().strip()}")
    frame = width * height * 3
    raw = out.stdout
    assert len(raw) % frame == 0, f"decoded {len(raw)} bytes is not whole {width}x{height} frames"
    frames = []
    for index in range(len(raw) // frame):
        chunk = raw[index * frame : (index + 1) * frame]
        frames.append([sum(chunk[c::3]) / (width * height) for c in range(3)])
    return frames


def rows_mode(directory: str) -> int:
    failures = 0
    for line in open(os.path.join(directory, "rows.txt")).read().split("\n"):
        if not line.strip():
            continue
        name, width, height, fps, frames = line.split()
        stream = probe(os.path.join(directory, f"{name}.mp4"))["streams"][0]
        observed = (stream.get("codec_name"), stream.get("profile"), stream.get("width"), stream.get("height"), stream.get("r_frame_rate"), int(stream.get("nb_read_frames", -1)), int(stream.get("nb_frames", -1)), round(float(stream.get("duration", -1)), 3))
        wanted = ("h264", "Constrained Baseline", int(width), int(height), f"{fps}/1", int(frames), int(frames), round(int(frames) / int(fps), 3))
        decoded = decode_rgb(os.path.join(directory, f"{name}.mp4"), int(width), int(height))
        ok = observed == wanted and len(decoded) == int(frames)
        failures += not ok
        print(f"[av1-oracle] {'PASS' if ok else 'FAIL'} {name} ffprobe={observed} decoded_frames={len(decoded)}" + ("" if ok else f" WANTED={wanted}"))
    print(f"[av1-oracle] rows failures={failures}")
    return 1 if failures else 0


def main() -> int:
    directory = sys.argv[1]
    if "--rows" in sys.argv:
        return rows_mode(directory)
    fixture = json.load(open(sys.argv[2] if len(sys.argv) > 2 else FIXTURE, encoding="utf-8"))
    failures = 0
    for case in fixture["cases"]:
        path = os.path.join(directory, f"{case['id']}.mp4")
        info = probe(path)
        stream = info["streams"][0]
        frames = sum(run["frames"] for run in case["runs"])
        checks = {
            "codec_name": (stream.get("codec_name"), "h264"),
            "profile": (stream.get("profile"), "Constrained Baseline"),
            "width": (stream.get("width"), case["width"]),
            "height": (stream.get("height"), case["height"]),
            "pix_fmt": (stream.get("pix_fmt"), "yuv420p"),
            "r_frame_rate": (stream.get("r_frame_rate"), f"{case['fps']}/1"),
            "nb_read_frames": (int(stream.get("nb_read_frames", -1)), frames),
            "nb_frames": (int(stream.get("nb_frames", -1)), frames),
            "duration": (round(float(stream.get("duration", -1)), 3), round(frames / case["fps"], 3)),
            "level": (stream.get("level"), case.get("expected", {}).get("levelIdc", stream.get("level"))),
        }
        decoded = decode_rgb(path, case["width"], case["height"])
        expected_colours = [run["rgb"] for run in case["runs"] for _ in range(run["frames"])]
        colour_ok = len(decoded) == len(expected_colours) and all(abs(got - want) <= 4 for frame, rgb in zip(decoded, expected_colours) for got, want in zip(frame, rgb))
        bad = {name: pair for name, pair in checks.items() if pair[0] != pair[1]}
        status = "PASS" if not bad and colour_ok else "FAIL"
        failures += status == "FAIL"
        print(f"[av1-oracle] {status} {case['id']} ffprobe={ {k: v[0] for k, v in checks.items()} } decoded_rgb={[[round(c, 1) for c in f] for f in decoded]}" + (f" MISMATCH={bad}" if bad else "") + ("" if colour_ok else " COLOUR-MISMATCH"))
    print(f"[av1-oracle] cases={len(fixture['cases'])} failures={failures}")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
