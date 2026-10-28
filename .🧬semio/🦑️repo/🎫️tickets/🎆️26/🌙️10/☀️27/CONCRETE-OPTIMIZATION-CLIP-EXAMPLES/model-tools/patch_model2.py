import sys
from pathlib import Path
p = Path(sys.argv[1]); s = p.read_text(encoding="utf-8")
def sub(old, new):
    global s
    assert s.count(old) == 1, old
    s = s.replace(old, new)
sub('''    half = {**base, a_id: a_levels[0], b_id: (b_levels[0] + b_levels[1]) / 2}
''', '''    at = lambda depth: evaluate({**centre, b_id: depth})[setup["response"]]
    full, half = at(b_levels[0]) - at(b_levels[1]), at(b_levels[0]) - at(sum(b_levels) / 2)
''')
sub('''            "half_overhang": {"design": half, "response": evaluate(half)[setup["response"]]}}''', '''            "centre_savings": {"half": half, "full": full, "share": half / full}}''')
p.write_text(s, encoding="utf-8")
