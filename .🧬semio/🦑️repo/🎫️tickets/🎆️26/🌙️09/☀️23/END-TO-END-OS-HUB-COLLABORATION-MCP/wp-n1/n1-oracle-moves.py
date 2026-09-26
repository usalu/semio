"""N1 one-off codemod: moves the compliance/schema oracle scripts that sat inside norm test-case folders into their subset's `🔮️oracles/` owner and rewrites every reference; removes the generated `oracle-report.json` and the scenario-less en1994 stub feature."""
import os, shutil
A = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/"
S = "/🏅️standards/🔖️1/🪆️subsets/✳️any/"
MOVES = [
    ("🌬️din16798", "🧬️schema/🧪️tests/⚖️compliance/validate_snapshot.py", "🔮️oracles/🧬️snapshot-schema/🐍️.py"),
    ("🧱️din4108", "🧬️schema/🧪️tests/⚖️compliance/validate_snapshot.py", "🔮️oracles/🧬️snapshot-schema/🐍️.py"),
    ("🏭️vdi3805", "🧪️tests/⚖️compliance-vdi3805-1/validate_schema.py", "🔮️oracles/🧬️snapshot-schema/🐍️.py"),
    ("🔩️en1993", "🧪️tests/⚖️compliance-oracle/validate_snapshot.py", "🔮️oracles/🧬️snapshot-schema/🐍️.py"),
    ("🔩️en1993", "🧪️tests/⚖️compliance-oracle/🐍️.py", "🔮️oracles/⚖️compliance/🐍️.py"),
    ("🧩️en1994", "🧪️tests/⚖️compliance-oracle/validate_snapshot.py", "🔮️oracles/🧬️snapshot-schema/🐍️.py"),
    ("🧩️en1994", "🧪️tests/⚖️compliance-oracle/🐍️.py", "🔮️oracles/⚖️compliance/🐍️.py"),
]
REWRITES = [
    ("🌬️din16798", "🧬️schema/🧪️tests/⚖️compliance/🦀️.rs", '"🧬️schema/🧪️tests/⚖️compliance/validate_snapshot.py"', '"🔮️oracles/🧬️snapshot-schema/🐍️.py"'),
    ("🧱️din4108", "🧬️schema/🧪️tests/⚖️compliance/🦀️.rs", '"🧬️schema/🧪️tests/⚖️compliance/validate_snapshot.py"', '"🔮️oracles/🧬️snapshot-schema/🐍️.py"'),
    ("🏭️vdi3805", "🧪️tests/⚖️compliance-vdi3805-1/🦀️.rs", 'oracle_dir().join("validate_schema.py")', 'oracle_dir().join("../../🔮️oracles/🧬️snapshot-schema/🐍️.py")'),
    ("🔩️en1993", "🧬️schema/🧪️tests/⚖️compliance/🦀️.rs", '"🧪️tests/⚖️compliance-oracle/validate_snapshot.py"', '"🔮️oracles/🧬️snapshot-schema/🐍️.py"'),
    ("🔩️en1993", "🧬️schema/🧪️tests/⚖️compliance/🦀️.rs", '"🧪️tests/⚖️compliance-oracle/🐍️.py"', '"🔮️oracles/⚖️compliance/🐍️.py"'),
    ("🧩️en1994", "🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs", '.join("🧪️tests").join("⚖️compliance-oracle").join("🐍️.py")', '.join("🔮️oracles").join("⚖️compliance").join("🐍️.py")'),
]
for fam, src, dst in MOVES:
    s, d = A + fam + S + src, A + fam + S + dst
    os.makedirs(os.path.dirname(d), exist_ok=True)
    assert not os.path.exists(d), d
    shutil.move(s, d); print("moved", fam, src, "->", dst)
for fam, rel, old, new in REWRITES:
    p = A + fam + S + rel
    t = open(p, encoding="utf-8").read()
    assert old in t, (p, old)
    open(p, "w", encoding="utf-8").write(t.replace(old, new)); print("rewrote", fam, rel)
os.remove(A + "🔩️en1993" + S + "🧪️tests/⚖️compliance-oracle/oracle-report.json")
os.remove(A + "🧩️en1994" + S + "🧪️tests/⚖️compliance-oracle/🥒️.feature")
for fam in ("🔩️en1993", "🧩️en1994"):
    d = A + fam + S + "🧪️tests/⚖️compliance-oracle"
    print(fam, "left in case:", os.listdir(d))
    if not os.listdir(d): os.rmdir(d)
