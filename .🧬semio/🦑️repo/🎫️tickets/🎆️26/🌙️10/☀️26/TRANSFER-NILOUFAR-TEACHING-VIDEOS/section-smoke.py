"""🧪 A minimal authored scene for checking the section build interface."""
from manim import Scene


class MinimalScene(Scene):
    """⬛ Keep one neutral frame on screen for a short media-path check."""

    def construct(self):
        """⏱️ Render a short scene without fonts or external assets."""
        self.wait(0.1)
