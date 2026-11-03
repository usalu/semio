"""📉 Independent statics of the continuous EN 1994 beam spans and slab strips: the sagging moment, hogging moment and largest shear force of the designed span under a unit uniform load or a unit concentrated force at its position, solved with PyNite (`Pynite.FEModel3D`) and scaled by the load. Layouts: end span (`continuous2Span`, `endSpan`: this span and its neighbour), interior span (neighbour, this span, neighbour) and cantilever (statically determinate: the span alone, fixed at the root). Permanent actions are individual per span (EN 1990 A1.3.1): `permanent_internals` takes the largest factored effect over the arrangements with γG,sup or γG,inf per span, serviceability `internals` loads every span. A variable action the designed span (sagging) resp. the two spans next to its first internal support (hogging, EN 1990 A1.2 / 6.4.3.2); the neighbours carry the same load at the same relative position. SI throughout."""

import functools
import itertools

import numpy as np
from Pynite import FEModel3D

LAYOUTS = ("continuous2Span", "endSpan", "interiorSpan", "cantilever")
SAMPLES = 801


def _solve(spans, loads, own, cantilever=False):
    """📐 (max sagging, max hogging, max |shear|) of span `own` under `loads` (span index -> ("U", q) or ("P", F, xi))."""
    fe = FEModel3D()
    fe.add_material("s", 210e9, 80e9, 0.3, 7850)
    fe.add_section("c", 1e-2, 1e-4, 1e-4, 1e-4)
    xs = [0.0]
    for length in spans:
        xs.append(xs[-1] + length)
    for i, x in enumerate(xs):
        fe.add_node(f"N{i}", x, 0, 0)
        if cantilever:
            if i == 0:
                fe.def_support("N0", True, True, True, True, True, True)
        else:
            fe.def_support(f"N{i}", i == 0, True, True, True, False, False)
    for i in range(len(spans)):
        fe.add_member(f"M{i}", f"N{i}", f"N{i + 1}", "s", "c")
    for i, load in loads.items():
        if load[0] == "U":
            fe.add_member_dist_load(f"M{i}", "Fy", -load[1], -load[1])
        else:
            fe.add_member_pt_load(f"M{i}", "Fy", -load[1], load[2] * spans[i])
    fe.analyze(check_statics=False)
    member = fe.members[f"M{own}"]
    moment = np.array(member.moment_array("Mz", SAMPLES))[1]
    shear = np.array(member.shear_array("Fy", SAMPLES))[1]
    return max(float(-moment.min()), 0.0), max(float(moment.max()), 0.0), float(np.abs(shear).max())


@functools.lru_cache(maxsize=None)
def unit_effects(support, span, adjacent, kind, xi):
    """🧮 (sagging, hogging, shear) of the designed span per unit load (xi None: a unit line load, else a unit force at the fraction xi) for a "permanent" or "variable" action; `adjacent` of 0 is a neighbour of equal length."""
    adj = adjacent if adjacent > 1e-9 else span
    if support == "cantilever":
        spans, own, cantilever = [span], 0, True
    elif support == "interiorSpan":
        spans, own, cantilever = [adj, span, adj], 1, False
    else:
        spans, own, cantilever = [span, adj], 0, False
    unit = ("U", 1.0) if xi is None else ("P", 1.0, xi)
    count = len(spans)
    if kind == "permanent":
        sagging_on = hogging_on = list(range(count))
    else:
        node = 1 if own == 0 else own
        sagging_on = [own]
        hogging_on = [i for i in range(count) if i + 1 == node or i == node]
    first = _solve(spans, {i: unit for i in sagging_on}, own, cantilever)
    second = first if kind == "permanent" else _solve(spans, {i: unit for i in hogging_on}, own, cantilever)
    return first[0], second[1], max(first[2], second[2])


@functools.lru_cache(maxsize=None)
def arrangement_effects(support, span, adjacent, xi, gamma_sup, gamma_inf):
    """🪜 (sagging, hogging, shear) of the designed span per unit permanent load, the largest over the 2^n arrangements with γG,sup or γG,inf on every span (EN 1990 A1.3.1)."""
    adj = adjacent if adjacent > 1e-9 else span
    if support == "cantilever":
        spans, own, cantilever = [span], 0, True
    elif support == "interiorSpan":
        spans, own, cantilever = [adj, span, adj], 1, False
    else:
        spans, own, cantilever = [span, adj], 0, False
    best = (0.0, 0.0, 0.0)
    for factors in itertools.product((gamma_sup, gamma_inf), repeat=len(spans)):
        found = _solve(spans, {i: ("U", f) if xi is None else ("P", f, xi) for i, f in enumerate(factors)}, own, cantilever)
        best = tuple(max(a, b) for a, b in zip(best, found))
    return best


def permanent_internals(action, span, adjacent, support, spacing, gamma_sup, gamma_inf):
    """🪜 Factored sagging, hogging and shear of a permanent action on the designed span, γG,sup on the unfavourable spans and γG,inf on the others."""
    q = action["qAreaPa"] * spacing
    if abs(q) > 1e-12:
        sag, hog, v = arrangement_effects(support, span, adjacent, None, gamma_sup, gamma_inf)
        return sag * q, hog * q, v * q
    if abs(action["fKN"]) > 1.0:
        x = action.get("xM", 0.0)
        xi = min(max(x / span, 0.0), 1.0) if x > 0.0 else 0.5
        sag, hog, v = arrangement_effects(support, span, adjacent, round(xi, 12), gamma_sup, gamma_inf)
        return sag * action["fKN"], hog * action["fKN"], v * action["fKN"]
    return 0.0, 0.0, 0.0


def internals(action, span, adjacent, support, spacing):
    """📐 Sagging, hogging and shear of one action on the designed span of a continuous layout: a line load by `qAreaPa`, else a concentrated force `fKN` at `xM` from the first support of the span (unset: mid-span)."""
    q = action["qAreaPa"] * spacing
    kind = "permanent" if action["kind"] == "permanent" else "variable"
    if abs(q) > 1e-12:
        sag, hog, v = unit_effects(support, span, adjacent, kind, None)
        return sag * q, hog * q, v * q
    if abs(action["fKN"]) > 1.0:
        x = action.get("xM", 0.0)
        xi = min(max(x / span, 0.0), 1.0) if x > 0.0 else 0.5
        sag, hog, v = unit_effects(support, span, adjacent, kind, round(xi, 12))
        return sag * action["fKN"], hog * action["fKN"], v * action["fKN"]
    return 0.0, 0.0, 0.0
