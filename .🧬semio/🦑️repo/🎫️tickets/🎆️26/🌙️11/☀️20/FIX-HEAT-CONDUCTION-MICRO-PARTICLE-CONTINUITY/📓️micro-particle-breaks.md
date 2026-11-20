# Micro Lattice Particle Breaks (Heizen / Wärmeleitung Beat1)

## Symptom

In the microscopic lattice beat, particle jiggling stops in chunks during
subtitle holds, then resumes — visible breaks.

## Cause

`hold_for` called `scene.wait(remaining)` with default `frozen_frame=None`.
Manim freezes the wait when no updater declares a `dt` parameter. The lattice
and `scene_timer` updaters are `(mob) -> …` only, so narration waits render as
static frames while the clock later jumps.

A second discontinuity: `is_hot` flips amp/freq instantly as the heat front
sweeps (`0.012/8` → `0.07/24`), so each particle pops when heated.

## Fix

1. `hold_for` waits with `frozen_frame=False` so ambient updaters keep running
   (matches its docstring: heat keeps rippling while the subtitle is read).
2. Lattice heat is a smooth 0→1 factor with fundamentals-style amp/freq blend;
   particle updaters take `dt` so auto-detect also stays live.
