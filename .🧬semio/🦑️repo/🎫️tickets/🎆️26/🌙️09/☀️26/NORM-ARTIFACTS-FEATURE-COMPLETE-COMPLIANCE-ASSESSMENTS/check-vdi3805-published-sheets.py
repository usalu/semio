"""Compare publisher scope fixtures with the production VDI 3805 sheet registry."""

import csv
import os
from pathlib import Path
import sqlite3
import subprocess

TICKET = Path(__file__).resolve().parent
ROOT = next(p.parent for p in TICKET.parents if p.name == ".🧬semio")
SCHEMA = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"
RUST = SCHEMA / "📸️snapshot/🗂️sheet-registry/🦀️.rs"
FIXTURE = SCHEMA / "💡️inferences/🧪️tests/🔬️unit/🧫️fixtures/published-sheet-scopes.tsv"
OUTPUT = TICKET / "🗑️generated"


def main():
    with FIXTURE.open(newline="", encoding="utf-8") as stream:
        fixture = [(int(part), int(year), int(month), de, en)
                   for part, year, month, de, en in list(csv.reader(stream, delimiter="\t"))[1:]]
    source = RUST.read_text(encoding="utf-8")
    registry = source[source.index("pub const SHEET_ENTRIES:"):]
    prelude = """
#![allow(dead_code)]
pub struct SheetId(u16);
pub struct EditionId { year: u16, month: u8 }
impl EditionId {
    const fn new(year: u16, month: u8) -> Self { Self { year, month } }
}
enum SchemaStatus { Published, Checked, Draft, Project, Withdrawn, Superseded, HistoricalProposal, Reserved }
enum Domain { Heating, Ventilation, Sanitary, BuildingAutomation, Electrical, Generic }
pub struct SheetEntry {
    id: SheetId,
    title_de: &'static str,
    title_en: &'static str,
    status: SchemaStatus,
    domains: &'static [Domain],
    part1_edition: EditionId,
    current_edition: EditionId,
}
"""
    driver = """
fn main() {
    for entry in SHEET_ENTRIES {
        let part = entry.id.0;
        let EditionId { year, month } = entry.current_edition;
        println!("{part}\t{year}\t{month}\t{}\t{}", entry.title_de, entry.title_en);
    }
}
"""
    OUTPUT.mkdir(exist_ok=True)
    harness = OUTPUT / "vdi3805-sheet-scopes.rs"
    executable = OUTPUT / ("vdi3805-sheet-scopes.exe" if os.name == "nt" else "vdi3805-sheet-scopes")
    harness.write_text(prelude + registry + driver, encoding="utf-8")
    subprocess.run(["rustc", "--edition=2024", str(harness), "-o", str(executable)], check=True)
    result = subprocess.check_output([str(executable)], text=True).splitlines()
    actual = [(int(part), int(year), int(month), de, en)
              for part, year, month, de, en in (row.split("\t") for row in result)]
    with sqlite3.connect(":memory:") as db:
        db.execute("CREATE TABLE publication (id INTEGER PRIMARY KEY, year INTEGER, month INTEGER, de TEXT, en TEXT)")
        db.executemany("INSERT INTO publication VALUES (?,?,?,?,?)", actual)
        for part, *expected in fixture:
            assert tuple(expected) == db.execute("SELECT year, month, de, en FROM publication WHERE id=?", (part,)).fetchone(), part
    print(f"[DEBUG] {len(fixture)} VDI 3805 published scopes matched the production Rust registry and SQLite.")


if __name__ == "__main__":
    main()
