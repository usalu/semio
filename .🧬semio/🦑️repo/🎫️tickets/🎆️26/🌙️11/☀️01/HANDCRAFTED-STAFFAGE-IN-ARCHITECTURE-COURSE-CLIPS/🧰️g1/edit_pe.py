import time
p = "🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🏭️primary-energy/🐍️.py"
s = open(p, encoding="utf8").read()
i = s.index("def _derrick(base, h=0.85):")
j = s.index("def _power_plant")
new = '''def _derrick(base, h=0.85):
    """🛢️ Oil pumpjack on the ground at ``base``, ``h`` tall; its walking beam nods through scene time."""
    rig = fitted_staffage("demand-primary-energy-pumpjack", width=1.6 * h, height=h, rate=1.0, loop=True)
    return rig.move_to(_pt(base) + UP * rig.height / 2)


'''
s = s[:i] + new + s[j:]
for _ in range(8):
    try:
        open(p, "w", encoding="utf8", newline="").write(s)
        break
    except OSError:
        time.sleep(1)
print("ok")
