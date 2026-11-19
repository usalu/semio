# Raffstore Reflection Fix

## Problem
Bounce used `bounce = -sun_dir`. That retraces the incident ray exactly, so after `Create(incoming)` the bounced segment sits on the same line and does not read as a reflection.

## Fix
Use `_mirror` off the lamella tilt, then force the result outdoors (negative x, positive y): up-and-out rejection that clearly forks from each hit point.

## Verify
Re-run Manim Sideview on `Beat5_Strahlung` and check the summer Raffstore beat: yellow rays stop on orange lamellae and leave upward-left, not through the room and not back on the same line.
