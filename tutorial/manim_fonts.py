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


#region Pango kerning fix (ManimCommunity/manim#2844)
# Manim rasterises a ``Text`` at the given ``font_size`` and only then scales the
# SVG down by ``TEXT_MOB_SCALE_FACTOR``. Below ~30 px Pango quantises every glyph
# advance to a whole pixel, so the letters in a 17–24 px caption or label sit at
# visibly uneven gaps — the "our fonts render badly" look this project kept
# chasing across revisions. Root cause and fix are in
# https://github.com/ManimCommunity/manim/issues/2844 : rasterise the text at a
# comfortably large size, where Pango has the sub-pixel room to space glyphs
# correctly, then scale the finished mobject down to whatever size the scene
# asked for. The kerning is decided at the large size and survives the scale.
#
# Applied once, by wrapping ``Text.__init__``, so it covers every path with no
# per-scene edits: ``body_text()`` and the shared helpers below, the bare
# ``Text(...)`` calls still scattered through the Heating ``scene_*.py`` files,
# and the legacy ``final_calculation/merged_scenes.py``. A scene that sizes a
# ``Text`` by an explicit ``width=``/``height=`` box is left untouched (scaling
# it afterwards would fight that box).
_KERNING_SAFE_FONT_SIZE: float = 60.0
_TEXT_KERNING_PATCHED: bool = False


def install_text_kerning_fix() -> None:
    """🔧 Wrap ``Text.__init__`` so small text is rasterised large, then scaled down."""
    global _TEXT_KERNING_PATCHED
    if _TEXT_KERNING_PATCHED:
        return

    from manim import DEFAULT_FONT_SIZE, Text

    _original_init = Text.__init__

    def _kerned_init(self, *args, font_size: float | None = None, **kwargs):
        target = float(DEFAULT_FONT_SIZE if font_size is None else font_size)
        boxed = kwargs.get("height") is not None or kwargs.get("width") is not None
        render = target if (boxed or target >= _KERNING_SAFE_FONT_SIZE) else _KERNING_SAFE_FONT_SIZE
        _original_init(self, *args, font_size=render, **kwargs)
        if render != target:
            self.scale(target / render)

    Text.__init__ = _kerned_init
    _TEXT_KERNING_PATCHED = True


install_text_kerning_fix()
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
# unevenly spaced glyphs. It is the companion to the render-large-then-scale
# kerning fix above (ManimCommunity/manim#2844): ligatures off removes the bad
# clusters, large rasterisation removes the uneven gaps between the rest. Every
# body ``Text`` goes through ``body_text`` / these defaults.
BODY_LINE_SPACING: float = 0.75
BODY_LINE_BUFF: float = 0.10


def apply_body_font() -> str:
    """✍️ Set Manim ``Text`` defaults: resolved face, line spacing, no ligatures, kerning fix."""
    from manim import Text

    install_text_kerning_fix()
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
# Nudged up again on the small end (label 17→19, body 20→22, subtitle 21→22,
# caption 23→24) once the render-large-then-scale kerning fix above landed: with
# glyphs finally spaced evenly the small labels no longer read as "ragged", so
# the remaining complaint was plain size. Everything here still clears ~3 % of
# frame height and the layout bands in manim_visuals absorb the extra width via
# fit_band()/next_to()/arrange().
SUBTITLE_FONT_SIZE: float = 22   # 3.5 % — beat subtitle under the heading
BODY_FONT_SIZE: float = 22       # 3.5 % — standard on-screen label
LABEL_FONT_SIZE: float = 19      # 3.0 % — small callouts, unit tags, legend rows
FORMULA_FONT_SIZE: float = 31    # 4.9 % — the dedicated formula panel
CAPTION_FONT_SIZE: float = 24    # 3.8 % — bottom-edge German subtitle bar
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
