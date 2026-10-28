import pathlib

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert old in s, old[:80]
    s = s.replace(old, new)


sub('''shift(_engine.LEFT * 0.35 + _engine.DOWN * 0.25)''', '''shift(_engine.RIGHT * 0.35 + _engine.DOWN * 0.62)''')
sub('''ties = VGroup(segments([((RIGHT, 0.85 + 0.4, 0), (RIGHT, 1.4, 0)), ((RIGHT, 0.85 - 0.4, 0), (RIGHT, 0.3, 0))], DATA, 1.4))''',
    '''ties = VGroup(segments([((RIGHT, 0.85 + 0.4, 0), (RIGHT, 1.4, 0))], DATA, 1.4), segments([((RIGHT, 0.85 - 0.4, 0), (RIGHT, 0.3, 0))], DATA, 1.4))''')
sub('''zoom = 2.5 / (ring_radius + distance + piloti["radius"])''', '''zoom = 2.0 / (ring_radius + distance + piloti["radius"])''')
sub('''mark = np.array([centre[0] - radius + 0.5, middle[1] + 0.5, 0.0])''', '''mark = np.array([centre[0] - radius + 0.55, middle[1] + 0.55, 0.0])''')
sub('''(-2.5, y - 0.36 - number * 0.23, 0)''', '''(-2.5, y - 0.36 - number * 0.27, 0)''')
p.write_text(s, encoding="utf-8")
print("ok")
