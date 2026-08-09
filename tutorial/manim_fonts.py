"""🔤 Resolve a real serif face for Manim ``Text`` across macOS, Windows, and Linux.

CSS generics like ``\"Serif\"`` appear in Pango's font list but are not installed
faces — glyph coverage and metrics become machine-dependent. Prefer a concrete
family that ships with the OS or common free font packs.

Also owns the one fixed type scale (title/subtitle/body/label/formula sizes)
and the one-call scene style setup — every scene draws sizes from here instead
of picking numbers by eye, per the standardized-template rules in
``tutorial/.agents/skills/generate-manim-tutorial/SKILL.md``.
"""

from __future__ import annotations

from pathlib import Path as _Path

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

# manimpango.list_fonts() asks fontconfig to enumerate every installed font.
# On a cold fontconfig cache (fresh devcontainer, fresh clone, right after
# installing a font) that first enumeration can hang for minutes — the
# process sits nearly idle, not crashed, so it looks like a broken render.
# Every scene calls apply_body_font() at import time, so every scene paid
# that cost. Cache the resolved name on disk: only the very first render
# after a cold cache ever waits: every one after just reads a one-line file.
_FONT_CACHE_PATH = _Path(__file__).resolve().parent / ".manim_font_cache"
#endregion


#region Resolve
def resolve_serif_font() -> str:
    """📚 Pick the first installed candidate; never return the CSS generic ``Serif``."""
    try:
        cached = _FONT_CACHE_PATH.read_text().strip()
    except OSError:
        cached = ""
    if cached:
        return cached

    try:
        import manimpango

        available = set(manimpango.list_fonts())
    except Exception:
        available = set()
    resolved = "Georgia"
    for name in _SERIF_CANDIDATES:
        if not available or name in available:
            resolved = name
            break

    try:
        _FONT_CACHE_PATH.write_text(resolved)
    except OSError:
        pass
    return resolved


BODY_FONT: str = resolve_serif_font()
#endregion


#region Scene helper
# Manim's default line_spacing scale (0.3) crowds Georgia/serif multi-line
# Text — captions and labels look glued together until each scene hand-tunes
# a different value and the next revision forgets. One shared scale + one
# center-aligned multi-line builder keep Heating/Cooling consistent.
#
# ``disable_ligatures=True`` is mandatory: Pango ligatures on Georgia/PT Serif
# collapse German clusters (``ss``, ``ch``, ``ämm``) into overlapping or
# unevenly spaced glyphs — the "fonts don't render well" look in finished
# videos. Every body ``Text`` goes through ``body_text`` / these defaults.
BODY_LINE_SPACING: float = 0.75
BODY_LINE_BUFF: float = 0.10


def apply_body_font() -> str:
    """✍️ Set Manim ``Text`` defaults: resolved face, line spacing, no ligatures."""
    from manim import Text

    Text.set_default(
        font=BODY_FONT,
        line_spacing=BODY_LINE_SPACING,
        disable_ligatures=True,
    )
    return BODY_FONT


def apply_scene_style(scene) -> str:
    """🎬 One call, first line of every ``construct()``: dark background + resolved body font.

    Replaces the two hand-typed steps (``self.camera.background_color = P_DEEP_DARK``
    then ``apply_body_font()``) that drifted apart across topic folders — see
    the standardized-template rules in the ``generate-manim-tutorial`` skill.
    """
    from manim_visuals import P_DEEP_DARK

    scene.camera.background_color = P_DEEP_DARK
    return apply_body_font()


def body_text(text: str, *, font_size: float, color: str, **kwargs):
    """🔤 One on-screen string with the shared face — never a bare ``Text(...)`` in scenes."""
    from manim import Text

    return Text(
        text,
        font=BODY_FONT,
        font_size=font_size,
        color=color,
        disable_ligatures=True,
        **kwargs,
    )


def wrap_body_lines(text: str, *, font_size: float, max_width: float) -> list[str]:
    """📐 Word-wrap at ``max_width`` while honouring explicit ``\\n`` breaks; size stays fixed."""
    paragraphs = text.split("\n")
    wrapped: list[str] = []
    for para in paragraphs:
        words = para.split()
        if not words:
            continue
        current = words[0]
        for word in words[1:]:
            trial = f"{current} {word}"
            if body_text(trial, font_size=font_size, color="#FFFFFF").width <= max_width:
                current = trial
            else:
                wrapped.append(current)
                current = word
        wrapped.append(current)
    return wrapped or [""]


def centered_body_text(
    text: str,
    *,
    font_size: float,
    color: str,
    line_buff: float | None = None,
    max_width: float | None = None,
):
    """📝 Center-aligned body copy; ``\\n`` / auto-wrap become a VGroup with a fixed vertical gap.

    A single Pango ``Text("a\\nb")`` left-aligns lines inside the SVG and inherits
    flaky vertical metrics — building one ``Text`` per line and arranging them
    with ``center=True`` is the stable path for titles, beat subtitles, and captions.
    """
    from manim import DOWN, VGroup

    if line_buff is None:
        line_buff = BODY_LINE_BUFF
    if max_width is None:
        lines = [part.strip() for part in text.split("\n") if part.strip()] or [""]
    else:
        lines = wrap_body_lines(text, font_size=font_size, max_width=max_width)
    mobs = [body_text(line, font_size=font_size, color=color) for line in lines]
    if len(mobs) == 1:
        return mobs[0]
    group = VGroup(*mobs)
    group.arrange(DOWN, buff=line_buff, center=True)
    return group
#endregion


#region Type scale
# The one fixed set of sizes every scene draws from — never a bespoke literal.
#
# Sized against the 8-unit frame height, not by eye: a Text's cap height is
# ≈ 0.0126 × font_size units, so the percentages below are what a viewer
# actually sees. The previous scale put labels at 2.2 % and body at 2.5 % of
# frame height, which read as too small in the finished videos; anything a
# viewer has to read while a diagram moves wants ≈ 3 % or more.
TITLE_FONT_SIZE: float = 34      # 5.3 % — top-center chapter/topic heading
# Subtitle and caption trimmed back down slightly from the first legibility pass
# (23/25) — they read a touch large next to the rest of the scale; body/label/
# title/formula stay put, those were the genuinely-too-small sizes.
SUBTITLE_FONT_SIZE: float = 21   # 3.3 % — beat subtitle under the heading
BODY_FONT_SIZE: float = 20       # 3.2 % — standard on-screen label
LABEL_FONT_SIZE: float = 17      # 2.7 % — small callouts, unit tags, legend rows
FORMULA_FONT_SIZE: float = 30    # 4.7 % — the dedicated formula panel
CAPTION_FONT_SIZE: float = 23    # 3.6 % — bottom-edge German subtitle bar
#endregion


#region Scene title
# One look for the top-middle chapter header of every tutorial scene.
TITLE_COLOR: str = "#FFFFFF"
TITLE_EDGE_BUFF: float = 0.35
TITLE_RUN_TIME: float = 1.4


def scene_title(text: str):
    """🏔️ Top-middle chapter header — same face, size, colour and placement everywhere."""
    from manim import UP

    title = centered_body_text(text, font_size=TITLE_FONT_SIZE, color=TITLE_COLOR)
    title.to_edge(UP, buff=TITLE_EDGE_BUFF)
    title.set_x(0)
    return title


def play_scene_title(scene, title, run_time: float = TITLE_RUN_TIME) -> None:
    """✍️ The single intro animation for a chapter header."""
    from manim import Write

    scene.play(Write(title), run_time=run_time)
#endregion


#region Beat subtitle
# Same teal line under the module title on every Heating/Cooling beat —
# font, size, colour, gap and x-centering must not drift per file.
BEAT_SUBTITLE_BUFF: float = 0.22
BEAT_SUBTITLE_FADE: float = 0.3


def beat_subtitle(text: str, title):
    """📎 Per-beat subtitle centered under the persistent module title."""
    from manim import DOWN
    from manim_visuals import P_TEAL

    sub = centered_body_text(text, font_size=SUBTITLE_FONT_SIZE, color=P_TEAL)
    sub.next_to(title, DOWN, buff=BEAT_SUBTITLE_BUFF)
    sub.set_x(0)
    return sub
#endregion
