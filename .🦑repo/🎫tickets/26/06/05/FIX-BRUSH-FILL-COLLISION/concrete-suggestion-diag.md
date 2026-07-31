# Concrete forest brush suggestion diagnosis

Target: `seed-left-001:v0` (`b-p1-t-t1-c3-l`, port `b-l`).

## Root cause

Brush collision probed overlap against the **attachment host** object. Mating a new piece at a host connector necessarily overlaps the host solid at the joint, so valid beam mates (including self) were rejected.

## Fix

Exclude `target.objectId` from brush/fill collision probes (TS `brushCollisionFreeCandidates`, `fillPreviewCollidesAccumulated`; Rust `brush_collision_free`, `fill_step_one`).

## Before fix (real GLB, budget 0.02)

- Compatible: 14 beam connectors
- Collision-free: 11 (missing `b-p1-t-t2-c1-l`, `b-p2-t-t2-c3-l`, `b-p2-t-t1-c1-l`)

## After fix (real GLB, budget 0.02)

- Collision-free: all 14 beam connectors including self `b-p1-t-t1-c3-l`
