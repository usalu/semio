# Before Implementation

All five new audience cases failed against the original pipeline: compose_videos rejected audience, the CLI only composed one variant, the audience contract was absent, render_segment passed no audience to Manim, and independent PyAV/NumPy decoding detected changing subtitle pixels in the spectator frames. Native fixture output was generated under this ticket and temporary test directories were removed automatically.

The first implementation passed four checks; the pixel oracle exposed H.264 quantization drift around the unchanged central square. Refined the assertion to require a uniformly empty spectator subtitle region for every frame and bound scene-region compression error independently.
