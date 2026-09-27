"""N1 one-off codemod: one location for every norm family's independent evaluate/compliance oracle —
`🔮️oracles/⚖️compliance/🐍️.py` (iso16757's, en1993's and en1994's already live there) — and the Rust parity tests that
spawn it rewritten to that path; the stub "test cases" that only held these scripts are removed."""
import os, shutil
A = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/"
S = "/🏅️standards/🔖️1/🪆️subsets/✳️any/"
TARGET = "🔮️oracles/⚖️compliance/🐍️.py"
MOVES = [
    ("⚡️din18599", "🧪️tests/⚡️balance-din18599-1/🐍️.py", [("🧬️schema/🧪️tests/🔬️oracle/🦀️.rs", '"🧪️tests/⚡️balance-din18599-1/🐍️.py"', f'"{TARGET}"')]),
    ("🌍️en1997", "🧪️tests/🌍️compliance-en1997-1/🐍️.py", [("🧬️schema/🧪️tests/⚖️compliance/🦀️.rs", '"🧪️tests/🌍️compliance-en1997-1/🐍️.py"', f'"{TARGET}"')]),
    ("🪨️en1996", "🧪️tests/⚖️evaluate-en1996-1/🐍️.py", [("🧬️schema/🧪️tests/🔬️oracle/🦀️.rs", "🧪️tests/⚖️evaluate-en1996-1/🐍️.", "🔮️oracles/⚖️compliance/🐍️.")]),
    ("🪵️en1995", "🔮️oracles/🐍️evaluate.py", [("🧬️schema/🧪️tests/⚖️compliance/🦀️.rs", '.join("🔮️oracles").join("🐍️evaluate.py")', '.join("🔮️oracles").join("⚖️compliance").join("🐍️.py")')]),
    ("🪶️en1999", "🔮️oracles/🐍️evaluate.py", [("🧬️schema/🧪️tests/⚖️compliance/🦀️.rs", '.join("🔮️oracles").join("🐍️evaluate.py")', '.join("🔮️oracles").join("⚖️compliance").join("🐍️.py")')]),
    ("🏋️en1991", "🔮️oracles/evaluate_en1991.py", [("🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs", '.join("🔮️oracles").join("evaluate_en1991.py")', '.join("🔮️oracles").join("⚖️compliance").join("🐍️.py")')]),
    ("🧱️din4108", "🔮️oracles/🐍️.py", [("🧬️schema/🧪️tests/⚖️compliance/🦀️.rs", '.join("🔮️oracles/🐍️.py")', f'.join("{TARGET}")')]),
    ("🫨️en1998", "🔮️oracles/🐍️.py", [("🧬️schema/🧪️tests/⚖️compliance/🦀️.rs", '.join("🔮️oracles/🐍️.py")', f'.join("{TARGET}")')]),
]
for family, source, rewrites in MOVES:
    base = A + family + S
    target = base + TARGET
    assert not os.path.exists(target), target
    for rel, old, new in rewrites:
        text = open(base + rel, encoding="utf-8").read()
        assert old in text, (family, rel, old)
        open(base + rel, "w", encoding="utf-8").write(text.replace(old, new))
    os.makedirs(os.path.dirname(target), exist_ok=True)
    shutil.move(base + source, target)
    print("moved", family, source, "->", TARGET)
for family, case in (("⚡️din18599", "🧪️tests/⚡️balance-din18599-1"), ("🌍️en1997", "🧪️tests/🌍️compliance-en1997-1"), ("🪨️en1996", "🧪️tests/⚖️evaluate-en1996-1"), ("🪵️en1995", "🧪️tests/⚖️evaluate-en1995-1")):
    shutil.rmtree(A + family + S + case)
    print("removed stub case", family, case)
