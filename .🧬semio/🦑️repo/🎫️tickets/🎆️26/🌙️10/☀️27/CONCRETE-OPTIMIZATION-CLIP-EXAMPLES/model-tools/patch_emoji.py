import re
from pathlib import Path
clip = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
def apply(path, mapping, add=None):
    s = path.read_text(encoding="utf-8")
    for kind, name, emoji in mapping:
        pattern = re.compile(r'(%s %s\b[^\n]*:\n\s+""")[^ \n]+ ' % (kind, re.escape(name)))
        s, n = pattern.subn(lambda m: m.group(1) + emoji + " ", s, count=1)
        assert n == 1, name
    for old, new in (add or []):
        assert s.count(old) == 1, old
        s = s.replace(old, new)
    path.write_text(s, encoding="utf-8")
apply(clip / "🧮️model/🐍️.py", [
    ("def", "_design", "🧰"), ("def", "_pick", "🥇"), ("def", "_orientation_study", "🔄"), ("def", "_surface", "🗻"), ("def", "_samples", "🎰"),
    ("def", "_factorial", "🧫"), ("def", "_fractional", "🪓"), ("def", "_variants", "🏘️"), ("def", "weighted_pick", "🧲"), ("def", "_pareto", "🏅"),
    ("def", "_genetic", "🦠"), ("def", "_gradient_path", "⛷️")],
    [("    def wrapper(*arguments):\n", '    def wrapper(*arguments):\n        """🪞 Return an independent copy of the cached result."""\n'),
     ("    def draw():\n", '    def draw():\n        """🎡 Advance the recorded Park–Miller generator and return a draw in (0, 1)."""\n'),
     ("    def individual(identifier, genes):\n", '    def individual(identifier, genes):\n        """👤 One evaluated member of a population."""\n'),
     ("        def tournament():\n", '        def tournament():\n            """🤼 Draw two contenders at random and let the lower electricity win."""\n')])
apply(clip / "🧪️tests/🐍️.py", [("class", "FactorialExperiments", "🔢"), ("class", "SchemaFirstStudy", "📜")])
