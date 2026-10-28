from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧪️tests/🐍️.py")
s = p.read_text(encoding="utf-8")
old = '''        local = [(a, b) for a in range(41) for b in range(41)
                 if grid[a, b] + .05 < np.delete(padded[a:a + 3, b:b + 3].ravel(), 4).min()]'''
assert old in s
s = s.replace(old, '''        local = [(a, b) for a in range(1, 40) for b in range(1, 40)
                 if grid[a, b] <= np.delete(padded[a:a + 3, b:b + 3].ravel(), 4).min()]''')
p.write_text(s, encoding="utf-8", newline="\n")
