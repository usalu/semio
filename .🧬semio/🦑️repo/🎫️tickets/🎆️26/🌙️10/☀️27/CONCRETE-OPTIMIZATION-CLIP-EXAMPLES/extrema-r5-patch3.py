import sys
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py"
s = open(p, encoding="utf-8").read()


def swap(old, new, count=1):
    global s
    assert s.count(old) == count, (s.count(old), old[:90])
    s = s.replace(old, new)


swap('"Turning toward south, the low winter sun heats the room for free: heating electricity falls."', '"Turning south, the low winter sun heats the room for free, so heating electricity falls."')
swap('"We minimize it: lower is better. The values come from a simplified annual teaching model."', '"We minimize it, so lower is better. The values come from a simplified teaching model of one year."')
swap('Its value, E at θ*, is the electricity in kWh/a."', 'Its value, E at θ*, is the annual electricity, in kWh/a."')
swap('"Bei 450° zeigt die Fassade wieder nach Osten, die Kurve endet also, wo sie begann."', '"Bei 450° zeigt die Fassade wieder nach Osten, daher endet die Kurve am Anfangswert."')
swap('        answer = _mn.Circle(radius=.2, color=GOOD, stroke_width=3)', '        ripple = self._l("ripples: hourly sun samples", "Wellen: stündliche Sonnenstände", (0, -2.5, 0), size=14, color=MUTED, width=3.4)\n        answer = _mn.Circle(radius=.2, color=GOOD, stroke_width=3)')
swap('*[_mn.FadeIn(item) for item in (*lines, *dots, *tiles, *notes)], _mn.FadeIn(strip)', '*[_mn.FadeIn(item) for item in (*lines, *dots, *tiles, *notes)], _mn.FadeIn(strip), _mn.FadeIn(ripple)')
swap('*[_mn.FadeOut(item) for item in (*lines[1:], *dots[1:], *tiles[1:], *notes[1:], strip)]', '*[_mn.FadeOut(item) for item in (*lines[1:], *dots[1:], *tiles[1:], *notes[1:], strip, ripple)]')
open(p, "w", encoding="utf-8").write(s)
print("patched3")
