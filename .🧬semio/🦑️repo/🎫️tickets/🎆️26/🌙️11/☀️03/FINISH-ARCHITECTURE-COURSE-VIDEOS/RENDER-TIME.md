# Render Time

The course preview is `Architecture_course_speaker_en_480p15.mp4` (3:59:47, 854×480, 15 fps, 48 kHz stereo). Lessons 1–6 and 12–26 are the current 480p15 masters. Lessons 7–11 (BIM objects, representations, models, exchange, consistency) are the 2026-10-28 1080p30 masters scaled to that same format. Their new 480p renders are not published.

## What is slow

`Beat6_Consistency` is inside play 29, `show("change", seconds=5.0)` in `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔗️consistency/🐍️.py`. At 15 fps that play is 75 frames, five seconds of picture. By frame 59 it had used 22 hours, and the last frames were about 2.5 hours each. The bar’s remaining estimate was about 40 hours for the last 16 frames.

Those late frames are the third part of the succession: `FadeTransform` from the old villa section to the new one, plus an `Indicate` on the live roof build-up. The first parts (drawing a small seal, then animating paving thickness) are the earlier, faster frames of the same play. The worker for this scene held about 6.3 GB of working set.

`FadeTransform` interpolates every point of both section drawings on every frame. Each section is a full Savoye cut, including the mesh finish (about 6,000 mesh cells and about 12,000 section rings). Redrawing that vector scene at 854×480 is already heavy. Interpolating two of them is the part whose per-frame time climbed from about 37 minutes to about 2.5 hours.

`Live` rebuilds the roof build-up only when the rounded paving thickness changes. That rebuild is a handful of polygons. It is not the multi-hour cost. The section drawings are built once, before the play, and then carried on screen.

A separate unittest process (pid 68096) has been alive since 2026-11-07 and has used about 160,000 seconds of CPU and about 3.8 GB. It is competing with the render. It was left running because it holds `full-slab-v4`.

## How to make the next render finish

Replace the section `FadeTransform` with a cut: fade the old section out and the new one in, or swap them in one frame. Do not interpolate two full mesh sections.

Keep `Live` on the build-up, and suspend it during any remaining section animation so a transform does not deepcopy the updater.

For shots that do not move the building, draw the section once and leave it as a static group. Do not run the mesh sectioner again per frame. The slab cache is already there for that.

After this play, the same scene still has the later beats (kept, load, grid, derived, spread, and the rest). Any of those that transform a full section will stall the same way. Objects, representations, and models, which the waiter will render next, use the same section drawing and need the same cut instead of a cross-fade.

The consistency render was left running. Stopping it is what lets the BIM job publish exchange and start the three rerenders.
