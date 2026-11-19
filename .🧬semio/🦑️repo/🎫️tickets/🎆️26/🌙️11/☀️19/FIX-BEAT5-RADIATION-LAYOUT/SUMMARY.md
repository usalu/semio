# Summary

Fixed false Raffstore reflection: bounced rays were collinear with incoming (`-sun_dir`), so reflection did not read.

Bounce is now specular off the lamella tilt, forced up-and-left outdoors. Details: `REFLECTION-FIX.md`.
