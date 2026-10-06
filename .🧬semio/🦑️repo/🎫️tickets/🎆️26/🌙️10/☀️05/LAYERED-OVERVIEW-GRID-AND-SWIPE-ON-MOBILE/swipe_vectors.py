"""👆️ Independent Python twin of the swipe geometry: prints the fixture sections of 🧫️fixtures/🥞️layered-overview/🔣️.json for the
swipe mode, one vector per line. Run: python swipe_vectors.py"""
import json
import math

SLOP, DISTANCE, FLICK, RUBBER, DECEL, STOP, SETTLE_MS, EPS = 10, 0.25, 0.3, 0.55, 0.998, 0.02, 320, 1e-4


def near_square(count):
    best = None
    for columns in range(1, count + 1):
        rows = math.ceil(count / columns)
        if columns < rows or columns - rows > 2:
            continue
        empty = columns * rows - count
        if best is None or empty < best[0] * best[1] - count or (empty == best[0] * best[1] - count and columns - rows < best[0] - best[1]):
            best = (columns, rows)
    return best


def centered_cells(count):
    columns, rows = near_square(count)
    cells = []
    for index in range(count):
        row = index // columns
        in_row = min(columns, count - row * columns)
        cells.append((math.floor((columns - in_row) / 2) + index % columns, row))
    return cells


def clamp(x, y, cells):
    rows = max(r for _, r in cells) + 1
    columns = max(c for c, _ in cells) + 1
    y = min(rows - 1, max(0, y))
    row = min(rows - 1, max(0, round_half_up(y)))
    in_row = [c for c, r in cells if r == row]
    first, last = (min(in_row), max(in_row)) if in_row else (0, columns - 1)
    return min(last, max(first, x)), y


def round_half_up(value):
    return math.floor(value + 0.5)


def nearest(x, y, cells):
    cx, cy = clamp(round_half_up(x), round_half_up(y), cells)
    return (round_half_up(cx), round_half_up(cy))


def resting(x, y):
    c, r = round_half_up(x), round_half_up(y)
    return (c, r) if abs(x - c) < EPS and abs(y - r) < EPS else None


def axis(dx, dy):
    if max(abs(dx), abs(dy)) < SLOP:
        return None
    return "x" if abs(dx) >= abs(dy) else "y"


def step(moved, velocity, view):
    direction = velocity if abs(velocity) >= FLICK else moved if abs(moved) >= DISTANCE * view else 0
    return -1 if direction > 0 else 1 if direction < 0 else 0


def target(start, ax, s, cells):
    if s == 0:
        return None
    c, r = start
    t = nearest(c + s, r, cells) if ax == "x" else nearest(c, r + s, cells)
    if t == start:
        return None
    return t if t in cells else None


def rubber(overshoot):
    return 1 - 1 / (max(0, overshoot) * RUBBER + 1)


def wrap_target(start, ax, s, cells):
    """The landing (cell, slot) on the wrapping strip, or None."""
    if s == 0:
        return None
    t = target(start, ax, s, cells)
    if t is not None:
        return t, t
    c, r = start
    if ax == "x":
        columns = [col for col, row in cells if row == r]
        w = (min(columns) if s > 0 else max(columns), r)
    else:
        rows = max(row for _, row in cells) + 1
        w = nearest(c, 0 if s > 0 else rows - 1, cells)
    if w == start:
        return None
    return w, ((c + s, r) if ax == "x" else (w[0], r + s))


def drag(start, ax, shift, cells):
    s = (shift > 0) - (shift < 0)
    landing = wrap_target(start, ax, s, cells)
    if landing is not None:
        t = landing[1]
        k = min(1, abs(shift))
        return (start[0] + (t[0] - start[0]) * k, start[1] + (t[1] - start[1]) * k)
    pull = s * rubber(abs(shift))
    return (start[0] + pull, start[1]) if ax == "x" else (start[0], start[1] + pull)


def settle(a, b, elapsed, duration=SETTLE_MS):
    t = min(1, max(0, elapsed / duration))
    if t >= 1:
        return b, True
    e = 1 - (1 - t) ** 3
    return (a[0] + (b[0] - a[0]) * e, a[1] + (b[1] - a[1]) * e), False


def fling(velocity, elapsed):
    kept = DECEL ** max(0, elapsed)
    after = velocity * kept
    return velocity * (kept - 1) / math.log(DECEL), after, abs(after) < STOP


def neighbour_warm(count, at, live, opened, budget):
    if opened is not None:
        return {"kind": "hold", "reason": "opened"}
    cells = centered_cells(count)
    ids = [f"p{index}" for index in range(count)]
    around = {landing[0] for a, s in [("y", -1), ("x", -1), ("x", 1), ("y", 1)] if (landing := wrap_target(at, a, s, cells)) is not None}
    for index, cell in enumerate(cells):
        if ids[index] not in live and cell in around:
            return {"kind": "boot", "id": ids[index]} if len(live) < budget else {"kind": "hold", "reason": "budget"}
    return {"kind": "hold", "reason": "complete"}


cell = lambda c: None if c is None else {"column": c[0], "row": c[1]}
offset = lambda o: {"x": round(o[0], 12) + 0.0, "y": round(o[1], 12) + 0.0}
out = {}
out["nearestCells"] = [
    {"name": n, "count": k, "offset": {"x": x, "y": y}, "cell": cell(nearest(x, y, centered_cells(k)))}
    for n, k, x, y in [
        ("a view at rest is its own cell", 9, 1, 1),
        ("a view past the middle rounds on", 9, 1.6, 0.4),
        ("a view outside the strip comes back to its edge", 9, -0.8, 3.4),
        ("an empty flank of a short row lands on its occupied columns", 10, 0, 2),
        ("the far empty flank lands on the last occupied column", 10, 3.2, 1.7),
    ]
]
out["restingCells"] = [
    {"name": n, "offset": {"x": x, "y": y}, "cell": cell(resting(x, y))}
    for n, x, y in [("exactly on a cell", 2, 1), ("within a ten-thousandth of a cell", 0.99995, 2.00004), ("mid swipe", 1.4, 0), ("a settle's last sliver", 1, 0.9998)]
]
out["swipeAxes"] = [
    {"name": n, "dx": dx, "dy": dy, "axis": axis(dx, dy)}
    for n, dx, dy in [("a tap stays a tap", 6, -8), ("sideways once past the slop", -10, 3), ("down once past the slop", 2, 12), ("a diagonal goes to the longer leg", -30, 31), ("a tie goes along the rows", 20, -20)]
]
out["swipeSteps"] = [
    {"name": n, "movedPx": m, "velocityPxPerMs": v, "viewPx": w, "step": step(m, v, w)}
    for n, m, v, w in [
        ("a quarter of the view to the left moves to the next cell", -94, 0.1, 375),
        ("just short of a quarter stays", -93, -0.1, 375),
        ("a long swipe to the right moves back", 200, 0, 375),
        ("a flick moves however short", -30, -0.3, 375),
        ("a flick back wins over the way it went", -150, 0.5, 375),
        ("a slow short drag stays", 40, 0.29, 812),
        ("a quarter of a tall view upward moves down a row", -203, 0, 812),
    ]
]
out["swipeTargets"] = [
    {"name": n, "count": k, "from": cell(f), "axis": a, "step": s, "target": cell(target(f, a, s, centered_cells(k)))}
    for n, k, f, a, s in [
        ("the next column", 9, (0, 0), "x", 1, ),
        ("the row below", 9, (1, 1), "y", 1),
        ("back a column", 9, (2, 1), "x", -1),
        ("up a row", 9, (1, 2), "y", -1),
        ("no column before the first", 9, (0, 1), "x", -1),
        ("no row below the last", 9, (2, 2), "y", 1),
        ("no move without a step", 9, (1, 1), "x", 0),
        ("down into a short row lands on its occupied columns", 10, (0, 1), "y", 1),
        ("down into a short row from its far side", 10, (3, 1), "y", 1),
        ("no column past the end of a short row", 10, (2, 2), "x", 1),
    ]
]
out["rubberBands"] = [{"overshoot": o, "pull": round(rubber(o), 15)} for o in [0, 0.1, 0.5, 1, 4, -1]]
out["swipeDrags"] = [
    {"name": n, "count": k, "from": cell(f), "axis": a, "shift": s, "offset": offset(drag(f, a, s, centered_cells(k)))}
    for n, k, f, a, s in [
        ("a fifth of the way to the next column", 9, (0, 0), "x", 0.2),
        ("never past the neighbour", 9, (0, 0), "x", 1.7),
        ("back toward the row above", 9, (1, 1), "y", -0.35),
        ("across the first column toward the last one, beside the view", 9, (0, 0), "x", -1),
        ("across the last row toward the top row, below the view", 9, (1, 2), "y", 0.5),
        ("toward a short row's occupied column, both axes at once", 10, (0, 1), "y", 0.5),
        ("on a rubber band where the column holds no other page", 2, (0, 0), "y", 0.5),
    ]
]
samples = lambda a, b, d: [{"elapsedMs": e, "offset": offset(settle(a, b, e, d)[0]), "done": settle(a, b, e, d)[1]} for e in [-10, 0, 80, 160, 240, 320, 400]]
out["settles"] = [
    {"name": "a released swipe sampled every 80 ms", "from": {"x": 0.3, "y": 1}, "to": {"x": 1, "y": 1}, "durationMs": 320, "samples": samples((0.3, 1), (1, 1), 320)},
    {"name": "a rubber band springing back", "from": {"x": -0.35, "y": 0}, "to": {"x": 0, "y": 0}, "durationMs": 320, "samples": samples((-0.35, 0), (0, 0), 320)},
]
out["flings"] = [
    {"name": n, "velocityPxPerMs": v, "elapsedMs": e, "distance": round(fling(v, e)[0], 9) + 0.0, "velocity": round(fling(v, e)[1], 12) + 0.0, "done": fling(v, e)[2]}
    for n, v, e in [
        ("no time travels nothing", 2, 0),
        ("one 60 Hz frame of a fast fling", 2, 1000 / 60),
        ("a second of a fast fling", 2, 1000),
        ("a fling upward travels upward", -1.5, 250),
        ("a slow fling stops at once", 0.019, 0),
        ("a fling comes to rest", 1, 1500),
        ("a clock running backwards travels nothing", 1, -5),
    ]
]
out["neighbourWarmBoots"] = [
    {"name": n, "count": k, "at": cell(a), "live": l, "openedId": o, "budget": b, "step": neighbour_warm(k, a, set(l), o, b)}
    for n, k, a, l, o, b in [
        ("warms the first neighbour in pane order", 9, (1, 1), ["p4"], None, 4),
        ("skips a neighbour that lives", 9, (1, 1), ["p4", "p1"], None, 4),
        ("the far column and the far row are neighbours too", 9, (0, 0), ["p0", "p1", "p3"], None, 9),
        ("never a diagonal", 9, (0, 0), ["p0", "p1", "p3", "p2", "p6"], None, 9),
        ("never warms past the budget", 9, (1, 1), ["p4", "p0", "p8"], None, 3),
        ("never warms while a page is opened", 9, (1, 1), [], "p4", 4),
        ("a page of a short row wraps down to the top row", 10, (1, 2), ["p8", "p5", "p9"], None, 4),
    ]
]
DIRECTIONS = [("up", "y", -1), ("left", "x", -1), ("right", "x", 1), ("down", "y", 1)]


def landing(found):
    return None if found is None else {"cell": cell(found[0]), "slot": {"x": found[1][0], "y": found[1][1]}}


def neighbours(start, cells):
    return [{"direction": d, **landing(t)} for d, a, s in DIRECTIONS if (t := wrap_target(start, a, s, cells)) is not None]


out["swipeWrapTargets"] = [
    {"name": n, "count": k, "from": cell(f), "axis": a, "step": s, "landing": landing(wrap_target(f, a, s, centered_cells(k)))}
    for n, k, f, a, s in [
        ("inside the strip the slot is the cell", 9, (1, 1), "x", 1),
        ("past the last column to the first, coming in from the right", 9, (2, 1), "x", 1),
        ("before the first column to the last, coming in from the left", 9, (0, 0), "x", -1),
        ("above the top row to the bottom row, coming in from above", 9, (1, 0), "y", -1),
        ("below the bottom row to the top row, coming in from below", 9, (2, 2), "y", 1),
        ("past the end of a short row to its first occupied column", 10, (2, 2), "x", 1),
        ("above the top row onto the short bottom row's nearest occupied column", 10, (0, 0), "y", -1),
        ("below a short row to the top row", 10, (1, 2), "y", 1),
        ("no move without a step", 9, (1, 1), "x", 0),
        ("one page alone wraps onto itself: no move", 1, (0, 0), "x", 1),
        ("a single row wraps vertically onto itself: no move", 2, (1, 0), "y", 1),
    ]
]
out["swipeNeighbours"] = [
    {"name": n, "count": k, "from": cell(f), "neighbours": neighbours(f, centered_cells(k))}
    for n, k, f in [
        ("the centre of the ring of nine has all four, each beside it", 9, (1, 1)),
        ("the first page sees the bottom row above and the last column on its left", 9, (0, 0)),
        ("the last page sees the top row below and the first column on its right", 9, (2, 2)),
        ("one page alone has none", 1, (0, 0)),
        ("two pages side by side are each other's neighbour on both sides", 2, (0, 0)),
        ("below a short row lies its nearest occupied column", 10, (0, 1)),
        ("a page of a short row wraps within its row and down to the top", 10, (1, 2)),
    ]
]
for key, vectors in out.items():
    print(f'  "{key}": [')
    print(",\n".join("    " + json.dumps(vector, ensure_ascii=False).replace("{", "{ ").replace("}", " }") for vector in vectors))
    print("  ],")
