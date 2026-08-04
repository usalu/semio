"""🔤 Resolve a real serif face for Manim ``Text`` across macOS, Windows, and Linux.

CSS generics like ``\"Serif\"`` appear in Pango's font list but are not installed
faces — glyph coverage and metrics become machine-dependent. Prefer a concrete
family that ships with the OS or common free font packs.
"""

from __future__ import annotations

#region Candidates
# Prefer readable book serifs first; Liberation/DejaVu cover Linux CI/devcontainers.
_SERIF_CANDIDATES: tuple[str, ...] = (
    "Georgia",
    "PT Serif",
    "Times New Roman",
    "Liberation Serif",
    "DejaVu Serif",
    "STIX Two Text",
    "Nimbus Roman",
    "FreeSerif",
    "Palatino",
)
#endregion


#region Resolve
def resolve_serif_font() -> str:
    """📚 Pick the first installed candidate; never return the CSS generic ``Serif``."""
    try:
        import manimpango

        available = set(manimpango.list_fonts())
    except Exception:
        available = set()
    for name in _SERIF_CANDIDATES:
        if not available or name in available:
            return name
    return "Georgia"


BODY_FONT: str = resolve_serif_font()
#endregion


#region Scene helper
def apply_body_font() -> str:
    """✍️ Set Manim ``Text`` default to the resolved body face; return its name."""
    from manim import Text

    Text.set_default(font=BODY_FONT)
    return BODY_FONT
#endregion


#region Scene title
# One look for the top-middle chapter header of every tutorial scene.
TITLE_FONT_SIZE: float = 30
TITLE_COLOR: str = "#FFFFFF"
TITLE_EDGE_BUFF: float = 0.35
TITLE_RUN_TIME: float = 1.4


def scene_title(text: str):
    """🏔️ Top-middle chapter header — same face, size, colour and placement everywhere."""
    from manim import Text, UP

    title = Text(text, font=BODY_FONT, font_size=TITLE_FONT_SIZE, color=TITLE_COLOR)
    title.to_edge(UP, buff=TITLE_EDGE_BUFF)
    title.set_x(0)
    return title


def play_scene_title(scene, title, run_time: float = TITLE_RUN_TIME) -> None:
    """✍️ The single intro animation for a chapter header."""
    from manim import Write

    scene.play(Write(title), run_time=run_time)
#endregion
