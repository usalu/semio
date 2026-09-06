#!/usr/bin/env python3
"""📐️ Wave B2 extent oracle — recomputes the declared work-item budget of puzzle2d's
`forceLayout`/`reorganize` and `redrawHandles` works from the same formulas the Rust
`extent()` implementations use, and sweeps the admissible document space for the worst case."""

def ceil(a, b):
    return -(-a // b) if b else 0

FORCE_UNITS_PER_STEP = 8_192
FORCE_NODES_PER_STEP = 512
FORCE_SCAN_PER_STEP = 256
FORCE_ITERATION_STAGE_STEPS = 4
FORCE_PROLOGUE_STAGE_STEPS = 8
FORCE_MAX_NODES, FORCE_MAX_EDGES, FORCE_MAX_HANDLES = 512, 4_096, 4_096
FORCE_WORK_BUDGET, FORCE_ITERATIONS_MIN, FORCE_ITERATIONS_MAX = 2_000_000, 24, 420

REDRAW_UNITS_PER_STEP = 256
REDRAW_STAGE_STEPS = 4
REDRAW_MAX_NODES, REDRAW_MAX_EDGES, REDRAW_MAX_HANDLES = 4_096, 4_096, 8_192

WORK_ITEMS = 4_096


def pairs(nodes):
    return nodes * (nodes - 1) // 2


def force_iterations(nodes, edges):
    cost = pairs(nodes) + edges
    if cost == 0:
        return 1
    return max(FORCE_ITERATIONS_MIN, min(FORCE_ITERATIONS_MAX, FORCE_WORK_BUDGET // cost))


def force_extent(nodes, edges, handles):
    if nodes > FORCE_MAX_NODES or edges > FORCE_MAX_EDGES or handles > FORCE_MAX_HANDLES:
        return None
    node_pass = max(1, ceil(nodes, FORCE_NODES_PER_STEP))
    iteration = ceil(pairs(nodes), FORCE_UNITS_PER_STEP) + ceil(edges, FORCE_UNITS_PER_STEP) + 2 * node_pass + FORCE_ITERATION_STAGE_STEPS
    items = ceil(nodes + handles + edges, FORCE_SCAN_PER_STEP) + 3 * node_pass + FORCE_PROLOGUE_STAGE_STEPS + iteration * force_iterations(nodes, edges)
    return items if items <= WORK_ITEMS else None


def redraw_extent(nodes, edges, handles):
    if nodes > REDRAW_MAX_NODES or edges > REDRAW_MAX_EDGES or handles > REDRAW_MAX_HANDLES:
        return None
    items = ceil(nodes + handles, REDRAW_UNITS_PER_STEP) + ceil(edges, REDRAW_UNITS_PER_STEP) + ceil(handles, REDRAW_UNITS_PER_STEP) + REDRAW_STAGE_STEPS
    return items if items <= WORK_ITEMS else None


CASES = [
    ("nakagin-capsule-tower", 180, 179, 358),
    ("concrete-forest", 1, 0, 11),
    ("empty board", 0, 0, 0),
    ("force ceiling", 512, 4_096, 4_096),
    ("former permitted max", 64, 512, 512),
    ("dense small graph", 98, 4_096, 4_096),
    ("redraw ceiling", 4_096, 4_096, 8_192),
]

if __name__ == "__main__":
    print(f"{'case':24}{'N':>6}{'E':>6}{'H':>6}{'iters':>7}{'force':>8}{'redraw':>8}")
    for name, n, e, h in CASES:
        print(f"{name:24}{n:6}{e:6}{h:6}{force_iterations(n, e):7}{str(force_extent(n, e, h)):>8}{str(redraw_extent(n, e, h)):>8}")
    worst = (0, None)
    for n in range(FORCE_MAX_NODES + 1):
        for e in {0, 1, n, 2 * n, 1_024, FORCE_MAX_EDGES}:
            if e > FORCE_MAX_EDGES:
                continue
            h = min(FORCE_MAX_HANDLES, 2 * n)
            value = force_extent(n, e, h)
            if value and value > worst[0]:
                worst = (value, (n, e, h))
    print(f"forceLayout worst admissible extent: {worst[0]} at (N,E,H)={worst[1]} against PUZZLE_COMMAND_WORK_ITEMS={WORK_ITEMS}")
    print(f"redrawHandles ceiling extent: {redraw_extent(REDRAW_MAX_NODES, REDRAW_MAX_EDGES, REDRAW_MAX_HANDLES)}")
