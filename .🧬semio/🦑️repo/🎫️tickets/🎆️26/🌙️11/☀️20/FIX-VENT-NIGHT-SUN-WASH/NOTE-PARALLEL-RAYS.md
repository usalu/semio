# Parallel Sun Rays

## Problem
A single `Polygon` wash converged toward the sun — radiation looked gathered in one point, unlike cooling / physics scenes.

## Fix
Use `sun_rays(sun, glass_x, ys, land_y)` + `shine(...)`: every ray shares one direction (parallel), starts clear of the sun, hits the window, then continues dimmer into the Speichermasse. Verschattung is drawn on top.

## Check
Shade beat (~16s): five parallel yellow strokes from sun through facade; weaker continuations inside the room.
