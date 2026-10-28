"""📐 Round 4 layout edits of the algorithms scene (scratch copy, then copied into place)."""
import sys
from pathlib import Path
src = Path(sys.argv[1]).read_text(encoding="utf-8")
for old, new in (
    ('''        static = mn.VGroup(mn.Line(base + np.array([-.6, 0, 0]), base + np.array([.6, 0, 0]), color=MUTED, stroke_width=2),
                           mn.Line(base + np.array([-width / 2 - .3, 0, 0]), base + np.array([-width / 2 - .3, height, 0]), color=MUTED, stroke_width=1.2, stroke_opacity=.6),
                           mn.Line(base + np.array([-width / 2 - .3, height, 0]), base + np.array([-width / 2 - .2, height, 0]), color=MUTED, stroke_width=1.2, stroke_opacity=.6))''',
     '''        static = mn.VGroup(mn.Line(base + np.array([-.6, 0, 0]), base + np.array([.6, 0, 0]), color=MUTED, stroke_width=2))'''),
    ('''self.heat["star"].get_center() + mn.UP * .3, size=14, color=GOOD, width=1.7))''', '''self.heat["star"].get_center() + mn.UP * .45 + mn.LEFT * .3, size=14, color=GOOD, width=1.5))'''),
    ('''ax.c2p(budget * .25, low + (minimum - low) * .33), color=GOOD, size=14, width=3.2)''', '''ax.c2p(0, low + (minimum - low) * .33) + mn.RIGHT * 1.75, color=GOOD, size=14, width=3.2)'''),
    ('''lens + mn.DOWN * .82, size=14, color=INK))''', '''lens + mn.UP * .85, size=14, color=INK))'''),
    ('''(CARD_X - 1.25, .3, 0), width=2.2''', '''(CARD_X - 1.25, .4, 0), width=2.2'''),
    ('''(CARD_X + 1.25, .3, 0), width=2.2''', '''(CARD_X + 1.25, .4, 0), width=2.2'''),
    ('''(CARD_X, -.72, 0), size=18, width=6.7)''', '''(CARD_X, -.58, 0), size=18, width=6.7)'''),
    ('''(CARD_X, -1.05, 0), color=GOOD, width=6.2, size=14)''', '''(CARD_X, -1.05, 0), color=GOOD, width=6.8, size=14)'''),
):
    assert src.count(old) == 1, old[:70]
    src = src.replace(old, new)

Path(sys.argv[2]).write_text(src, encoding="utf-8")
