# Full Room Temperature Fill And Floor Rays

Scene: `Beat5_Nachtlueftung`.

## Problem
1. Red/blue Speichermasse fill was a short slab (`height=0.5`) sitting on the floor — looked half-full and overlapped the floor stroke.
2. Sun rays used `mass.get_top()` as land height, so they stopped mid-room.

## Fix
1. Size the fill to the clear air box of `_room` with padding so it stays inside walls/floor/ceiling.
2. Aim `sun_rays(..., land_y=floor_y)` at the padded floor face.

Also raised `ray_pack["in"]` opacity from 0.28 to 0.45 so the floor-reaching legs stay visible.
