from pathlib import Path
import sqlite3
import subprocess

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
a = next((root/"✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108").glob("🏅*/🔖*/🪆*/✳*"))
fixtures = (a/"🧪️tests/🧫️fixtures/💧️condensate-materials/🔣️.tsv").read_text().splitlines()
db = sqlite3.connect(":memory:")
for row in fixtures:
    v,w,limited,resists,timber,products,cap,status = map(float,row.split())
    observed = db.execute("""
        WITH evidence(valid) AS (VALUES (:w=1 AND min(:timber,:products)>=0)),
        limits AS (SELECT valid, CASE WHEN valid AND :limited=1 THEN 0.5 ELSE 1 END AS cap FROM evidence)
        SELECT cap, CASE
            WHEN :m<0 THEN -1 WHEN :m=0 THEN 1 WHEN :m>cap THEN 0
            WHEN valid THEN :r=1 AND :timber<=5 AND :products<=3 ELSE -1 END
        FROM limits
    """,dict(m=v,w=w,limited=limited,r=resists,timber=timber,products=products)).fetchone()
    assert observed == (cap,status), (row, observed)

source = a/"🧬️schema/💡️inferences/⚖️checks/💧️part-3-moisture-protection/📏️material-limits/🦀️.rs"
binary = ticket/"🗑️generated/din4108-material-limits"
subprocess.run(["repo","tools","exec","--light","rustup","--","run","stable","rustc","--edition","2024","--test","--crate-name","din4108_material_limits_test",str(source),"-o",str(binary)],check=True,cwd=root)
subprocess.run([binary],check=True,cwd=root)
print(f"[DEBUG] DIN 4108-3 material verification {len(fixtures)} cases matched SQLite and Rust.")
