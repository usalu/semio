# Fix Solar Beat4 Sun Sync
- Rays/beams rebuilt from sun.get_center() + roof/window after stage fit
- Beams include sun vertex so GrowFromPoint stays aligned
- MoveAlongPath uses ArcBetweenPoints(sun, winter_anchor)
- Winter caption after summer FadeOut (was flipping early)
