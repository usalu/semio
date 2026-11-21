"""Compare production EN 1991 load reductions with independent SQLite queries."""

from pathlib import Path
import json
import math
import os
import sqlite3
import subprocess


TICKET = Path(__file__).resolve().parent
ROOT = next(p.parent for p in TICKET.parents if p.name == ".🧬semio")
NORM = ROOT / "✏️s/🔌️plugins/📕️norm"
ARTIFACT = NORM / "🗿️artifacts/🏋️en1991"
PARTS = next(p / "🦀️.rs" for p in ARTIFACT.rglob("🧱️parts") if p.parent.name == "⚖️checks")
NDP = NORM / "⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json"
OUTPUT = TICKET / "🗑️generated"


def main():
    area_rows = [(category, area, storeys) for category in ("A1", "B1", "C1", "D1", "E1.1", "E1.2", "H", "Z")
                 for area in (10.0, 20.0, 40.0, 200.0) for storeys in (2, 3, 5, 20)]
    snow_rows = [(v, p) for v in (0.0, 0.2, 0.4, 0.8) for p in (0.0, 0.01, 0.02, 0.10, 0.20, 0.21, 0.5)]
    rust = PARTS.read_text(encoding="utf-8")
    start, end = rust.index("    /// 📐 Area reduction", rust.index("pub mod part_1_1")) if "    /// 📐 Area reduction" in rust else rust.index("    /// 📐 Area factor"), rust.index("    /// 📆 Return-period factor")
    size = rust.index("    ///", rust.index("    pub fn alpha_n(", start) + 20)
    snow_end = rust.index("    ///", end + 12)
    ndp = json.loads(NDP.read_text(encoding="utf-8"))["entries"]
    values = [f'    "{row["id"]}" => {float(row["naDe"])!r},' for row in ndp
              if row["id"].startswith(("ndp.en1991.alpha-a.de-", "ndp.en1991.alpha-n.de-"))]
    prelude = """
#[derive(Clone, Copy)] enum AnnexChoice { En, De }
fn psi_0(_: &str) -> f64 { 0.7 }
fn ndp_number(id: &str, _: AnnexChoice) -> f64 {
match id {
""" + "\n".join(values) + """
    _ => panic!("unregistered {id}"),
}
}
"""
    area = ", ".join(f'("{name}", {area}, {storeys})' for name, area, storeys in area_rows)
    snow = ", ".join(f"({v}, {p})" for v, p in snow_rows)
    driver = f"""
fn main() {{
    let _ = AnnexChoice::En;
    for (category, area, storeys) in [{area}] {{
        println!("{{}} {{}}", alpha_a(category, area, AnnexChoice::De), alpha_n(category, storeys, AnnexChoice::De));
    }}
    for (v, p) in [{snow}] {{
        if let Some(factor) = return_period_factor(v, p) {{ println!("{{factor}}"); }}
        else {{ println!("unavailable"); }}
    }}
}}
"""
    source = OUTPUT / "en1991-load-reductions.rs"
    source.write_text(prelude + rust[start:size] + rust[end:snow_end] + driver, encoding="utf-8")
    executable = OUTPUT / ("en1991-load-reductions.exe" if os.name == "nt" else "en1991-load-reductions")
    subprocess.run(["rustc", "--edition=2024", str(source), "-o", str(executable)], check=True)
    lines = subprocess.check_output([str(executable)], text=True).splitlines()
    assert len(lines) == len(area_rows) + len(snow_rows)
    with sqlite3.connect(":memory:") as db:
        db.create_function("ln", 1, math.log)
        db.create_function("sqrt", 1, math.sqrt)
        for row, line in zip(area_rows, lines):
            expected = db.execute("""
                WITH inputs AS (SELECT ? category, ? area, ? n)
                SELECT MIN(1, CASE
                    WHEN category IN ('A1', 'B1', 'Z') THEN 0.5 + 10 / area
                    WHEN category IN ('C1', 'D1', 'E1.1') THEN 0.7 + 10 / area
                    ELSE 1 END),
                    CASE WHEN n > 2 AND category IN ('A1', 'B1', 'C1', 'D1', 'Z') THEN 0.7 + 0.6 / n ELSE 1 END
                    FROM inputs
            """, (row[0], row[1], float(row[2]))).fetchone()
            assert all(math.isclose(a, b, abs_tol=1e-12) for a, b in zip(map(float, line.split()), expected)), (row, line, expected)
        for row, line in zip(snow_rows, lines[len(area_rows):]):
            expected, = db.execute("""
                WITH inputs AS (SELECT ? v, ? p)
                SELECT CASE WHEN p > 0 AND p <= 0.2
                    THEN (1 - v * SQRT(6) / ? * (LN(-LN(1-p)) + 0.57722)) / (1 + 2.5923*v) END
                FROM inputs
            """, (*row, math.pi)).fetchone()
            assert (line == "unavailable" if expected is None else math.isclose(float(line), expected, abs_tol=1e-12)), (row, line, expected)
    print(f"[DEBUG] {len(area_rows)} German load-factor and {len(snow_rows)} snow-return-period cases matched Rust and SQLite.")


if __name__ == "__main__":
    main()
