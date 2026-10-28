"""🎬 Render a calm single-topic review through the production DOM browser port."""
from __future__ import annotations

import argparse
import copy
import importlib
import importlib.util
import json
import math
import os
import shutil
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = next(path for path in HERE.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
BASE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"


def load_motion():
    """📦 Load the adjacent tested pure sampler without exporting third-party types."""
    spec = importlib.util.spec_from_file_location("calm_cursor_motion", HERE / "cursor-motion.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def center(rect):
    """🎯 Resolve the center of an actual measured chip."""
    return [rect["x"] + rect["width"] / 2, rect["y"] + rect["height"] / 2]


def contains(rect, point):
    """🔲 Test a hotspot against native screen-space bounds."""
    return rect["x"] <= point[0] < rect["x"] + rect["width"] and rect["y"] <= point[1] < rect["y"] + rect["height"]


def root_cards(metadata):
    """🌳 Expose all root topic chips from the actual DOM."""
    return next(layer for layer in metadata["layers"] if layer["id"] == "root")["cards"]


def route_point(route, amount):
    """🛤️ Follow measured route distance without changing the timeline easing."""
    lengths = [math.dist(first, last) for first, last in zip(route, route[1:])]
    distance = min(1, max(0, amount)) * sum(lengths)
    for index, length in enumerate(lengths):
        if distance <= length or index == len(lengths) - 1:
            fraction = min(1, distance / length) if length else 0
            return [a + (b - a) * fraction for a, b in zip(route[index], route[index + 1])]
        distance -= length
    return list(route[-1])


def frame_schedule(duration, fps, capture_fps=12):
    """⏱️ Map each delivery frame to its explicitly quantized native DOM observation."""
    if isinstance(duration, bool) or not isinstance(duration, (int, float)) or not math.isfinite(duration) or duration <= 0:
        raise ValueError("Duration must be finite and positive")
    if any(isinstance(rate, bool) or not isinstance(rate, int) or rate <= 0 for rate in (fps, capture_fps)):
        raise ValueError("Frame rates must be positive integers")
    native_count, output_count = round(duration * capture_fps), round(duration * fps)
    if not native_count or not output_count:
        raise ValueError("Schedule needs at least one frame")
    return {"native_count": native_count, "output_count": output_count,
            "native_indices": [min(native_count - 1, ((index + 1) * capture_fps + fps - 1) // fps - 1)
                               for index in range(output_count)]}


def endpoint_stability(first, last):
    """🧮 Isolate the existing image oracle and compare actual root snapshot pixels."""
    from PIL import Image, ImageChops, ImageStat

    with Image.open(first) as start, Image.open(last) as end:
        difference = ImageChops.difference(start.convert("RGB"), end.convert("RGB"))
    mse = sum(value * value for value in ImageStat.Stat(difference).rms) / 3
    return {"exactPixels": difference.getbbox() is None,
            "psnrDb": 10 * math.log10(255 * 255 / mse) if mse else None,
            "minimumAcceptedPsnrDb": 45}


def main():
    """🚦 Own one explicitly localized and cancellable standalone review render."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--language", required=True, choices=("en", "de"))
    options = parser.parse_args()
    motion = load_motion()
    contract = motion.load_contract()
    duration = sum(stage["durationSeconds"] for stage in contract["stages"])
    fps, (width, height) = contract["fps"], contract["resolution"]
    capture_fps = 12
    target_id = contract["target"]["id"]
    output = (HERE / "🗑️generated" / "calm-cursor-final").resolve()
    output.mkdir(parents=True, exist_ok=True)
    os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(output)
    runtime = importlib.import_module(BASE + ".🎬️clip.⚙️runtime.🐍️")
    browser = importlib.import_module(BASE + ".🎬️clip.🪟️presentation.🌐️browser.🐍️")
    hierarchy = importlib.import_module(BASE + ".🎬️clip.🪟️presentation.🥞️hierarchy.🐍️")
    renderer = importlib.import_module(BASE + ".🎬️clip.🪟️presentation.🥞️hierarchy.🖌️render.🐍️")
    interaction = importlib.import_module(BASE + ".🎬️clip.🪟️presentation.🖱️interaction.🐍️")
    session = runtime.ToolSession()
    park = list(interaction.interaction_contract()["timeline"]["park"])
    catalog = hierarchy.hierarchy_contract()
    probes = []
    probe_index = 0

    def frame(point, revealed=False, *, path, elapsed=2000):
        """🪟 Build a real root state with only one optional topic reveal."""
        state = renderer.initial_hierarchy()
        state.update(revealed_topic=target_id if revealed else None, camera=renderer._camera(point))
        return {"output": str(path), "hierarchy": state, "media": [],
                "cursor": {"point": list(point), "kind": "arrow", "pressed": False},
                "transition_elapsed": elapsed, "canonical_overview": False}

    def capture(frames):
        """🌐 Keep the external browser behind its production tool-independent port."""
        if session.cancelled.is_set():
            raise InterruptedError("Calm cursor cancelled")
        return browser.render_batch({"schema": "semio-presentation-browser/v1", "work_directory": str(output),
                                     "viewport": {"width": width, "height": height}, "language": options.language,
                                     "catalog": catalog, "frames": frames}, session=session)

    def probe(points, *, revealed=False):
        """📐 Measure every chip using the actual camera at candidate pointer positions."""
        nonlocal probe_index
        frames = []
        for point in points:
            frames.append(frame(point, revealed, path=output / "p" / f"{probe_index:03d}.png"))
            probe_index += 1
        result = capture(frames)["metadata"]
        probes.extend({"input": value, "browser": actual} for value, actual in zip(frames, result))
        return result

    try:
        print("[DEBUG] Calm cursor: measuring native optimization chip", flush=True)
        first = probe([park])[0]
        target = center(next(card for card in root_cards(first) if card["id"] == target_id)["chip_rect"])
        for _ in range(8):
            actual = probe([target])[0]
            measured = center(next(card for card in root_cards(actual) if card["id"] == target_id)["chip_rect"])
            if math.dist(target, measured) < 0.1:
                break
            target = measured
        hover_probe = probe([target], revealed=True)[0]
        hover_rect = next(card for card in root_cards(hover_probe) if card["id"] == target_id)["chip_rect"]
        if not contains(hover_rect, target):
            raise RuntimeError("Revealed target moved away from the measured stationary pointer")
        route = [park, target]
        schedule = frame_schedule(duration, fps, capture_fps)
        count = schedule["native_count"]
        frames, timeline = [], []
        for index in range(count):
            normalized = index / (duration * capture_fps)
            scalar, revealed, stage = motion.sample(normalized, (0, 0), (1, 1))
            point = route_point(route, scalar[0])
            resting = stage in ("idle", "settle")
            frames.append(frame(point, revealed, path=output / "f" / f"{index:04d}.png",
                                elapsed=2000 if resting else 1000 / capture_fps))
            timeline.append({"frame": index, "seconds": index / capture_fps, "normalizedTime": normalized,
                             "stage": stage, "point": point, "revealed": revealed,
                             "hierarchy": copy.deepcopy(frames[-1]["hierarchy"]),
                             "pressed": False, "canonical_overview": False})
        print(f"[DEBUG] Calm cursor: rendering {count} native frames at {capture_fps} fps for {schedule['output_count']} delivery frames at {fps} fps", flush=True)
        result = capture(frames)
        metadata = {"language": options.language, "target": contract["target"], "controls": {"park": park, "target": target, "route": route},
                    "frames": result["metadata"], "timeline": timeline, "totalSeconds": duration, "fps": fps,
                    "captureFps": capture_fps, "captureScope": "One compiled standalone DOM snapshot; no shared canonical handoff asset requested",
                    "deliveredObservations": [{"frame": index, "seconds": index / fps, "nativeFrameIndex": native,
                                               "sourceSeconds": timeline[native]["seconds"], "point": timeline[native]["point"],
                                               "stage": timeline[native]["stage"], "revealed": timeline[native]["revealed"]}
                                              for index, native in enumerate(schedule["native_indices"])],
                    "resolution": [width, height], "source_files": result["source_files"]}
        (output / "metadata.json").write_text(json.dumps(metadata, ensure_ascii=False), encoding="utf-8")
        (output / "probes.json").write_text(json.dumps(probes, ensure_ascii=False), encoding="utf-8")
        (output / "plan.json").write_text(json.dumps(frames, ensure_ascii=False), encoding="utf-8")
        for planned, actual in zip(timeline, result["metadata"]):
            if actual["pointer_events"]:
                raise RuntimeError("Preview emitted a pressed or committed browser event")
            root = next(layer for layer in actual["layers"] if layer["id"] == "root")
            if any(pane["opened"] for layer in actual["layers"] for pane in layer["panes"]):
                raise RuntimeError("Preview opened a topic or video")
            for card in root["cards"]:
                if card["id"] != target_id and contains(card["chip_rect"], planned["point"]):
                    raise RuntimeError(f"Pointer crossed unrelated chip {card['id']}")
            if planned["revealed"]:
                rect = next(card for card in root["cards"] if card["id"] == target_id)["chip_rect"]
                if not contains(rect, planned["point"]):
                    raise RuntimeError("Stationary dwell left the actual target chip")
        stability = endpoint_stability(frames[0]["output"], frames[-1]["output"])
        (output / "endpoint-stability.json").write_text(json.dumps(stability), encoding="utf-8")
        if stability["psnrDb"] is not None and stability["psnrDb"] < 45:
            raise RuntimeError("First and final root snapshots differ visibly")
        movie = output / "calm-cursor.mp4"
        runtime.run_command([*runtime.ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-y",
                             "-framerate", str(capture_fps), "-i", str(output / "f" / "%04d.png"), "-an",
                             "-vf", f"fps={fps}:round=down:start_time=0", "-frames:v", str(schedule["output_count"]),
                             "-c:v", "libx264", "-preset", "fast", "-crf", "16", "-pix_fmt", "yuv420p",
                             "-movflags", "+faststart", str(movie)], session=session, log=output / f"encode-{time.time_ns()}.log")
        hover_frame = next(item["frame"] for item in timeline if item["seconds"] >= 5.5)
        shutil.copy2(output / "f" / f"{hover_frame:04d}.png", output / "calm-cursor.png")
        print(f"[DEBUG] Calm cursor review ready: {movie.name}", flush=True)
    except BaseException:
        session.cancel()
        raise


if __name__ == "__main__":
    main()
