"""🎞️ Verify the registered FFmpeg resampling filter using independent decoded labels."""

import argparse
from pathlib import Path

import av
import numpy as np
from PIL import Image

ROOT = Path(__file__).resolve().parent
OUTPUT = ROOT / "🗑️generated" / "cursor-resampling-test"
COLORS = [16, 80, 144, 208]


def prepare():
    """🎨 Author four independently distinguishable native capture images."""
    OUTPUT.mkdir(parents=True, exist_ok=True)
    for index, value in enumerate(COLORS):
        Image.new("RGB", (32, 32), (value, value, value)).save(OUTPUT / f"in{index}.png")


def verify():
    """🔬 Decode every output frame and reproduce the timestamp map with NumPy."""
    with av.open(str(OUTPUT / "result.mkv")) as container:
        stream = container.streams.video[0]
        frames = list(container.decode(stream))
        assert len(frames) == 10, len(frames)
        assert stream.average_rate == 30, stream.average_rate
        starts = np.floor_divide(np.arange(4) * 30, 12)
        expected = np.searchsorted(starts, np.arange(10), side="right") - 1
        actual = []
        for frame in frames:
            pixels = frame.to_ndarray(format="rgb24")
            label = int(pixels[0, 0, 0])
            np.testing.assert_array_equal(pixels, np.full((32, 32, 3), label))
            actual.append(COLORS.index(label))
        np.testing.assert_array_equal(actual, expected)
        np.testing.assert_array_equal(actual, [0, 0, 1, 1, 1, 2, 2, 3, 3, 3])
        print("[DEBUG] FFmpeg round=down: all 10 decoded frames match the independent NumPy timestamp map.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("action", choices=("prepare", "verify"))
    options = parser.parse_args()
    {"prepare": prepare, "verify": verify}[options.action]()
