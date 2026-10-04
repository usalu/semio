#!/usr/bin/env python3
"""🔬️ Work package A5: judges recorded traces of the clearance reference world with numpy, independently of the module's own ``overlaps``.

Run from the repository root after ``clearance_world.ts --trace n`` (and ``--without <rule> --name <name>``):
``.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/clearance_trace_check.py`` (``.venv/bin/python``
elsewhere). Every ``🗑️generated/a5/trace-*.json`` holds the boxes of every body at the end of every tick. A pair
overlaps when the rectangle it shares has a positive width and a positive height (broadcast ``numpy.minimum`` and
``numpy.maximum``); the script counts the ticks with at least one such pair and holds the count to the one the world
recorded with ``overlaps`` of the module. Exit code 1 when they disagree.
"""

import glob
import json
import os
import sys

import numpy

HERE = os.path.dirname(os.path.abspath(__file__))


def overlapping(boxes):
    """💥️ How many pairs of boxes share an area."""
    if len(boxes) < 2:
        return 0
    rows = numpy.array(boxes, dtype=float)
    width = numpy.minimum(rows[:, None, 2], rows[None, :, 2]) - numpy.maximum(rows[:, None, 0], rows[None, :, 0])
    height = numpy.minimum(rows[:, None, 3], rows[None, :, 3]) - numpy.maximum(rows[:, None, 1], rows[None, :, 1])
    return int(numpy.triu((width > 0) & (height > 0), 1).sum())


def main():
    """🧾️ Judges every trace and prints one line per trace."""
    disagreements = 0
    for path in sorted(glob.glob(os.path.join(HERE, "🗑️generated", "a5", "trace-*.json"))):
        with open(path, "rb") as handle:
            trace = json.loads(handle.read())
        counts = [overlapping(boxes) for boxes in trace["boxes"]]
        ticks = sum(1 for count in counts if count > 0)
        pairs = sum(counts)
        agree = ticks == trace["overlaps"]
        disagreements += 0 if agree else 1
        sys.stdout.buffer.write(("%s: ticks=%d bodies=%d overlap-ticks numpy=%d module=%d pairs=%d %s\n" % (os.path.basename(path), len(counts), sum(len(boxes) for boxes in trace["boxes"]), ticks, trace["overlaps"], pairs, "agree" if agree else "DISAGREE")).encode("utf-8"))
    return 0 if disagreements == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
