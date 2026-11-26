#!/usr/bin/env python3
"""Independent Python oracle for the TUI chart scaling math (ticket FLEET-MONITORING, lane M1).

Re-implements from the written spec, not from the Rust source: quantization, dot rows, column resampling, tier columns,
value formatting, gauge levels and the sparkline text. Writes the language-agnostic fixture that the Rust test
`🔬️charts` reads: `python3 oracle.py <fixture.json>`. Prints the fixture to stdout without an argument.
"""
import json
import math
import sys

FIXED_PY = lambda x, d: f"{x:.{d}f}"
RAMP = "▁▂▃▄▅▆▇█"
LABELS = {
    "en": {"decimal": ".", "ms": "ms", "s": "s", "min": "min", "h": "h", "d": "d", "no_value": "-"},
    "de": {"decimal": ",", "ms": "ms", "s": "s", "min": "Min", "h": "Std", "d": "T", "no_value": "-"},
}


def floor_half_up(x):
    return math.floor(x + 0.5)


def eighths(v, lo, hi, levels):
    if hi <= lo or levels == 0:
        return max((levels + 1) // 2, 1)
    n = min(max((v - lo) / (hi - lo), 0.0), 1.0)
    return 1 + floor_half_up(n * (levels - 1))


def row_fill(height, row):
    return min(max(height - row * 8, 0), 8)


def dot_row(v, lo, hi, dots):
    n = 0.5 if hi <= lo else min(max((v - lo) / (hi - lo), 0.0), 1.0)
    return floor_half_up((1.0 - n) * max(dots - 1, 0))


def filled_eighths(fraction, cells):
    return floor_half_up(min(max(fraction, 0.0), 1.0) * cells * 8.0)


def level_of(fraction, warn, crit, polarity):
    if polarity == "high":
        c, w = fraction >= crit, fraction >= warn
    else:
        c, w = fraction <= crit, fraction <= warn
    return "critical" if c else ("warn" if w else "ok")


def eff_end(b):
    return max(b[1], b[0] + 1)


def valid(b):
    s = b[2]
    return s if s is not None and all(math.isfinite(x) for x in s) else None


def vspan(i, b, axis):
    return (b[0], eff_end(b)) if axis == "time" else (i, i + 1)


def domain(sets, axis):
    if axis == "index":
        return (0, len(sets[0])) if sets and sets[0] else None
    allb = [b for s in sets for b in s]
    if not allb:
        return None
    start = min(b[0] for b in allb)
    end = max(eff_end(b) for b in allb)
    return (start, max(end, start + 1))


def columns(buckets, axis, dom, width):
    v0, v1 = dom
    span = max(v1 - v0, 1)
    out = []
    for c in range(width):
        c0 = v0 + span * c // width
        c1 = max(v0 + span * (c + 1) // width, c0 + 1)
        acc = None
        for i, b in enumerate(buckets):
            s, e = vspan(i, b, axis)
            ov = min(e, c1) - max(s, c0)
            st = valid(b)
            if ov <= 0 or st is None:
                continue
            if acc is None:
                acc = [st[0], st[1], st[2] * ov, st[3], ov]
            else:
                acc = [min(acc[0], st[0]), max(acc[1], st[1]), acc[2] + st[2] * ov, st[3], acc[4] + ov]
        out.append(None if acc is None else [acc[0], acc[1], acc[2] / acc[4], acc[3]])
    return out


def tier_starts(buckets):
    return [i for i in range(1, len(buckets)) if buckets[i][1] - buckets[i][0] != buckets[i - 1][1] - buckets[i - 1][0]]


def position_column(pos, dom, width):
    span = max(dom[1] - dom[0], 1)
    return min(max(pos - dom[0], 0) * width // span, max(width - 1, 0))


def tier_columns(buckets, axis, dom, width):
    return [[position_column(vspan(i, buckets[i], axis)[0], dom, width), i] for i in tier_starts(buckets)]


def pick(st, agg):
    return st[{"mean": 2, "last": 3, "min": 0, "max": 1}[agg]]


def extent(buckets, agg, band):
    lo = hi = None
    for b in buckets:
        st = valid(b)
        if st is None:
            continue
        v = pick(st, agg)
        l, h = (min(st[0], v), max(st[1], v)) if band else (v, v)
        lo = l if lo is None else min(lo, l)
        hi = h if hi is None else max(hi, h)
    return None if lo is None else (lo, hi)


def spark(case):
    buckets, width, rows = case["buckets"], case["width"], case["rows"]
    ext = extent(buckets, case["aggregate"], case["band"])
    rng = case["range"]
    res = (min(rng), max(rng)) if rng else ext
    dom = domain([buckets], case["axis"])
    cols = columns(buckets, case["axis"], dom, width) if (dom and res) else [None] * width
    lo, hi = res if res else (0.0, 0.0)
    levels = rows * 8
    grid = [[" "] * width for _ in range(rows)]
    for x, col in enumerate(cols):
        if col is None:
            grid[rows - 1][x] = "·"
            continue
        h = eighths(pick(col, case["aggregate"]), lo, hi, levels)
        bl, bh = eighths(col[0], lo, hi, levels), eighths(col[1], lo, hi, levels)
        for r in range(rows):
            f = row_fill(h, r)
            if f > 0:
                ch = RAMP[f - 1]
            elif case["band"] and bh > r * 8 and bl <= r * 8 + 8:
                ch = "░"
            else:
                ch = " "
            grid[rows - 1 - r][x] = ch
    return ["".join(r) for r in grid]


def digits(m):
    return 0 if m >= 100 else (1 if m >= 10 else 2)


def scaled(m, base, steps, whole_first):
    step = 0
    while step + 1 < len(steps) and m >= base ** (step + 1):
        step += 1
    while True:
        v = m / base ** step
        places = 0 if step == 0 and (whole_first or v == math.floor(v)) else digits(v)
        text = FIXED_PY(v, places)
        if step + 1 < len(steps) and float(text) >= base:
            step += 1
            continue
        return text + steps[step]


def duration(ms, L):
    def pair(a, au, b, bu):
        return f"{a}{au}" if b == 0 else f"{a}{au} {b}{bu}"
    if ms < 1000:
        return f"{ms}{L['ms']}"
    if ms < 10_000:
        return f"{ms // 1000}.{ms % 1000 // 100}{L['s']}"
    if ms < 60_000:
        return f"{ms // 1000}{L['s']}"
    if ms < 3_600_000:
        return pair(ms // 60_000, L["min"], ms % 60_000 // 1000, L["s"])
    if ms < 86_400_000:
        return pair(ms // 3_600_000, L["h"], ms % 3_600_000 // 60_000, L["min"])
    return pair(ms // 86_400_000, L["d"], ms % 86_400_000 // 3_600_000, L["h"])


def format_value(v, unit, locale):
    L = LABELS[locale]
    if not math.isfinite(v):
        return L["no_value"]
    m = abs(v)
    count = ["", "k", "M", "G", "T"]
    byt = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"]
    if unit == "count":
        body = scaled(m, 1000.0, count, False)
    elif unit == "bytes":
        body = scaled(m, 1024.0, byt, True)
    elif unit == "percent":
        body = FIXED_PY(m, 1 if m < 10 else 0) + "%"
    elif unit == "rate":
        body = scaled(m, 1000.0, count, False) + "/s"
    elif unit == "byte_rate":
        body = scaled(m, 1024.0, byt, True) + "/s"
    else:
        body = duration(int(floor_half_up(m)), L)
    return ("-" if v < 0 else "") + body.replace(".", L["decimal"])


def build():
    out = {"schema": "semio-ui-tui-charts-fixture/1"}
    out["eighths"] = [
        {"v": v, "lo": lo, "hi": hi, "levels": n, "expect": eighths(v, lo, hi, n)}
        for v, lo, hi, n in [(0, 0, 10, 8), (10, 0, 10, 8), (5, 0, 10, 8), (5.0, 5.0, 5.0, 8), (3.3, 0, 10, 8), (-4, 0, 10, 8), (99, 0, 10, 8), (0.07, 0, 1, 24), (0.5, 0, 1, 24), (2, 2, 2, 24), (7, 0, 100, 16)]
    ]
    out["dot_row"] = [
        {"v": v, "lo": lo, "hi": hi, "dots": n, "expect": dot_row(v, lo, hi, n)}
        for v, lo, hi, n in [(0, 0, 10, 8), (10, 0, 10, 8), (5, 0, 10, 8), (5, 5, 5, 8), (1, 0, 3, 12), (2.5, 0, 3, 12), (7, 0, 100, 4), (200, 0, 100, 4)]
    ]
    out["filled_eighths"] = [{"fraction": f, "cells": c, "expect": filled_eighths(f, c)} for f, c in [(0, 10), (1, 10), (0.5, 10), (0.333, 7), (1.7, 5), (-1, 5), (0.073, 20)]]
    out["level"] = [
        {"fraction": f, "warn": w, "critical": c, "polarity": p, "expect": level_of(f, w, c, p)}
        for f, w, c, p in [(0.5, 0.7, 0.9, "high"), (0.7, 0.7, 0.9, "high"), (0.9, 0.7, 0.9, "high"), (1.2, 0.7, 0.9, "high"), (0.5, 0.2, 0.1, "low"), (0.2, 0.2, 0.1, "low"), (0.05, 0.2, 0.1, "low"), (0.0, 0.7, 0.9, "high")]
    ]
    tiers = [[0, 60000, [1, 3, 2, 2]], [60000, 120000, [2, 4, 3, 3]], [120000, 180000, None], [180000, 180000 + 1000, [5, 5, 5, 5]], [181000, 182000, [6, 8, 7, 7]], [182000, 183000, [1, 1, 1, 1]], [183000, 184000, [4, 4, 4, 4]]]
    cols = []
    for name, axis, dom, width, buckets in [
        ("uniform", "time", [0, 4000], 8, [[i * 1000, i * 1000 + 1000, [i, i + 1, i + 0.5, i + 0.75]] for i in range(4)]),
        ("gap", "time", [0, 4000], 8, [[0, 1000, [1, 1, 1, 1]], [1000, 2000, None], [2000, 3000, None], [3000, 4000, [9, 9, 9, 9]]]),
        ("tiers_time", "time", [0, 184000], 23, tiers),
        ("tiers_index", "index", [0, 7], 23, tiers),
        ("narrow", "time", [0, 184000], 3, tiers),
        ("wider_than_buckets", "index", [0, 2], 7, [[0, 10, [1, 2, 1.5, 2]], [10, 20, [3, 4, 3.5, 4]]]),
        ("nan_is_gap", "time", [0, 2000], 4, [[0, 1000, [float("nan"), 1, 1, 1]], [1000, 2000, [2, 2, 2, 2]]]),
    ]:
        cols.append({"name": name, "axis": axis, "domain": dom, "width": width, "buckets": [[b[0], b[1], b[2]] for b in buckets], "expect": columns(buckets, axis, tuple(dom), width), "tier_columns": tier_columns(buckets, axis, tuple(dom), width)})
    out["columns"] = cols
    out["domain"] = [{"axis": a, "buckets": tiers, "expect": list(domain([tiers], a))} for a in ("time", "index")]

    def sp(name, width, rows, buckets, axis="time", rng=None, agg="mean", band=False):
        case = {"name": name, "width": width, "rows": rows, "axis": axis, "range": rng, "aggregate": agg, "band": band, "buckets": buckets}
        case["expect"] = spark(case)
        return case

    sec = lambda i, st: [i * 1000, i * 1000 + 1000, st]
    p = lambda v: [v, v, v, v]
    out["spark"] = [
        sp("empty", 8, 1, []),
        sp("all_gaps", 6, 1, [[0, 1000, None], [1000, 2000, None]]),
        sp("gaps", 8, 1, [sec(0, p(1)), sec(1, p(2)), sec(2, None), sec(3, None), sec(4, p(8)), sec(5, p(4)), sec(6, None), sec(7, p(5))]),
        sp("one_point", 6, 1, [sec(0, p(42))]),
        sp("flat", 8, 1, [sec(i, p(3)) for i in range(8)]),
        sp("flat_fixed_range", 8, 1, [sec(i, p(3)) for i in range(8)], rng=[0, 6]),
        sp("ramp", 8, 1, [sec(i, p(i)) for i in range(8)]),
        sp("spikes", 12, 1, [sec(i, p(100 if i in (3, 8) else 1)) for i in range(12)]),
        sp("spike_max_aggregate", 4, 1, [sec(0, [1, 1, 1, 1]), sec(1, [1, 90, 5, 1]), sec(2, [1, 1, 1, 1]), sec(3, [1, 1, 1, 1])], agg="max"),
        sp("band", 8, 3, [sec(i, [i, i + 6, i + 3, i + 2]) for i in range(8)], band=True),
        sp("band_off", 8, 3, [sec(i, [i, i + 6, i + 3, i + 2]) for i in range(8)], band=False),
        sp("narrow", 3, 1, [sec(i, p(i)) for i in range(8)]),
        sp("narrow_mean_of_columns", 2, 1, [sec(0, p(0)), sec(1, p(10)), sec(2, p(0)), sec(3, p(10))]),
        sp("tiers_time", 23, 1, tiers),
        sp("tiers_index", 23, 1, tiers, axis="index"),
        sp("fixed_range_clamps", 6, 1, [sec(i, p(v)) for i, v in enumerate([-5, 0, 5, 10, 15, 20])], rng=[0, 10]),
        sp("last_aggregate", 4, 1, [sec(i, [0, 10, 5, v]) for i, v in enumerate([0, 3, 6, 10])], agg="last"),
    ]
    fmt = []
    vals = {"count": [0, 7, 42.5, 999, 1000, 1234, 15_000, 2_500_000, 999_999, 3.2e9, 0.5, -12], "bytes": [0, 1, 1023, 1023.6, 1024, 1536, 10 * 1024, 1048575, 1048576, 5 * 1024 ** 3, 1234 * 1024 ** 4, 2 * 1024 ** 5, 3 * 1024 ** 6], "percent": [0, 5.25, 9.96, 10, 73.4, 100, 250, -3.5], "rate": [0, 0.5, 12, 1500, 2_000_000], "byte_rate": [0, 1024, 3 * 1024 ** 2, 5.5 * 1024 ** 3], "duration": [0, 999, 1000, 2500, 9999, 10_000, 59_999, 60_000, 65_000, 3_599_999, 3_600_000, 5_400_000, 86_399_999, 86_400_000, 90_000_000, 3 * 86_400_000 + 5 * 3_600_000, 1500.4]}
    for unit, vs in vals.items():
        for loc in ("en", "de"):
            for v in vs:
                u = "duration" if unit == "duration" else unit
                fmt.append({"unit": "duration_ms" if unit == "duration" else unit, "locale": loc, "v": v, "expect": format_value(float(v), "duration_ms" if unit == "duration" else unit, loc)})
    fmt.append({"unit": "bytes", "locale": "en", "v": None, "expect": "-"})
    out["format"] = fmt
    return out


def jsonable(x):
    if isinstance(x, float) and not math.isfinite(x):
        return "nan"
    if isinstance(x, list):
        return [jsonable(i) for i in x]
    if isinstance(x, dict):
        return {k: jsonable(v) for k, v in x.items()}
    return x


if __name__ == "__main__":
    text = json.dumps(jsonable(build()), indent=1, ensure_ascii=False)
    if len(sys.argv) > 1:
        import os
        tmp = sys.argv[1] + ".tmp"
        open(tmp, "w").write(text + "\n")
        os.replace(tmp, sys.argv[1])
    else:
        print(text)
