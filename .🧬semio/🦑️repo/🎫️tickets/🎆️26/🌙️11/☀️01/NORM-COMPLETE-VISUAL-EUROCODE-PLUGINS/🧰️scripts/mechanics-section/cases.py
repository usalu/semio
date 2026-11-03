"""Blessed section cases (SI metres): polygons shared by the prototype, the sectionproperties oracle and the Rust tests."""
import math


def rect(w, h):
    return [(0, 0), (w, 0), (w, h), (0, h)]


def box(x0, y0, x1, y1):
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def i_section(b, h, tf, tw):
    x0, x1 = (b - tw) / 2, (b + tw) / 2
    return [(0, 0), (b, 0), (b, tf), (x1, tf), (x1, h - tf), (b, h - tf), (b, h), (0, h), (0, h - tf), (x0, h - tf), (x0, tf), (0, tf)]


def channel(h, b, tf, tw):
    return [(0, 0), (b, 0), (b, tf), (tw, tf), (tw, h - tf), (b, h - tf), (b, h), (0, h)]


def angle(a, b, t):
    return [(0, 0), (a, 0), (a, t), (t, t), (t, b), (0, b)]


def tee(b, h, tf, tw):
    x0, x1 = (b - tw) / 2, (b + tw) / 2
    return [(x0, 0), (x1, 0), (x1, h - tf), (b, h - tf), (b, h), (0, h), (0, h - tf), (x0, h - tf)]


def mono_i(bt, bb, h, tft, tfb, tw):
    c = max(bt, bb) / 2
    return [(c - bb / 2, 0), (c + bb / 2, 0), (c + bb / 2, tfb), (c + tw / 2, tfb), (c + tw / 2, h - tft), (c + bt / 2, h - tft), (c + bt / 2, h), (c - bt / 2, h), (c - bt / 2, h - tft), (c - tw / 2, h - tft), (c - tw / 2, tfb), (c - bb / 2, tfb)]


def ring(d, n, cx=0.0, cy=0.0):
    return [(cx + d / 2 * math.cos(2 * math.pi * k / n), cy + d / 2 * math.sin(2 * math.pi * k / n)) for k in range(n)]


CASES = {
    "rectangle": (rect(0.07, 0.14), []),
    "i-section": (i_section(0.1, 0.12, 0.008, 0.006), []),
    "channel": (channel(0.2, 0.075, 0.011, 0.0085), []),
    "angle": (angle(0.1, 0.1, 0.01), []),
    "rhs": (rect(0.1, 0.06), [box(0.004, 0.004, 0.096, 0.056)]),
    "chs": (ring(0.1, 96), [ring(0.09, 96)]),
    "polygon-hole": (rect(0.1, 0.08), [[(0.02 + 0.015 * math.cos(2 * math.pi * k / 6), 0.04 + 0.015 * math.sin(2 * math.pi * k / 6)) for k in range(6)]]),
    "tee": (tee(0.1, 0.1, 0.01, 0.008), []),
    "mono-i": (mono_i(0.12, 0.08, 0.2, 0.012, 0.008, 0.007), []),
}
